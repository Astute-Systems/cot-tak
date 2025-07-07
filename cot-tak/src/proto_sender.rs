//
// Copyright (c) 2025, Astute Systems PTY LTD
//
// This file is part of the VivoeX SDK project developed by Astute Systems.
//
// See the commercial LICENSE file in the project root for full license details.
//
//! Functions for sending CursorOnTarget messages via UDP protobuf CoT Version 1

use protobuf::Message;

// Create the protobuf rust code
include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));

use crate::cot_base::{CursorOnTarget, Sendable};
use crate::host::{get_cot_version, get_known_host_id, get_os};
use crate::time::Time;
use crate::udp_sender::UdpSender;
use cotevent::CotEvent;
use std::io::Result;
use std::time::{SystemTime, UNIX_EPOCH};
use takmessage::TakMessage;
use takv::Takv;

pub struct ProtoSender {
    tak_message: TakMessage,
    sender: UdpSender,
}

/// ProtoSender sends CursorOnTarget messages via UDP protobuf CoT Version 1
/// # Example
/// ```
/// use cot::proto_sender::ProtoSender;
/// use cot::cot::CursorOnTarget;
/// use std::io::Result;
///
/// fn main() -> Result<()> {
///     let mut sender = ProtoSender::new("239.2.3.1", 6969)?;
///     let cot = CursorOnTarget::new(); // Create a CursorOnTarget message
///     sender.send(&cot);
///     Ok(())
/// }
/// ```
///
impl ProtoSender {
    /// Create a new ProtoSender
    pub fn new(addr: &str, port: u16) -> Result<Self> {
        let tak_message = TakMessage::new();
        // Create a new udp sender
        let sender = UdpSender::new(addr, port).expect("Couldn't setup multicast");

        Ok(ProtoSender {
            tak_message,
            sender,
        })
    }

    fn _current_time_millis() -> u128 {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        since_the_epoch.as_millis()
    }

    /// Send a CursorOnTarget message for an aircraft
    pub fn send(&mut self, cursor_on_target: &impl Sendable) {
        let cot:CursorOnTarget = cursor_on_target.get_cot();
        // Populate CotEvent
        let mut cot_event = CotEvent::new();
        cot_event.type_ = cot.type_string.as_ref().unwrap().to_string();
        cot_event.uid = cot.uid.clone();
        cot_event.how = "h-e".to_string();
        // Convert to u32
        cot_event.sendTime = Time::now_millis() as u64;
        cot_event.startTime = Time::now_millis() as u64;
        // Plus 2 mins
        cot_event.staleTime = Time::stale_time_millis() as u64 + 120000;
        cot_event.lon = cot.point.longitude;
        cot_event.lat = cot.point.latitude;
        // Height Above Ellipsoid
        cot_event.hae = cot.point.hae;
        // Circular Error
        cot_event.ce = cot.point.ce;
        // Linear Error
        cot_event.le = cot.point.le;

        // Populate TAKV
        let mut takv = Takv::new();
        takv.version = get_cot_version();
        takv.platform = "RustCot".to_string();
        takv.os = get_os();
        takv.device = get_known_host_id();

        // Assuming `Detail` is another struct that needs to be populated
        let mut detail = detail::Detail::new();
        detail.xmlDetail = cot.get_xml_detail().to_string();
        detail.takv = ::protobuf::MessageField::some(takv);
        cot_event.detail = ::protobuf::MessageField::some(detail);

        self.tak_message.cotEvent = ::protobuf::MessageField::some(cot_event);

        // Serialize the TakMessage to bytes
        let message_bytes = self
            .tak_message
            .write_to_bytes()
            .expect("Failed to serialize TakMessage");

        // Add UDP header bytes 191 1 191
        let mut header: Vec<u8> = vec![191, 1, 191];
        header.extend_from_slice(&message_bytes);

        self.sender
            .send(&header)
            .expect("Couldn't send data via UDP");
    }
}
