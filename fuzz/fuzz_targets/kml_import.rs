#![no_main]
//! Untrusted KML: no panics, and every produced event must be valid CoT.

use cottak_convert::geojson::ImportLimits;
use cottak_convert::kml::events_from_kml;
use cottak_core::Event;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Ok(text) = std::str::from_utf8(data) else { return };
    let limits = ImportLimits { max_features: 64, max_vertices: 256 };
    if let Ok(import) = events_from_kml(text, &limits) {
        assert!(import.events.len() <= 64);
        for event in import.events {
            event.validate().expect("imported events must be valid");
            let again: Event = event.to_xml_string().expect("serialises").parse().expect("parses");
            assert_eq!(again, event);
        }
    }
});
