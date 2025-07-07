//
// Copyright (c) 2025, Astute Systems PTY LTD
//
// This file is part of the VivoeX SDK project developed by Astute Systems.
//
// See the commercial LICENSE file in the project root for full license details.
//

use cottak::config::Config;
use cottak::cot_base::{CursorOnTarget, Point};
use cottak::cot_bullseye::Bullseye;
use cottak::proto_sender::ProtoSender;

// Brisbane Botanical Gardens
const LONGITUDE: f64 = 153.0290348;
const LATITUDE: f64 = -27.4748166;

pub fn bullseye() {
    // Read the defaults from the configuration
    let config = Config::new();

    let bullseye = Bullseye::new(
        CursorOnTarget::new(
            "SPx_#89438".to_string(),
            None,
            None,
            "h-e".to_string(),
            Some("UNCLASSIFIED".to_string()),
            Point::new(LATITUDE, LONGITUDE),
        ),
        "SPx_#89438".to_string(),
        "Bullseye for SPx".to_string(),
        false,
        false,
        true,
        20.0,
        5,
        true,
        150.0,
        "SPx Radar Source".to_string(),
        "M".to_string(),
    );

    let mut sender = ProtoSender::new(&config.udp.address as &str, config.udp.port)
        .expect("Couldn't setup multicast");

    sender.send(&bullseye);
}
