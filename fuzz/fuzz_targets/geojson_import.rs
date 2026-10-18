#![no_main]
//! Untrusted GeoJSON: no panics, and every produced event must be valid CoT.

use cottak_convert::geojson::{events_from_str, ImportLimits};
use cottak_core::Event;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Ok(text) = std::str::from_utf8(data) else { return };
    let limits = ImportLimits { max_features: 64, max_vertices: 256 };
    if let Ok(events) = events_from_str(text, &limits) {
        for event in events {
            event.validate().expect("imported events must be valid");
            let again: Event = event.to_xml_string().expect("serialises").parse().expect("parses");
            assert_eq!(again, event);
        }
    }
});
