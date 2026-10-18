//! Error type for the Marti client.

/// Convenience alias for results returned by this crate.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Errors from talking to the Marti REST API.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// The TLS settings are unusable.
    #[error(transparent)]
    Tls(#[from] cottak_net::Error),

    /// The HTTP request failed (connection, TLS verification, timeout, ...).
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// Not authenticated (HTTP 401): missing or rejected certificate or password.
    #[error("authentication required or rejected (HTTP 401)")]
    Unauthorized,

    /// Authenticated but not allowed (HTTP 403).
    #[error("permission denied (HTTP 403)")]
    Forbidden,

    /// The resource does not exist (HTTP 404).
    #[error("not found (HTTP 404)")]
    NotFound,

    /// The resource already exists (HTTP 409), for example a mission name in use.
    #[error("conflict: already exists (HTTP 409)")]
    Conflict,

    /// Any other unsuccessful status.
    #[error("unexpected HTTP status {status}: {body}")]
    Status {
        /// Status code.
        status: u16,
        /// Start of the response body.
        body: String,
    },

    /// The answer could not be understood.
    #[error("malformed server response: {0}")]
    BadResponse(String),

    /// A response or upload exceeded the configured size limit.
    #[error("size limit of {0} bytes exceeded")]
    TooLarge(usize),

    /// An argument was rejected before any request was made.
    #[error("invalid argument: {0}")]
    InvalidArgument(String),

    /// A CoT document returned by the server was invalid.
    #[error(transparent)]
    Cot(#[from] cottak_core::Error),
}
