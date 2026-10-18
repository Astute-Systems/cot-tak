//! Error type for enrollment.

/// Convenience alias for results returned by this crate.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Errors from onboarding or certificate enrollment.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// An onboarding URL was malformed.
    #[error("invalid enrollment URL: {0}")]
    InvalidUrl(String),

    /// The TLS settings for talking to the enrollment service are unusable.
    #[error(transparent)]
    Tls(#[from] cottak_net::Error),

    /// The HTTP request failed (connection, TLS verification, timeout, ...).
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// The server rejected the credentials (HTTP 401 or 403).
    #[error("the server rejected the credentials (HTTP {0})")]
    Unauthorized(u16),

    /// The server answered with an unexpected status.
    #[error("unexpected HTTP status {status}: {body}")]
    Status {
        /// Status code.
        status: u16,
        /// Start of the response body, for diagnosis.
        body: String,
    },

    /// A response was larger than the safety limit.
    #[error("response exceeds {0} bytes")]
    ResponseTooLarge(usize),

    /// The server's answer could not be understood.
    #[error("malformed server response: {0}")]
    BadResponse(String),

    /// Key or certificate request generation failed.
    #[error("certificate request error: {0}")]
    Csr(#[from] rcgen::Error),

    /// The certificate returned by the server does not match our private key.
    #[error("the issued certificate does not match the generated private key")]
    KeyMismatch,

    /// A PKCS#12 file could not be read.
    #[error("PKCS#12 error: {0}")]
    Pkcs12(String),

    /// A file could not be written or read.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}
