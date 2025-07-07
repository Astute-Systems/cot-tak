//
// Copyright (c) 2025, Astute Systems PTY LTD
//
// This file is part of the VivoeX SDK project developed by Astute Systems.
//
// See the commercial LICENSE file in the project root for full license details.
//
//! Create a geofence XML/protobuf object.
//! ``` xml
//!
//! <?xml version="1.0" encoding="utf-16" standalone="yes"?>
//! <event version="2.0" uid="3d7a574e-bf1a-4a2b-940b-6cb0cfdfc603" type="u-d-f" time="2025-02-16T02:39:07.68Z" start="2025-02-16T02:39:07.68Z" stale="2025-02-23T02:39:07.68Z" how="h-e">
//!     <point lat="-27.5328117725345" lon="152.981293794545" hae="9999999" ce="9999999" le="9999999" />
//!     <detail>
//!         <contact callsign="Shape 1" />
//!         <strokeColor value="-16711936" />
//!         <fillColor value="822148864" />
//!         <remarks />
//!         <link point="-27.53389779,152.97971872" />
//!         <link point="-27.53316960,152.97965588" />
//!         <link point="-27.53246943,152.97956162" />
//!         <link point="-27.53067697,152.97976058" />
//!         <link point="-27.53083566,152.98122671" />
//!         <link point="-27.53110638,152.98187600" />
//!         <link point="-27.53128373,152.98282900" />
//!         <link point="-27.53138641,152.98321648" />
//!         <link point="-27.53236666,152.98301754" />
//!         <link point="-27.53232001,152.98228447" />
//!         <link point="-27.53224341,152.98179512" />
//!         <link point="-27.53224534,152.98178179" />
//!         <link point="-27.53320692,152.98156189" />
//!         <link point="-27.53392577,152.98141529" />
//!         <link point="-27.53487801,152.98126870" />
//!         <link point="-27.53601717,152.98104387" />
//!         <link point="-27.53577079,152.97998075" />
//!         <link point="-27.53389779,152.97971872" />
//!         <height value="0.00" />
//!         <height_unit value="4" />
//!         <archive />
//!     </detail>
//! </event>
//! ```
//!

use crate::cot_base::{CursorOnTarget, Point, Sendable};

pub struct FencePoint {
    pub latitude: f64,
    pub longitude: f64,
}

pub struct Geofence {
    pub cot: CursorOnTarget,
    callsign: String,
    contact: String,
    message: String,
    points: Vec<FencePoint>,
    height: f64,
    height_unit: i32,
}

impl Geofence {
    pub fn new(
        callsign: String,
        contact: String,
        message: String,
        points: Vec<FencePoint>,
        height: f64,
        height_unit: i32,
    ) -> Geofence {
        let cot = CursorOnTarget::new(
            "geofence".to_string(),
            None,
            Some("u-d-f".to_string()),
            "h-e".to_string(),
            Some("UNCLASSIFIED".to_string()),
            Point {
                latitude: 0.0,
                longitude: 0.0,
                hae: CursorOnTarget::HAE_NONE,
                ce: CursorOnTarget::CE_NONE,
                le: CursorOnTarget::LE_NONE,
            },
        );
        let mut geofence= Geofence {
            cot: cot,
            callsign: callsign,
            contact: contact,
            message: message,
            points: points,
            height: height,
            height_unit: height_unit,
        };
        geofence.update_xml();
        geofence
    }
}
impl Sendable for Geofence{
    fn update_xml(&mut self) {
        let mut links = "".to_string();
        for point in &self.points {
            links.push_str(&format!(
                r#"        <link point="{lat},{lon}" />"#,
                lat = point.latitude,
                lon = point.longitude,
            ));
            links.push_str("\n");
        }

        let details = format! {r#"        <contact callsign="{callsign}" />
        <strokeColor value="{stroke_colour}" />
        <fillColor value="{fill_value}" />
        <remarks>{remarks}</remarks>
        {links}
        <height value="{height}" />
        <height_unit value="{height_unit}" />
        <archive />"#,
            callsign = self.callsign,
            stroke_colour = -16711936,
            fill_value = 822148864,
            remarks = self.message,
            links = links,
            height = self.height,
            height_unit = self.height_unit,
        };

        self.cot.set_xml_detail(details);
    }
    fn modify_field(&mut self, field: &str, value: &str) {
        match field {
            "callsign" => self.callsign = value.to_string(),
            "contact" => self.contact = value.to_string(),
            "message" => self.message = value.to_string(),
            "height" => self.height = value.parse().unwrap(),
            "height_unit" => self.height_unit = value.parse().unwrap(),
            _ => (),
        }
        self.update_xml();
    }
    fn get_cot(&self)-> CursorOnTarget {
        self.cot.clone()
    }
}
