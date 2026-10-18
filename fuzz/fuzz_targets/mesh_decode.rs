#![no_main]
//! Mesh datagrams arrive from anyone on the network.

use cottak_proto::TakFrame;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = TakFrame::decode_mesh(data);
});
