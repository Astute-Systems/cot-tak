//
// Copyright (c) 2025, Astute Systems PTY LTD
//
// This file is part of the VivoeX SDK project developed by Astute Systems.
//
// See the commercial LICENSE file in the project root for full license details.
//

use cottak::config::Config;
use cottak::cot::{Cot, FileShare};
use cottak::cot_base::{CursorOnTarget, Point};
use cottak::cot_types::{CoTType, CotClassification};
use cottak::proto_sender::ProtoSender;

// Victoria Bridge
const BRIDGE_LONGITUDE: f64 = 153.0212346463404;
const BRIDGE_LATITUDE: f64 = -27.472237211183156;
const UID: &str = "CITY_#89437";

pub fn fileshare(send_data_package: bool) {
    let config = Config::new();
    let mut fileshare_option: Option<FileShare> = None;

    if send_data_package {
        let mut fileshare = FileShare::new(
            "TestPackage".to_string(),
            UID.to_string(),
            "test".to_string(),
            config.get_address(),
            0,
            "".to_string(),
            "us".to_string(),
            "ME".to_string(),
        );

        fileshare.add_file("./cot/src/data/640x480.png");
        fileshare.add_file("./cot/src/data/800x600.png");
        fileshare.add_file("./cot/src/data/1920x1080.png");
        fileshare_option = Some(fileshare);
    }

    // Setup classification of the CoT message
    let cot_type = cottak::cot_types::lookup_cot_type(
        CoTType::GndStructureCivilianBridge,
        CotClassification::Neutral,
    );

    // Brisbane City Hall
    let bridge = Cot::new(
        CursorOnTarget::new(
            UID.to_string(),
            Some(120), // Two minutes, if non then default
            Some(cot_type.to_string()),
            "h-e".to_string(),
            Some("UNCLASSIFIED".to_string()),
            Point {
                latitude: BRIDGE_LATITUDE,
                longitude: BRIDGE_LONGITUDE,
                hae: CursorOnTarget::HAE_NONE,
                ce: CursorOnTarget::CE_NONE,
                le: CursorOnTarget::LE_NONE,
            },
        ),
        Some("A test package containing three different images of various sizes".to_string()),
        "Victoria Bridge".to_string(),
        false,
        None,
        None,
        Some(0.0),
        None,
        None,
        None,
        None,
        None,
        fileshare_option,
    );

    // Print the address
    let mut sender = ProtoSender::new(&config.udp.address as &str, config.udp.port)
        .expect("Couldn't setup multicast");

    sender.send(&bridge);
}
