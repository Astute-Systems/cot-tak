#![no_main]
//! JSON from a TAK Server (or something pretending to be one) must decode or fail cleanly.

use cottak_marti::models::{ApiResponse, ClientEndpoint, Contact, Group, Mission, MissionChange};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(m) = serde_json::from_slice::<Mission>(data) {
        let _ = serde_json::to_vec(&m).expect("decoded models must serialise");
    }
    let _ = serde_json::from_slice::<ApiResponse<Vec<Mission>>>(data);
    let _ = serde_json::from_slice::<ApiResponse<Vec<MissionChange>>>(data);
    let _ = serde_json::from_slice::<ApiResponse<Vec<Group>>>(data);
    let _ = serde_json::from_slice::<ApiResponse<Vec<ClientEndpoint>>>(data);
    let _ = serde_json::from_slice::<Vec<Contact>>(data);
});
