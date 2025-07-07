//
// Copyright (c) 2025, Astute Systems PTY LTD
//
// This file is part of the VivoeX SDK project developed by Astute Systems.
//
// See the commercial LICENSE file in the project root for full license details.
//
//! Create a bullseye XML/protobuf object.
//!
//! # Example XML for bullseye message
//!
//! ``` xml
//! <?xml version="1.0" encoding="utf-16" standalone="yes"?>
//! <event version="2.0" uid="be8800d5-41da-4031-8fad-43fa9d4d152f" type="u-r-b-bullseye" time="2025-01-27T07:27:44.76Z" start="2025-01-27T07:27:44.76Z" stale="2025-01-28T07:27:44.76Z" how="h-e">
//! <point lat="-27.4275407437565" lon="153.019370809429" hae="9999999" ce="9999999" le="9999999" />
//! <detail>
//!     <archive />
//!     <contact callsign="Bullseye 1" />
//!     <bullseye title="Bullseye 1" edgeToCenter="false" mils="false" hasRangeRings="true" ringDist="100.0000" ringNum="1" rangeRingVisible="false" distance="1076.69" bullseyeUID="752e0714-6549-4a02-abbc-aab2f75d3bb7" bearingRef="M" />
//! </detail>
//! </event>
//! ```
//!

use crate::cot_base::{CursorOnTarget, Sendable};
use std::fmt::{Debug, Formatter, Result as FmtResult};

/// Bullseye object
/// * `uid` - Unique identifier
/// * `callsign` - Callsign
/// * `point` - Point
/// * `title` - Title
/// * `edge_to_center` - Edge to center
/// * `mils` - Mils
/// * `has_range_rings` - Has range rings
/// * `ring_dist` - Ring distance
/// * `ring_num` - Ring number
/// * `range_ring_visible` - Range ring visible
/// * `distance` - Distance
/// * `bullseye_uid` - Bullseye unique identifier
/// * `bearing_ref` - Bearing reference
///
pub struct Bullseye {
    pub cot: CursorOnTarget,
    callsign: String,
    title: String,
    edge_to_center: bool,
    mils: bool,
    has_range_rings: bool,
    ring_dist: f64,
    ring_num: i32,
    range_ring_visible: bool,
    distance: f64,
    bullseye_uid: String,
    bearing_ref: String,
}

impl Debug for Bullseye {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("Bullseye")
            .field("cot", &self.cot)
            .field("callsign", &self.callsign)
            .field("title", &self.title)
            .field("edge_to_center", &self.edge_to_center)
            .field("mils", &self.mils)
            .field("has_range_rings", &self.has_range_rings)
            .field("ring_dist", &self.ring_dist)
            .field("ring_num", &self.ring_num)
            .field("range_ring_visible", &self.range_ring_visible)
            .field("distance", &self.distance)
            .field("bullseye_uid", &self.bullseye_uid)
            .field("bearing_ref", &self.bearing_ref)
            .finish()
    }
}

impl Bullseye {
    pub fn new(
        cot: CursorOnTarget,
        callsign: String,
        title: String,
        edge_to_center: bool,
        mils: bool,
        has_range_rings: bool,
        ring_dist: f64,
        ring_num: i32,
        range_ring_visible: bool,
        distance: f64,
        bullseye_uid: String,
        bearing_ref: String,
    ) -> Bullseye {
        let mut new_bullseye = Bullseye {
            cot,
            callsign,
            title,
            edge_to_center,
            mils,
            has_range_rings,
            ring_dist,
            ring_num,
            range_ring_visible,
            distance,
            bullseye_uid,
            bearing_ref,
        };

        new_bullseye.cot.type_string = Some("u-r-b-bullseye".to_string());
        new_bullseye.update_xml();

        new_bullseye
    }

    pub fn get_xml(&mut self) -> String {
        self.cot.get_xml()
    }

    pub fn get_xml_bytes(&mut self) -> Vec<u8> {
        self.cot.get_xml_bytes()
    }
}
impl Sendable for Bullseye {
    fn update_xml(&mut self) {
        // Get UTime
        let details = format!(
            r#"        <archive />
        <contact callsign="{callsign}" />
        <bullseye title="{title}" edgeToCenter="{edge_to_centre}" mils="{mils}" hasRangeRings="{rings}" ringDist="{dist}" ringNum="{num}" rangeRingVisible="{visable}" distance="{distance}" bullseyeUID="{bullseye_id}" bearingRef="{bearing}" />"#,
            callsign = self.callsign,
            title = self.title,
            edge_to_centre = self.edge_to_center,
            mils = self.mils,
            rings = self.has_range_rings,
            dist = self.ring_dist,
            num = self.ring_num,
            visable = self.range_ring_visible,
            distance = self.distance,
            bullseye_id = self.bullseye_uid,
            bearing = self.bearing_ref
        );

        self.cot.set_xml_detail(details);
    }
    fn modify_field(&mut self, field: &str, value: &str) {
        match field {
            "callsign" => self.callsign = value.to_string(),
            "title" => self.title = value.to_string(),
            "edge_to_center" => self.edge_to_center = value.parse().unwrap(),
            "mils" => self.mils = value.parse().unwrap(),
            "has_range_rings" => self.has_range_rings = value.parse().unwrap(),
            "ring_dist" => self.ring_dist = value.parse().unwrap(),
            "ring_num" => self.ring_num = value.parse().unwrap(),
            "range_ring_visible" => self.range_ring_visible = value.parse().unwrap(),
            "distance" => self.distance = value.parse().unwrap(),
            "bullseye_uid" => self.bullseye_uid = value.to_string(),
            "bearing_ref" => self.bearing_ref = value.to_string(),
            _ => println!("Field not found"),
        }
    }
    fn get_cot(&self)-> CursorOnTarget {
        self.cot.clone()
    }   
}