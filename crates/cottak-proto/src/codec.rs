//! `tokio_util` codec for TAK Protocol streaming connections.

use crate::error::Error;
use crate::frame::{self, DEFAULT_MAX_FRAME_LEN};
use crate::TakFrame;
use bytes::BytesMut;
use cottak_core::Event;
use tokio_util::codec::{Decoder, Encoder};

/// Frames [`TakFrame`]s as protobuf streaming messages (`0xbf <len> <payload>`).
///
/// Use after [`negotiate`](crate::negotiate) has switched the connection to
/// protocol version 1.
#[derive(Debug, Clone)]
pub struct TakCodec {
    max_len: usize,
}

impl TakCodec {
    /// A codec with the default 8 MiB payload limit.
    pub fn new() -> Self {
        Self {
            max_len: DEFAULT_MAX_FRAME_LEN,
        }
    }

    /// Sets the largest accepted payload, in bytes.
    #[must_use]
    pub fn with_max_len(mut self, max_len: usize) -> Self {
        self.max_len = max_len;
        self
    }
}

impl Default for TakCodec {
    fn default() -> Self {
        Self::new()
    }
}

impl Decoder for TakCodec {
    type Item = TakFrame;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<TakFrame>, Error> {
        TakFrame::decode_stream(src, self.max_len)
    }
}

impl Encoder<TakFrame> for TakCodec {
    type Error = Error;

    fn encode(&mut self, item: TakFrame, dst: &mut BytesMut) -> Result<(), Error> {
        frame::encode_stream(&item.to_message()?, dst);
        Ok(())
    }
}

impl Encoder<Event> for TakCodec {
    type Error = Error;

    fn encode(&mut self, item: Event, dst: &mut BytesMut) -> Result<(), Error> {
        self.encode(TakFrame::from_event(item), dst)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cottak_core::Point;

    #[test]
    fn codec_round_trip() {
        let ev = Event::new(
            "U1",
            "a-f-G".parse().unwrap(),
            Point::new(1.0, 2.0).unwrap(),
        );
        let mut c = TakCodec::new();
        let mut buf = BytesMut::new();
        c.encode(ev.clone(), &mut buf).unwrap();
        c.encode(ev.clone(), &mut buf).unwrap();
        for _ in 0..2 {
            assert_eq!(c.decode(&mut buf).unwrap().unwrap().event.unwrap(), ev);
        }
        assert!(c.decode(&mut buf).unwrap().is_none());
    }

    #[test]
    fn codec_enforces_limit() {
        let ev = Event::new(
            "U1",
            "a-f-G".parse().unwrap(),
            Point::new(1.0, 2.0).unwrap(),
        );
        let mut buf = BytesMut::new();
        TakCodec::new().encode(ev, &mut buf).unwrap();
        assert!(matches!(
            TakCodec::new().with_max_len(4).decode(&mut buf),
            Err(Error::FrameTooLarge { .. })
        ));
    }
}
