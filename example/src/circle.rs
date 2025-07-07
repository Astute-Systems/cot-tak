//
// Copyright (c) 2025, Astute Systems PTY LTD
//
// This file is part of the VivoeX SDK project developed by Astute Systems.
//
// See the commercial LICENSE file in the project root for full license details.
//

use cottak::config::Config;
use cottak::cot_base::{CursorOnTarget, Point};
use cottak::cot_circle::Circle;
use cottak::proto_sender::ProtoSender;

// Story Bridge
const LONGITUDE: f64 = 153.0355417;
const LATITUDE: f64 = -27.4635122;

pub fn building() {
    // Read the defaults from the configuration
    let config = Config::new();

    // Brisbane City Hall
    let building = Circle::new(
        CursorOnTarget::new(
            "CITY_#89440".to_string(),
            None,
            None,
            "h-e".to_string(),
            Some("UNCLASSIFIED".to_string()),
            Point::new(LATITUDE, LONGITUDE),
        ),
        "Radar Rings".to_string(),
        "red".to_string(),
        "red".to_string(),
        2.0,
        "red".to_string(),
        "SPx Radar range rings".to_string(),
        8,
        50.0,
    );

    let mut sender = ProtoSender::new(&config.udp.address as &str, config.udp.port)
        .expect("Couldn't setup multicast");

    sender.send(&building);
}
