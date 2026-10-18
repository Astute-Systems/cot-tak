#![no_main]
//! Any text accepted as a CoT event must serialise to XML that parses back to an equal event.

use cottak_core::Event;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Ok(text) = std::str::from_utf8(data) else { return };
    if let Ok(event) = text.parse::<Event>() {
        let xml = event.to_xml_string().expect("a parsed event must serialise");
        let again: Event = xml.parse().expect("own output must parse");
        assert_eq!(again, event);
    }
});
