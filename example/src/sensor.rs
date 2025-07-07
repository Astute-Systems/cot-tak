//
// Copyright (c) 2025, Astute Systems PTY LTD
//
// This file is part of the VivoeX SDK project developed by Astute Systems.
//
// See the commercial LICENSE file in the project root for full license details.
//

use cottak::config::Config;
use cottak::cot_base::{CursorOnTarget, Point, Sendable};
use cottak::cot_sensor::Sensor;
use cottak::proto_sender::ProtoSender;

// Queens Street Mall
const CAMERA_LONGITUDE: f64 = 153.02443692573377;
const CAMERA_LATITUDE: f64 = -27.468600317016175;

// Cycle through red, orange, yellow, green, blue, indigo, violet
fn next(steps: i32, total_steps: i32) -> (f64, f64, f64) {
    let fraction = steps as f64 / total_steps as f64;
    let angle = fraction * 2.0 * std::f64::consts::PI;

    let new_red = 0.5 * (1.0 + (angle + 0.0 * std::f64::consts::PI / 3.0).cos());
    let new_green = 0.5 * (1.0 + (angle + 2.0 * std::f64::consts::PI / 3.0).cos());
    let new_blue = 0.5 * (1.0 + (angle + 4.0 * std::f64::consts::PI / 3.0).cos());

    (new_red, new_green, new_blue)
}

pub fn camera(animate: bool) {
    // Read the defaults from the configuration
    let config = Config::new();

    // Brisbane City Hall
    let mut camera = Sensor::new(
        CursorOnTarget::new(
            "CAMERA_#89437".to_string(),
            Some(240), // Four minutes, if non then default
            None,      // Not needed for sensor
            "h-e".to_string(),
            Some("UNCLASSIFIED".to_string()),
            Point {
                latitude: CAMERA_LATITUDE,
                longitude: CAMERA_LONGITUDE,
                hae: CursorOnTarget::HAE_NONE,
                ce: CursorOnTarget::CE_NONE,
                le: CursorOnTarget::LE_NONE,
            },
        ),
        "camera_uid".to_string(),
        "video_uid".to_string(),
        "Example Camera".to_string(),
        "Camera".to_string(),
        90.0,
        0.0,
        0.5,
        50.0,
        0.0,
        0.5,
        false,
        0.7,
        250,
    );

    let mut sender = ProtoSender::new(&config.udp.address as &str, config.udp.port)
        .expect("Couldn't setup multicast");

    if animate {
        // Sensor is going to rotate bearing 360 degrees
        let mut azimuth = 0;
        let mut fov = 0.0;
        let mut range = 0.0;

        for _ in 0..360 {
            azimuth += 1;
            if azimuth >= 360 {
                azimuth = 0;
            }
            camera.modify_field("azimuth", azimuth.to_string().as_str());
            sender.send(&camera);
            // Delay for 2ms
            std::thread::sleep(std::time::Duration::from_millis(5));
        }

        // Sensor changes fov and reverts back to original
        for _ in 0..100 {
            fov += 1.0;
            camera.modify_field("fov", fov.to_string().as_str());
            sender.send(&camera);
            // Delay for 2ms
            std::thread::sleep(std::time::Duration::from_millis(5));
        }

        for _ in 0..100 {
            fov -= 1.0;
            camera.modify_field("fov", fov.to_string().as_str());
            sender.send(&camera);
            // Delay for 2ms
            std::thread::sleep(std::time::Duration::from_millis(5));
        }

        // Sensor changes range and reverts back to original
        for _ in 0..100 {
            range += 1.0;
            camera.modify_field("range", range.to_string().as_str());
            sender.send(&camera);
            // Delay for 2ms
            std::thread::sleep(std::time::Duration::from_millis(5));
        }

        for _ in 0..100 {
            range -= 1.0;
            camera.modify_field("range", range.to_string().as_str());
            camera.update_xml();
            sender.send(&camera);
            // Delay for 2ms
            std::thread::sleep(std::time::Duration::from_millis(5));
        }

        // Cycle through red, orange, yellow, green, blue, indigo, violet
        for step in 0..300 {
            match next(step, 100) {
                (red, green, blue) => {
                    camera.modify_field("fov_red", red.to_string().as_str());
                    camera.modify_field("fov_green", green.to_string().as_str());
                    camera.modify_field("fov_blue", blue.to_string().as_str());
                }
            }
            sender.send(&camera);

            std::thread::sleep(std::time::Duration::from_millis(5));
        }
    }
}
