//! Conversion between [`cottak_core::Event`] and the protobuf `CotEvent`.
//!
//! The mapping follows the rules documented in `detail.proto`:
//!
//! * `contact`, `__group`, `precisionlocation`, `status`, `takv` and `track`
//!   become typed messages **only** when the whole element can be represented
//!   exactly (single occurrence, no children, no unknown attributes, all
//!   required attributes present and canonical).
//! * Everything else is serialised into `xmlDetail` unchanged.
//!
//! The protobuf form has no place for the CoT `version` attribute or for
//! unknown `<event>` attributes; they are not carried, and decoding yields
//! `version="2.0"`.

use crate::error::{Error, Result};
use crate::pb;
use cottak_core::point::UNKNOWN;
use cottak_core::xml::XmlElement;
use cottak_core::{CotType, Detail, Event, Point};
use time::OffsetDateTime;

const NS_PER_MS: i128 = 1_000_000;

fn to_ms(t: OffsetDateTime) -> Result<u64> {
    u64::try_from(t.unix_timestamp_nanos().div_euclid(NS_PER_MS)).map_err(|_| Error::Timestamp)
}

fn from_ms(ms: u64) -> Result<OffsetDateTime> {
    OffsetDateTime::from_unix_timestamp_nanos(i128::from(ms) * NS_PER_MS)
        .map_err(|_| Error::Timestamp)
}

/// `true` if `el` has only the listed attributes, each once, and no children.
fn attrs_only(el: &XmlElement, allowed: &[&str]) -> bool {
    el.children.is_empty()
        && el.attrs.iter().all(|(k, _)| allowed.contains(&k.as_str()))
        && allowed
            .iter()
            .all(|a| el.attrs.iter().filter(|(k, _)| k == a).count() <= 1)
}

fn req(el: &XmlElement, name: &str) -> Option<String> {
    el.attr(name).map(str::to_owned)
}

fn num(el: &XmlElement, name: &str) -> Option<f64> {
    el.attr(name)?
        .trim()
        .parse::<f64>()
        .ok()
        .filter(|v| v.is_finite())
}

fn to_contact(el: &XmlElement) -> Option<pb::Contact> {
    if !attrs_only(el, &["endpoint", "callsign"]) || el.attr("endpoint") == Some("") {
        return None;
    }
    Some(pb::Contact {
        endpoint: req(el, "endpoint").unwrap_or_default(),
        callsign: req(el, "callsign")?,
    })
}

fn to_group(el: &XmlElement) -> Option<pb::Group> {
    attrs_only(el, &["name", "role"]).then(|| {
        Some(pb::Group {
            name: req(el, "name")?,
            role: req(el, "role")?,
        })
    })?
}

fn to_precision(el: &XmlElement) -> Option<pb::PrecisionLocation> {
    attrs_only(el, &["geopointsrc", "altsrc"]).then(|| {
        Some(pb::PrecisionLocation {
            geopointsrc: req(el, "geopointsrc")?,
            altsrc: req(el, "altsrc")?,
        })
    })?
}

fn to_status(el: &XmlElement) -> Option<pb::Status> {
    if !attrs_only(el, &["battery"]) {
        return None;
    }
    let raw = el.attr("battery")?;
    let battery: u32 = raw.parse().ok()?;
    (battery.to_string() == raw).then_some(pb::Status { battery })
}

fn to_takv(el: &XmlElement) -> Option<pb::Takv> {
    attrs_only(el, &["device", "platform", "os", "version"]).then(|| {
        Some(pb::Takv {
            device: req(el, "device")?,
            platform: req(el, "platform")?,
            os: req(el, "os")?,
            version: req(el, "version")?,
        })
    })?
}

fn to_track(el: &XmlElement) -> Option<pb::Track> {
    attrs_only(el, &["speed", "course"]).then(|| {
        Some(pb::Track {
            speed: num(el, "speed")?,
            course: num(el, "course")?,
        })
    })?
}

/// Splits out the element named `name` if it occurs exactly once and converts.
fn take<T>(
    rest: &mut Vec<&XmlElement>,
    name: &str,
    convert: impl Fn(&XmlElement) -> Option<T>,
) -> Option<T> {
    let mut hits = rest.iter().enumerate().filter(|(_, e)| e.name == name);
    let (idx, el) = hits.next()?;
    if hits.next().is_some() {
        return None;
    }
    let value = convert(el)?;
    rest.remove(idx);
    Some(value)
}

