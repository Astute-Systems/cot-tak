//
// Copyright (c) 2025, Astute Systems PTY LTD
//
// This file is part of the VivoeX SDK project developed by Astute Systems.
//
// See the commercial LICENSE file in the project root for full license details.
//
//! This module contains a TAK package creator.
//!
//! ``` xml
//! <?xml version="1.0" encoding="utf-16" standalone="yes"?>
//! <event version="2.0" uid="94006f11-57c3-413a-9378-730f1e766b6d" type="u-rb-a" time="2025-02-16T03:18:22.09Z" start="2025-02-16T03:18:22.09Z" stale="2025-02-17T03:18:22.09Z" how="h-e">
//!     <point lat="-27.5254515830341" lon="152.981411287509" hae="9999999" ce="9999999" le="9999999" />
//!     <detail>
//!         <contact callsign="Range &amp; Bearing 0" />
//!         <range value="817.237338333117" />
//!         <bearing value="101.494027056968" />
//!         <inclination value="0" />
//!         <rangeUnits value="1" />
//!         <bearingUnits value="0" />
//!         <northRef value="1" />
//!         <color value="-16711936" />
//!         <strokeColor value="-16711936" />
//!         <strokeWeight value="3.0" />
//!         <archive />
//!         <remarks />
//!     </detail>
//! </event>
//! ```
//!

use crate::cot_base::{CursorOnTarget, Sendable};
pub struct Measure {
    pub cot: CursorOnTarget,
    callsign: String,
    range: f64,
    bearing: f64,
    inclination: f64,
    range_units: i32,
    bearing_units: i32,
    north_ref: i32,
    color: i32,
    stroke_color: i32,
    stroke_weight: f64,
}

impl Measure {
    pub fn new(
        cot: CursorOnTarget,
        callsign: String,
        range: f64,
        bearing: f64,
        inclination: f64,
        range_units: i32,
        bearing_units: i32,
        north_ref: i32,
        color: i32,
        stroke_color: i32,
        stroke_weight: f64,
    ) -> Measure {
        let mut new_measure = Measure {
            cot,
            callsign,
            range,
            bearing,
            inclination,
            range_units,
            bearing_units,
            north_ref,
            color,
            stroke_color,
            stroke_weight,
        };

        new_measure.cot.type_string = Some("u-rb-a".to_string());
        new_measure.cot.how_string = "h-e".to_string();

        new_measure.update_xml();
        new_measure
    }
}
impl Sendable for Measure {
    fn update_xml(&mut self) {
        let details = format! {r#"        <contact callsign="{callsign}" />
        <range value="{range}" />
        <bearing value="{bearing}" />
        <inclination value="{inclination}" />
        <rangeUnits value="{range_units}" />
        <bearingUnits value="{bearing_units}" />
        <northRef value="{north_ef}" />
        <color value="{color}" />
        <strokeColor value="{stroke_color}" />
        <strokeWeight value="{stroke_weight}" />
        <archive />
        <remarks />"#,
            callsign = self.callsign,
            range = self.range,
            bearing = self.bearing,
            inclination = self.inclination,
            range_units = self.range_units,
            bearing_units = self.bearing_units,
            north_ef = self.north_ref,
            color = self.color,
            stroke_color = self.stroke_color,
            stroke_weight = self.stroke_weight,
        };

        self.cot.set_xml_detail(details);
    }
    fn modify_field(&mut self, field: &str, value: &str) {
        match field {
            "callsign" => self.callsign = value.to_string(),
            "range" => self.range = value.parse().unwrap(),
            "bearing" => self.bearing = value.parse().unwrap(),
            "inclination" => self.inclination = value.parse().unwrap(),
            "range_units" => self.range_units = value.parse().unwrap(),
            "bearing_units" => self.bearing_units = value.parse().unwrap(),
            "north_ref" => self.north_ref = value.parse().unwrap(),
            "color" => self.color = value.parse().unwrap(),
            "stroke_color" => self.stroke_color = value.parse().unwrap(),
            "stroke_weight" => self.stroke_weight = value.parse().unwrap(),
            _ => println!("Field not found"),
        }
    }
    fn get_cot(&self)-> CursorOnTarget {
        self.cot.clone()
    }
}
