//
// Copyright (c) 2025, Astute Systems PTY LTD
//
// This file is part of the VivoeX SDK project developed by Astute Systems.
//
// See the commercial LICENSE file in the project root for full license details.
//
//! Create a chat message protobuf object.
//!

use crate::cot_base::{CursorOnTarget, Point, Sendable};
use crate::time::Time;

/// Bullseye object
/// * `uid` - Unique identifier
/// * `callsign` - Callsign
/// * `point` - Point
/// * `title` - Title
/// * `edge_to_center` - Edge to center
/// * `mils` - Mils
/// * `has_range_rings` - Has range rings
/// * `ring_dist` - Ring distance
/// * `ring_num` - Ring number
/// * `range_ring_visible` - Range ring visible
/// * `distance` - Distance
/// * `bullseye_uid` - Bullseye unique identifier
/// * `bearing_ref` - Bearing reference
///
pub struct Chat {
    pub cot: CursorOnTarget,
    pub callsign: String,
    pub contact: String,
    pub message: String,
}

impl Chat {
    pub fn new(callsign: String, contact: String, message: String) -> Chat {
        let mut cot = CursorOnTarget::new(
            "chat".to_string(),
            None,
            Some("h-g-i-g-oa".to_string()),
            "b-t-f".to_string(),
            Some("UNCLASSIFIED".to_string()),
            Point {
                latitude: 0.0,
                longitude: 0.0,
                hae: CursorOnTarget::HAE_NONE,
                ce: CursorOnTarget::CE_NONE,
                le: CursorOnTarget::LE_NONE,
            },
        );

        let uuid = cot.get_uid();
        let details = format! {r#"        <__chat id="{id}" chatroom="{chatroom}" senderCallsign="{callsign}" groupOwner="{owner}">
            <chatgrp id="{id2}" uid0="{uid0}" uid1="{uid1}"/>
        </__chat>
        <link uid="{link_uid}" type="{link_type}" relation="{relation}"/>
        <remarks source="A_SOURCE.{source}" sourceID="{source_id}" to="{to}" time="{time}">{message}</remarks>"#,
            id = contact,
            chatroom = contact,
            callsign = callsign,
            owner = false,
            id2 = contact,
            uid0 = uuid,
            uid1 = contact,
            link_uid = uuid,
            link_type = "a-f-G-U-C-I",
            relation = "p-p",
            source = uuid,
            source_id = uuid,
            to = contact,
            time = Time::time_to_string(Time::now()),
            message = message
        };

        cot.set_xml_detail(details);

        Chat {
            cot,
            callsign,
            contact,
            message,
        }
    }

    pub fn get_xml(&mut self) -> String {
        self.cot.get_xml()
    }

    pub fn get_xml_bytes(&mut self) -> Vec<u8> {
        self.cot.get_xml_bytes()
    }
}
impl Sendable for Chat {
    fn update_xml(&mut self) {
        let uuid = self.cot.uid.clone();
        let details = format! {r#"        <__chat id="{id}" chatroom="{chatroom}" senderCallsign="{callsign}" groupOwner="{owner}">
            <chatgrp id="{id2}" uid0="{uid0}" uid1="{uid1}"/>
        </__chat>
        <link uid="{link_uid}" type="{link_type}" relation="{relation}"/>
        <remarks source="A_SOURCE.{source}" sourceID="{source_id}" to="{to}" time="{time}">{message}</remarks>"#,
            id = self.contact,
            chatroom = self.contact,
            callsign = self.callsign,
            owner = false,
            id2 = self.contact,
            uid0 = uuid,
            uid1 = self.contact,
            link_uid = uuid,
            link_type = "a-f-G-U-C-I",
            relation = "p-p",
            source = uuid,
            source_id = uuid,
            to = self.contact,
            time = Time::time_to_string(Time::now()),
            message = self.message
        };
        self.cot.set_xml_detail(details);
    }
    fn modify_field(&mut self, field: &str, value: &str) {
        match field {
            "callsign" => self.callsign = value.to_string(),
            "contact" => self.contact = value.to_string(),
            "message" => self.message = value.to_string(),
            _ => println!("Field not found"),
        }
    }
    fn get_cot(&self) -> CursorOnTarget {
        self.cot.clone()    
    }
}