//! Drone Remote ID (ASTM F3411, EN 4709-002) to CoT, with the `remoteid` feature.
//!
//! Decoding, per-aircraft merging and the spoofing verdict come from the
//! [`remoteid`] crate; this module maps the result onto the TAK picture:
//!
//! * [`aircraft`]: the aircraft as `a-u-A-M-F-Q` (unknown UAV) with track,
//!   callsign and remarks carrying the trust verdict and its reasons,
//! * [`operator`]: the operator or takeoff point from the System message as a
//!   `b-m-p-s-m` spot marker.
//!
//! ```
//! use cottak_convert::remote_id;
//! use remoteid::{BasicId, IdType, Location, Message, UaType, UasData};
//!
//! let mut uas = UasData::default();
//! uas.update(&Message::BasicId(BasicId::new(IdType::SerialNumber, UaType::HelicopterOrMultirotor, "1596F3507452")?));
//! let mut loc = Location::default();
//! loc.latitude = -27.47;
//! loc.longitude = 153.02;
//! uas.update(&Message::Location(loc));
//!
//! let event = remote_id::aircraft(&uas, None).expect("has id and position")?;
//! assert_eq!(event.uid, "rid-1596F3507452");
//! assert_eq!(event.cot_type.as_str(), "a-u-A-M-F-Q");
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

use crate::error::Result;
use cottak_core::detail::{Contact, Remarks, Track};
use cottak_core::{CotType, Event, Point};
use remoteid::trust::Assessment;
use remoteid::{Location, UasData};
use std::fmt::Write as _;

/// CoT type for a Remote ID aircraft: unknown affiliation UAV (`a-u-A-M-F-Q`).
pub const AIRCRAFT_TYPE: &str = "a-u-A-M-F-Q";
/// CoT type for the operator or takeoff point.
pub const OPERATOR_TYPE: &str = "b-m-p-s-m";

/// The aircraft as a CoT event, or `None` without an ID or a position.
///
/// With `trust`, the remarks start with the verdict (`spoofed`,
/// `suspicious`, `plausible`, `authenticated`) and its reasons, so a
/// spoofed broadcast is never shown as a plain drone.
pub fn aircraft(uas: &UasData, trust: Option<&Assessment>) -> Option<Result<Event>> {
    let id = uas.primary_id()?;
    let loc = uas.location.as_ref()?;
    let (lat, lon) = loc.position()?;
    Some(build_aircraft(uas, loc, id, lat, lon, trust))
}

fn build_aircraft(
    uas: &UasData,
    loc: &Location,
    id: &str,
    lat: f64,
    lon: f64,
    trust: Option<&Assessment>,
) -> Result<Event> {
    let mut point = Point::new(lat, lon)?;
    if let Some(h) = loc.altitude_geo {
        point = point.with_hae(f64::from(h));
    }
    let mut remarks = String::new();
    if let Some(a) = trust {
        let _ = write!(remarks, "Remote ID trust: {}", a.verdict);
        let reasons: Vec<&str> = a.reasons.iter().map(remoteid::Reason::name).collect();
        if !reasons.is_empty() {
            let _ = write!(remarks, " ({})", reasons.join(", "));
        }
        if let Some(tool) = a.spoofer {
            let _ = write!(remarks, ", tool: {}", tool.name());
        }
        remarks.push('.');
    }
    if let Some(op) = operator_id(uas) {
        let _ = write!(remarks, "{}Operator ID: {op}.", sep(&remarks));
    }
    if let Some(d) = uas
        .self_id
        .as_ref()
        .and_then(|s| s.description_str())
        .filter(|s| !s.is_empty())
    {
        let _ = write!(remarks, "{}Self ID: {d}.", sep(&remarks));
    }
    let mut event =
        Event::new(format!("rid-{id}"), CotType::new(AIRCRAFT_TYPE)?, point).with(Contact::new(id));
    if loc.direction.is_some() || loc.speed_horizontal.is_some() {
        event = event.with(Track {
            course: loc.direction.map(f64::from),
            speed: loc.speed_horizontal.map(f64::from),
            extra: Vec::new(),
        });
    }
    if !remarks.is_empty() {
        event = event.with(Remarks::new(remarks));
    }
    Ok(event)
}

