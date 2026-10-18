//! Error type for TAK Protocol handling.

/// Convenience alias for results returned by this crate.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Errors from encoding, decoding or framing TAK Protocol messages.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// A CoT model or XML error.
    #[error(transparent)]
    Core(#[from] cottak_core::Error),

    /// The protobuf payload could not be decoded.
    #[error("protobuf decode error: {0}")]
    Decode(#[from] prost::DecodeError),

    /// A frame did not start with the `0xbf` magic byte.
    #[error("bad magic byte 0x{0:02x}, expected 0xbf")]
    BadMagic(u8),

    /// A header varint was longer than 10 bytes.
    #[error("malformed varint in frame header")]
    BadVarint,

    /// A frame is larger than the configured limit.
    #[error("frame of {len} bytes exceeds the limit of {max}")]
    FrameTooLarge {
        /// Declared payload length.
        len: u64,
        /// Configured maximum.
        max: usize,
    },

    /// A mesh datagram used a protocol version this crate cannot decode.
    #[error("unsupported TAK protocol version {0}")]
    UnsupportedVersion(u64),

    /// A mesh datagram was truncated or malformed.
    #[error("malformed mesh datagram")]
    BadDatagram,

    /// A timestamp is outside the range the wire format can carry.
    #[error("timestamp out of range")]
    Timestamp,

    /// An I/O error from the underlying transport (codec only).
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
