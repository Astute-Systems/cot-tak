//! Typed views of the common `<detail>` children.

use super::DetailItem;
use crate::xml::{Attrs, XmlElement, XmlNode};

macro_rules! str_attrs_item {
    (
        $(#[$meta:meta])* $name:ident, $elem:literal,
        { $( $(#[$fmeta:meta])* $field:ident => $attr:literal ),+ $(,)? }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq, Default)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct $name {
            $( $(#[$fmeta])* pub $field: Option<String>, )+
            /// Attributes this crate does not model, preserved verbatim.
            pub extra: Vec<(String, String)>,
        }

        impl DetailItem for $name {
            const NAME: &'static str = $elem;

            fn from_xml(el: &XmlElement) -> Option<Self> {
                let mut a = Attrs::new(el);
                Some(Self {
                    $( $field: a.take($attr), )+
                    extra: a.into_extra(),
                })
            }

            fn to_xml(&self) -> XmlElement {
                XmlElement::new($elem)
                    $( .with_opt_attr($attr, self.$field.clone()) )+
                    .with_extra_attrs(&self.extra)
            }
        }
    };
}

macro_rules! argb_item {
    ($(#[$meta:meta])* $name:ident, $elem:literal, $attr:literal, $alt:literal) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct $name {
            /// Colour packed as `0xAARRGGBB`, stored as a signed integer on the wire.
            pub argb: i32,
        }

        impl $name {
            /// Creates a colour from channel values (`r`, `g`, `b`, `a`).
            pub const fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
                Self { argb: i32::from_be_bytes([a, r, g, b]) }
            }

            /// Returns `(r, g, b, a)`.
            pub const fn rgba(self) -> (u8, u8, u8, u8) {
                let [a, r, g, b] = self.argb.to_be_bytes();
                (r, g, b, a)
            }
        }

        impl DetailItem for $name {
            const NAME: &'static str = $elem;

            fn from_xml(el: &XmlElement) -> Option<Self> {
                let raw = el.attr($attr).or_else(|| el.attr($alt))?;
                Some(Self { argb: raw.trim().parse().ok()? })
            }

            fn to_xml(&self) -> XmlElement {
                XmlElement::new($elem).with_attr($attr, self.argb.to_string())
            }
        }
    };
}

str_attrs_item!(
    /// `<contact>`: how to address the entity.
    Contact, "contact",
    {
        /// Human readable callsign.
        callsign => "callsign",
        /// Routing endpoint, for example `*:-1:stcp`.
        endpoint => "endpoint",
        /// Phone number.
        phone => "phone",
    }
);

impl Contact {
    /// Creates a contact with a callsign.
    pub fn new(callsign: impl Into<String>) -> Self {
        Self {
            callsign: Some(callsign.into()),
            ..Self::default()
        }
    }
}

str_attrs_item!(
    /// `<takv>`: client software and device information.
    Takv, "takv",
    {
        /// Device model.
        device => "device",
        /// Platform name, for example `ATAK-CIV`.
        platform => "platform",
        /// Operating system.
        os => "os",
        /// Software version.
        version => "version",
    }
);

str_attrs_item!(
    /// `<__group>`: team membership.
    Group, "__group",
    {
        /// Team colour name, for example `Cyan`.
        name => "name",
        /// Role, for example `Team Lead`.
        role => "role",
    }
);

str_attrs_item!(
    /// `<precisionlocation>`: sources of the position fix.
    PrecisionLocation, "precisionlocation",
    {
        /// Source of the horizontal position, for example `GPS`.
        geopointsrc => "geopointsrc",
        /// Source of the altitude, for example `GPS`.
        altsrc => "altsrc",
    }
);

str_attrs_item!(
    /// `<uid>`: device-level identity.
    Uid, "uid",
    {
        /// Device callsign (`Droid`).
        droid => "Droid",
    }
);

str_attrs_item!(
    /// `<usericon>`: custom map icon.
    UserIcon, "usericon",
    {
        /// Icon set path, for example `COT_MAPPING_2525B/a-f/a-f-G`.
        iconsetpath => "iconsetpath",
    }
);

argb_item!(
    /// `<color argb=".."/>`: marker colour. Older senders use `value`, which is also read.
    Color, "color", "argb", "value"
);
argb_item!(
    /// `<strokeColor value=".."/>`: outline colour of a shape (`argb` is also read).
    StrokeColor, "strokeColor", "value", "argb"
);
argb_item!(
    /// `<fillColor value=".."/>`: fill colour of a shape (`argb` is also read).
    FillColor, "fillColor", "value", "argb"
);

/// `<strokeWeight>`: outline width of a shape.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StrokeWeight {
    /// Width in device-independent pixels.
    pub value: f64,
}

impl DetailItem for StrokeWeight {
    const NAME: &'static str = "strokeWeight";

    fn from_xml(el: &XmlElement) -> Option<Self> {
        Some(Self {
            value: el
                .attr("value")?
                .trim()
                .parse()
                .ok()
                .filter(|v: &f64| v.is_finite())?,
        })
    }

    fn to_xml(&self) -> XmlElement {
        XmlElement::new(Self::NAME).with_attr("value", self.value.to_string())
    }
}

/// `<track>`: heading and speed.
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Track {
    /// Course over ground in degrees true.
    pub course: Option<f64>,
    /// Speed over ground in metres per second.
    pub speed: Option<f64>,
    /// Attributes this crate does not model, preserved verbatim.
    pub extra: Vec<(String, String)>,
}

impl Track {
    /// Creates a track from course (degrees) and speed (m/s).
    pub fn new(course: f64, speed: f64) -> Self {
        Self {
            course: Some(course),
            speed: Some(speed),
            extra: Vec::new(),
        }
    }
}

impl DetailItem for Track {
    const NAME: &'static str = "track";

    fn from_xml(el: &XmlElement) -> Option<Self> {
        let mut a = Attrs::new(el);
        Some(Self {
            course: a.take_parsed("course"),
            speed: a.take_parsed("speed"),
            extra: a.into_extra(),
        })
    }

    fn to_xml(&self) -> XmlElement {
        XmlElement::new(Self::NAME)
            .with_opt_attr("course", self.course.map(|v| v.to_string()))
            .with_opt_attr("speed", self.speed.map(|v| v.to_string()))
            .with_extra_attrs(&self.extra)
    }
}

/// `<status>`: device status.
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Status {
    /// Battery percentage.
    pub battery: Option<f64>,
    /// Whether the unit reports itself ready.
    pub readiness: Option<bool>,
    /// Attributes this crate does not model, preserved verbatim.
    pub extra: Vec<(String, String)>,
}

impl DetailItem for Status {
    const NAME: &'static str = "status";

    fn from_xml(el: &XmlElement) -> Option<Self> {
        let mut a = Attrs::new(el);
        Some(Self {
            battery: a.take_parsed("battery"),
            readiness: a.take_parsed("readiness"),
            extra: a.into_extra(),
        })
    }

    fn to_xml(&self) -> XmlElement {
        XmlElement::new(Self::NAME)
            .with_opt_attr("battery", self.battery.map(|v| v.to_string()))
            .with_opt_attr("readiness", self.readiness.map(|v| v.to_string()))
            .with_extra_attrs(&self.extra)
    }
}

/// `<remarks>`: free text.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Remarks {
    /// The remark text.
    pub text: String,
    /// Originating callsign.
    pub source: Option<String>,
    /// Originating UID.
    pub source_id: Option<String>,
    /// Intended recipient.
    pub to: Option<String>,
    /// Time the remark was made (kept as text).
    pub time: Option<String>,
    /// Attributes this crate does not model, preserved verbatim.
    pub extra: Vec<(String, String)>,
}

impl Remarks {
    /// Creates a remark with text only.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            ..Self::default()
        }
    }
}

