//! Control messages: ping, pong and delete.

use super::{base, how, null_point};
use crate::detail::Link;
use crate::error::Result;
use crate::event::Event;
use crate::xml::XmlElement;
use time::Duration;

/// CoT type of a ping.
pub const PING_TYPE: &str = "t-x-c-t";
/// CoT type of a pong (reply to a ping).
pub const PONG_TYPE: &str = "t-x-c-t-r";
/// CoT type of a delete request.
pub const DELETE_TYPE: &str = "t-x-d-d";

/// A keepalive ping from the client with the given UID.
pub fn ping(uid: impl Into<String>) -> Result<Event> {
    base(
        uid,
        PING_TYPE,
        how::MACHINE,
        null_point()?,
        Duration::seconds(20),
    )
}

/// A pong in reply to a ping.
pub fn pong(uid: impl Into<String>) -> Result<Event> {
    base(
        uid,
        PONG_TYPE,
        how::MACHINE,
        null_point()?,
        Duration::seconds(20),
    )
}

/// `true` if the event is a ping.
pub fn is_ping(ev: &Event) -> bool {
    ev.cot_type.as_str() == PING_TYPE
}

/// `true` if the event is a pong.
pub fn is_pong(ev: &Event) -> bool {
    ev.cot_type.as_str() == PONG_TYPE
}

/// Asks receivers to remove the entity `target_uid` (of type `target_type`).
pub fn delete(target_uid: impl Into<String>, target_type: impl Into<String>) -> Result<Event> {
    let target_uid = target_uid.into();
    let mut ev = base(
        Event::random_uid(),
        DELETE_TYPE,
        how::HUMAN_GIS,
        null_point()?,
        Duration::minutes(1),
    )?
    .with(Link {
        uid: Some(target_uid),
        cot_type: Some(target_type.into()),
        relation: Some("none".to_owned()),
        ..Link::default()
    });
    ev.detail.push_raw(XmlElement::new("__forcedelete"));
    Ok(ev)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ping_pong() {
        let p = ping("client-1").unwrap();
        assert!(is_ping(&p) && !is_pong(&p));
        assert!(is_pong(&pong("server").unwrap()));
        assert_eq!(p.point.lat, 0.0);
    }

    #[test]
    fn delete_targets_uid() {
        let ev = delete("TARGET-1", "a-f-G").unwrap();
        assert_eq!(
            ev.detail.get::<Link>().unwrap().uid.as_deref(),
            Some("TARGET-1")
        );
        assert!(ev.detail.raw("__forcedelete").is_some());
        assert_eq!(ev.to_xml_string().unwrap().parse::<Event>().unwrap(), ev);
    }
}
