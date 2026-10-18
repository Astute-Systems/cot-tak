#![no_main]
//! Small text parsers fed from outside: onboarding URLs, type catalogues, symbol codes, CoT types.

use cottak_convert::catalog::TypeCatalog;
use cottak_convert::Sidc;
use cottak_core::CotType;
use cottak_enroll::EnrollmentUrl;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Ok(text) = std::str::from_utf8(data) else { return };
    let _ = EnrollmentUrl::parse(text);
    let _ = TypeCatalog::from_xml(text);
    if let Ok(t) = text.parse::<CotType>() {
        if let Some(sidc) = Sidc::from_cot(&t) {
            assert_eq!(sidc.to_cot().expect("a produced SIDC must convert back"), t);
        }
    }
    if let Ok(s) = text.parse::<Sidc>() {
        let _ = s.to_cot();
    }
});
