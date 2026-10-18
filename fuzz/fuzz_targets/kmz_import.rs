#![no_main]
//! Untrusted KMZ archives: no panics, bounded work, and every event produced must be valid CoT.

use cottak_convert::geojson::ImportLimits;
use cottak_convert::kml::{events_from_kmz, KmzLimits};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let limits = ImportLimits { max_features: 64, max_vertices: 256 };
    let kmz = KmzLimits { max_archive_bytes: 1 << 20, max_kml_bytes: 256 << 10, max_entries: 64 };
    if let Ok(import) = events_from_kmz(data, &limits, &kmz) {
        assert!(import.events.len() <= 64);
        for event in import.events {
            event.validate().expect("imported events must be valid");
        }
    }
});
