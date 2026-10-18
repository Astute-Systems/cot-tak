//! Parsing of representative TAK-client traffic.

use cottak_core::detail::{
    Chat, Contact, Emergency, Group, Link, PrecisionLocation, Status, Takv, Track,
};
use cottak_core::messages::chat::GeoChat;
use cottak_core::Event;

const PLI: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<event version="2.0" uid="ANDROID-deadbeef" type="a-f-G-U-C" how="m-g"
 time="2025-06-01T03:04:05.123Z" start="2025-06-01T03:04:05.123Z" stale="2025-06-01T03:10:20.123Z">
<point lat="-27.4698" lon="153.0251" hae="31.2" ce="4.5" le="9999999.0"/>
<detail>
<takv os="34" version="5.3.0.1" device="GOOGLE PIXEL 8" platform="ATAK-CIV"/>
<contact endpoint="*:-1:stcp" callsign="VIPER 1"/>
<uid Droid="VIPER 1"/>
<precisionlocation altsrc="GPS" geopointsrc="GPS"/>
<__group role="Team Lead" name="Dark Blue"/>
<status battery="76"/>
<track course="212.4" speed="1.3"/>
</detail></event>"#;

const GEOCHAT: &str = r#"<event version="2.0" uid="GeoChat.U1.All Chat Rooms.m-1" type="b-t-f"
 how="h-g-i-g-o" time="2025-06-01T03:04:05.000Z" start="2025-06-01T03:04:05.000Z"
 stale="2025-06-02T03:04:05.000Z">
<point lat="0.0" lon="0.0" hae="9999999.0" ce="9999999.0" le="9999999.0"/>
<detail>
<__chat parent="RootContactGroup" groupOwner="false" messageId="m-1" chatroom="All Chat Rooms"
 id="All Chat Rooms" senderCallsign="VIPER 1"><chatgrp uid0="U1" uid1="All Chat Rooms" id="All Chat Rooms"/></__chat>
<link uid="U1" type="a-f-G-U-C" relation="p-p"/>
<remarks source="BAO.F.ATAK.U1" to="All Chat Rooms" time="2025-06-01T03:04:05.000Z">Moving &amp; ready</remarks>
<__serverdestination destinations="10.0.0.1:4242:tcp:U1"/>
</detail></event>"#;

const EMERGENCY: &str = r#"<event version="2.0" uid="U1-9-1-1" type="b-a-o-tbl" how="h-e"
 time="2025-06-01T03:04:05.000Z" start="2025-06-01T03:04:05.000Z" stale="2025-06-01T03:09:05.000Z">
<point lat="1" lon="2" hae="0" ce="9999999" le="9999999"/>
<detail><link uid="U1" type="a-f-G-U-C" relation="p-p"/><contact callsign="VIPER 1"/>
<emergency type="911 Alert">VIPER 1</emergency></detail></event>"#;

#[test]
fn position_report() {
    let e: Event = PLI.parse().unwrap();
    let d = &e.detail;
    assert_eq!(
        d.get::<Contact>().unwrap().callsign.as_deref(),
        Some("VIPER 1")
    );
    assert_eq!(
        d.get::<Takv>().unwrap().platform.as_deref(),
        Some("ATAK-CIV")
    );
    assert_eq!(d.get::<Group>().unwrap().role.as_deref(), Some("Team Lead"));
    assert_eq!(d.get::<Status>().unwrap().battery, Some(76.0));
    assert_eq!(d.get::<Track>().unwrap().speed, Some(1.3));
    assert_eq!(
        d.get::<PrecisionLocation>().unwrap().altsrc.as_deref(),
        Some("GPS")
    );
    assert_eq!(e.point.le, None);
    assert_eq!(e.point.ce, Some(4.5));
}

#[test]
fn geochat_with_server_extension() {
    let e: Event = GEOCHAT.parse().unwrap();
    assert!(e.detail.get::<Chat>().is_some());
    assert!(e.detail.get::<Link>().is_some());
    let msg = GeoChat::from_event(&e).unwrap();
    assert!(msg.is_broadcast());
    assert_eq!(msg.text, "Moving & ready");
    // Unmodelled server extension survives a round trip.
    let again: Event = e.to_xml_string().unwrap().parse().unwrap();
    assert!(again.detail.raw("__serverdestination").is_some());
    assert_eq!(again, e);
}

#[test]
fn emergency_alert() {
    let e: Event = EMERGENCY.parse().unwrap();
    let em = e.detail.get::<Emergency>().unwrap();
    assert_eq!(em.emergency_type.as_deref(), Some("911 Alert"));
    assert_eq!(em.cancel, None);
    assert_eq!(em.callsign, "VIPER 1");
}
