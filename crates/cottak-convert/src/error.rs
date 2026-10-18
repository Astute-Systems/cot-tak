//! Errors for conversions.

/// Convenience alias for results returned by this crate.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Errors from converting between formats.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// The input is not the expected kind of JSON.
    #[error("invalid GeoJSON: {0}")]
    InvalidGeoJson(String),

    /// The input is not a usable KMZ archive.
    #[error("invalid KMZ: {0}")]
    InvalidKmz(String),

    /// The input is not a KML document.
    #[error("invalid KML: {0}")]
    InvalidKml(String),

    /// The input is valid but uses something this converter does not support.
    #[error("unsupported: {0}")]
    Unsupported(&'static str),

    /// The input exceeds a configured limit.
    #[error("input exceeds limit: {0}")]
    Limit(&'static str),

    /// A CoT model error (invalid coordinates, type, ...).
    #[error(transparent)]
    Core(#[from] cottak_core::Error),

    /// A symbol code is malformed.
    #[error("invalid symbol code `{0}`")]
    InvalidSidc(String),

    /// A type-catalog document could not be read.
    #[error("invalid type catalog: {0}")]
    Catalog(String),
}
