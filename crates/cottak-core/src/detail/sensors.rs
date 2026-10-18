//! Sensor, video, medical-evacuation and emergency detail elements.

use super::DetailItem;
use crate::xml::{XmlElement, XmlNode};

attrs_item!(
    /// `<sensor>`: field of view of a camera or other sensor.
    Sensor, "sensor",
    {
        /// Horizontal field of view in degrees.
        fov: f64 => "fov",
        /// Blue component of the field-of-view colour (0..1).
        fov_blue: f64 => "fovBlue",
        /// Green component of the field-of-view colour (0..1).
        fov_green: f64 => "fovGreen",
        /// Red component of the field-of-view colour (0..1).
        fov_red: f64 => "fovRed",
        /// Alpha of the field-of-view colour (0..1).
        fov_alpha: f64 => "fovAlpha",
        /// Range of the sensor in metres.
        range: f64 => "range",
        /// Azimuth in degrees.
        azimuth: f64 => "azimuth",
        /// Magnetic reference used for display.
        display_magnetic_reference: f64 => "displayMagneticReference",
        /// Hide the field-of-view wedge.
        hide_fov: bool => "hideFov",
    }
);

attrs_item!(
    /// `<ConnectionEntry>`: one way to reach a video feed.
    ConnectionEntry, "ConnectionEntry",
    {
        /// Protocol, for example `rtsp` or `udp`.
        protocol: String => "protocol",
        /// Path component of the URL.
        path: String => "path",
        /// Host or address.
        address: String => "address",
        /// Port.
        port: u16 => "port",
        /// UID of the feed.
        uid: String => "uid",
        /// Human readable alias.
        alias: String => "alias",
        /// Rover port.
        rover_port: i32 => "roverPort",
        /// Whether RTSP should use reliable transport.
        rtsp_reliable: bool => "rtspReliable",
        /// Ignore embedded KLV metadata.
        ignore_embedded_klv: bool => "ignoreEmbeddedKLV",
        /// Network timeout in milliseconds.
        network_timeout: i32 => "networkTimeout",
        /// Buffer time in milliseconds.
        buffer_time: i32 => "bufferTime",
    }
);

/// `<__video>`: a video feed, either by URL or by connection entries.
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Video {
    /// UID of the feed (when referencing a feed defined elsewhere).
    pub uid: Option<String>,
    /// Feed URL.
    pub url: Option<String>,
    /// Connection entries.
    pub entries: Vec<ConnectionEntry>,
    /// Attributes this crate does not model, preserved verbatim.
    pub extra: Vec<(String, String)>,
    /// Child elements other than `<ConnectionEntry>`, preserved verbatim.
    pub extra_children: Vec<XmlElement>,
}

impl DetailItem for Video {
    const NAME: &'static str = "__video";

    fn from_xml(el: &XmlElement) -> Option<Self> {
        let mut a = crate::xml::Attrs::new(el);
        let mut v = Self {
            uid: a.take("uid"),
            url: a.take("url"),
            ..Self::default()
        };
        for c in el.elements() {
            match ConnectionEntry::from_xml(c).filter(|_| c.name == ConnectionEntry::NAME) {
                Some(e) => v.entries.push(e),
                None => v.extra_children.push(c.clone()),
            }
        }
        v.extra = a.into_extra();
        Some(v)
    }

    fn to_xml(&self) -> XmlElement {
        let mut el = XmlElement::new(Self::NAME)
            .with_opt_attr("uid", self.uid.clone())
            .with_opt_attr("url", self.url.clone())
            .with_extra_attrs(&self.extra);
        for e in &self.entries {
            el = el.with_child(e.to_xml());
        }
        for c in &self.extra_children {
            el = el.with_child(c.clone());
        }
        el
    }
}

