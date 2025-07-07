//
// Copyright (c) 2025, Astute Systems PTY LTD
//
// This file is part of the VivoeX SDK project developed by Astute Systems.
//
// See the commercial LICENSE file in the project root for full license details.
//
//! Create a circle XML/protobuf object.
//!
//! # Example XML for Circle message
//!
//! ``` xml
//! <?xml version="1.0" encoding="utf-16" standalone="yes"?>
//! <event version="2.0" uid="527456a2-d5f0-4df9-af66-c959a176c0ac" type="u-r-b-c-c" time="2025-02-07T04:04:21.67Z" start="2025-02-07T04:04:21.67Z" stale="2025-02-08T04:04:21.67Z" how="h-e">
//!     <point lat="52.1062697071755" lon="2.28286608929758" hae="9999999" ce="9999999" le="9999999" />
//!     <detail>
//!         <contact callsign="R&amp;B Circle 2" />
//!         <rangeUnits value="1" />
//!         <shape>
//!             <ellipse minor="5000" angle="360" major="5000" />
//!             <ellipse minor="10000" angle="360" major="10000" />
//!             <ellipse minor="15000" angle="360" major="15000" />
//!             <ellipse minor="20000" angle="360" major="20000" />
//!             <ellipse minor="25000" angle="360" major="25000" />
//!             <ellipse minor="30000" angle="360" major="30000" />
//!             <ellipse minor="35000" angle="360" major="35000" />
//!             <ellipse minor="40000" angle="360" major="40000" />
//!             <ellipse minor="45000" angle="360" major="45000" />
//!             <ellipse minor="50000" angle="360" major="50000" />
//!             <link relation="p-c" uid="527456a2-d5f0-4df9-af66-c959a176c0ac.Style" type="b-x-KmlStyle">
//!                 <Style>
//!                     <LineStyle>
//!                         <color>ffff0000</color>
//!                         <width>3</width>
//!                         <alpha>0</alpha>
//!                     </LineStyle>
//!                     <PolyStyle>
//!                         <color>00ff0000</color>
//!                     </PolyStyle>
//!                 </Style>
//!             </link>
//!         </shape>
//!         <color argb="-16711681" />
//!         <strokeColor value="-16711681" />
//!         <strokeWeight value="3.0" />
//!         <fillColor value="16776960" />
//!         <archive />
//!         <remarks />
//!     </detail>
//! </event>
//! ```
//!

use crate::cot_base::{CursorOnTarget, Sendable};
use std::fmt::{Debug, Formatter, Result as FmtResult};

/// Circle object
/// * `callsign` - Callsign
/// * `point` - Point
/// * `color` - Color
/// * `stroke_color` - Stroke color
/// * `stroke_weight` - Stroke weight
/// * `fill_color` - Fill color
/// * `remarks` - Remarks
///     
pub struct Circle {
    pub cot: CursorOnTarget,
    callsign: String,
    color: String,
    stroke_color: String,
    stroke_weight: f64,
    fill_color: String,
    remarks: String,
    rings: i32,
    spacing: f64,
}

impl Debug for Circle {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("Circle")
            .field("cot", &self.cot)
            .field("callsign", &self.callsign)
            .field("color", &self.color)
            .field("stroke_color", &self.stroke_color)
            .field("stroke_weight", &self.stroke_weight)
            .field("fill_color", &self.fill_color)
            .field("remarks", &self.remarks)
            .field("rings", &self.rings)
            .field("spacing", &self.spacing)
            .finish()
    }
}

impl Circle {
    pub fn new(
        cot: CursorOnTarget,
        callsign: String,
        color: String,
        stroke_color: String,
        stroke_weight: f64,
        fill_color: String,
        remarks: String,
        rings: i32,
        spacing: f64,
    ) -> Self {
        let mut new_circle = Circle {
            cot,
            callsign,
            color,
            stroke_color,
            stroke_weight,
            fill_color,
            remarks,
            rings,
            spacing,
        };

        new_circle.cot.type_string = Some("u-r-b-c-c".to_string());
        new_circle.update_xml();
        new_circle
    }

    pub fn get_xml(&mut self) -> String {
        self.cot.get_xml()
    }

    pub fn get_xml_bytes(&mut self) -> Vec<u8> {
        self.cot.get_xml_bytes()

    }
    pub fn get_field(&self, field: &str) -> String {
        match field {
            "callsign" => self.callsign.to_string(),
            "color" => self.color.to_string(),
            "stroke_color" => self.stroke_color.to_string(),
            "stroke_weight" => self.stroke_weight.to_string(),
            "fill_color" => self.fill_color.to_string(),
            "remarks" => self.remarks.to_string(),
            "rings" => self.rings.to_string(),
            "spacing" => self.spacing.to_string(),
            _ => "".to_string(),
        }
    }
}
impl Sendable for Circle {
    fn update_xml(&mut self) {
        let mut elipses = "".to_string();
        // Create the elipses using the rings and spacing
        for i in 1..self.rings + 1 {
            let minor = i as f64 * self.spacing;
            let major = i as f64 * self.spacing;
            elipses.push_str(&format!(
                r#"            <ellipse minor="{minor}" angle="360" major="{major}" />"#,
                minor = minor,
                major = major,
            ));
            // If not the last item append a line break
            if i != self.rings {
                elipses.push_str("\n");
            }
        }

        let details = format!(
            r#"        <contact callsign="{callsign}" />
        <rangeUnits value="1" />
        <shape>
{eclipses}
            <link relation="p-c" uid="{uid2}" type="b-x-KmlStyle">
                <Style>
                    <LineStyle>
                        <color>00fff000</color>
                        <width>3</width>
                        <alpha>0.5</alpha>
                    </LineStyle>
                    <PolyStyle>
                        <color>00ff0000</color>
                    </PolyStyle>
                </Style>
            </link>
        </shape>
        <color argb="-16711681" />
        <strokeColor value="-16711681" />
        <strokeWeight value="3.0" />
        <fillColor value="16776960" />
        <archive />
        <remarks />"#,
            callsign = self.callsign.to_string(),
            eclipses = elipses,
            uid2 = "527456a2-d5f0-4df9-af66-c959a176c0ac.Style",
        );
        self.cot.set_xml_detail(details);
    
    }
    fn modify_field(&mut self, field: &str, value: &str) {
        match field {
            "callsign" => self.callsign = value.to_string(),
            "color" => self.color = value.to_string(),
            "stroke_color" => self.stroke_color = value.to_string(),
            "stroke_weight" => self.stroke_weight = value.parse().unwrap(),
            "fill_color" => self.fill_color = value.to_string(),
            "remarks" => self.remarks = value.to_string(),
            "rings" => self.rings = value.parse().unwrap(),
            "spacing" => self.spacing = value.parse().unwrap(),
            _ => (),
        }
        self.update_xml();
    }
    fn get_cot(&self)-> CursorOnTarget {
        self.cot.clone()
    }
}
