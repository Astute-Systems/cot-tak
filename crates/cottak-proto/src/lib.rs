//! TAK Protocol v1 (protobuf) support for Cursor on Target.
//!
//! * [`convert`] – [`Event`] ⇄ protobuf `CotEvent`, following the mapping
//!   rules of the TAK Protocol specification.
//! * [`frame`] – streaming (`0xbf <len>`) and mesh (`0xbf <ver> 0xbf`)
//!   framing, incremental and size-limited.
//! * [`negotiate`] – the XML handshake that upgrades a streaming connection
//!   from legacy XML to protobuf.
//! * [`TakFrame`] – a decoded message: optional control info plus an optional event.
//! * `codec` (cargo feature) – a `tokio_util` codec for streaming connections.
//!
//! The `.proto` definitions are compiled at build time by a pure-Rust
//! compiler; no `protoc` is required.
//!
//! ```
//! use bytes::BytesMut;
//! use cottak_core::{Event, Point};
//! use cottak_proto::{frame, TakFrame};
//!
//! let event = Event::new("U1", "a-f-G-U-C".parse()?, Point::new(1.0, 2.0)?);
//!
//! let mut wire = BytesMut::new();
//! TakFrame::from_event(event.clone()).encode_stream(&mut wire)?;
//!
//! let frame = TakFrame::decode_stream(&mut wire, frame::DEFAULT_MAX_FRAME_LEN)?.unwrap();
//! assert_eq!(frame.event.unwrap().uid, "U1");
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

pub mod convert;
pub mod error;
pub mod frame;
pub mod negotiate;

#[cfg(feature = "codec")]
mod codec;
#[cfg(feature = "codec")]
pub use codec::TakCodec;

/// Generated protobuf types for `atakmap.commoncommo.protobuf.v1`.
#[allow(
    missing_docs,
    clippy::all,
    clippy::pedantic,
    rustdoc::invalid_html_tags,
    rustdoc::bare_urls
)]
pub mod pb {
    include!(concat!(
        env!("OUT_DIR"),
        "/atakmap.commoncommo.protobuf.v1.rs"
    ));
}

pub use error::{Error, Result};

use bytes::BytesMut;
use cottak_core::Event;

/// Protocol support advertised by a peer (`TakControl`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Control {
    /// Lowest version the sender can decode (a wire value of 0 means 1).
    pub min_version: u32,
    /// Highest version the sender can decode (a wire value of 0 means 1).
    pub max_version: u32,
    /// UID of the sending contact, if given.
    pub contact_uid: Option<String>,
}

impl Control {
    /// A control record for a peer that supports exactly version 1.
    pub fn v1() -> Self {
        Self {
            min_version: 1,
            max_version: 1,
            contact_uid: None,
        }
    }
}

impl From<&pb::TakControl> for Control {
    fn from(c: &pb::TakControl) -> Self {
        let fix = |v: u32| if v == 0 { 1 } else { v };
        Self {
            min_version: fix(c.min_proto_version),
            max_version: fix(c.max_proto_version),
            contact_uid: Some(c.contact_uid.clone()).filter(|s| !s.is_empty()),
        }
    }
}

/// One decoded `TakMessage`.
#[derive(Debug, Clone, PartialEq)]
pub struct TakFrame {
    /// Control information, if the sender included it.
    pub control: Option<Control>,
    /// The event, if the message carries one.
    pub event: Option<Event>,
}

impl TakFrame {
    /// A frame carrying only an event.
    pub fn from_event(event: Event) -> Self {
        Self {
            control: None,
            event: Some(event),
        }
    }

    /// A frame carrying only control information.
    pub fn from_control(control: Control) -> Self {
        Self {
            control: Some(control),
            event: None,
        }
    }

    /// Converts to the protobuf message.
    pub fn to_message(&self) -> Result<pb::TakMessage> {
        Ok(pb::TakMessage {
            tak_control: self.control.as_ref().map(|c| pb::TakControl {
                min_proto_version: c.min_version,
                max_proto_version: c.max_version,
                contact_uid: c.contact_uid.clone().unwrap_or_default(),
            }),
            cot_event: self.event.as_ref().map(convert::event_to_pb).transpose()?,
        })
    }

    /// Converts from the protobuf message.
    pub fn from_message(m: &pb::TakMessage) -> Result<Self> {
        Ok(Self {
            control: m.tak_control.as_ref().map(Control::from),
            event: m.cot_event.as_ref().map(convert::pb_to_event).transpose()?,
        })
    }

    /// Appends this frame to `out` as a streaming frame.
    pub fn encode_stream(&self, out: &mut BytesMut) -> Result<()> {
        frame::encode_stream(&self.to_message()?, out);
        Ok(())
    }

    /// Removes one streaming frame from `buf`, if a complete one is present.
    pub fn decode_stream(buf: &mut BytesMut, max_len: usize) -> Result<Option<Self>> {
        frame::decode_stream(buf, max_len)?
            .map(|m| Self::from_message(&m))
            .transpose()
    }

    /// Encodes this frame as a mesh datagram.
    pub fn encode_mesh(&self) -> Result<Vec<u8>> {
        Ok(frame::encode_mesh(&self.to_message()?))
    }

    /// Decodes a mesh datagram.
    pub fn decode_mesh(datagram: &[u8]) -> Result<Self> {
        let (_, m) = frame::decode_mesh(datagram)?;
        Self::from_message(&m)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cottak_core::detail::Contact;
    use cottak_core::Point;

    #[test]
    fn frame_round_trip_over_stream_and_mesh() {
        let ev = Event::new(
            "U1",
            "a-f-G-U-C".parse().unwrap(),
            Point::new(-27.0, 153.0).unwrap(),
        )
        .with(Contact::new("ALPHA"));
        let f = TakFrame {
            control: Some(Control {
                contact_uid: Some("U1".into()),
                ..Control::v1()
            }),
            event: Some(ev),
        };

        let mut b = BytesMut::new();
        f.encode_stream(&mut b).unwrap();
        assert_eq!(
            TakFrame::decode_stream(&mut b, frame::DEFAULT_MAX_FRAME_LEN)
                .unwrap()
                .unwrap(),
            f
        );

        assert_eq!(TakFrame::decode_mesh(&f.encode_mesh().unwrap()).unwrap(), f);
    }

    #[test]
    fn zero_versions_mean_one() {
        let c = Control::from(&pb::TakControl::default());
        assert_eq!((c.min_version, c.max_version, c.contact_uid), (1, 1, None));
    }

    #[test]
    fn control_only_frame() {
        let f = TakFrame::from_control(Control::v1());
        let mut b = BytesMut::new();
        f.encode_stream(&mut b).unwrap();
        let back = TakFrame::decode_stream(&mut b, 1024).unwrap().unwrap();
        assert!(back.event.is_none() && back.control == Some(Control::v1()));
    }
}
