//
// Copyright (c) 2025, Astute Systems PTY LTD
//
// This file is part of the VivoeX SDK project developed by Astute Systems.
//
// See the commercial LICENSE file in the project root for full license details.
//
//! This module contains system information functions.
//!

use crate::cot::Cot;
use crate::cot_base::{CursorOnTarget, Point};
use crate::udp_sender::UdpSender;
use std::fmt::{Debug, Formatter, Result as FmtResult};

/// 225446       Time of fix 22:54:46 UTC
/// A            Navigation receiver warning A = OK, V = warning
/// 4916.45,N    Latitude 49 deg. 16.45 min North
/// 12311.12,W   Longitude 123 deg. 11.12 min West
/// 000.5        Speed over ground, Knots
/// 054.7        Course Made Good, True
/// 191194       Date of fix  19 November 1994
/// 020.3,E      Magnetic variation 20.3 deg East
/// *68          mandatory checksum
pub struct NmeaGprmc {
    time: String,
    nav_receiver_warning: String,
    latitude: f32,
    north_south: String,
    longitude: f32,
    east_west: String,
    speed_over_ground: f32,
    course_made_good: f32,
    date_of_fix: String,
    magnetic_variation: f32,
    checksum: String,
}

impl Debug for NmeaGprmc {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("NmeaGprmc")
            .field("time", &self.time)
            .field("nav_receiver_warning", &self.nav_receiver_warning)
            .field("latitude", &self.latitude)
            .field("north_south", &self.north_south)
            .field("longitude", &self.longitude)
            .field("east_west", &self.east_west)
            .field("speed_over_ground", &self.speed_over_ground)
            .field("course_made_good", &self.course_made_good)
            .field("date_of_fix", &self.date_of_fix)
            .field("magnetic_variation", &self.magnetic_variation)
            .field("checksum", &self.checksum)
            .finish()
    }
}

pub fn device_nmea_gprmc(nmea_string: &str) -> NmeaGprmc {
    // Strip whitespace
    let nmea_string = nmea_string.trim();
    // Strip \n \r \0
    let nmea_string = nmea_string.trim_end_matches(|c| c == '\r' || c == '\n' || c == '\0');

    // Loop through comma separated values
    let mut nmea_values = nmea_string.split(",");
    if "$GPRMC" != nmea_values.next().unwrap() {
        panic!("Not a GPRMC string: {}", nmea_string);
    }
    NmeaGprmc {
        time: nmea_values.next().unwrap().to_string(),
        nav_receiver_warning: nmea_values.next().unwrap().to_string(),
        latitude: nmea_values.next().unwrap().parse::<f32>().unwrap(),
        north_south: nmea_values.next().unwrap().to_string(),
        longitude: nmea_values.next().unwrap().parse::<f32>().unwrap(),
        east_west: nmea_values.next().unwrap().to_string(),
        speed_over_ground: nmea_values.next().unwrap().parse::<f32>().unwrap(),
        course_made_good: nmea_values.next().unwrap().parse::<f32>().unwrap(),
        date_of_fix: nmea_values.next().unwrap().to_string(),
        magnetic_variation: nmea_values.next().unwrap().parse::<f32>().unwrap(),
        checksum: nmea_values.next().unwrap().to_string(),
    }
}

pub fn transmit_nmea_gprmc(nmea_gprmc: &NmeaGprmc, cot_type: &str, xml: bool) {
    // Convert nmea_gprmc.latitude degrees / minutes / north south to decimal degrees
    let new_lat = nmea_gprmc.latitude / 100.0;
    let new_lat = new_lat + (nmea_gprmc.latitude % 100.0) / 60.0;
    let new_lat = if nmea_gprmc.north_south == "S" {
        -new_lat
    } else {
        new_lat
    };

    // Convert nmea_gprmc.longitude degrees / minutes / east west to decimal degrees
    let new_lon = nmea_gprmc.longitude / 100.0;
    let new_lon = new_lon + (nmea_gprmc.longitude % 100.0) / 60.0;
    let new_lon = if nmea_gprmc.east_west == "W" {
        -new_lon
    } else {
        new_lon
    };

    let point = Point {
        latitude: new_lat as f64,
        longitude: new_lon as f64,
        hae: CursorOnTarget::HAE_NONE,
        ce: CursorOnTarget::CE_NONE,
        le: CursorOnTarget::LE_NONE,
    };

    let remark: String = format!(
        "GPRMC: Time: {}, Nav Receiver Warning: {}, Date of Fix: {}",
        nmea_gprmc.time, nmea_gprmc.nav_receiver_warning, nmea_gprmc.date_of_fix,
    );

    let callsign = "Ship".to_string();

    let mut sensor = Cot::new(
        CursorOnTarget::new(
            "Ship_5".to_string(),
            None,
            Some(cot_type.to_string()),
            "h-e".to_string(),
            None,
            point,
        ),
        Some(remark),
        callsign,
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

    // Send the CoT message over UDP
    if xml {
        // Send XML
        let sender = UdpSender::new("239.2.3.1", 6969).expect("Couldn't setup multicast");
        sender
            .send(&sensor.cot.get_xml_bytes())
            .expect("Couldn't send data via UDP");
    } else {
        // Send protobuf
    }
}