impl DetailItem for Remarks {
    const NAME: &'static str = "remarks";

    fn from_xml(el: &XmlElement) -> Option<Self> {
        let mut a = Attrs::new(el);
        Some(Self {
            text: el.text(),
            source: a.take("source"),
            source_id: a.take("sourceID"),
            to: a.take("to"),
            time: a.take("time"),
            extra: a.into_extra(),
        })
    }

    fn to_xml(&self) -> XmlElement {
        let mut el = XmlElement::new(Self::NAME)
            .with_opt_attr("source", self.source.clone())
            .with_opt_attr("sourceID", self.source_id.clone())
            .with_opt_attr("to", self.to.clone())
            .with_opt_attr("time", self.time.clone())
            .with_extra_attrs(&self.extra);
        if !self.text.is_empty() {
            el.children.push(XmlNode::Text(self.text.clone()));
        }
        el
    }
}

/// `<archive/>`: ask TAK Server to retain the event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Archive;

impl DetailItem for Archive {
    const NAME: &'static str = "archive";

    fn from_xml(_: &XmlElement) -> Option<Self> {
        Some(Self)
    }

    fn to_xml(&self) -> XmlElement {
        XmlElement::new(Self::NAME)
    }
}

/// A `"lat,lon[,hae]"` position as used by `<link point="..."/>`.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LinkPoint {
    /// Latitude in decimal degrees.
    pub lat: f64,
    /// Longitude in decimal degrees.
    pub lon: f64,
    /// Height above ellipsoid, if present.
    pub hae: Option<f64>,
}

