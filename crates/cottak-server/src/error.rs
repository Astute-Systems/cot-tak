//! Error type for the server.

/// Convenience alias for results returned by this crate.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Errors from configuring or starting a [`Server`](crate::Server).
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// A socket or file error.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// The TLS configuration is unusable (bad PEM, mismatched key, ...).
    #[error("invalid TLS configuration: {0}")]
    TlsConfig(String),

    /// The server configuration is unusable.
    #[error("invalid server configuration: {0}")]
    InvalidConfig(String),
}
