//
// Copyright (c) 2025, Astute Systems PTY LTD
//
// This file is part of the VivoeX SDK project developed by Astute Systems.
//
// See the commercial LICENSE file in the project root for full license details.
//

use crate::cot::Cot;
use crate::cot_base::Point;
use strum::IntoEnumIterator; // 0.17.1 //used in tests

#[allow(dead_code)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::cot_base::CursorOnTarget;
    use crate::cot_types::{
        cot_classification_to_string, lookup_cot_type, CoTType, CotClassification,
    };
    use crate::udp_sender::UdpSender;

    // Brisbane City Hall
    const LONGITUDE: f64 = 153.02351661489064;
    const LATITUDE: f64 = -27.46891737509902;

    #[test]
    fn test_get_license() {
        let path = Path::new("LICENSE");
        let license = fs::read_to_string(path).expect("Unable to read file");
        assert_eq!(license, "License will go here");
    }

    pub fn test_publish_all_cot_all(lat: f64, classification: CotClassification) {
        // Check all CoT types are published as friendly

        // Get the configuration instance
        let config = Config::get_instance();

        // Create a UDP sender
        let xml = UdpSender::new(
            &config.lock().unwrap().udp.address as &str,
            config.lock().unwrap().udp.port,
        )
        .expect("Couldn't setup multicast");

        // loop over all CoTTypes
        for (index, cot_type) in CoTType::iter().enumerate() {
            // Point
            let point = Point {
                latitude: lat,
                longitude: LONGITUDE,
                hae: CursorOnTarget::HAE_NONE,
                ce: CursorOnTarget::CE_NONE,
                le: CursorOnTarget::LE_NONE,
            };

            // Set the COT tye
            let cot_type = lookup_cot_type(cot_type, classification.clone());

            // Brisbane City Hall
            let cot = CursorOnTarget::new(
                "CITY_#89436".to_string(),
                None,
                Some(cot_type.to_string()),
                "h-e".to_string(),
                None,
                point,
            );
            let mut cot = Cot::new(
                cot,
                None,
                "CITY_HALL".to_string(),
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

            // Shift the longitude 100m to the right for each CoT type
            let long = LONGITUDE + (index as f64 * 0.001);

            // Generate unique uid and callsign
            let class = cot_classification_to_string(classification);
            cot.cot.uid = format!("{}_ID_{}", class, index);
            cot.cot.type_string = Some(cot_type.to_string());
            cot.cot.point.longitude = long;

            // println!(
            //     "Sending CoT message for {} {} {}",
            //     cot.uid.clone().unwrap(),
            //     cot.callsign.clone().unwrap(),
            //     cot.longitude.clone().unwrap()
            // );

            // Send the CoT message
            xml.send(&cot.cot.get_xml_bytes())
                .expect("Failed to send CoT message");

            xml.close();
        }
    }

    #[test]
    fn test_check_config() {
        // Check config.toml contains section [general]
        let path = Path::new("config.toml");
        let config = fs::read_to_string(path).expect("Unable to read file");
        // String contains [general]
        assert!(config.contains("[general]"));
        assert!(config.contains("[udp]"));
        assert!(config.contains("[tcp]"));
        assert!(config.contains("[ssl]"));
    }

    #[test]
    fn test_publish_all_cot_friendly() {
        test_publish_all_cot_all(LATITUDE + 0.001, CotClassification::Friend);
        test_publish_all_cot_all(LATITUDE + 0.002, CotClassification::Hostile);
        test_publish_all_cot_all(LATITUDE + 0.003, CotClassification::Neutral);
        test_publish_all_cot_all(LATITUDE + 0.004, CotClassification::Unknown);
        test_publish_all_cot_all(LATITUDE + 0.005, CotClassification::Pending);
        test_publish_all_cot_all(LATITUDE + 0.006, CotClassification::AssumedFriend);
        test_publish_all_cot_all(LATITUDE + 0.007, CotClassification::Suspect);
    }

    #[test]
    fn send_building() {
        // Read the configuration file
        let config =
            Config::from_file(&Config::get_config_file_path()).expect("Failed to read config file");

        // Setup classification of the CoT message
        let cot_type = lookup_cot_type(CoTType::GndBuilding, CotClassification::Friend);

        let mut building = Cot::new(
            CursorOnTarget::new(
                "CITY_#89436".to_string(),
                None,
                Some(cot_type.to_string()),
                "h-e".to_string(),
                None,
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
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );

        // Create a UDP sender
        let xml = UdpSender::new(&config.udp.address as &str, config.udp.port)
            .expect("Couldn't setup multicast");

        // Send the CoT message
        xml.send(&building.cot.get_xml_bytes())
            .expect("Failed to send CoT message");
    }

    #[test]
    fn data_package() {
        use crate::cot::FileShare;
        use crate::data_package::create_data_package;
        let mut fileshare = FileShare::new(
            "TestPackage".to_string(),
            uuid::Uuid::new_v4().to_string(),
            "test".to_string(),
            "http://localhost:8080".to_string(),
            0,
            "".to_string(),
            "us".to_string(),
            "ME".to_string(),
        );

        fileshare.add_file("./data/640x480.png");
        fileshare.add_file("./data/800x600.png");
        fileshare.add_file("./data/1920x1080.png");

        create_data_package(&mut fileshare, "../target/.").expect("Failed to create data package");
    }
}