attrs_item!(
    /// `<_medevac_>`: the 9-line medical evacuation request.
    Medevac, "_medevac_",
    {
        /// Title of the request.
        title: String => "title",
        /// `true` for a CASEVAC, `false` for a MEDEVAC.
        casevac: bool => "casevac",
        /// Radio frequency.
        freq: f64 => "freq",
        /// Line 3, urgent patients.
        urgent: u8 => "urgent",
        /// Line 3, priority patients.
        priority: u8 => "priority",
        /// Line 3, routine patients.
        routine: u8 => "routine",
        /// Line 5 extraction equipment required.
        extraction_equipment: bool => "extraction_equipment",
        /// Line 7 security at the pickup site.
        security: u8 => "security",
        /// Line 7 HLZ marking method.
        hlz_marking: u8 => "hlz_marking",
        /// Line 8 US military patients.
        us_military: u8 => "us_military",
        /// Line 8 US civilian patients.
        us_civilian: u8 => "us_civilian",
        /// Line 8 non-US military patients.
        nonus_military: u8 => "nonus_military",
        /// Line 8 non-US civilian patients.
        nonus_civilian: u8 => "nonus_civilian",
        /// Line 8 enemy prisoners of war.
        epw: u8 => "epw",
        /// Line 8 children.
        child: u8 => "child",
        /// Line 9 terrain: none.
        terrain_none: bool => "terrain_none",
        /// Line 9 obstacles.
        obstacles: String => "obstacles",
        /// Free-text MEDLINE remarks.
        medline_remarks: String => "medline_remarks",
        /// Zone protection selection.
        zone_prot_selection: u8 => "zone_prot_selection",
        /// Protected zone coordinates.
        zone_protected_coord: String => "zone_protected_coord",
        /// Protected zone marker.
        zone_prot_marker: String => "zone_prot_marker",
    }
);

/// `<emergency>`: an emergency beacon (`b-a-o-tbl`, `b-a-o-can`, ...).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Emergency {
    /// Emergency type text, for example `911 Alert`.
    pub emergency_type: Option<String>,
    /// `true` when this event cancels an earlier alert.
    pub cancel: Option<bool>,
    /// Callsign of the sender (element text).
    pub callsign: String,
    /// Attributes this crate does not model, preserved verbatim.
    pub extra: Vec<(String, String)>,
}

impl DetailItem for Emergency {
    const NAME: &'static str = "emergency";

    fn from_xml(el: &XmlElement) -> Option<Self> {
        let mut a = crate::xml::Attrs::new(el);
        Some(Self {
            emergency_type: a.take("type"),
            cancel: a.take_parsed("cancel"),
            callsign: el.text(),
            extra: a.into_extra(),
        })
    }

    fn to_xml(&self) -> XmlElement {
        let mut el = XmlElement::new(Self::NAME)
            .with_opt_attr("type", self.emergency_type.clone())
            .with_opt_attr("cancel", self.cancel.map(|b| b.to_string()))
            .with_extra_attrs(&self.extra);
        if !self.callsign.is_empty() {
            el.children.push(XmlNode::Text(self.callsign.clone()));
        }
        el
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn medevac_parses_legacy_attributes() {
        let el = XmlElement::parse(
            r#"<_medevac_ title="MEDEVAC 1" casevac="true" freq="30.5" urgent="1" priority="0"
            routine="2" extraction_equipment="false" security="1" hlz_marking="2" us_military="1"
            terrain_none="true" obstacles="wires" terrain_slope="3"/>"#,
        )
        .unwrap();
        let m = Medevac::from_xml(&el).unwrap();
        assert_eq!(m.freq, Some(30.5));
        assert_eq!(m.urgent, Some(1));
        assert_eq!(m.terrain_none, Some(true));
        assert_eq!(m.extra, vec![("terrain_slope".to_owned(), "3".to_owned())]);
        assert_eq!(Medevac::from_xml(&m.to_xml()).unwrap(), m);
    }

    #[test]
    fn out_of_range_counts_are_not_truncated() {
        let el = XmlElement::new("_medevac_").with_attr("urgent", "300");
        let m = Medevac::from_xml(&el).unwrap();
        assert_eq!(m.urgent, None);
        assert_eq!(m.to_xml().attr("urgent"), Some("300"));
    }

    #[test]
    fn video_round_trip() {
        let el = XmlElement::parse(
            r#"<__video url="rtsp://h/p"><ConnectionEntry protocol="rtsp" address="h" port="554"
            path="/p" uid="v1" alias="cam"/><x/></__video>"#,
        )
        .unwrap();
        let v = Video::from_xml(&el).unwrap();
        assert_eq!(v.entries[0].port, Some(554));
        assert_eq!(v.extra_children.len(), 1);
        assert_eq!(Video::from_xml(&v.to_xml()).unwrap(), v);
    }

    #[test]
    fn emergency_round_trip() {
        let e = Emergency {
            emergency_type: Some("911 Alert".into()),
            cancel: Some(false),
            callsign: "ALPHA".into(),
            extra: vec![],
        };
        assert_eq!(Emergency::from_xml(&e.to_xml()).unwrap(), e);
    }
}
