//! Emergency alerts (`b-a-o-*`, `b-a-g`).

use super::{base, how};
use crate::detail::{Contact, Emergency, Link};
use crate::error::Result;
use crate::event::Event;
use crate::point::Point;
use time::Duration;

/// Kind of emergency alert.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum EmergencyKind {
    /// 911 alert (`b-a-o-tbl`).
    Alert911,
    /// Ring the bell (`b-a-o-pan`).
    RingTheBell,
    /// Troops in contact (`b-a-o-opn`).
    InContact,
    /// Geofence breached (`b-a-g`).
    GeoFenceBreached,
}

impl EmergencyKind {
    /// CoT type of the alert.
    pub const fn cot_type(self) -> &'static str {
        match self {
            Self::Alert911 => "b-a-o-tbl",
            Self::RingTheBell => "b-a-o-pan",
            Self::InContact => "b-a-o-opn",
            Self::GeoFenceBreached => "b-a-g",
        }
    }

    /// Text placed in the `type` attribute of `<emergency>`.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Alert911 => "911 Alert",
            Self::RingTheBell => "Ring The Bell",
            Self::InContact => "In Contact",
            Self::GeoFenceBreached => "Geo-fence Breached",
        }
    }
}

/// CoT type used to cancel an alert.
pub const CANCEL_TYPE: &str = "b-a-o-can";

fn build(
    sender_uid: &str,
    callsign: &str,
    kind: EmergencyKind,
    point: Point,
    cancel: bool,
) -> Result<Event> {
    let cot_type = if cancel { CANCEL_TYPE } else { kind.cot_type() };
    Ok(base(
        format!("{sender_uid}-9-1-1"),
        cot_type,
        how::HUMAN,
        point,
        Duration::minutes(2),
    )?
    .with(Link {
        uid: Some(sender_uid.to_owned()),
        cot_type: Some("a-f-G-U-C".to_owned()),
        relation: Some("p-p".to_owned()),
        ..Link::default()
    })
    .with(Contact::new(callsign))
    .with(Emergency {
        emergency_type: Some(kind.label().to_owned()),
        cancel: Some(cancel),
        callsign: callsign.to_owned(),
        extra: Vec::new(),
    }))
}

/// Raises an emergency of the given kind at `point`.
pub fn raise(sender_uid: &str, callsign: &str, kind: EmergencyKind, point: Point) -> Result<Event> {
    build(sender_uid, callsign, kind, point, false)
}

/// Cancels a previously raised emergency.
pub fn cancel(
    sender_uid: &str,
    callsign: &str,
    kind: EmergencyKind,
    point: Point,
) -> Result<Event> {
    build(sender_uid, callsign, kind, point, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raise_and_cancel() {
        let p = Point::new(1.0, 2.0).unwrap();
        let on = raise("U1", "ALPHA", EmergencyKind::Alert911, p).unwrap();
        let off = cancel("U1", "ALPHA", EmergencyKind::Alert911, p).unwrap();
        assert_eq!(on.cot_type.as_str(), "b-a-o-tbl");
        assert_eq!(off.cot_type.as_str(), "b-a-o-can");
        assert_eq!(on.uid, off.uid, "cancel must reuse the alert uid");
        assert_eq!(on.detail.get::<Emergency>().unwrap().cancel, Some(false));
        assert_eq!(off.detail.get::<Emergency>().unwrap().cancel, Some(true));
        assert_eq!(on.to_xml_string().unwrap().parse::<Event>().unwrap(), on);
    }
}
