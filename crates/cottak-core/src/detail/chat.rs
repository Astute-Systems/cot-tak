//! GeoChat detail elements: `__chat` and `__chatReceipt`.

use super::DetailItem;
use crate::xml::{Attrs, XmlElement};

attrs_item!(
    /// `<chatgrp>`: sender and destination of a chat message.
    ChatGrp, "chatgrp",
    {
        /// UID of the sender.
        uid0: String => "uid0",
        /// UID of the recipient (or room).
        uid1: String => "uid1",
        /// Identifier of the conversation.
        id: String => "id",
    }
);

macro_rules! chat_item {
    ($(#[$meta:meta])* $name:ident, $elem:literal) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Default)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct $name {
            /// Parent contact group, normally `RootContactGroup`.
            pub parent: Option<String>,
            /// Whether the sender owns the group.
            pub group_owner: Option<bool>,
            /// Unique id of the message.
            pub message_id: Option<String>,
            /// Display name of the chat room (the recipient callsign for a direct message).
            pub chatroom: Option<String>,
            /// Identifier of the chat room (the recipient UID for a direct message).
            pub id: Option<String>,
            /// Callsign of the sender.
            pub sender_callsign: Option<String>,
            /// Sender/recipient pairs.
            pub groups: Vec<ChatGrp>,
            /// Attributes this crate does not model, preserved verbatim.
            pub extra: Vec<(String, String)>,
            /// Child elements other than `<chatgrp>`, preserved verbatim.
            pub extra_children: Vec<XmlElement>,
        }

        impl DetailItem for $name {
            const NAME: &'static str = $elem;

            fn from_xml(el: &XmlElement) -> Option<Self> {
                let mut a = Attrs::new(el);
                let mut groups = Vec::new();
                let mut extra_children = Vec::new();
                for c in el.elements() {
                    match ChatGrp::from_xml(c).filter(|_| c.name == ChatGrp::NAME) {
                        Some(g) => groups.push(g),
                        None => extra_children.push(c.clone()),
                    }
                }
                Some(Self {
                    parent: a.take("parent"),
                    group_owner: a.take_parsed("groupOwner"),
                    message_id: a.take("messageId"),
                    chatroom: a.take("chatroom"),
                    id: a.take("id"),
                    sender_callsign: a.take("senderCallsign"),
                    groups,
                    extra: a.into_extra(),
                    extra_children,
                })
            }

            fn to_xml(&self) -> XmlElement {
                let mut el = XmlElement::new($elem)
                    .with_opt_attr("parent", self.parent.clone())
                    .with_opt_attr("groupOwner", self.group_owner.map(|b| b.to_string()))
                    .with_opt_attr("messageId", self.message_id.clone())
                    .with_opt_attr("chatroom", self.chatroom.clone())
                    .with_opt_attr("id", self.id.clone())
                    .with_opt_attr("senderCallsign", self.sender_callsign.clone())
                    .with_extra_attrs(&self.extra);
                for g in &self.groups {
                    el = el.with_child(g.to_xml());
                }
                for c in &self.extra_children {
                    el = el.with_child(c.clone());
                }
                el
            }
        }
    };
}

chat_item!(
    /// `<__chat>`: marks an event as a GeoChat message.
    Chat, "__chat"
);
chat_item!(
    /// `<__chatReceipt>`: marks an event as a delivery or read receipt.
    ChatReceipt, "__chatReceipt"
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chat_round_trip_with_unknown_children() {
        let xml = r#"<__chat parent="RootContactGroup" groupOwner="false" messageId="m1"
            chatroom="BRAVO" id="U2" senderCallsign="ALPHA" x="1">
            <chatgrp uid0="U1" uid1="U2" id="U2"/><future a="b"/></__chat>"#;
        let el = XmlElement::parse(xml).unwrap();
        let chat = Chat::from_xml(&el).unwrap();
        assert_eq!(chat.group_owner, Some(false));
        assert_eq!(chat.groups.len(), 1);
        assert_eq!(chat.groups[0].uid1.as_deref(), Some("U2"));
        assert_eq!(chat.extra_children.len(), 1);
        assert_eq!(Chat::from_xml(&chat.to_xml()).unwrap(), chat);
    }
}
