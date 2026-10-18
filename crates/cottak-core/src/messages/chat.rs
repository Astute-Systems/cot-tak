//! GeoChat messages (`b-t-f`).

use super::{base, how, null_point};
use crate::detail::{Chat, ChatGrp, Link, Remarks};
use crate::error::Result;
use crate::event::Event;
use crate::timestamp;
use time::{Duration, OffsetDateTime};

/// Chat room id and name used for messages to everyone.
pub const ALL_CHAT_ROOMS: &str = "All Chat Rooms";

/// Default parent contact group.
pub const ROOT_CONTACT_GROUP: &str = "RootContactGroup";

/// A GeoChat text message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeoChat {
    /// UID of the sender.
    pub sender_uid: String,
    /// Callsign of the sender.
    pub sender_callsign: String,
    /// Display name of the room (the recipient callsign for a direct message).
    pub chatroom: String,
    /// Identifier of the room (the recipient UID for a direct message).
    pub chatroom_id: String,
    /// Unique id of this message.
    pub message_id: String,
    /// Message text.
    pub text: String,
    /// When the message was written.
    pub time: OffsetDateTime,
}

impl GeoChat {
    /// A direct message to one contact.
    pub fn direct(
        sender_uid: impl Into<String>,
        sender_callsign: impl Into<String>,
        recipient_uid: impl Into<String>,
        recipient_callsign: impl Into<String>,
        text: impl Into<String>,
    ) -> Self {
        Self {
            sender_uid: sender_uid.into(),
            sender_callsign: sender_callsign.into(),
            chatroom: recipient_callsign.into(),
            chatroom_id: recipient_uid.into(),
            message_id: Event::random_uid(),
            text: text.into(),
            time: timestamp::now(),
        }
    }

    /// A message to the "All Chat Rooms" room.
    pub fn to_all(
        sender_uid: impl Into<String>,
        sender_callsign: impl Into<String>,
        text: impl Into<String>,
    ) -> Self {
        Self {
            sender_uid: sender_uid.into(),
            sender_callsign: sender_callsign.into(),
            chatroom: ALL_CHAT_ROOMS.to_owned(),
            chatroom_id: ALL_CHAT_ROOMS.to_owned(),
            message_id: Event::random_uid(),
            text: text.into(),
            time: timestamp::now(),
        }
    }

    /// The event UID: `GeoChat.<sender>.<room id>.<message id>`.
    pub fn event_uid(&self) -> String {
        format!(
            "GeoChat.{}.{}.{}",
            self.sender_uid, self.chatroom_id, self.message_id
        )
    }

    /// Builds the CoT event for this message.
    pub fn to_event(&self) -> Result<Event> {
        let mut ev = base(
            self.event_uid(),
            "b-t-f",
            how::HUMAN_GIS,
            null_point()?,
            Duration::days(1),
        )?;
        ev.time = self.time;
        ev.start = self.time;
        ev.stale = self.time + Duration::days(1);

        let chat = Chat {
            parent: Some(ROOT_CONTACT_GROUP.to_owned()),
            group_owner: Some(false),
            message_id: Some(self.message_id.clone()),
            chatroom: Some(self.chatroom.clone()),
            id: Some(self.chatroom_id.clone()),
            sender_callsign: Some(self.sender_callsign.clone()),
            groups: vec![ChatGrp {
                uid0: Some(self.sender_uid.clone()),
                uid1: Some(self.chatroom_id.clone()),
                id: Some(self.chatroom_id.clone()),
                extra: Vec::new(),
            }],
            ..Chat::default()
        };
        let link = Link {
            uid: Some(self.sender_uid.clone()),
            cot_type: Some("a-f-G-U-C".to_owned()),
            relation: Some("p-p".to_owned()),
            ..Link::default()
        };
        let remarks = Remarks {
            text: self.text.clone(),
            source: Some(format!("BAO.F.ATAK.{}", self.sender_uid)),
            source_id: Some(self.sender_uid.clone()),
            to: Some(self.chatroom_id.clone()),
            time: Some(timestamp::format(self.time)?),
            extra: Vec::new(),
        };
        Ok(ev.with(chat).with(link).with(remarks))
    }

    /// Reads a GeoChat message from an event, if it is one.
    pub fn from_event(ev: &Event) -> Option<Self> {
        if ev.cot_type.as_str() != "b-t-f" {
            return None;
        }
        let chat = ev.detail.get::<Chat>()?;
        let remarks = ev.detail.get::<Remarks>()?;
        let group = chat.groups.first();
        let sender_uid = group
            .and_then(|g| g.uid0.clone())
            .or_else(|| ev.detail.get::<Link>().and_then(|l| l.uid))
            .or_else(|| remarks.source_id.clone())?;
        let chatroom_id = chat
            .id
            .clone()
            .or_else(|| group.and_then(|g| g.uid1.clone()))
            .or_else(|| remarks.to.clone())?;
        Some(Self {
            sender_uid,
            sender_callsign: chat.sender_callsign.clone()?,
            chatroom: chat.chatroom.clone().unwrap_or_else(|| chatroom_id.clone()),
            chatroom_id,
            message_id: chat.message_id.clone().unwrap_or_else(|| ev.uid.clone()),
            text: remarks.text,
            time: remarks
                .time
                .as_deref()
                .and_then(|t| timestamp::parse(t).ok())
                .unwrap_or(ev.time),
        })
    }

    /// `true` if addressed to the "All Chat Rooms" room.
    pub fn is_broadcast(&self) -> bool {
        self.chatroom_id == ALL_CHAT_ROOMS
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direct_message_round_trip_via_xml() {
        let msg = GeoChat::direct("U1", "ALPHA", "U2", "BRAVO", "hello <b> & co");
        let ev = msg.to_event().unwrap();
        assert_eq!(ev.cot_type.as_str(), "b-t-f");
        assert!(ev.uid.starts_with("GeoChat.U1.U2."));

        let parsed: Event = ev.to_xml_string().unwrap().parse().unwrap();
        assert_eq!(parsed, ev);
        let back = GeoChat::from_event(&parsed).unwrap();
        assert_eq!(back, msg);
        assert!(!back.is_broadcast());
    }

    #[test]
    fn broadcast_message() {
        let msg = GeoChat::to_all("U1", "ALPHA", "all hands");
        let back = GeoChat::from_event(&msg.to_event().unwrap()).unwrap();
        assert!(back.is_broadcast());
        assert_eq!(back.chatroom, ALL_CHAT_ROOMS);
    }

    #[test]
    fn non_chat_events_are_ignored() {
        let ev = crate::messages::control::ping("U1").unwrap();
        assert!(GeoChat::from_event(&ev).is_none());
    }
}
