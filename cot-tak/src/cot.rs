//
// Copyright (c) 2025, Astute Systems PTY LTD
//
// This file is part of the VivoeX SDK project developed by Astute Systems.
//
// See the commercial LICENSE file in the project root for full license details.
//
//! Create a generic XML/protobuf object.
//!
//! # Example XML for CoT message (Ferry)
//!
//! ``` xml
//! <? xml version="1.0" encoding="utf-16" standalone="yes"?>
//! <event version="2.0" type="a-f-S-X-M-F" uid="D86D5BD7AAF5C603A783FE8A6BF771A6_Barrambin II" how="m-g" time="2025-01-23T23:14:29.000000Z" start="2025-01-23T23:14:29.000000Z" stale="2025-01-23T23:16:29.000000Z" access="UNCLASSIFIED">
//! <point lat="-27.46538734436035" lon="153.0381317138672" le="0" hae="0" ce="0" />
//! <detail>
//!     <remarks>SEQ Ferries astutesys@takserver</remarks>
//!     <contact callsign="Barrambin II" />
//!     <track course="9999" speed="13.421639442443848" />
//!     <status />
//!     <takv version="1.0.0" platform="AstuteCot" os="Linux" device="GXA-1" />
//!     <_aircot_ cot_host_id="astutesys@takserver" squawk="4446" icao="7CAE40" />
//! </detail>
//! </event>
//! ```

use crate::config::Config;
use crate::cot_base::{self, Sendable};
use crate::data_package::create_data_package;
use crate::host::get_known_host_id;
use crate::{cot_base::CursorOnTarget, host::get_os};
use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::path::Path;
use uuid::Uuid;

pub struct Cot {
    pub cot: CursorOnTarget,
     remarks: Option<String>,
     callsign: String,
     military: bool,
     squawk: Option<String>,
     altitude: Option<f64>,
     speed: Option<f64>,
     battery: Option<i32>,
     course: Option<f64>,
     vertical_rate: Option<f64>,
     status: Option<String>,
     last_seen: Option<u64>,
     fileshare: Option<FileShare>,
}

impl Default for Cot {
    fn default() -> Self {
        Cot {
            cot: CursorOnTarget::default(),
            remarks: None,
            callsign: "UNKNOWN".to_string(),
            military: false,
            squawk: None,
            altitude: None,
            speed: None,
            battery: None,
            course: None,
            vertical_rate: None,
            status: None,
            last_seen: None,
            fileshare: None,
        }
    }
}

// Display
impl Debug for Cot {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("Cot")
            .field("cot", &self.cot)
            .field("remarks", &self.remarks)
            .field("callsign", &self.callsign)
            .field("military", &self.military)
            .field("squawk", &self.squawk)
            .field("altitude", &self.altitude)
            .field("speed", &self.speed)
            .field("battery", &self.battery)
            .field("course", &self.course)
            .field("vertical_rate", &self.vertical_rate)
            .field("status", &self.status)
            .field("last_seen", &self.last_seen)
            .field("fileshare", &self.fileshare)
            .finish()
    }
}

#[derive(Hash)]
pub struct FileShare {
    pub name: String,
    pub uid: String,
    pub filename: String,
    pub sender_url: String,
    pub size_in_bytes: i32,
    pub sha256: String,
    pub sender_uid: String,
    pub sender_callsign: String,
    pub attachment_paths: Vec<String>,
}

impl FileShare {
    pub fn new(
        name: String,
        uid: String,
        filename: String,
        sender_url: String,
        size_in_bytes: i32,
        sha256: String,
        sender_uid: String,
        sender_callsign: String,
    ) -> Self {
        FileShare {
            name,
            uid,
            filename,
            sender_url,
            size_in_bytes,
            sha256,
            sender_uid,
            sender_callsign,
            attachment_paths: Vec::new(),
        }
    }

    /// Hash the FileShare
    /// # Returns
    /// * A String
    /// # Example
    /// ```rust
    /// use cot_rs::cot_base::FileShare;
    /// let fileshare = FileShare::new("name".to_string(), "uid".to_string(), "filename".to_string(), "sender_url".to_string(), 100, "sha256".to_string(), "sender_uid".to_string(), "sender_callsign".to_string());
    /// let hash = fileshare.hash();
    /// ```
    ///
    pub fn hash(&self) -> String {
        let result = cot_base::CursorOnTarget::object_hash(self);
        format!("{:x}", result)
    }

    pub fn add_file(&mut self, file: &str) {
        // Push to back
        self.attachment_paths.push(file.to_string());
    }