impl LinkPoint {
    fn parse(s: &str) -> Option<Self> {
        let mut it = s
            .split(',')
            .map(|p| p.trim().parse::<f64>().ok().filter(|v| v.is_finite()));
        let lat = it.next()??;
        let lon = it.next()??;
        let hae = match it.next() {
            None => None,
            Some(v) => Some(v?),
        };
        if it.next().is_some() || !(-90.0..=90.0).contains(&lat) || !(-180.0..=180.0).contains(&lon)
        {
            return None;
        }
        Some(Self { lat, lon, hae })
    }

    fn format(self) -> String {
        match self.hae {
            Some(h) => format!("{},{},{}", self.lat, self.lon, h),
            None => format!("{},{}", self.lat, self.lon),
        }
    }
}

/// `<link>`: relation to another entity, or a vertex of a shape or route.
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Link {
    /// UID of the linked entity.
    pub uid: Option<String>,
    /// Relation, for example `p-p` (parent) or `c` (child).
    pub relation: Option<String>,
    /// CoT type of the linked entity.
    pub cot_type: Option<String>,
    /// Callsign of the parent.
    pub parent_callsign: Option<String>,
    /// Production time (kept as text).
    pub production_time: Option<String>,
    /// Position, for shape vertices and route points.
    pub point: Option<LinkPoint>,
    /// Callsign of the linked entity.
    pub callsign: Option<String>,
    /// Remarks.
    pub remarks: Option<String>,
    /// URL, for linked resources.
    pub url: Option<String>,
    /// MIME type of the linked resource.
    pub mime: Option<String>,
    /// Version string.
    pub version: Option<String>,
    /// Attributes this crate does not model, preserved verbatim.
    pub extra: Vec<(String, String)>,
}

impl DetailItem for Link {
    const NAME: &'static str = "link";

    fn from_xml(el: &XmlElement) -> Option<Self> {
        let mut a = Attrs::new(el);
        Some(Self {
            uid: a.take("uid"),
            relation: a.take("relation"),
            cot_type: a.take("type"),
            parent_callsign: a.take("parent_callsign"),
            production_time: a.take("production_time"),
            point: a.take_parsed::<LinkPointStr>("point").map(|p| p.0),
            callsign: a.take("callsign"),
            remarks: a.take("remarks"),
            url: a.take("url"),
            mime: a.take("mime"),
            version: a.take("version"),
            extra: a.into_extra(),
        })
    }

    fn to_xml(&self) -> XmlElement {
        XmlElement::new(Self::NAME)
            .with_opt_attr("uid", self.uid.clone())
            .with_opt_attr("relation", self.relation.clone())
            .with_opt_attr("type", self.cot_type.clone())
            .with_opt_attr("parent_callsign", self.parent_callsign.clone())
            .with_opt_attr("production_time", self.production_time.clone())
            .with_opt_attr("point", self.point.map(LinkPoint::format))
            .with_opt_attr("callsign", self.callsign.clone())
            .with_opt_attr("remarks", self.remarks.clone())
            .with_opt_attr("url", self.url.clone())
            .with_opt_attr("mime", self.mime.clone())
            .with_opt_attr("version", self.version.clone())
            .with_extra_attrs(&self.extra)
    }
}

/// Adapter so `Attrs::take_parsed` can parse a [`LinkPoint`].
struct LinkPointStr(LinkPoint);

impl std::str::FromStr for LinkPointStr {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        LinkPoint::parse(s).map(Self).ok_or(())
    }
}

/// One `<dest>` of a [`Marti`] routing element.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MartiDest {
    /// Callsign to deliver to.
    pub callsign: Option<String>,
    /// Mission to deliver to.
    pub mission: Option<String>,
    /// Attributes this crate does not model, preserved verbatim.
    pub extra: Vec<(String, String)>,
}