/// The operator (or takeoff point) as a CoT spot marker, or `None` without
/// an ID or an operator position in the System message.
pub fn operator(uas: &UasData) -> Option<Result<Event>> {
    let id = uas.primary_id()?;
    let sys = uas.system.as_ref()?;
    let (lat, lon) = sys.operator_position()?;
    let callsign = match operator_id(uas) {
        Some(op) => format!("{op} (operator of {id})"),
        None => format!("Operator of {id}"),
    };
    Some((|| {
        let mut point = Point::new(lat, lon)?;
        if let Some(h) = sys.operator_altitude_geo {
            point = point.with_hae(f64::from(h));
        }
        Ok(
            Event::new(format!("rid-op-{id}"), CotType::new(OPERATOR_TYPE)?, point)
                .with(Contact::new(callsign)),
        )
    })())
}

fn operator_id(uas: &UasData) -> Option<&str> {
    uas.operator_id
        .as_ref()
        .and_then(|o| o.operator_id_str())
        .filter(|s| !s.is_empty())
}

fn sep(s: &str) -> &'static str {
    if s.is_empty() {
        ""
    } else {
        " "
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cottak_core::detail::{Contact, Remarks, Track};
    use remoteid::{BasicId, IdType, Message, OperatorId, Reason, System, UaType, Verdict};

    fn drone() -> UasData {
        let mut uas = UasData::default();
        uas.update(&Message::BasicId(
            BasicId::new(
                IdType::SerialNumber,
                UaType::HelicopterOrMultirotor,
                "1596F3507452",
            )
            .unwrap(),
        ));
        let loc = Location {
            latitude: -27.47,
            longitude: 153.02,
            altitude_geo: Some(120.0),
            direction: Some(90.0),
            speed_horizontal: Some(12.5),
            ..Location::default()
        };
        uas.update(&Message::Location(loc));
        uas.update(&Message::OperatorId(
            OperatorId::new("FIN87astrdge12k8").unwrap(),
        ));
        let sys = System {
            operator_latitude: -27.48,
            operator_longitude: 153.01,
            ..System::default()
        };
        uas.update(&Message::System(sys));
        uas
    }

    #[test]
    fn aircraft_carries_position_track_and_operator() {
        let e = aircraft(&drone(), None).unwrap().unwrap();
        assert_eq!(e.uid, "rid-1596F3507452");
        assert_eq!(e.cot_type.as_str(), AIRCRAFT_TYPE);
        assert_eq!(
            (e.point.lat, e.point.lon, e.point.hae),
            (-27.47, 153.02, Some(120.0))
        );
        let t: Track = e.detail.get().unwrap();
        assert_eq!((t.course, t.speed), (Some(90.0), Some(12.5)));
        assert_eq!(
            e.detail.get::<Contact>().unwrap().callsign.as_deref(),
            Some("1596F3507452")
        );
        assert_eq!(
            e.detail.get::<Remarks>().unwrap().text,
            "Operator ID: FIN87astrdge12k8."
        );
        e.validate().unwrap();
    }

    #[test]
    fn spoofed_verdict_lands_in_remarks() {
        let mut a = Assessment::default();
        a.verdict = Verdict::Spoofed;
        a.reasons.insert(Reason::IdentityClone);
        let e = aircraft(&drone(), Some(&a)).unwrap().unwrap();
        let text = e.detail.get::<Remarks>().unwrap().text;
        assert!(
            text.starts_with("Remote ID trust: spoofed (identity_clone)."),
            "{text}"
        );
    }

    #[test]
    fn operator_marker_from_system_message() {
        let e = operator(&drone()).unwrap().unwrap();
        assert_eq!(e.uid, "rid-op-1596F3507452");
        assert_eq!(e.cot_type.as_str(), OPERATOR_TYPE);
        assert_eq!((e.point.lat, e.point.lon), (-27.48, 153.01));
        assert_eq!(
            e.detail.get::<Contact>().unwrap().callsign.as_deref(),
            Some("FIN87astrdge12k8 (operator of 1596F3507452)")
        );
    }

    #[test]
    fn nothing_without_id_or_position() {
        let mut uas = drone();
        uas.location = None;
        assert!(aircraft(&uas, None).is_none());
        uas.system = None;
        assert!(operator(&uas).is_none());
        assert!(aircraft(&UasData::default(), None).is_none());
    }
}