    pub fn get_fileshare(fileshare: &mut FileShare) -> String {
        let config = Config::get_instance();
        let package_name =
            create_data_package(fileshare, config.lock().unwrap().general.filestore.as_str())
                .expect("Failed to create data package");

        // Get the filename with no path
        let filename = Path::new(&package_name)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();

        // Prepend dir onto URL
        let sender_url = format!("{}/{}", fileshare.sender_url, filename);

        // Lock the config
        let config = config.lock().unwrap();
        let config_callsign = config.general.callsign.clone();
        let config_uid = config.general.uuid.clone();
        // Unlock the config
        drop(config);

        format!(
            r#"
        <fileshare name="{name}" filename="{filename}" senderUrl="{senderUrl}" sizeInBytes="{sizeInBytes}" sha256="{sha256}" senderUid="{senderUid}" senderCallsign="{senderCallsign}"/>
        <ackrequest uid="{uid2}" ackrequested="{ackrequested}" tag="{tag}"/>"#,
            name = fileshare.name,
            filename = fileshare.filename,
            senderUrl = sender_url,
            sizeInBytes = fileshare.size_in_bytes,
            sha256 = fileshare.sha256,
            senderUid = config_uid,
            senderCallsign = config_callsign,
            uid2 = config_uid,
            ackrequested = true,
            tag = fileshare.name,
        )
    }
}

impl Debug for FileShare {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("FileShare")
            .field("name", &self.name)
            .field("uid", &self.uid)
            .field("filename", &self.filename)
            .field("sender_url", &self.sender_url)
            .field("size_in_bytes", &self.size_in_bytes)
            .field("sha256", &self.sha256)
            .field("sender_uid", &self.sender_uid)
            .field("sender_callsign", &self.sender_callsign)
            .field("attachment_paths", &self.attachment_paths)
            .finish()
    }
}

impl Cot {
    pub fn get_takv() -> String {
        format!(
            r#"<takv version="{version}" platform="{platform}" os="{os}" device="{host}" />"#,
            version = "1.0.0",
            platform = "RustCot",
            os = get_os(),
            host = get_known_host_id(),
        )
    }

    pub fn new(
        cot: CursorOnTarget,
        remarks: Option<String>,
        callsign: String,
        military: bool,
        squawk: Option<String>,
        altitude: Option<f64>,
        speed: Option<f64>,
        battery: Option<i32>,
        course: Option<f64>,
        vertical_rate: Option<f64>,
        status: Option<String>,
        last_seen: Option<u64>,
        fileshare: Option<FileShare>,
    ) -> Self {
        let mut new_cot = Cot {
            cot,
            remarks,
            callsign,
            military,
            squawk,
            altitude,
            speed,
            battery,
            course,
            vertical_rate,
            status,
            last_seen,
            fileshare,
        };

        new_cot.update_xml();

        new_cot
    }

    ///
    /// Generate a unique identifier
    /// # Returns
    /// * `String` - Unique identifier sting as a UUID V4
    /// # Example
    /// ```rust
    /// use cot_rs::cot_base::CursorOnTarget;
    /// let uid = CursorOnTarget::generate_uid();
    /// ```
    ///
    pub fn generate_uid() -> String {
        Uuid::new_v4().to_string()
    }


    
}
impl Sendable for Cot {

    /// modify a field 
    /// # Arguments
    /// * `field` - The field to modify
    /// * `value` - The value to set
    /// # Example
    /// ```rust     
    /// use cot_rs::cot::Cot;
    /// use cot_rs::cot_base::CursorOnTarget;
    /// let mut cot = Cot::default();
    /// cot.modify_field("callsign", "Barrambin II");
    /// ```
    
    fn modify_field(&mut self, field: &str, value: &str) {
        match field {
            "remarks" => self.remarks = Some(value.to_string()),
            "callsign" => self.callsign = value.to_string(),
            "military" => self.military = value.parse().unwrap(),
            "squawk" => self.squawk = Some(value.to_string()),
            "altitude" => self.altitude = Some(value.parse().unwrap()),
            "speed" => self.speed = Some(value.parse().unwrap()),
            "battery" => self.battery = Some(value.parse().unwrap()),
            "course" => self.course = Some(value.parse().unwrap()),
            "vertical_rate" => self.vertical_rate = Some(value.parse().unwrap()),
            "status" => self.status = Some(value.to_string()),
            "last_seen" => self.last_seen = Some(value.parse().unwrap()),
            _ => (),
        }
        self.update_xml();
    }

    fn update_xml(&mut self) {
        // If remarks some
        let remarks = match &self.remarks {
            Some(remarks) => format!(r#"{remarks}"#, remarks = remarks),
            None => "".to_string(),
        };

        // If speed some
        let speed = match self.speed {
            Some(speed) => format!(r#"speed="{}""#, speed),
            None => "".to_string(),
        };
        // If course some
        let course = match self.course {
            Some(course) => format!(r#"course="{}""#, course),
            None => "".to_string(),
        };
        // If status some
        let status = match &self.status {
            Some(status) => format!(r#"status="{}""#, status),
            None => "".to_string(),
        };

        self.cot.set_xml_detail(format!(
            r#"        <remarks>{remarks}</remarks>
        <contact callsign="{callsign}" />
        <track {course} {speed} />
        <status {status} />
        {takv}"#,
            remarks = remarks,
            callsign = self.callsign,
            course = course,
            speed = speed,
            status = status,
            takv = Cot::get_takv(),
        ));
    }
    fn get_cot(&self)-> CursorOnTarget {
        self.cot.clone()
    }
}
