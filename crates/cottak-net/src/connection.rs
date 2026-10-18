//! A framed connection that speaks XML or TAK Protocol v1 over any byte stream.

use crate::error::{Error, Result};
use bytes::BytesMut;
use cottak_core::Event;
use cottak_proto::{frame, Control, TakFrame};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

const XML_HEADER: &[u8] = b"<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n";
const EVENT_END: &[u8] = b"</event>";

/// The wire protocol in use on a connection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    /// Legacy XML (protocol version 0): `<event>` elements back to back.
    Xml,
    /// TAK Protocol version 1: `0xbf <len> <protobuf>` frames.
    V1,
}

/// One item read from a peer.
#[derive(Debug)]
pub enum Incoming {
    /// A CoT event.
    Event(Box<Event>),
    /// A `TakControl` record (protocol version 1 only).
    Control(Control),
    /// A well-delimited message that could not be understood; the stream is intact.
    Invalid(String),
}

pub(crate) trait Io: AsyncRead + AsyncWrite + Unpin + Send {}
impl<T: AsyncRead + AsyncWrite + Unpin + Send> Io for T {}

/// A framed connection that reads and writes CoT events as XML or TAK Protocol v1.
///
/// This is the building block shared by the client and by servers (see the
/// `cottak-server` crate). It starts out speaking [`Protocol::Xml`].
pub struct Connection<S> {
    io: S,
    buf: BytesMut,
    protocol: Protocol,
    /// Bytes of `buf` already searched for `</event>`.
    scanned: usize,
    max_len: usize,
}

impl<S: AsyncRead + AsyncWrite + Unpin> Connection<S> {
    /// Wraps `io`. Messages larger than `max_len` bytes are an error.
    pub fn new(io: S, max_len: usize) -> Self {
        Self {
            io,
            buf: BytesMut::with_capacity(8 * 1024),
            protocol: Protocol::Xml,
            scanned: 0,
            max_len,
        }
    }

    /// The wire protocol currently in use.
    pub fn protocol(&self) -> Protocol {
        self.protocol
    }

    /// Switches the wire protocol. Bytes already buffered are decoded with the
    /// new protocol, which is what the negotiation rules require.
    pub fn set_protocol(&mut self, p: Protocol) {
        self.protocol = p;
        self.scanned = 0;
    }

    /// Reads the next item. Cancel-safe: dropping the future loses no data.
    pub async fn read(&mut self) -> Result<Incoming> {
        loop {
            if let Some(item) = self.try_decode()? {
                return Ok(item);
            }
            if self.io.read_buf(&mut self.buf).await? == 0 {
                return Err(Error::Closed);
            }
        }
    }

    fn try_decode(&mut self) -> Result<Option<Incoming>> {
        match self.protocol {
            Protocol::Xml => self.try_decode_xml(),
            Protocol::V1 => self.try_decode_v1(),
        }
    }

    fn try_decode_xml(&mut self) -> Result<Option<Incoming>> {
        let from = self.scanned.saturating_sub(EVENT_END.len() - 1);
        let Some(pos) = find(&self.buf[from..], EVENT_END) else {
            self.scanned = self.buf.len();
            if self.buf.len() > self.max_len {
                return Err(Error::MessageTooLarge(self.max_len));
            }
            return Ok(None);
        };
        let end = from + pos + EVENT_END.len();
        let chunk = self.buf.split_to(end);
        self.scanned = 0;
        let item = match std::str::from_utf8(&chunk) {
            Ok(text) => match Event::from_xml_str(text) {
                Ok(ev) => Incoming::Event(Box::new(ev)),
                Err(e) => Incoming::Invalid(e.to_string()),
            },
            Err(e) => Incoming::Invalid(format!("invalid UTF-8: {e}")),
        };
        Ok(Some(item))
    }

    fn try_decode_v1(&mut self) -> Result<Option<Incoming>> {
        loop {
            let Some(msg) = frame::decode_stream(&mut self.buf, self.max_len)? else {
                return Ok(None);
            };
            match TakFrame::from_message(&msg) {
                Ok(TakFrame {
                    event: Some(ev), ..
                }) => return Ok(Some(Incoming::Event(Box::new(ev)))),
                Ok(TakFrame {
                    control: Some(c), ..
                }) => return Ok(Some(Incoming::Control(c))),
                Ok(_) => {}
                Err(e) => return Ok(Some(Incoming::Invalid(e.to_string()))),
            }
        }
    }