impl MartiDest {
    /// Addresses a callsign.
    pub fn callsign(callsign: impl Into<String>) -> Self {
        Self {
            callsign: Some(callsign.into()),
            ..Self::default()
        }
    }
}

/// `<marti>`: directed routing through TAK Server.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Marti {
    /// Destinations; an empty list means normal broadcast routing.
    pub dests: Vec<MartiDest>,
}

impl DetailItem for Marti {
    const NAME: &'static str = "marti";

    fn from_xml(el: &XmlElement) -> Option<Self> {
        let dests = el
            .elements()
            .filter(|d| d.name == "dest")
            .map(|d| {
                let mut a = Attrs::new(d);
                MartiDest {
                    callsign: a.take("callsign"),
                    mission: a.take("mission"),
                    extra: a.into_extra(),
                }
            })
            .collect();
        Some(Self { dests })
    }

    fn to_xml(&self) -> XmlElement {
        let mut el = XmlElement::new(Self::NAME);
        for d in &self.dests {
            el = el.with_child(
                XmlElement::new("dest")
                    .with_opt_attr("callsign", d.callsign.clone())
                    .with_opt_attr("mission", d.mission.clone())
                    .with_extra_attrs(&d.extra),
            );
        }
        el
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::detail::Detail;

    fn rt<T: DetailItem + PartialEq + std::fmt::Debug>(item: &T) {
        let xml = item.to_xml();
        let reparsed = XmlElement::parse(&xml.to_xml()).unwrap();
        assert_eq!(T::from_xml(&reparsed).as_ref(), Some(item));
    }

    #[test]
    fn typed_round_trips() {
        rt(&Contact {
            callsign: Some("A".into()),
            endpoint: Some("*:-1:stcp".into()),
            ..Contact::default()
        });
        rt(&Takv {
            device: Some("Pixel".into()),
            platform: Some("ATAK-CIV".into()),
            ..Takv::default()
        });
        rt(&Group {
            name: Some("Cyan".into()),
            role: Some("Team Lead".into()),
            ..Group::default()
        });
        rt(&Track::new(90.5, 1.25));
        rt(&Status {
            battery: Some(87.0),
            readiness: Some(true),
            ..Status::default()
        });
        rt(&Remarks::new("a < b & \"c\"\nsecond line"));
        rt(&Color::from_rgba(255, 0, 0, 255));
        rt(&StrokeWeight { value: 3.0 });
        rt(&Archive);
        rt(&Marti {
            dests: vec![MartiDest::callsign("BRAVO")],
        });
        rt(&Link {
            uid: Some("u".into()),
            relation: Some("p-p".into()),
            point: Some(LinkPoint {
                lat: 1.5,
                lon: -2.5,
                hae: Some(10.0),
            }),
            ..Link::default()
        });
    }

    #[test]
    fn unknown_attributes_are_preserved() {
        let el = XmlElement::parse(r#"<contact callsign="A" vendor="x"/>"#).unwrap();
        let c = Contact::from_xml(&el).unwrap();
        assert_eq!(c.extra, vec![("vendor".to_owned(), "x".to_owned())]);
        assert_eq!(c.to_xml().attr("vendor"), Some("x"));
    }

    #[test]
    fn unparseable_value_stays_in_extra() {
        let el = XmlElement::parse(r#"<track course="north" speed="2"/>"#).unwrap();
        let t = Track::from_xml(&el).unwrap();
        assert_eq!(t.course, None);
        assert_eq!(t.speed, Some(2.0));
        assert_eq!(t.to_xml().attr("course"), Some("north"));
    }

    #[test]
    fn argb_channels() {
        let c = Color::from_rgba(255, 0, 0, 255);
        assert_eq!(c.argb, -65536);
        assert_eq!(c.rgba(), (255, 0, 0, 255));
    }

    #[test]
    fn set_replaces_and_dedups() {
        let mut d = Detail::new();
        d.push(Contact::new("A"));
        d.push(Contact::new("B"));
        d.set(Contact::new("C"));
        assert_eq!(d.get_all::<Contact>().len(), 1);
        assert_eq!(d.get::<Contact>().unwrap().callsign.as_deref(), Some("C"));
    }

    #[test]
    fn link_point_rejects_out_of_range() {
        assert!(LinkPoint::parse("95,0").is_none());
        assert!(LinkPoint::parse("1,2,3,4").is_none());
        assert!(LinkPoint::parse("1,x").is_none());
    }
}
