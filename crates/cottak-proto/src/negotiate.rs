//! TAK Protocol negotiation for streaming connections.
//!
//! A connection starts as legacy XML (protocol version 0). A server that
//! speaks the TAK Protocol offers versions with a `t-x-takp-v` event; the
//! client asks for one with `t-x-takp-q`; the server answers with
//! `t-x-takp-r`. After an accepted response both sides switch to framed
//! protobuf (see [`crate::frame`]).
//!
//! ```
//! use cottak_proto::negotiate;
//!
//! let offer = negotiate::offer("neg-1", &[1])?;
//! assert_eq!(negotiate::parse_offer(&offer), Some(vec![1]));
//!
//! let req = negotiate::request("neg-1", 1)?;
//! assert_eq!(negotiate::parse_request(&req), Some(1));
//!
//! let resp = negotiate::response("neg-1", true)?;
//! assert_eq!(negotiate::parse_response(&resp), Some(true));
//! # Ok::<(), cottak_core::Error>(())
//! ```

use cottak_core::xml::XmlElement;
use cottak_core::{CotType, Event, Point, Result};
use time::Duration;

/// Event type of the server's offer of supported versions.
pub const OFFER_TYPE: &str = "t-x-takp-v";
/// Event type of the client's request for a version.
pub const REQUEST_TYPE: &str = "t-x-takp-q";
/// Event type of the server's response.
pub const RESPONSE_TYPE: &str = "t-x-takp-r";

fn control_event(uid: &str, cot_type: &str, control: XmlElement) -> Result<Event> {
    let point = Point::new(0.0, 0.0)?
        .with_hae(0.0)
        .with_errors(999_999.0, 999_999.0);
    let mut ev = Event::new(uid, CotType::new(cot_type)?, point)
        .with_how("m-g")
        .with_stale_in(Duration::minutes(1));
    ev.detail.push_raw(control);
    Ok(ev)
}

fn control(child: XmlElement) -> XmlElement {
    XmlElement::new("TakControl").with_child(child)
}

fn control_child<'a>(ev: &'a Event, cot_type: &str) -> Option<&'a XmlElement> {
    if ev.cot_type.as_str() != cot_type {
        return None;
    }
    ev.detail.raw("TakControl")
}

/// Server side: advertises the protocol versions it supports.
pub fn offer(uid: &str, versions: &[u32]) -> Result<Event> {
    let mut control = XmlElement::new("TakControl");
    for v in versions {
        control = control
            .with_child(XmlElement::new("TakProtocolSupport").with_attr("version", v.to_string()));
    }
    control_event(uid, OFFER_TYPE, control)
}

/// Reads the versions from an offer, or `None` if `ev` is not an offer.
pub fn parse_offer(ev: &Event) -> Option<Vec<u32>> {
    let control = control_child(ev, OFFER_TYPE)?;
    Some(
        control
            .elements()
            .filter(|e| e.name == "TakProtocolSupport")
            .filter_map(|e| e.attr("version")?.trim().parse().ok())
            .collect(),
    )
}

/// Client side: requests `version` from the versions the server offered.
pub fn request(uid: &str, version: u32) -> Result<Event> {
    control_event(
        uid,
        REQUEST_TYPE,
        control(XmlElement::new("TakRequest").with_attr("version", version.to_string())),
    )
}

/// Reads the requested version, or `None` if `ev` is not a request.
pub fn parse_request(ev: &Event) -> Option<u32> {
    control_child(ev, REQUEST_TYPE)?
        .child("TakRequest")?
        .attr("version")?
        .trim()
        .parse()
        .ok()
}

/// Server side: accepts or denies a request.
pub fn response(uid: &str, accepted: bool) -> Result<Event> {
    control_event(
        uid,
        RESPONSE_TYPE,
        control(XmlElement::new("TakResponse").with_attr("status", accepted.to_string())),
    )
}

/// Reads the response status, or `None` if `ev` is not a response.
pub fn parse_response(ev: &Event) -> Option<bool> {
    control_child(ev, RESPONSE_TYPE)?
        .child("TakResponse")?
        .attr("status")?
        .trim()
        .parse()
        .ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn offer_lists_all_versions() {
        let ev = offer("u", &[1, 2]).unwrap();
        assert_eq!(parse_offer(&ev), Some(vec![1, 2]));
        assert_eq!(parse_request(&ev), None);
        let again: Event = ev.to_xml_string().unwrap().parse().unwrap();
        assert_eq!(parse_offer(&again), Some(vec![1, 2]));
    }

    #[test]
    fn parses_spec_example() {
        let xml = r#"<event version='2.0' uid='protouid' type='t-x-takp-v' time='2025-01-01T00:00:00Z'
            start='2025-01-01T00:00:00Z' stale='2025-01-01T00:01:00Z' how='m-g'>
            <point lat='0.0' lon='0.0' hae='0.0' ce='999999' le='999999'/>
            <detail><TakControl><TakProtocolSupport version="1"/></TakControl></detail></event>"#;
        let ev: Event = xml.parse().unwrap();
        assert_eq!(parse_offer(&ev), Some(vec![1]));
    }

    #[test]
    fn request_and_response() {
        assert_eq!(parse_request(&request("u", 1).unwrap()), Some(1));
        assert_eq!(parse_response(&response("u", true).unwrap()), Some(true));
        assert_eq!(parse_response(&response("u", false).unwrap()), Some(false));
    }
}
