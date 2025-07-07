//
// Copyright (c) 2025, Astute Systems PTY LTD
//
// This file is part of the VivoeX SDK project developed by Astute Systems.
//
// See the commercial LICENSE file in the project root for full license details.
//

use cottak::config::Config;
use cottak::cot_base::{CursorOnTarget, Point};
use cottak::cot_casevac::Casevac;
use cottak::proto_sender::ProtoSender;
use cottak::time::Time;

// Casevac
const LONGITUDE: f64 = 153.0268557;
const LATITUDE: f64 = -27.4841133;

pub fn casevac() {
    // Read the defaults from the configuration
    let config = Config::new();

    // Brisbane City Hall
    let building = Casevac::new(
        CursorOnTarget::new(
            "MED_00001".to_string(),          // UID
            None,                             // Parent UID
            None,                             // Type
            "h-e".to_string(),                // How
            Some("UNCLASSIFIED".to_string()), // Classification
            Point::new(LATITUDE, LONGITUDE),  // Point
        ),
        "CALLSIGN".to_string(),                                // Callsign
        "Brisbane City Hall".to_string(),                      // Contact
        "Message".to_string(),                                 // Message
        Some("Hospital Hellipad".to_string()),                 // Remarks
        "PARENT".to_string(),                                  // Parent
        Time::time_to_string(Time::now()),                     // Time
        true,                                                  // Staleness
        "MED.16.024253".to_string(),                           // Med Title
        false,                                                 // Casevac
        0.0,                                                   // Frequency
        5,                                                     // Urgency
        2,                                                     // Priority
        15,                                                    // Routine
        true,                                                  // Extraction Equipment
        0,                                                     // Security
        3,                                                     // Markings
        30,                                                    // US Military
        6,                                                     // US Civilian
        1,                                                     // Non-US Military
        0,                                                     // Non-US Civilian
        2,                                                     // EPW
        0,                                                     // Child
        true,                                                  // Terrain None
        "None".to_string(),                                    // Obstacles
        "Operational".to_string(),                             // Medline Remarks
        0,                                                     // Zone Prot Selection
        "56J MQ 98448 54681".to_string(),                      // Zone Protected Coord
        "-27.5316104,152.9842845,NaN,NaN,NaN,???".to_string(), // Zone Prot Marker
        "HELLION".to_string(),                                 // Contact Callsign
    );

    let mut sender = ProtoSender::new(&config.udp.address as &str, config.udp.port)
        .expect("Couldn't setup multicast");

    sender.send(&building);
}
