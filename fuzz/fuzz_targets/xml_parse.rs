#![no_main]
//! The hardened XML tree parser must never panic, and what it accepts must survive a
//! serialise/parse round trip.

use cottak_core::xml::XmlElement;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Ok(text) = std::str::from_utf8(data) else { return };
    if let Ok(el) = XmlElement::parse(text) {
        let again = XmlElement::parse(&el.to_xml()).expect("own output must parse");
        assert_eq!(again, el);
    }
});
