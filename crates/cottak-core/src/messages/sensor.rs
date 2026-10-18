//! Sensor field-of-view markers with an optional video link.

use super::{base, how};
use crate::detail::{Archive, Contact, Link, Sensor, Video};
use crate::error::Result;
use crate::event::Event;
use crate::point::Point;
use crate::timestamp;
use time::Duration;

/// CoT type of a sensor point-of-interest marker.
pub const SENSOR_TYPE: &str = "b-m-p-s-p-loc";

/// Builds a sensor marker at `point`, linked to its parent platform.
pub fn field_of_view(
    uid: impl Into<String>,
    callsign: &str,
    point: Point,
    parent_uid: &str,
    parent_callsign: &str,
    sensor: Sensor,
    video: Option<Video>,
) -> Result<Event> {
    let mut ev = base(
        uid,
        SENSOR_TYPE,
        how::MACHINE_GPS,
        point,
        Duration::minutes(5),
    )?
    .with(Contact::new(callsign))
    .with(Link {
        uid: Some(parent_uid.to_owned()),
        cot_type: Some("a-f-G-U-C-I".to_owned()),
        parent_callsign: Some(parent_callsign.to_owned()),
        relation: Some("p-p".to_owned()),
        production_time: Some(timestamp::format(timestamp::now())?),
        ..Link::default()
    })
    .with(Archive)
    .with(sensor);
    if let Some(v) = video {
        ev = ev.with(v);
    }
    Ok(ev)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::detail::ConnectionEntry;

    #[test]
    fn sensor_with_video() {
        let s = Sensor {
            fov: Some(60.0),
            azimuth: Some(90.0),
            range: Some(500.0),
            ..Sensor::default()
        };
        let v = Video {
            entries: vec![ConnectionEntry {
                protocol: Some("rtsp".into()),
                ..ConnectionEntry::default()
            }],
            ..Video::default()
        };
        let ev = field_of_view(
            "s1",
            "CAM",
            Point::new(1.0, 2.0).unwrap(),
            "U1",
            "ALPHA",
            s.clone(),
            Some(v),
        )
        .unwrap();
        assert_eq!(ev.detail.get::<Sensor>().unwrap(), s);
        assert!(ev.detail.get::<Video>().is_some());
        assert_eq!(ev.to_xml_string().unwrap().parse::<Event>().unwrap(), ev);
    }
}