fn detail_to_pb(detail: &Detail) -> pb::Detail {
    let mut rest: Vec<&XmlElement> = detail.elements().iter().collect();
    let mut out = pb::Detail {
        contact: take(&mut rest, "contact", to_contact),
        group: take(&mut rest, "__group", to_group),
        precision_location: take(&mut rest, "precisionlocation", to_precision),
        status: take(&mut rest, "status", to_status),
        takv: take(&mut rest, "takv", to_takv),
        track: take(&mut rest, "track", to_track),
        xml_detail: String::new(),
    };
    for el in rest {
        out.xml_detail.push_str(&el.to_xml());
    }
    out
}

fn fmt_f64(v: f64) -> String {
    v.to_string()
}

fn pb_to_elements(d: &pb::Detail) -> Result<Vec<XmlElement>> {
    let mut elements: Vec<XmlElement> = Vec::new();
    let mut typed: Vec<XmlElement> = Vec::new();
    if let Some(c) = &d.contact {
        let mut el = XmlElement::new("contact");
        if !c.endpoint.is_empty() {
            el = el.with_attr("endpoint", c.endpoint.clone());
        }
        typed.push(el.with_attr("callsign", c.callsign.clone()));
    }
    if let Some(g) = &d.group {
        typed.push(
            XmlElement::new("__group")
                .with_attr("name", g.name.clone())
                .with_attr("role", g.role.clone()),
        );
    }
    if let Some(p) = &d.precision_location {
        typed.push(
            XmlElement::new("precisionlocation")
                .with_attr("geopointsrc", p.geopointsrc.clone())
                .with_attr("altsrc", p.altsrc.clone()),
        );
    }
    if let Some(s) = &d.status {
        typed.push(XmlElement::new("status").with_attr("battery", s.battery.to_string()));
    }
    if let Some(t) = &d.takv {
        typed.push(
            XmlElement::new("takv")
                .with_attr("device", t.device.clone())
                .with_attr("platform", t.platform.clone())
                .with_attr("os", t.os.clone())
                .with_attr("version", t.version.clone()),
        );
    }
    if let Some(t) = &d.track {
        typed.push(
            XmlElement::new("track")
                .with_attr("speed", fmt_f64(t.speed))
                .with_attr("course", fmt_f64(t.course)),
        );
    }

    if !d.xml_detail.is_empty() {
        let wrapped = format!("<detail>{}</detail>", d.xml_detail);
        elements = XmlElement::parse(&wrapped)?.elements().cloned().collect();
    }
    // Per the spec, data already present in xmlDetail wins over the typed message.
    for t in typed {
        if !elements.iter().any(|e| e.name == t.name) {
            elements.push(t);
        }
    }
    Ok(elements)
}

fn opt(s: &str) -> Option<String> {
    (!s.is_empty()).then(|| s.to_owned())
}

fn unknown_to_none(v: f64) -> Option<f64> {
    (v < UNKNOWN).then_some(v)
}

/// Converts an event to its protobuf form.
pub fn event_to_pb(ev: &Event) -> Result<pb::CotEvent> {
    let has_detail = !ev.detail.is_empty();
    Ok(pb::CotEvent {
        r#type: ev.cot_type.to_string(),
        access: ev.access.clone().unwrap_or_default(),
        qos: ev.qos.clone().unwrap_or_default(),
        opex: ev.opex.clone().unwrap_or_default(),
        uid: ev.uid.clone(),
        send_time: to_ms(ev.time)?,
        start_time: to_ms(ev.start)?,
        stale_time: to_ms(ev.stale)?,
        how: ev.how.clone(),
        lat: ev.point.lat,
        lon: ev.point.lon,
        hae: ev.point.hae.unwrap_or(UNKNOWN),
        ce: ev.point.ce.unwrap_or(UNKNOWN),
        le: ev.point.le.unwrap_or(UNKNOWN),
        detail: has_detail.then(|| detail_to_pb(&ev.detail)),
    })
}