    /// Encodes and writes one event in the current protocol.
    pub async fn write_event(&mut self, ev: &Event) -> Result<()> {
        let mut out = BytesMut::new();
        match self.protocol {
            Protocol::Xml => {
                out.extend_from_slice(XML_HEADER);
                out.extend_from_slice(ev.to_xml_string()?.as_bytes());
            }
            Protocol::V1 => TakFrame::from_event(ev.clone()).encode_stream(&mut out)?,
        }
        self.io.write_all(&out).await?;
        self.io.flush().await?;
        Ok(())
    }
}

fn find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|w| w == needle)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cottak_core::Point;
    use tokio::io::duplex;

    fn ev(uid: &str) -> Event {
        Event::new(uid, "a-f-G".parse().unwrap(), Point::new(1.0, 2.0).unwrap())
    }

    #[tokio::test]
    async fn xml_round_trip_with_back_to_back_and_split_messages() {
        let (a, mut b) = duplex(64 * 1024);
        let mut conn = Connection::new(a, 1 << 20);

        // Two events in one write, then a third split across writes.
        let one = format!(
            "{}{}",
            ev("A").to_xml_string().unwrap(),
            ev("B").to_xml_string().unwrap()
        );
        let three = ev("C").to_xml_string().unwrap();
        b.write_all(one.as_bytes()).await.unwrap();
        b.write_all(&three.as_bytes()[..30]).await.unwrap();

        for want in ["A", "B"] {
            let Incoming::Event(e) = conn.read().await.unwrap() else {
                panic!("not an event")
            };
            assert_eq!(e.uid, want);
        }
        b.write_all(&three.as_bytes()[30..]).await.unwrap();
        let Incoming::Event(e) = conn.read().await.unwrap() else {
            panic!("not an event")
        };
        assert_eq!(e.uid, "C");
    }

    #[tokio::test]
    async fn bad_xml_is_reported_without_killing_the_stream() {
        let (a, mut b) = duplex(4096);
        let mut conn = Connection::new(a, 1 << 20);
        let good = ev("OK").to_xml_string().unwrap();
        b.write_all(format!("<event>garbage</event>{good}").as_bytes())
            .await
            .unwrap();
        assert!(matches!(conn.read().await.unwrap(), Incoming::Invalid(_)));
        assert!(matches!(conn.read().await.unwrap(), Incoming::Event(e) if e.uid == "OK"));
    }

    #[tokio::test]
    async fn oversized_xml_is_rejected() {
        let (a, mut b) = duplex(4096);
        let mut conn = Connection::new(a, 100);
        b.write_all(&[b'x'; 500]).await.unwrap();
        assert!(matches!(
            conn.read().await,
            Err(Error::MessageTooLarge(100))
        ));
    }

    #[tokio::test]
    async fn write_xml_has_header_and_no_trailing_newline() {
        let (a, mut b) = duplex(4096);
        let mut conn = Connection::new(a, 1 << 20);
        conn.write_event(&ev("W")).await.unwrap();
        drop(conn);
        let mut got = String::new();
        b.read_to_string(&mut got).await.unwrap();
        assert!(got.starts_with("<?xml version=\"1.0\""));
        assert!(got.ends_with("</event>"));
    }

    #[tokio::test]
    async fn protocol_switch_decodes_already_buffered_bytes() {
        let (a, mut b) = duplex(8192);
        let mut conn = Connection::new(a, 1 << 20);
        // Response event (XML) immediately followed by a protobuf frame in one write.
        let resp = ev("RESP").to_xml_string().unwrap();
        let mut wire = BytesMut::from(resp.as_bytes());
        TakFrame::from_event(ev("PB"))
            .encode_stream(&mut wire)
            .unwrap();
        b.write_all(&wire).await.unwrap();

        assert!(matches!(conn.read().await.unwrap(), Incoming::Event(e) if e.uid == "RESP"));
        conn.set_protocol(Protocol::V1);
        assert!(matches!(conn.read().await.unwrap(), Incoming::Event(e) if e.uid == "PB"));
    }

    #[tokio::test]
    async fn eof_is_closed() {
        let (a, b) = duplex(64);
        drop(b);
        let mut conn = Connection::new(a, 1024);
        assert!(matches!(conn.read().await, Err(Error::Closed)));
    }
}
