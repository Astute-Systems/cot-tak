//
// Copyright (c) 2025, Astute Systems PTY LTD
//
// This file is part of the VivoeX SDK project developed by Astute Systems.
//
// See the commercial LICENSE file in the project root for full license details.
//
//! The basic CoT message structure used by all CoT messages.

use crate::config::Config;
use crate::cot_types::{lookup_cot_type, CoTType, CotClassification};
use crate::time::Time;
use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

/// Point data includes GPS coordinates
/// * `latitude` - Latitude in decimal degrees
/// * `longitude` - Longitude in decimal degrees
/// * `hae` - Height above the WGS ellipsoid in meters
/// * `ce` - Circular Error in meters
/// * `le` - Linear Error in meters
///
#[derive(Clone)]
pub struct Point {
    pub latitude: f64,
    pub longitude: f64,
    pub hae: f64, // Height above the WGS ellipsoid in meters
    pub ce: f64,  // Circular Error in meters
    pub le: f64,  // Linear Error in meters
}

impl Default for Point {
    fn default() -> Self {
        Point {
            latitude: 0.0,
            longitude: 0.0,
            hae: CursorOnTarget::HAE_NONE,
            ce: CursorOnTarget::CE_NONE,
            le: CursorOnTarget::LE_NONE,
        }
    }
}

impl Debug for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("Point")
            .field("latitude", &self.latitude)
            .field("longitude", &self.longitude)
            .field("hae", &self.hae)
            .field("ce", &self.ce)
            .field("le", &self.le)
            .finish()
    }
}

impl Point {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Point {
            latitude: latitude,
            longitude: longitude,
            hae: CursorOnTarget::HAE_NONE,
            ce: CursorOnTarget::CE_NONE,
            le: CursorOnTarget::LE_NONE,
        }
    }
}

///
/// Cursor on Target message
/// * `uid` - Unique identifier is some otherwise will create UUID V4
/// * `stale_time` - Stale time in seconds, if some. Otherwise default
/// * `type_string` - Type string
/// * `how_string` - How string
/// * `access` - Access i.e. "UNCLASSIFIED"
/// * `point` - Point data includes GPS coordinates
/// * `detail` - Detail XML
/// * `config` - Configuration
///
#[derive(Clone)]
pub struct CursorOnTarget {
    pub uid: String,
    pub stale_time: Option<i32>,
    pub type_string: Option<String>,
    pub how_string: String,
    pub access: Option<String>,
    pub point: Point,
    pub detail: Option<String>,
    config: Arc<Mutex<Config>>,
}

impl Default for CursorOnTarget {
    fn default() -> Self {
        let config = Config::get_instance();
        let cot_type = lookup_cot_type(CoTType::GndUnit, CotClassification::Unknown);

        CursorOnTarget {
            uid: Uuid::new_v4().to_string(),
            stale_time: None,
            type_string: cot_type.to_string().into(),
            how_string: "m-g".to_string(),
            access: Some("UNCLASSIFIED".to_string()),
            point: Point::default(),
            detail: None,
            config: config,
        }
    }
}

impl CursorOnTarget {
    pub const HAE_NONE: f64 = 9999999.0; // Height above the WGS ellipsoid in meters
    pub const CE_NONE: f64 = 9999999.0; // Circular Error in meters
    pub const LE_NONE: f64 = 9999999.0; // Linear Error in meters

    pub fn new(
        uid: String,
        stale_time: Option<i32>,
        type_string: Option<String>,
        how_string: String,
        access: Option<String>,
        point: Point,
    ) -> Self {
        let config = Config::get_instance();
        CursorOnTarget {
            uid: uid,
            stale_time: stale_time,
            type_string: type_string.into(),
            how_string: how_string,
            point: point,
            access: access,
            detail: None,
            config: config,
        }
    }

    pub fn get_xml_detail(&self) -> String {
        self.detail.as_ref().unwrap().to_string()
    }

    pub fn set_xml_detail(&mut self, detail: String) {
        self.detail = Some(detail);
    }