/// Converts a protobuf `CotEvent` to an event, validating it.
pub fn pb_to_event(m: &pb::CotEvent) -> Result<Event> {
    let point = Point {
        lat: m.lat,
        lon: m.lon,
        hae: unknown_to_none(m.hae),
        ce: unknown_to_none(m.ce),
        le: unknown_to_none(m.le),
    };
    let mut ev = Event::new(m.uid.clone(), CotType::new(m.r#type.clone())?, point);
    ev.how.clone_from(&m.how);
    ev.time = from_ms(m.send_time)?;
    ev.start = from_ms(m.start_time)?;
    ev.stale = from_ms(m.stale_time)?;
    ev.access = opt(&m.access);
    ev.qos = opt(&m.qos);
    ev.opex = opt(&m.opex);
    if let Some(d) = &m.detail {
        ev.detail = Detail::from_elements(pb_to_elements(d)?);
    }
    ev.validate()?;
    Ok(ev)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cottak_core::detail::{Contact, Group, Remarks, Status, Takv, Track};

    fn sample() -> Event {
        Event::new(
            "U1",
            "a-f-G-U-C".parse().unwrap(),
            Point::new(1.5, 2.5).unwrap().with_hae(10.0),
        )
        .with(Contact {
            callsign: Some("ALPHA".into()),
            endpoint: Some("*:-1:stcp".into()),
            ..Contact::default()
        })
        .with(Group {
            name: Some("Cyan".into()),
            role: Some("Team Lead".into()),
            ..Group::default()
        })
        .with(Takv {
            device: Some("d".into()),
            platform: Some("ATAK-CIV".into()),
            os: Some("34".into()),
            version: Some("5".into()),
            ..Takv::default()
        })
        .with(Track::new(90.0, 2.5))
        .with(Remarks::new("hello"))
    }

    #[test]
    fn typed_elements_move_into_messages_and_back() {
        let ev = sample();
        let m = event_to_pb(&ev).unwrap();
        let d = m.detail.as_ref().unwrap();
        assert_eq!(d.contact.as_ref().unwrap().callsign, "ALPHA");
        assert!(d.track.is_some() && d.takv.is_some() && d.group.is_some());
        assert_eq!(d.xml_detail, "<remarks>hello</remarks>");
        let back = pb_to_event(&m).unwrap();
        assert_eq!(back.point, ev.point);
        assert_eq!(back.detail.get::<Contact>(), ev.detail.get::<Contact>());
        assert_eq!(back.detail.get::<Track>(), ev.detail.get::<Track>());
        assert_eq!(back.detail.get::<Remarks>(), ev.detail.get::<Remarks>());
    }

    #[test]
    fn elements_that_do_not_map_exactly_stay_in_xml() {
        let ev = sample()
            // extra attribute: whole element must stay XML
            .with(Status {
                battery: Some(50.0),
                readiness: Some(true),
                ..Status::default()
            });
        let m = event_to_pb(&ev).unwrap();
        let d = m.detail.unwrap();
        assert!(d.status.is_none());
        assert!(d.xml_detail.contains("<status"));
    }

    #[test]
    fn non_canonical_battery_stays_in_xml() {
        let mut ev = sample();
        ev.detail
            .push_raw(XmlElement::new("status").with_attr("battery", "050"));
        let d = event_to_pb(&ev).unwrap().detail.unwrap();
        assert!(d.status.is_none());
        let back = pb_to_event(&event_to_pb(&ev).unwrap()).unwrap();
        assert_eq!(
            back.detail.raw("status").unwrap().attr("battery"),
            Some("050")
        );
    }

    #[test]
    fn duplicate_elements_stay_in_xml() {
        let mut ev = sample();
        ev.detail.push(Contact::new("SECOND"));
        let d = event_to_pb(&ev).unwrap().detail.unwrap();
        assert!(d.contact.is_none());
        assert_eq!(d.xml_detail.matches("<contact").count(), 2);
    }

    #[test]
    fn xml_detail_wins_over_typed_message() {
        let mut m = event_to_pb(&sample()).unwrap();
        let d = m.detail.as_mut().unwrap();
        d.xml_detail.push_str("<contact callsign=\"FROM-XML\"/>");
        let ev = pb_to_event(&m).unwrap();
        assert_eq!(ev.detail.get_all::<Contact>().len(), 1);
        assert_eq!(
            ev.detail.get::<Contact>().unwrap().callsign.as_deref(),
            Some("FROM-XML")
        );
    }

    #[test]
    fn unknown_point_values_round_trip() {
        let ev = Event::new("U", "a-f-G".parse().unwrap(), Point::new(0.0, 0.0).unwrap());
        let m = event_to_pb(&ev).unwrap();
        assert_eq!(m.hae.to_bits(), UNKNOWN.to_bits());
        let back = pb_to_event(&m).unwrap();
        assert_eq!(
            (back.point.hae, back.point.ce, back.point.le),
            (None, None, None)
        );
        assert!(m.detail.is_none());
    }

    #[test]
    fn time_is_millisecond_precise_and_pre_epoch_rejected() {
        let ev = sample();
        let back = pb_to_event(&event_to_pb(&ev).unwrap()).unwrap();
        assert_eq!(back.time, ev.time);
        let mut old = ev;
        old.time = OffsetDateTime::UNIX_EPOCH - time::Duration::seconds(1);
        assert!(matches!(event_to_pb(&old), Err(Error::Timestamp)));
    }

    #[test]
    fn bad_xml_detail_is_an_error_not_a_panic() {
        let mut m = event_to_pb(&sample()).unwrap();
        m.detail.as_mut().unwrap().xml_detail = "<unclosed>".into();
        assert!(pb_to_event(&m).is_err());
    }
}
