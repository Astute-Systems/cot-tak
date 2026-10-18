//! An embeddable TAK Server.
//!
//! [`Server`] accepts TAK clients over plain TCP and mutual TLS, speaks both
//! legacy XML and TAK Protocol version 1 (with the standard negotiation),
//! answers keepalive pings, and routes Cursor on Target events between the
//! connected clients:
//!
//! * events go to every other client, or only to the callsigns named in a
//!   `<marti><dest callsign="..."/></marti>` element;
//! * the latest position of every entity is kept and replayed to new clients;
//! * a delete message (`t-x-d-d`) removes entities from that store.
//!
//! * [`ServerConfig::listen_marti`] adds a mock of the HTTPS API (Marti):
//!   missions with content, subscriptions and a change log, file upload and
//!   download, contacts, client endpoints and groups. Events addressed with
//!   `<marti><dest mission="..."/></marti>` go to the mission's subscribers
//!   and the mission keeps the latest state of each of its UIDs. The
//!   `cottak-marti` client works against it.
//!
//! It is meant for integration tests, simulators, and small isolated
//! networks. It is **not** a replacement for the official TAK Server: there
//! are no user accounts, no groups or channels, no persistence, and no
//! federation. A TLS listener built with [`ServerTlsConfig::mutual`]
//! authenticates clients by certificate, but every authenticated client sees
//! every other client's traffic, and the identity a client claims (UID,
//! callsign, mission `creatorUid`) is taken at its word. All state is held in
//! memory, bounded by [`ServerConfig`] limits.
//!
//! ```no_run
//! use cottak_server::{Server, ServerConfig, ServerEvent};
//!
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! let server = Server::start(ServerConfig::new().listen_tcp("127.0.0.1:8087")).await?;
//! println!("listening on {:?}", server.local_addrs());
//!
//! let mut events = server.subscribe();
//! while let Ok(event) = events.recv().await {
//!     if let ServerEvent::Event { event, .. } = event {
//!         println!("{} {}", event.uid, event.cot_type);
//!     }
//! }
//! server.shutdown().await;
//! # Ok(()) }
//! ```

mod config;
mod error;
mod hub;
mod marti;
mod registry;
mod session;

pub use config::{BasicAuth, ServerConfig, ServerTlsConfig};
pub use error::{Error, Result};
pub use hub::{ClientInfo, ServerEvent};

use config::ListenerKind;
use cottak_core::Event;
use hub::Hub;
use registry::Limits;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::sync::{broadcast, watch};
use tokio::task::{JoinHandle, JoinSet};
use tokio::time::{sleep, timeout};
use tokio_rustls::TlsAcceptor;
use tracing::{debug, warn};

/// A running TAK Server. Dropping it stops all listeners and disconnects all
/// clients; call [`shutdown`](Self::shutdown) to wait until that has happened.
#[derive(Debug)]
pub struct Server {
    addrs: Vec<SocketAddr>,
    hub: Arc<Hub>,
    shutdown: watch::Sender<bool>,
    tasks: Vec<JoinHandle<()>>,
}

impl std::fmt::Debug for Hub {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Hub").finish_non_exhaustive()
    }
}

impl Server {
    /// Validates `config`, binds every listener and starts serving on the
    /// current Tokio runtime.
    pub async fn start(config: ServerConfig) -> Result<Self> {
        config.validate()?;

        // Validate all TLS material before binding anything.
        let mut acceptors = Vec::new();
        for l in &config.listeners {
            acceptors.push(match &l.tls {
                Some(tls) => Some(TlsAcceptor::from(tls.server_config()?)),
                None => None,
            });
        }
        let mut listeners = Vec::new();
        for l in &config.listeners {
            listeners.push(TcpListener::bind(&l.addr).await?);
        }
        let addrs = listeners
            .iter()
            .map(TcpListener::local_addr)
            .collect::<std::io::Result<Vec<_>>>()?;

        let hub = Arc::new(Hub::new(
            config.client_queue,
            config.max_clients,
            config.max_stored_events,
            config.replay_on_connect,
            config.observer_queue,
            Limits {
                missions: config.max_missions,
                files: config.max_files,
                bytes: config.max_store_bytes,
                uids_per_mission: 10_000,
            },
        ));
        let config = Arc::new(config);
        let (shutdown, shutdown_rx) = watch::channel(false);
        let tasks = listeners
            .into_iter()
            .zip(acceptors)
            .zip(config.listeners.iter().map(|l| l.kind).collect::<Vec<_>>())
            .map(|((listener, acceptor), kind)| {
                tokio::spawn(accept_loop(
                    listener,
                    acceptor,
                    kind,
                    Arc::clone(&hub),
                    Arc::clone(&config),
                    shutdown_rx.clone(),
                ))
            })
            .collect();
        Ok(Self {
            addrs,
            hub,
            shutdown,
            tasks,
        })
    }

