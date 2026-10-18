//! Medical evacuation request (9-line MEDEVAC / CASEVAC).

use super::{base, how};
use crate::detail::{Archive, Contact, Link, Medevac, Remarks, Status};
use crate::error::Result;
use crate::event::Event;
use crate::point::Point;
use crate::timestamp;
use time::Duration;

/// CoT type of a CASEVAC / MEDEVAC request.
pub const MEDEVAC_TYPE: &str = "b-r-f-h-c";

/// Builds a MEDEVAC request at the pickup `point` from the given sender.
pub fn request(
    sender_uid: &str,
    sender_callsign: &str,
    point: Point,
    medevac: Medevac,
    remarks: Option<&str>,
) -> Result<Event> {
    let ev = base(
        Event::random_uid(),
        MEDEVAC_TYPE,
        how::HUMAN_GIS,
        point,
        Duration::hours(1),
    )?;
    let link = Link {
        uid: Some(sender_uid.to_owned()),
        cot_type: Some("a-f-G-U-C-I".to_owned()),
        parent_callsign: Some(sender_callsign.to_owned()),
        relation: Some("p-p".to_owned()),
        production_time: Some(timestamp::format(timestamp::now())?),
        ..Link::default()
    };
    let status = Status {
        readiness: Some(false),
        ..Status::default()
    };
    Ok(ev
        .with(link)
        .with(Archive)
        .with(status)
        .with(Remarks::new(remarks.unwrap_or_default()))
        .with(medevac)
        .with(Contact::new(sender_callsign)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_round_trip() {
        let m = Medevac {
            title: Some("MEDEVAC 1".into()),
            casevac: Some(true),
            urgent: Some(1),
            freq: Some(30.5),
            ..Medevac::default()
        };
        let ev = request(
            "U1",
            "ALPHA",
            Point::new(1.0, 2.0).unwrap(),
            m.clone(),
            Some("bleeding"),
        )
        .unwrap();
        assert_eq!(ev.cot_type.as_str(), MEDEVAC_TYPE);
        assert_eq!(ev.detail.get::<Medevac>().unwrap(), m);
        assert_eq!(ev.detail.get::<Remarks>().unwrap().text, "bleeding");
        assert_eq!(ev.to_xml_string().unwrap().parse::<Event>().unwrap(), ev);
    }
}
