//! Error types.

use std::fmt;

/// Convenience alias for results returned by this crate.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Errors produced while parsing, validating or serialising CoT data.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// The input was not well-formed XML.
    #[error("malformed XML: {0}")]
    Xml(String),

    /// The XML document exceeded a safety limit (size or nesting depth).
    #[error("XML input exceeds limit: {0}")]
    Limit(&'static str),

    /// The XML contained a construct that is rejected for security reasons
    /// (for example a `<!DOCTYPE>` declaration).
    #[error("forbidden XML construct: {0}")]
    Forbidden(&'static str),

    /// The root element was not `<event>`.
    #[error("expected <event> root element, found <{0}>")]
    NotAnEvent(String),

    /// A required attribute or element was missing.
    #[error("missing required {what} `{name}` on <{element}>")]
    Missing {
        /// Kind of thing that is missing (`attribute` or `element`).
        what: &'static str,
        /// Name of the missing attribute or element.
        name: &'static str,
        /// Element the item was expected on.
        element: &'static str,
    },

    /// An attribute value could not be parsed.
    #[error("invalid value `{value}` for `{name}` on <{element}>: {reason}")]
    InvalidValue {
        /// Element carrying the attribute.
        element: &'static str,
        /// Attribute name.
        name: &'static str,
        /// The offending value.
        value: String,
        /// Why it was rejected.
        reason: &'static str,
    },

    /// A timestamp was not valid CoT / RFC 3339 time.
    #[error("invalid timestamp `{0}`")]
    InvalidTime(String),

    /// A CoT type string was syntactically invalid.
    #[error("invalid CoT type `{0}`")]
    InvalidType(String),

    /// A collection has too few items.
    #[error("{what} needs at least {min} items, got {got}")]
    TooFew {
        /// What was being built.
        what: &'static str,
        /// Minimum number of items.
        min: usize,
        /// Number supplied.
        got: usize,
    },

    /// A value is outside its permitted range.
    #[error("{field} out of range: {value}")]
    OutOfRange {
        /// Field name.
        field: &'static str,
        /// The offending value.
        value: f64,
    },
}

impl Error {
    pub(crate) fn xml(e: impl fmt::Display) -> Self {
        Self::Xml(e.to_string())
    }

    pub(crate) fn invalid(
        element: &'static str,
        name: &'static str,
        value: &str,
        reason: &'static str,
    ) -> Self {
        Self::InvalidValue {
            element,
            name,
            value: value.to_owned(),
            reason,
        }
    }
}
