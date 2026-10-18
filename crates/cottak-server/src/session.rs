//! One client connection.

use crate::config::ServerConfig;
use crate::hub::Hub;
use cottak_core::messages::control;
use cottak_core::Event;
use cottak_net::{Connection, Error, Incoming, Protocol, Result};
use cottak_proto::negotiate;
use std::collections::VecDeque;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::sync::watch;
use tokio::time::{sleep_until, timeout, Instant};
use tracing::{debug, warn};

/// Serves one accepted (and, for TLS, already handshaken) connection until it
/// ends, then removes it from the hub.
pub(crate) async fn run<S>(
    io: S,
    peer: SocketAddr,
    tls: bool,
    hub: Arc<Hub>,
    config: Arc<ServerConfig>,
    mut shutdown: watch::Receiver<bool>,
) where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let Some(registration) = hub.register(peer, tls) else {
        debug!(%peer, "rejecting connection: server is full");
        return;
    };
    let id = registration.id;
    debug!(%peer, id, tls, "client connected");
    let result = serve(io, registration, id, &hub, &config, &mut shutdown).await;
    hub.unregister(id);
    match result {
        Ok(()) | Err(Error::Closed) => debug!(%peer, id, "client disconnected"),
        Err(e) => warn!(%peer, id, error = %e, "client dropped"),
    }
}

async fn serve<S>(
    io: S,
    mut registration: crate::hub::Registration,
    id: u64,
    hub: &Hub,
    config: &ServerConfig,
    shutdown: &mut watch::Receiver<bool>,
) -> Result<()>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let mut conn = Connection::new(io, config.max_message_len);
    let mut pending: VecDeque<Arc<Event>> = std::mem::take(&mut registration.replay).into();

    if !config.protocol_versions.is_empty() {
        let offer = negotiate::offer(&config.server_uid, &config.protocol_versions)?;
        write(config, &mut conn, &offer).await?;
    }

    let mut last_rx = Instant::now();
    loop {
        tokio::select! {
            biased;
            _ = shutdown.changed() => return Ok(()),
            incoming = conn.read() => {
                last_rx = Instant::now();
                match incoming? {
                    Incoming::Event(ev) => handle_event(config, hub, id, &mut conn, *ev).await?,
                    Incoming::Control(c) => debug!(id, ?c, "ignoring TakControl"),
                    Incoming::Invalid(reason) => warn!(id, %reason, "ignoring invalid message"),
                }
            }
            Some(ev) = registration.rx.recv() => write(config, &mut conn, &ev).await?,
            () = std::future::ready(()), if !pending.is_empty() => {
                if let Some(ev) = pending.pop_front() {
                    write(config, &mut conn, &ev).await?;
                }
            }
            () = sleep_until(last_rx + config.read_timeout) => return Err(Error::ReadTimeout),
        }
    }
}

async fn handle_event<S>(
    config: &ServerConfig,
    hub: &Hub,
    id: u64,
    conn: &mut Connection<S>,
    ev: Event,
) -> Result<()>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    if control::is_ping(&ev) {
        return write(config, conn, &control::pong(&config.server_uid)?).await;
    }
    if control::is_pong(&ev) {
        return Ok(());
    }
    if let Some(version) = negotiate::parse_request(&ev) {
        let accepted =
            conn.protocol() == Protocol::Xml && config.protocol_versions.contains(&version);
        write(config, conn, &negotiate::response(&ev.uid, accepted)?).await?;
        if accepted {
            // Everything after our response, including bytes already
            // buffered, is protobuf.
            conn.set_protocol(Protocol::V1);
            hub.set_protocol(id, Protocol::V1);
        }
        return Ok(());
    }
    if negotiate::parse_offer(&ev).is_some() || negotiate::parse_response(&ev).is_some() {
        // Only servers offer or answer; ignore a confused peer.
        return Ok(());
    }
    hub.ingest(Some(id), ev);
    Ok(())
}

async fn write<S>(config: &ServerConfig, conn: &mut Connection<S>, ev: &Event) -> Result<()>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    timeout(config.write_timeout, conn.write_event(ev))
        .await
        .map_err(|_| Error::WriteTimeout)?
}
