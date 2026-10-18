//! The TAK Server client: connect, upgrade to protobuf, keep alive, reconnect.

use crate::config::{ClientConfig, ProtocolPreference};
use crate::connection::{Connection, Incoming, Io, Protocol};
use crate::error::{Error, Result};
use crate::tls::{server_name, TlsConfig};
use cottak_core::messages::control;
use cottak_core::Event;
use cottak_proto::negotiate;
use rustls::pki_types::ServerName;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::sync::{mpsc, watch, Mutex};
use tokio::task::JoinHandle;
use tokio::time::{interval_at, sleep_until, timeout, Instant, MissedTickBehavior};
use tokio_rustls::TlsConnector;
use tracing::{debug, trace, warn};

/// What the client reports to the application.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum ClientEvent {
    /// A connection is up. Outgoing events queued while disconnected are now sent.
    Connected {
        /// Protocol in use when the connection came up (always XML at first).
        protocol: Protocol,
    },
    /// The server accepted a protocol upgrade.
    ProtocolChanged(Protocol),
    /// A CoT event from the server.
    Event(Box<Event>),
    /// A well-delimited message arrived that could not be understood. The
    /// connection is unaffected.
    Invalid {
        /// Why the message was rejected.
        reason: String,
    },
    /// The connection failed or was never established; a retry is scheduled.
    Disconnected {
        /// Human readable cause.
        reason: String,
        /// Delay before the next attempt.
        retry_in: Duration,
    },
}

/// Receives [`ClientEvent`]s. Dropping it shuts the client down.
#[derive(Debug)]
pub struct ClientEvents {
    rx: mpsc::Receiver<ClientEvent>,
}

impl ClientEvents {
    /// Waits for the next event; `None` once the client has stopped.
    ///
    /// The client stops reading from the server while this queue is full, so
    /// keep draining it.
    pub async fn recv(&mut self) -> Option<ClientEvent> {
        self.rx.recv().await
    }
}

struct Inner {
    tx: mpsc::Sender<Event>,
    shutdown: watch::Sender<bool>,
    task: Mutex<Option<JoinHandle<()>>>,
}

/// Sends events to the server and controls the client. Cheap to clone.
#[derive(Clone)]
pub struct ClientHandle {
    inner: Arc<Inner>,
}

impl std::fmt::Debug for ClientHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClientHandle").finish_non_exhaustive()
    }
}

impl ClientHandle {
    /// Queues an event, waiting if the queue is full.
    ///
    /// Events queued while the connection is down are sent after reconnecting,
    /// unless they have gone stale and `drop_stale` is set.
    pub async fn send(&self, event: Event) -> Result<()> {
        self.inner.tx.send(event).await.map_err(|_| Error::Shutdown)
    }

    /// Queues an event without waiting; fails if the queue is full.
    pub fn try_send(&self, event: Event) -> Result<()> {
        self.inner.tx.try_send(event).map_err(|_| Error::Shutdown)
    }

    /// Stops the client and waits for its task to finish.
    pub async fn shutdown(&self) {
        let _ = self.inner.shutdown.send(true);
        let task = self.inner.task.lock().await.take();
        if let Some(task) = task {
            let _ = task.await;
        }
    }
}

/// Entry point for connecting to a TAK Server.
#[derive(Debug)]
pub struct TakClient;

impl TakClient {
    /// Validates `config` and starts the client task on the current Tokio runtime.
    ///
    /// Connection problems are not errors here: the client keeps retrying with
    /// backoff and reports each failure as [`ClientEvent::Disconnected`].
    pub fn start(config: ClientConfig) -> Result<(ClientHandle, ClientEvents)> {
        let tls = config
            .tls
            .as_ref()
            .map(|t| prepare_tls(&config.addr, t))
            .transpose()?;
        crate::tls::host_of(&config.addr)?;

        let (tx, out_rx) = mpsc::channel(config.send_queue.max(1));
        let (ev_tx, ev_rx) = mpsc::channel(config.recv_queue.max(1));
        let (shutdown_tx, shutdown_rx) = watch::channel(false);

        let task = tokio::spawn(supervise(config, tls, out_rx, ev_tx, shutdown_rx));
        let handle = ClientHandle {
            inner: Arc::new(Inner {
                tx,
                shutdown: shutdown_tx,
                task: Mutex::new(Some(task)),
            }),
        };
        Ok((handle, ClientEvents { rx: ev_rx }))
    }
}

