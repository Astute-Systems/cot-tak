//
// Copyright (c) 2025, Astute Systems PTY LTD
//
// This file is part of the VivoeX SDK project developed by Astute Systems.
//
// See the commercial LICENSE file in the project root for full license details.
//
//! Create a sensor XML/protobuf object.
//!
//! # Example XML for sensor message
//!
//! ``` xml
//! <?xml version="1.0" encoding="utf-16" standalone="yes"?>
//! <event version="2.0" uid="50509786-584e-4256-b6d7-871af8d43153" type="b-m-p-s-p-loc" time="2025-01-23T22:44:17.56Z" start="2025-01-23T22:44:17.56Z" stale="2025-01-30T22:44:17.56Z" how="h-g-i-g-o">
//!     <point lat="-27.4648075" lon="153.0353337" hae="9999999" ce="9999999" le="9999999" />
//!     <detail>
//!         <contact callsign="Story Bridge West" />
//!         <link type="a-f-G-U-C-I" uid="S-1-12-1-2335450818-1167661710-439542947-1473530812" parent_callsign="HELLION" relation="p-p" production_time="2025-01-23T22:41:32Z" />
//!         <archive />
//!         <sensor fov="58" fovBlue="0" displayMagneticReference="0" range="506" fovGreen="0" fovAlpha="0.223529411764706" hideFov="false" fovRed="0.498039215686275" azimuth="267" />
//!         <remarks />
//!         <__video uid="7fb5de7e-668e-4071-a811-aba67012b4a1" />
//!     </detail>
//! </event>
//! ```

use crate::cot_base::{CursorOnTarget, Sendable};
use crate::time::Time;
use std::fmt::{Debug, Formatter, Result as FmtResult};

pub struct Video {
    pub protocol: Option<String>,
    pub path: Option<String>,
    pub address: Option<String>,
    pub port: Option<u16>,
    pub uid: Option<String>,
    pub alias: Option<String>,
    pub rover_port: Option<i32>,
    pub rtsp_reliable: Option<bool>,
    pub ignore_embedded_klv: Option<bool>,
    pub network_timeout: Option<i32>,
    pub buffer_time: Option<i32>,
}

/// Sensor object
/// * `uid` - Unique identifier
/// * `video_uid` - Video unique identifier
/// * `parent_callsign` - Parent callsign
/// * `callsign` - Callsign
/// * `point` - Point
/// * `fov` - Field of view
/// * `fov_blue` - Field of view blue
/// * `display_magnetic_reference` - Display magnetic reference
/// * `range` - Range
/// * `fov_green` - Field of view green
/// * `fov_alpha` - Field of view alpha
/// * `hide_fov` - Hide field of view
/// * `fov_red` - Field of view red
/// * `azimuth` - Azimuth
/// * `config` - Configuration
///
pub struct Sensor {
    pub cot: CursorOnTarget,
    uid: String,
    video_uid: String,
    parent_callsign: String,
    callsign: String,
    fov: f64,
    fov_blue: f64,
    display_magnetic_reference: f64,
    range: f64,
    fov_green: f64,
    fov_alpha: f64,
    hide_fov: bool,
    fov_red: f64,
    azimuth: i32,
}

impl Debug for Sensor {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("Sensor")
            .field("cot", &self.cot)
            .field("fov", &self.fov)
            .field("fov_blue", &self.fov_blue)
            .field(
                "display_magnetic_reference",
                &self.display_magnetic_reference,
            )
            .field("range", &self.range)
            .field("fov_green", &self.fov_green)
            .field("fov_alpha", &self.fov_alpha)
            .field("hide_fov", &self.hide_fov)
            .field("fov_red", &self.fov_red)
            .field("azimuth", &self.azimuth)
            .finish()
    }
}

