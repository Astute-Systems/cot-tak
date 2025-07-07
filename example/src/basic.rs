//
// Copyright (c) 2025, Astute Systems PTY LTD
//
// This file is part of the VivoeX SDK project developed by Astute Systems.
//
// See the commercial LICENSE file in the project root for full license details.
//

use cottak::config::Config;
use cottak::cot::Cot;
use cottak::cot_base::{CursorOnTarget, Point};
use cottak::cot_types::{CoTType, CotClassification};
use cottak::proto_sender::ProtoSender;

// Brisbane City Hall
const LONGITUDE: f64 = 153.02351661489064;
const LATITUDE: f64 = -27.46891737509902;

pub fn building() {
    // Read the defaults from the configuration
    let config = Config::new();

    // Setup classification of the CoT message
    let cot_type =
        cottak::cot_types::lookup_cot_type(CoTType::GndBuilding, CotClassification::Friend);

    // Brisbane City Hall
    let building = Cot::new(
        CursorOnTarget::new(
            "CITY_#89436".to_string(),
            None,
            Some(cot_type.to_string()),
            "h-e".to_string(),
            Some("UNCLASSIFIED".to_string()),
            Point {
                latitude: LATITUDE,
                longitude: LONGITUDE,
                hae: CursorOnTarget::HAE_NONE,
                ce: CursorOnTarget::CE_NONE,
                le: CursorOnTarget::LE_NONE,
            },
        ),
        None,
        "Brisbane City Hall".to_string(),
        false,
        None,
        None,
        Some(0.0),
        None,
        None,
        None,
        None,
        None,
        None,
    );

    let mut sender = ProtoSender::new(&config.udp.address as &str, config.udp.port)
        .expect("Couldn't setup multicast");

    sender.send(&building);
}
