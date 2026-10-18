//! Property tests for conversion and framing.

use bytes::BytesMut;
use cottak_core::detail::{Contact, Remarks, Takv, Track};
use cottak_core::{Event, Point};
use cottak_proto::{frame, TakFrame};
use proptest::prelude::*;

fn event_strategy() -> impl Strategy<Value = Event> {
    (
        "[A-Za-z0-9_.-]{1,24}",
        -90.0f64..=90.0,
        -180.0f64..=180.0,
        proptest::option::of(-500.0f64..9000.0),
        "[ -~]{0,16}",
        "[ -~\u{A0}-\u{2FF}]{0,40}",
        any::<bool>(),
    )
        .prop_map(|(uid, lat, lon, hae, callsign, remark, with_extras)| {
            let mut p = Point::new(lat, lon).unwrap();
            p.hae = hae;
            let mut ev = Event::new(uid, "a-f-G-U-C".parse().unwrap(), p)
                .with(Contact::new(callsign))
                .with(Remarks::new(remark));
            if with_extras {
                ev = ev.with(Track::new(10.5, 2.25)).with(Takv {
                    device: Some("d".into()),
                    platform: Some("p".into()),
                    os: Some("o".into()),
                    version: Some("v".into()),
                    ..Takv::default()
                });
            }
            ev
        })
}

proptest! {
    /// Event -> protobuf -> Event preserves everything the wire carries.
    #[test]
    fn event_survives_protobuf(ev in event_strategy()) {
        let frame = TakFrame::from_event(ev.clone());
        let mut wire = BytesMut::new();
        frame.encode_stream(&mut wire).unwrap();
        let back = TakFrame::decode_stream(&mut wire, frame::DEFAULT_MAX_FRAME_LEN)
            .unwrap().unwrap().event.unwrap();
        prop_assert_eq!(&back.uid, &ev.uid);
        prop_assert_eq!(back.point, ev.point);
        prop_assert_eq!(back.time, ev.time);
        prop_assert_eq!(back.stale, ev.stale);
        prop_assert_eq!(back.detail.get::<Contact>(), ev.detail.get::<Contact>());
        prop_assert_eq!(back.detail.get::<Remarks>(), ev.detail.get::<Remarks>());
        prop_assert_eq!(back.detail.get::<Track>(), ev.detail.get::<Track>());
        prop_assert_eq!(back.detail.get::<Takv>(), ev.detail.get::<Takv>());
    }

    /// Any split of the byte stream decodes to the same frames.
    #[test]
    fn arbitrary_chunking(evs in prop::collection::vec(event_strategy(), 1..4), chunk in 1usize..40) {
        let mut wire = BytesMut::new();
        for e in &evs {
            TakFrame::from_event(e.clone()).encode_stream(&mut wire).unwrap();
        }
        let bytes = wire.to_vec();
        let mut buf = BytesMut::new();
        let mut got = Vec::new();
        for piece in bytes.chunks(chunk) {
            buf.extend_from_slice(piece);
            while let Some(f) = TakFrame::decode_stream(&mut buf, frame::DEFAULT_MAX_FRAME_LEN).unwrap() {
                got.push(f.event.unwrap().uid);
            }
        }
        prop_assert!(buf.is_empty());
        let want: Vec<_> = evs.iter().map(|e| e.uid.clone()).collect();
        prop_assert_eq!(got, want);
    }

    /// Hostile input must produce an error or "need more", never a panic or huge allocation.
    #[test]
    fn decoders_never_panic(data in prop::collection::vec(any::<u8>(), 0..300)) {
        let mut buf = BytesMut::from(&data[..]);
        let _ = TakFrame::decode_stream(&mut buf, 1024);
        let _ = TakFrame::decode_mesh(&data);
    }
}