type Tls = (TlsConnector, ServerName<'static>);

fn prepare_tls(addr: &str, tls: &TlsConfig) -> Result<Tls> {
    Ok((tls.connector()?, server_name(addr, tls)?))
}

async fn supervise(
    config: ClientConfig,
    tls: Option<Tls>,
    mut out_rx: mpsc::Receiver<Event>,
    ev_tx: mpsc::Sender<ClientEvent>,
    mut shutdown: watch::Receiver<bool>,
) {
    let mut attempt: u32 = 0;
    loop {
        let result = session(
            &config,
            tls.as_ref(),
            &mut out_rx,
            &ev_tx,
            &mut shutdown,
            &mut attempt,
        )
        .await;
        let err = match result {
            Ok(()) => return,
            Err(e) => e,
        };
        let retry_in = config.backoff.delay(attempt);
        attempt = attempt.saturating_add(1);
        warn!(error = %err, ?retry_in, "TAK connection lost");
        let event = ClientEvent::Disconnected {
            reason: err.to_string(),
            retry_in,
        };
        if ev_tx.send(event).await.is_err() {
            return;
        }
        tokio::select! {
            () = tokio::time::sleep(retry_in) => {}
            _ = shutdown.changed() => return,
            () = ev_tx.closed() => return,
        }
    }
}

async fn connect(config: &ClientConfig, tls: Option<&Tls>) -> Result<Box<dyn Io>> {
    let attempt = async {
        let tcp = TcpStream::connect(&config.addr).await?;
        tcp.set_nodelay(true)?;
        let keepalive = socket2::TcpKeepalive::new().with_time(Duration::from_secs(60));
        socket2::SockRef::from(&tcp).set_tcp_keepalive(&keepalive)?;
        let io: Box<dyn Io> = match tls {
            Some((connector, name)) => Box::new(connector.connect(name.clone(), tcp).await?),
            None => Box::new(tcp),
        };
        Ok::<_, Error>(io)
    };
    timeout(config.connect_timeout, attempt)
        .await
        .map_err(|_| Error::ConnectTimeout)?
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum State {
    /// Talking XML; an upgrade may still be offered.
    Xml,
    /// Upgrade requested; sending is paused until the server answers.
    AwaitingResponse,
    /// Talking protobuf.
    Proto,
}

/// Runs one connection. `Ok(())` means the client was asked to stop.
async fn session(
    config: &ClientConfig,
    tls: Option<&Tls>,
    out_rx: &mut mpsc::Receiver<Event>,
    ev_tx: &mpsc::Sender<ClientEvent>,
    shutdown: &mut watch::Receiver<bool>,
    attempt: &mut u32,
) -> Result<()> {
    let io = tokio::select! {
        r = connect(config, tls) => r?,
        _ = shutdown.changed() => return Ok(()),
    };
    *attempt = 0;
    let mut conn = Connection::new(io, config.max_message_len);
    debug!(addr = %config.addr, "connected");
    if ev_tx
        .send(ClientEvent::Connected {
            protocol: Protocol::Xml,
        })
        .await
        .is_err()
    {
        return Ok(());
    }

    let mut state = State::Xml;
    let mut upgrade_deadline = Instant::now();
    let mut last_rx = Instant::now();
    let mut ping = interval_at(Instant::now() + config.ping_interval, config.ping_interval);
    ping.set_missed_tick_behavior(MissedTickBehavior::Delay);

    loop {
        let awaiting = state == State::AwaitingResponse;
        tokio::select! {
            biased;
            _ = shutdown.changed() => return Ok(()),
            () = ev_tx.closed() => return Ok(()),
            incoming = conn.read() => {
                last_rx = Instant::now();
                match incoming? {
                    Incoming::Event(ev) => {
                        match handle_incoming(config, &mut conn, &mut state, &mut upgrade_deadline, *ev).await? {
                            Forward::Drop => {}
                            Forward::Event(ev) => {
                                if ev_tx.send(ClientEvent::Event(ev)).await.is_err() {
                                    return Ok(());
                                }
                            }
                            Forward::Upgraded => {
                                if ev_tx.send(ClientEvent::ProtocolChanged(Protocol::V1)).await.is_err() {
                                    return Ok(());
                                }
                            }
                        }
                    }
                    Incoming::Control(c) => trace!(?c, "ignoring TakControl"),
                    Incoming::Invalid(reason) => {
                        if ev_tx.send(ClientEvent::Invalid { reason }).await.is_err() {
                            return Ok(());
                        }
                    }
                }
            }
            out = out_rx.recv(), if !awaiting => {
                let Some(ev) = out else { return Ok(()) };
                if config.drop_stale && ev.is_stale() {
                    debug!(uid = %ev.uid, "dropping stale outgoing event");
                    continue;
                }
                write(config, &mut conn, &ev).await?;
            }
            _ = ping.tick(), if !awaiting => {
                write(config, &mut conn, &control::ping(config.uid.clone())?).await?;
            }
            () = sleep_until(last_rx + config.read_timeout) => return Err(Error::ReadTimeout),
            () = sleep_until(upgrade_deadline), if awaiting => return Err(Error::NegotiationTimeout),
        }
    }
}

enum Forward {
    Drop,
    Event(Box<Event>),
    Upgraded,
}

async fn handle_incoming(
    config: &ClientConfig,
    conn: &mut Connection<Box<dyn Io>>,
    state: &mut State,
    upgrade_deadline: &mut Instant,
    ev: Event,
) -> Result<Forward> {
    // Keepalive traffic is internal.
    if control::is_pong(&ev) || control::is_ping(&ev) {
        return Ok(Forward::Drop);
    }
    if let Some(versions) = negotiate::parse_offer(&ev) {
        if *state == State::Xml
            && config.protocol == ProtocolPreference::PreferProtobuf
            && versions.contains(&1)
        {
            write(config, conn, &negotiate::request(&ev.uid, 1)?).await?;
            *state = State::AwaitingResponse;
            *upgrade_deadline = Instant::now() + config.negotiate_timeout;
        }
        return Ok(Forward::Drop);
    }
    if let Some(accepted) = negotiate::parse_response(&ev) {
        if *state == State::AwaitingResponse {
            if accepted {
                // Everything after the response is protobuf, including bytes already buffered.
                conn.set_protocol(Protocol::V1);
                *state = State::Proto;
                return Ok(Forward::Upgraded);
            }
            *state = State::Xml;
        }
        return Ok(Forward::Drop);
    }
    Ok(Forward::Event(Box::new(ev)))
}

async fn write(
    config: &ClientConfig,
    conn: &mut Connection<Box<dyn Io>>,
    ev: &Event,
) -> Result<()> {
    timeout(config.write_timeout, conn.write_event(ev))
        .await
        .map_err(|_| Error::WriteTimeout)?
}
