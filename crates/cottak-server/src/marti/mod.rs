//! A mock of TAK Server's HTTPS API (Marti): missions, file sync, contacts and groups.
//!
//! It is served on listeners added with
//! [`ServerConfig::listen_marti`](crate::ServerConfig::listen_marti) and shares its state
//! with the streaming side: contacts come from the connected clients, mission
//! subscribers receive events addressed to their mission, and a mission's
//! `cot` document holds the latest known event of each of its UIDs.

mod http;
mod json;
mod multipart;
mod routes;

use crate::config::ServerConfig;
use crate::hub::Hub;
use http::{ReadError, Reader, Response};
use std::sync::Arc;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::sync::watch;
use tokio::time::timeout;

/// Serves HTTP requests on one connection until it closes or the server shuts down.
pub(crate) async fn serve<S>(
    io: S,
    tls: bool,
    hub: Arc<Hub>,
    config: Arc<ServerConfig>,
    mut shutdown: watch::Receiver<bool>,
) where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let mut reader = Reader::new(io, config.marti_max_body);
    loop {
        let read = tokio::select! {
            biased;
            _ = shutdown.changed() => return,
            r = timeout(config.marti_request_timeout, reader.read_request()) => r,
        };
        let req = match read {
            Ok(Ok(req)) => req,
            Ok(Err(ReadError::Refused(status, why))) => {
                let _ = timeout(
                    config.write_timeout,
                    reader.write_response(&Response::text(status, why), false),
                )
                .await;
                return;
            }
            // Closed, I/O error or an idle keep-alive connection.
            Ok(Err(ReadError::Closed | ReadError::Io)) | Err(_) => return,
        };
        let keep_alive = req.keep_alive;
        let ctx = routes::Context {
            hub: &hub,
            config: &config,
            tls,
        };
        let response = routes::handle(&ctx, &req);
        let sent = timeout(
            config.write_timeout,
            reader.write_response(&response, keep_alive),
        )
        .await;
        if !keep_alive || !matches!(sent, Ok(Ok(()))) {
            return;
        }
    }
}