    /// The bound address of every listener, in the order they were added.
    /// Useful when a listener was configured with port 0.
    pub fn local_addrs(&self) -> &[SocketAddr] {
        &self.addrs
    }

    /// Currently connected clients.
    pub fn clients(&self) -> Vec<ClientInfo> {
        self.hub.clients()
    }

    /// The latest non-stale position of every known entity.
    pub fn stored_events(&self) -> Vec<Arc<Event>> {
        self.hub.stored()
    }

    /// Observes connections and every routed event.
    ///
    /// A receiver that falls behind skips events
    /// ([`RecvError::Lagged`](broadcast::error::RecvError::Lagged)).
    pub fn subscribe(&self) -> broadcast::Receiver<ServerEvent> {
        self.hub.subscribe()
    }

    /// Sends an event as if it came from the server itself: to every client,
    /// or to the callsigns in its `<marti>` element.
    pub fn broadcast(&self, event: Event) {
        self.hub.ingest(None, event);
    }

    /// Stops accepting, disconnects every client and waits for all tasks.
    pub async fn shutdown(self) {
        let _ = self.shutdown.send(true);
        for task in self.tasks {
            let _ = task.await;
        }
    }
}

async fn accept_loop(
    listener: TcpListener,
    acceptor: Option<TlsAcceptor>,
    kind: ListenerKind,
    hub: Arc<Hub>,
    config: Arc<ServerConfig>,
    mut shutdown: watch::Receiver<bool>,
) {
    let mut sessions = JoinSet::new();
    loop {
        tokio::select! {
            biased;
            _ = shutdown.changed() => break,
            Some(_) = sessions.join_next(), if !sessions.is_empty() => {}
            accepted = listener.accept() => {
                let (stream, peer) = match accepted {
                    Ok(x) => x,
                    Err(e) => {
                        // Typically out of file descriptors; back off instead of spinning.
                        warn!(error = %e, "accept failed");
                        sleep(Duration::from_millis(100)).await;
                        continue;
                    }
                };
                if sessions.len() >= config.max_clients {
                    debug!(%peer, "rejecting connection: server is full");
                    continue;
                }
                let _ = stream.set_nodelay(true);
                let (hub, config, shutdown) =
                    (Arc::clone(&hub), Arc::clone(&config), shutdown.clone());
                let acceptor = acceptor.clone();
                sessions.spawn(async move {
                    match acceptor {
                        None => serve(kind, stream, peer, false, hub, config, shutdown).await,
                        Some(acceptor) => {
                            match timeout(config.handshake_timeout, acceptor.accept(stream)).await {
                                Ok(Ok(tls)) => serve(kind, tls, peer, true, hub, config, shutdown).await,
                                Ok(Err(e)) => debug!(%peer, error = %e, "TLS handshake failed"),
                                Err(_) => debug!(%peer, "TLS handshake timed out"),
                            }
                        }
                    }
                });
            }
        }
    }
    // Sessions end on their own when they see the shutdown signal.
    while sessions.join_next().await.is_some() {}
}

async fn serve<S>(
    kind: ListenerKind,
    io: S,
    peer: SocketAddr,
    tls: bool,
    hub: Arc<Hub>,
    config: Arc<ServerConfig>,
    shutdown: watch::Receiver<bool>,
) where
    S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin,
{
    match kind {
        ListenerKind::Tak => session::run(io, peer, tls, hub, config, shutdown).await,
        ListenerKind::Marti => marti::serve(io, tls, hub, config, shutdown).await,
    }
}