impl Sensor {
    pub fn new(
        cot: CursorOnTarget,
        uid: String,
        video_uid: String,
        parent_callsign: String,
        callsign: String,
        fov: f64,
        fov_blue: f64,
        display_magnetic_reference: f64,
        range: f64,
        fov_green: f64,
        fov_alpha: f64,
        hide_fov: bool,
        fov_red: f64,
        azimuth: i32,
    ) -> Self {
        // Need to set the correct type
        let mut updated_cot = cot;
        updated_cot.type_string = Some("b-m-p-s-p-loc".to_string());
        let mut new_sensor = Sensor {
            cot: updated_cot,
            uid: uid.clone(),
            video_uid: video_uid.clone(),
            callsign: callsign.clone(),
            parent_callsign: parent_callsign.clone(),
            fov,
            fov_blue,
            display_magnetic_reference,
            range,
            fov_green,
            fov_alpha,
            hide_fov,
            fov_red,
            azimuth,
        };

        new_sensor.update_xml();

        new_sensor
    }

    /// Get XML text
    /// # Returns
    /// * `String` - XML text
    /// # Example
    /// ```
    /// let xml = sensor.get_xml();
    /// ```
    ///
    pub fn get_xml(&mut self) -> String {
        self.cot.get_xml()
    }

    /// Get XML encoded bytes
    /// # Returns
    /// * `Vec<u8>` - XML encoded bytes
    /// # Example
    /// ```
    /// let xml = sensor.get_xml_bytes();
    /// ```
    ///
    pub fn get_xml_bytes(&mut self) -> Vec<u8> {
        self.cot.get_xml_bytes()
    }
}
impl Sendable for Sensor {
    fn update_xml(&mut self) {
        let mut detail = String::new();
        detail.push_str(&format!(
            "<contact callsign=\"{}\" />\n",
            self.callsign
        ));
        detail.push_str(&format!(
            "<link type=\"a-f-G-U-C-I\" uid=\"{}\" parent_callsign=\"{}\" relation=\"p-p\" production_time=\"{}\" />\n",
            self.video_uid,
            self.parent_callsign,
            Time::now().to_string()
        ));
        detail.push_str("<archive />\n");
        detail.push_str(&format!(
            "<sensor fov=\"{}\" fovBlue=\"{}\" displayMagneticReference=\"{}\" range=\"{}\" fovGreen=\"{}\" fovAlpha=\"{}\" hideFov=\"{}\" fovRed=\"{}\" azimuth=\"{}\" />\n",
            self.fov, self.fov_blue, self.display_magnetic_reference, self.range, self.fov_green, self.fov_alpha, self.hide_fov, self.fov_red, self.azimuth
        ));
        detail.push_str("<remarks />\n");
        detail.push_str(&format!(
            "<__video uid=\"{}\" />\n",
            self.video_uid
        ));

        self.cot.set_xml_detail(detail);
    }
    fn modify_field(&mut self, field: &str, value: &str) {
        match field {
            "uid" => {
                self.uid = value.to_string();
            }
            "video_uid" => {
                self.video_uid = value.to_string();
            }
            "parent_callsign" => {
                self.parent_callsign = value.to_string();
            }
            "callsign" => {
                self.callsign = value.to_string();
            }
            "fov" => {
                self.fov = value.parse().unwrap();
            }
            "fov_blue" => {
                self.fov_blue = value.parse().unwrap();
            }
            "display_magnetic_reference" => {
                self.display_magnetic_reference = value.parse().unwrap();
            }
            "range" => {
                self.range = value.parse().unwrap();
            }
            "fov_green" => {
                self.fov_green = value.parse().unwrap();
            }
            "fov_alpha" => {
                self.fov_alpha = value.parse().unwrap();
            }
            "hide_fov" => {
                self.hide_fov = value.parse().unwrap();
            }
            "fov_red" => {
                self.fov_red = value.parse().unwrap();
            }
            "azimuth" => {
                self.azimuth = value.parse().unwrap();
            }
            _ => {}
        }
        self.update_xml();
    }
    fn get_cot(&self)-> CursorOnTarget {
        self.cot.clone()
    }
    
}