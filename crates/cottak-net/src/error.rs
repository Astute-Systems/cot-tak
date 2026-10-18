//! Error type for the network layer.

/// Convenience alias for results returned by this crate.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Errors from connecting to, or talking with, a TAK peer.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// A socket or file error.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// A CoT model or XML error.
    #[error(transparent)]
    Core(#[from] cottak_core::Error),

    /// A TAK Protocol framing or conversion error.
    #[error(transparent)]
    Proto(#[from] cottak_proto::Error),

    /// The TLS handshake or session failed.
    #[error("TLS error: {0}")]
    Tls(#[from] rustls::Error),

    /// The TLS configuration is unusable (missing CA, bad PEM, ...).
    #[error("invalid TLS configuration: {0}")]
    TlsConfig(String),

    /// The server address could not be parsed or resolved to a TLS name.
    #[error("invalid address `{0}`")]
    InvalidAddress(String),

    /// Connecting took longer than the configured timeout.
    #[error("connect timed out")]
    ConnectTimeout,

    /// Nothing was received within the read timeout; the peer is presumed dead.
    #[error("no data received within the read timeout")]
    ReadTimeout,

    /// A write could not complete within the write timeout.
    #[error("write timed out")]
    WriteTimeout,

    /// The peer did not answer a protocol upgrade request in time.
    #[error("protocol negotiation timed out")]
    NegotiationTimeout,

    /// A message exceeded the configured size limit.
    #[error("message exceeds the limit of {0} bytes")]
    MessageTooLarge(usize),

    /// The peer closed the connection.
    #[error("connection closed by peer")]
    Closed,

    /// The client was shut down.
    #[error("client has shut down")]
    Shutdown,
}