    pub fn get_xml(&mut self) -> String {
        // Get UTime
        let time: i64 = Time::now();
        let track_le = format!(r#"le="{}""#, self.point.le);
        let track_hae = format!(r#"hae="{}""#, self.point.hae);
        let track_ce = format!(r#"ce="{}""#, self.point.ce);
        let access = match &self.access {
            Some(access) => format!(r#"access="{}""#, access),
            None => "".to_string(),
        };

        if self.stale_time.is_none() {
            self.stale_time = Some(self.config.lock().unwrap().general.stale as i32);
        }

        format!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes" ?>
<event version="2.0" type="{cot_type}" uid="{uid}" how="{how}" time="{time}" start="{start}" stale="{stale}" {access}>
    <point lat="{lat}" lon="{lon}" {le} {hae} {ce} />
    <detail>
{detail}
    </detail>
</event>
"#,
            cot_type = self.type_string.as_ref().unwrap(),
            uid = self.uid,
            how = self.how_string,
            time = Time::time_to_string(time),
            start = Time::time_to_string(time),
            stale = Time::time_to_string(Time::stale_time()),
            access = access,
            lat = self.point.latitude,
            lon = self.point.longitude,
            le = track_le,
            hae = track_hae,
            ce = track_ce,
            detail = self.get_xml_detail(),
        )
    }

    pub fn get_xml_bytes(&mut self) -> Vec<u8> {
        let xml = self.get_xml();
        xml.into_bytes()
    }

    ///
    /// Delete a CoT message
    /// # Arguments
    /// * `uid` - Unique identifier of the CoT message to delete
    /// # Returns
    /// * `String` - XML text
    /// # Example
    ///
    /// ``` xml
    /// <?xml version="1.0" standalone="yes"?>
    /// <event start="2012-01-01T00:00:00Z" time="2012-01-01T00:00:00Z" stale="2020-01-01T00:00:00Z" how="m-g" type="t-x-d-d" uid="does-not-matter-must-be-unique" version="2.0">
    ///     <detail>
    ///         <link uid="the-uid-of-the-map-item-to-remove" relation="none" type="none"/>
    ///         <__forcedelete/>
    ///     </detail>
    /// <point lat="0.0" lon="0.0" ce="9999999" le="9999999" hae="9999999"/>
    /// </event>
    /// ```
    pub fn delete(uid: String) -> String {
        format!(
            r#"<?xml version="1.0" standalone="yes"?>
    <event start="{start}" time="{time}" stale="{stale}" how="m-g" type="t-x-d-d" uid="{message_uid}" version="2.0">
        <detail>
            <link uid="{uid}" relation="none" type="none"/>
            <__forcedelete/>
        </detail>
    <point lat="0.0" lon="0.0" ce="9999999" le="9999999" hae="9999999"/>
</event>"#,
            start = Time::time_to_string(Time::now()),
            time = Time::time_to_string(Time::now()),
            stale = Time::time_to_string(Time::stale_time()),
            message_uid = uuid::Uuid::new_v4(),
            uid = uid,
        )
    }

    pub fn get_uid(&self) -> String {
        uuid::Uuid::new_v4().to_string()
    }

    /// Hash a generic type
    /// # Arguments
    /// * `t` - A generic type that implements std::hash::Hash
    /// # Returns
    /// * A u64
    /// # Example
    /// ```
    /// let hash = CursorOnTarget::object_hash("Hello World");
    /// ```
    ///
    pub fn object_hash<T: std::hash::Hash>(t: T) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::Hasher;
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }
}

impl Debug for CursorOnTarget {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("CursorOnTarget")
            .field("uid", &self.uid)
            .field("type_string", &self.type_string)
            .field("how_string", &self.how_string)
            .field("access", &self.access)
            .field("point", &self.point)
            .field("detail", &self.detail)
            .finish()
    }
}

pub trait Sendable{
    fn update_xml(&mut self);
    fn modify_field(&mut self, field: &str, value: &str);
    fn get_cot(&self)-> CursorOnTarget;
}