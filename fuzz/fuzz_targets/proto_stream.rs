#![no_main]
//! Hostile TAK Protocol streams: framing, protobuf decoding and event conversion must not
//! panic, and decoded events must be re-encodable.

use bytes::BytesMut;
use cottak_proto::{frame, TakFrame};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let mut buf = BytesMut::from(data);
    // A small limit keeps the fuzzer from spending its time on allocation.
    while let Ok(Some(msg)) = frame::decode_stream(&mut buf, 64 * 1024) {
        if let Ok(frame) = TakFrame::from_message(&msg) {
            let mut out = BytesMut::new();
            frame.encode_stream(&mut out).expect("a decoded frame must re-encode");
            let again = TakFrame::decode_stream(&mut out, 1 << 20)
                .expect("own output must decode")
                .expect("own output is complete");
            // Round trip through the wire format is stable after one normalisation.
            let mut out2 = BytesMut::new();
            again.encode_stream(&mut out2).expect("re-encode");
            let third = TakFrame::decode_stream(&mut out2, 1 << 20).unwrap().unwrap();
            assert_eq!(third, again);
        }
    }
});
