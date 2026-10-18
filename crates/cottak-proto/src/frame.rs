//! TAK Protocol framing.
//!
//! * **Streaming** (TAK Server connections): `0xbf <varint length> <payload>`.
//! * **Mesh** (UDP datagrams and one-shot TCP): `0xbf <varint version> 0xbf <payload>`.
//!
//! The payload is one protobuf `TakMessage`.

use crate::error::{Error, Result};
use crate::pb::TakMessage;
use bytes::{Buf, BufMut, BytesMut};
use prost::Message;

/// Magic byte that starts every TAK Protocol header.
pub const MAGIC: u8 = 0xBF;

/// The only payload version defined so far.
pub const VERSION_1: u32 = 1;

/// Default limit for a single streamed payload (8 MiB).
pub const DEFAULT_MAX_FRAME_LEN: usize = 8 * 1024 * 1024;

fn put_varint(out: &mut BytesMut, mut v: u64) {
    while v >= 0x80 {
        out.put_u8((v & 0x7F) as u8 | 0x80);
        v >>= 7;
    }
    out.put_u8((v & 0x7F) as u8);
}

/// Reads an unsigned varint. `Ok(None)` means more bytes are needed.
fn read_varint(buf: &[u8]) -> Result<Option<(u64, usize)>> {
    let mut value = 0u64;
    for (i, &b) in buf.iter().enumerate().take(10) {
        value |= u64::from(b & 0x7F) << (7 * i);
        if b & 0x80 == 0 {
            return Ok(Some((value, i + 1)));
        }
    }
    if buf.len() >= 10 {
        Err(Error::BadVarint)
    } else {
        Ok(None)
    }
}

/// Appends a streaming frame for `msg` to `out`.
pub fn encode_stream(msg: &TakMessage, out: &mut BytesMut) {
    let len = msg.encoded_len();
    out.reserve(1 + 10 + len);
    out.put_u8(MAGIC);
    put_varint(out, len as u64);
    msg.encode_raw(out);
}

/// Tries to remove one streaming frame from the front of `buf`.
///
/// Returns `Ok(None)` if the frame is not complete yet; call again after more
/// data arrives. On error the stream is out of sync and the connection should
/// be closed.
pub fn decode_stream(buf: &mut BytesMut, max_len: usize) -> Result<Option<TakMessage>> {
    let Some(&first) = buf.first() else {
        return Ok(None);
    };
    if first != MAGIC {
        return Err(Error::BadMagic(first));
    }
    let Some((len, n)) = read_varint(&buf[1..])? else {
        return Ok(None);
    };
    let len_usize = usize::try_from(len).ok().filter(|&l| l <= max_len);
    let Some(len_usize) = len_usize else {
        return Err(Error::FrameTooLarge { len, max: max_len });
    };
    let header = 1 + n;
    if buf.len() < header + len_usize {
        return Ok(None);
    }
    buf.advance(header);
    let payload = buf.split_to(len_usize);
    Ok(Some(TakMessage::decode(payload.freeze())?))
}

/// Encodes a mesh datagram (`0xbf <version> 0xbf <payload>`).
pub fn encode_mesh(msg: &TakMessage) -> Vec<u8> {
    let mut out = BytesMut::with_capacity(3 + msg.encoded_len());
    out.put_u8(MAGIC);
    put_varint(&mut out, u64::from(VERSION_1));
    out.put_u8(MAGIC);
    msg.encode_raw(&mut out);
    out.to_vec()
}

