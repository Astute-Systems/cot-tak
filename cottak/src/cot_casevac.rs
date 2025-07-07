//
// Copyright (c) 2025, Astute Systems PTY LTD
//
// This file is part of the VivoeX SDK project developed by Astute Systems.
//
// See the commercial LICENSE file in the project root for full license details.
//
//! Create a casevac XML/protobuf object.
//!
//! ``` xml
//! <?xml version="1.0" encoding="utf-16" standalone="yes"?>
//! <event version="2.0" uid="02fad1aa-fe54-44bf-98e0-5f84f051e46c" type="b-r-f-h-c" time="2025-02-16T02:44:27.82Z" start="2025-02-16T02:44:27.82Z" stale="2025-02-17T02:44:27.82Z" how="h-g-i-g-o">
//!     <point lat="-27.5316104" lon="152.9842845" hae="9999999" ce="9999999" le="9999999" />
//!     <detail>
//!         <link type="a-f-G-U-C-I" uid="S-1-12-1-2335450818-1167661710-439542947-1473530812" parent_callsign="HELLION" relation="p-p" production_time="2025-02-16T02:42:53Z" />
//!         <archive />
//!         <status readiness="false" />
//!         <remarks>Operational </remarks>
//!         <_medevac_ title="MED.16.024253" casevac="false" freq="0.0" urgent="5" priority="2" routine="15" extraction_equipment="true" security="0" hlz_marking="3" us_military="30" us_civilian="6" nonus_military="1" nonus_civilian="0" epw="2" child="0" terrain_none="true" obstacles="None" medline_remarks="Operational " zone_prot_selection="0" zone_protected_coord="56J MQ 98448 54681" zone_prot_marker="-27.5316104,152.9842845,NaN,NaN,NaN,???" />
//!         <contact callsign="HELLION" />
//!     </detail>
//! </event>
//! ```

use crate::cot_base::{CursorOnTarget, Sendable};
use crate::time::Time;

/// Casevac object
/// * `cot` - CursorOnTarget
/// * `callsign` - Callsign
/// * `contact` - Contact
/// * `message` - Message
/// * `remarks` - Remarks
///
#[derive(Clone)]
pub struct Casevac {
    pub cot: CursorOnTarget,
    callsign: String,
    contact: String,
    message: String,
    remarks: Option<String>,
    parent_callsign: String,
    production_time: String,
    readiness: bool,
    med_title: String,
    casevac: bool,
    freq: f64,
    urgency: u8,
    priority: u8,
    routine: u8,
    extraction_equipment: bool,
    security: u8,
    markings: u8,
    us_military: u8,
    us_civilian: u8,
    nonus_military: u8,
    nonus_civilian: u8,
    epw: u8,
    child: u8,
    terrain_none: bool,
    obstacles: String,
    medline_remarks: String,
    zone_prot_selection: u8,
    zone_protected_coord: String,
    zone_prot_marker: String,
    contact_callsign: String,
}

impl Casevac {
    pub fn new(
        cot: CursorOnTarget,
        callsign: String,
        contact: String,
        message: String,
        remarks: Option<String>,
        parent_callsign: String,
        production_time: String,
        readiness: bool,
        med_title: String,
        casevac: bool,
        freq: f64,
        urgency: u8,
        priority: u8,
        routine: u8,
        extraction_equipment: bool,
        security: u8,
        markings: u8,
        us_military: u8,
        us_civilian: u8,
        nonus_military: u8,
        nonus_civilian: u8,
        epw: u8,
        child: u8,
        terrain_none: bool,
        obstacles: String,
        medline_remarks: String,
        zone_prot_selection: u8,
        zone_protected_coord: String,
        zone_prot_marker: String,
        contact_callsign: String,
    ) -> Casevac {
        let mut new_casevac = Casevac {
            cot,
            callsign,
            contact,
            message,
            remarks,
            parent_callsign,
            production_time,
            readiness,
            med_title,
            casevac,
            freq,
            urgency,
            priority,
            routine,
            extraction_equipment,
            security,
            markings,
            us_military,
            us_civilian,
            nonus_military,
            nonus_civilian,
            epw,
            child,
            terrain_none,
            obstacles,
            medline_remarks,
            zone_prot_selection,
            zone_protected_coord,
            zone_prot_marker,
            contact_callsign,
        };

        new_casevac.cot.type_string = Some("b-r-f-h-c".to_string());
        new_casevac.cot.how_string = "h-g-i-g-o".to_string();
        new_casevac.update_xml();
        new_casevac
    }
}
impl Sendable for Casevac {
    fn update_xml(&mut self) {
        let remark = match self.remarks {
            Some(ref r) => r.clone(),
            None => "".to_string(),
        };

        let details = format! {r#"         <link type="a-f-G-U-C-I" uid="{uid}" parent_callsign="{parent_callsign}" relation="p-p" production_time="{production_time}" />
         <archive />
         <status readiness="{readiness}" />
         <remarks>{remarks}</remarks>
         <_medevac_ title="{med_title}" casevac="{casevac}" freq="{freq}" urgent="{urgency}" priority="{priority}" routine="{routine}" extraction_equipment="{extraction_equipment}" security="{security}" hlz_marking="{markings}" us_military="{us_military}" us_civilian="{us_civilian}" nonus_military="{nonus_military}" nonus_civilian="{nonus_civilian}" epw="{epw}" child="{child}" terrain_none="{terrain_none}" obstacles="{obstacles}" medline_remarks="{medline_remarks}" zone_prot_selection="{zone_prot_selection}" zone_protected_coord="{zone_protected_coord}" zone_prot_marker="{zone_prot_marker}" />
         <contact callsign="{contact_callsign}" />"#,
            uid = self.cot.uid,
            parent_callsign = self.contact,
            production_time = Time::time_to_string(Time::now()),
            readiness = self.readiness,
            remarks = remark,
            med_title = self.med_title,
            casevac = self.casevac,
            freq = self.freq,
            urgency = self.urgency,
            priority = self.priority,
            routine = self.routine,
            extraction_equipment = self.extraction_equipment,
            security = self.security,
            markings = self.markings,
            us_military = self.us_military,
            us_civilian = self.us_civilian,
            nonus_military = self.nonus_military,
            nonus_civilian = self.nonus_civilian,
            epw = self.epw,
            child = self.child,
            terrain_none = self.terrain_none,
            obstacles = self.obstacles,
            medline_remarks = self.medline_remarks,
            zone_prot_selection = self.zone_prot_selection,
            zone_protected_coord = self.zone_protected_coord,
            zone_prot_marker = self.zone_prot_marker,
            contact_callsign = self.contact_callsign,
        };
        self.cot.set_xml_detail(details);
    }
    fn modify_field(&mut self, field: &str, value: &str) {
        match field {
            "callsign" => self.callsign = value.to_string(),
            "contact" => self.contact = value.to_string(),
            "message" => self.message = value.to_string(),
            "remarks" => self.remarks = Some(value.to_string()),
            "parent_callsign" => self.parent_callsign = value.to_string(),
            "production_time" => self.production_time = value.to_string(),
            "readiness" => self.readiness = value.parse().unwrap(),
            "med_title" => self.med_title = value.to_string(),
            "casevac" => self.casevac = value.parse().unwrap(),
            "freq" => self.freq = value.parse().unwrap(),
            "urgency" => self.urgency = value.parse().unwrap(),
            "priority" => self.priority = value.parse().unwrap(),
            "routine" => self.routine = value.parse().unwrap(),
            "extraction_equipment" => self.extraction_equipment = value.parse().unwrap(),
            "security" => self.security = value.parse().unwrap(),
            "markings" => self.markings = value.parse().unwrap(),
            "us_military" => self.us_military = value.parse().unwrap(),
            "us_civilian" => self.us_civilian = value.parse().unwrap(),
            "nonus_military" => self.nonus_military = value.parse().unwrap(),
            "nonus_civilian" => self.nonus_civilian = value.parse().unwrap(),
            "epw" => self.epw = value.parse().unwrap(),
            "child" => self.child = value.parse().unwrap(),
            "terrain_none" => self.terrain_none = value.parse().unwrap(),
            "obstacles" => self.obstacles = value.to_string(),
            "medline_remarks" => self.medline_remarks = value.to_string(),
            "zone_prot_selection" => self.zone_prot_selection = value.parse().unwrap(),
            "zone_protected_coord" => self.zone_protected_coord = value.to_string(),
            "zone_prot_marker" => self.zone_prot_marker = value.to_string(),
            "contact_callsign" => self.contact_callsign = value.to_string(),
            _ => println!("Field not found"),
        }
    }
    fn get_cot(&self) -> CursorOnTarget {
        self.cot.clone()
    }
}