/// Decodes a mesh datagram, returning the protocol version and message.
pub fn decode_mesh(datagram: &[u8]) -> Result<(u32, TakMessage)> {
    if datagram.first() != Some(&MAGIC) {
        return Err(datagram
            .first()
            .map_or(Error::BadDatagram, |&b| Error::BadMagic(b)));
    }
    let Some((version, n)) = read_varint(&datagram[1..])? else {
        return Err(Error::BadDatagram);
    };
    if datagram.get(1 + n) != Some(&MAGIC) {
        return Err(Error::BadDatagram);
    }
    if version != u64::from(VERSION_1) {
        return Err(Error::UnsupportedVersion(version));
    }
    Ok((VERSION_1, TakMessage::decode(&datagram[2 + n..])?))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pb::{CotEvent, TakControl};

    fn msg(uid: &str) -> TakMessage {
        TakMessage {
            tak_control: Some(TakControl {
                min_proto_version: 1,
                max_proto_version: 1,
                contact_uid: uid.into(),
            }),
            cot_event: Some(CotEvent {
                uid: uid.into(),
                r#type: "a-f-G".into(),
                ..CotEvent::default()
            }),
        }
    }

    #[test]
    fn stream_round_trip_and_known_bytes() {
        let m = TakMessage::default();
        let mut b = BytesMut::new();
        encode_stream(&m, &mut b);
        assert_eq!(&b[..], &[0xBF, 0x00], "empty payload has zero length");

        let mut b = BytesMut::new();
        encode_stream(&msg("U1"), &mut b);
        assert_eq!(b[0], MAGIC);
        assert_eq!(
            decode_stream(&mut b, DEFAULT_MAX_FRAME_LEN)
                .unwrap()
                .unwrap(),
            msg("U1")
        );
        assert!(b.is_empty());
    }

    #[test]
    fn partial_frames_wait_for_more_data() {
        let mut full = BytesMut::new();
        encode_stream(&msg("U1"), &mut full);
        let mut b = BytesMut::new();
        for byte in &full[..full.len() - 1] {
            b.put_u8(*byte);
            assert!(decode_stream(&mut b, DEFAULT_MAX_FRAME_LEN)
                .unwrap()
                .is_none());
        }
        b.put_u8(full[full.len() - 1]);
        assert!(decode_stream(&mut b, DEFAULT_MAX_FRAME_LEN)
            .unwrap()
            .is_some());
    }

    #[test]
    fn back_to_back_frames() {
        let mut b = BytesMut::new();
        encode_stream(&msg("A"), &mut b);
        encode_stream(&msg("B"), &mut b);
        let first = decode_stream(&mut b, DEFAULT_MAX_FRAME_LEN)
            .unwrap()
            .unwrap();
        let second = decode_stream(&mut b, DEFAULT_MAX_FRAME_LEN)
            .unwrap()
            .unwrap();
        assert_eq!((first, second), (msg("A"), msg("B")));
        assert!(decode_stream(&mut b, DEFAULT_MAX_FRAME_LEN)
            .unwrap()
            .is_none());
    }

    #[test]
    fn multi_byte_length_varint() {
        let big = TakMessage {
            cot_event: Some(CotEvent {
                uid: "x".repeat(300),
                ..CotEvent::default()
            }),
            ..TakMessage::default()
        };
        let mut b = BytesMut::new();
        encode_stream(&big, &mut b);
        assert!(b[1] & 0x80 != 0, "length needs two varint bytes");
        assert_eq!(
            decode_stream(&mut b, DEFAULT_MAX_FRAME_LEN)
                .unwrap()
                .unwrap(),
            big
        );
    }

    #[test]
    fn rejects_bad_input() {
        let mut b = BytesMut::from(&b"<?xml"[..]);
        assert!(matches!(
            decode_stream(&mut b, 100),
            Err(Error::BadMagic(b'<'))
        ));

        let mut b = BytesMut::from(
            &[
                MAGIC, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            ][..],
        );
        assert!(matches!(decode_stream(&mut b, 100), Err(Error::BadVarint)));

        let mut b = BytesMut::from(&[MAGIC, 0xE8, 0x07][..]); // 1000 bytes
        assert!(matches!(
            decode_stream(&mut b, 100),
            Err(Error::FrameTooLarge {
                len: 1000,
                max: 100
            })
        ));

        let mut b = BytesMut::from(&[MAGIC, 0x02, 0xFF, 0xFF][..]); // payload is not protobuf
        assert!(matches!(decode_stream(&mut b, 100), Err(Error::Decode(_))));
    }

    #[test]
    fn mesh_round_trip() {
        let d = encode_mesh(&msg("U1"));
        assert_eq!(&d[..3], &[MAGIC, 0x01, MAGIC]);
        assert_eq!(decode_mesh(&d).unwrap(), (1, msg("U1")));
    }

    #[test]
    fn mesh_rejects_bad_datagrams() {
        assert!(matches!(decode_mesh(b""), Err(Error::BadDatagram)));
        assert!(matches!(
            decode_mesh(b"<event/>"),
            Err(Error::BadMagic(b'<'))
        ));
        assert!(matches!(
            decode_mesh(&[MAGIC, 0x01]),
            Err(Error::BadDatagram)
        ));
        assert!(matches!(
            decode_mesh(&[MAGIC, 0x01, 0x00]),
            Err(Error::BadDatagram)
        ));
        assert!(matches!(
            decode_mesh(&[MAGIC, 0x02, MAGIC]),
            Err(Error::UnsupportedVersion(2))
        ));
    }
}
