//! The CoT `<event>`: the unit of exchange on every TAK transport.

use crate::cot_type::CotType;
use crate::detail::{Detail, DetailItem};
use crate::error::{Error, Result};
use crate::point::Point;
use crate::timestamp;
use crate::xml::{Attrs, XmlElement};
use std::fmt;
use std::str::FromStr;
use time::{Duration, OffsetDateTime};

/// Default time-to-live applied by [`Event::new`].
pub const DEFAULT_STALE: Duration = Duration::minutes(2);

/// Common values for the `how` attribute (how the position was derived).
pub mod how {
    /// Machine-generated, GPS derived.
    pub const MACHINE_GPS: &str = "m-g";
    /// Machine-generated, other.
    pub const MACHINE: &str = "m";
    /// Human entered, GIS derived (for example placed on a map).
    pub const HUMAN_GIS: &str = "h-g-i-g-o";
    /// Human entered.
    pub const HUMAN: &str = "h-e";
}

/// A Cursor on Target event.
///
/// Construct with [`Event::new`] and the `with_*` methods, or parse with
/// [`Event::from_xml_str`] / [`str::parse`].
///
/// ```
/// use cottak_core::detail::{Contact, Track};
/// use cottak_core::{CotType, Event, Point};
///
/// let event = Event::new(
///     "ALPHA-1",
///     "a-f-G-U-C".parse::<CotType>()?,
///     Point::new(-27.4689, 153.0235)?,
/// )
/// .with(Contact::new("ALPHA-1"))
/// .with(Track::new(90.0, 1.5));
///
/// let xml = event.to_xml_string()?;
/// let parsed: Event = xml.parse()?;
/// assert_eq!(parsed, event);
/// # Ok::<(), cottak_core::Error>(())
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub struct Event {
    /// CoT schema version, normally `2.0`.
    pub version: String,
    /// Globally unique identifier of the entity this event describes.
    pub uid: String,
    /// Event type.
    pub cot_type: CotType,
    /// How the position was derived; see [`how`].
    pub how: String,
    /// When the event was generated.
    pub time: OffsetDateTime,
    /// When the event becomes valid.
    pub start: OffsetDateTime,
    /// When the event should be considered out of date.
    pub stale: OffsetDateTime,
    /// Access marking.
    pub access: Option<String>,
    /// Quality of service marking.
    pub qos: Option<String>,
    /// Operational exercise marking.
    pub opex: Option<String>,
    /// Location.
    pub point: Point,
    /// Extension elements.
    pub detail: Detail,
    /// Event attributes this crate does not model, preserved verbatim.
    pub extra: Vec<(String, String)>,
}

impl Event {
    /// Creates an event valid from now, stale after [`DEFAULT_STALE`].
    pub fn new(uid: impl Into<String>, cot_type: CotType, point: Point) -> Self {
        let now = timestamp::now();
        Self {
            version: "2.0".to_owned(),
            uid: uid.into(),
            cot_type,
            how: how::MACHINE_GPS.to_owned(),
            time: now,
            start: now,
            stale: now + DEFAULT_STALE,
            access: None,
            qos: None,
            opex: None,
            point,
            detail: Detail::new(),
            extra: Vec::new(),
        }
    }

    /// Generates a random UID (UUID v4).
    pub fn random_uid() -> String {
        uuid::Uuid::new_v4().to_string()
    }

    /// Sets the `how` attribute.
    #[must_use]
    pub fn with_how(mut self, how: impl Into<String>) -> Self {
        self.how = how.into();
        self
    }

    /// Sets the stale time relative to `start`.
    #[must_use]
    pub fn with_stale_in(mut self, ttl: Duration) -> Self {
        self.stale = self.start + ttl;
        self
    }

    /// Replaces the detail block.
    #[must_use]
    pub fn with_detail(mut self, detail: Detail) -> Self {
        self.detail = detail;
        self
    }

    /// Sets a typed detail element (see [`Detail::set`]).
    #[must_use]
    pub fn with<T: DetailItem>(mut self, item: T) -> Self {
        self.detail.set(item);
        self
    }

    /// `true` if `now` is after the stale time.
    pub fn is_stale_at(&self, now: OffsetDateTime) -> bool {
        now > self.stale
    }

    /// `true` if the event is stale right now.
    pub fn is_stale(&self) -> bool {
        self.is_stale_at(OffsetDateTime::now_utc())
    }

    /// Checks invariants that the wire format cannot express.
    pub fn validate(&self) -> Result<()> {
        if self.uid.trim().is_empty() {
            return Err(Error::invalid(
                "event",
                "uid",
                &self.uid,
                "must not be empty",
            ));
        }
        if self.stale < self.start {
            return Err(Error::invalid(
                "event",
                "stale",
                "",
                "stale is before start",
            ));
        }
        // Text that cannot be written as XML would otherwise be altered silently on output.
        let strings = [
            Some(&self.version),
            Some(&self.uid),
            Some(&self.how),
            self.access.as_ref(),
            self.qos.as_ref(),
            self.opex.as_ref(),
        ];
        for s in strings.into_iter().flatten() {
            crate::xml::check_text(s)?;
        }
        for (k, v) in &self.extra {
            crate::xml::check_text(v)?;
            XmlElement::new(k.clone()).validate()?;
        }
        self.detail
            .elements()
            .iter()
            .try_for_each(XmlElement::validate)?;
        self.point.validate()
    }

    /// Parses an `<event>` document.
    pub fn from_xml_str(xml: &str) -> Result<Self> {
        Self::from_element(&XmlElement::parse(xml)?)
    }

    /// Builds an event from a parsed `<event>` element.
    pub fn from_element(el: &XmlElement) -> Result<Self> {
        if el.name != "event" {
            return Err(Error::NotAnEvent(el.name.clone()));
        }
        let mut a = Attrs::new(el);
        let mut req = |name: &'static str| {
            a.take(name).ok_or(Error::Missing {
                what: "attribute",
                name,
                element: "event",
            })
        };
        let uid = req("uid")?;
        let type_str = req("type")?;
        let how = req("how")?;
        let time = timestamp::parse(&req("time")?)?;
        let start = timestamp::parse(&req("start")?)?;
        let stale = timestamp::parse(&req("stale")?)?;

        let point_el = el.child("point").ok_or(Error::Missing {
            what: "element",
            name: "point",
            element: "event",
        })?;

        let event = Self {
            version: a.take("version").unwrap_or_else(|| "2.0".to_owned()),
            uid,
            cot_type: CotType::new(type_str)?,
            how,
            time,
            start,
            stale,
            access: a.take("access"),
            qos: a.take("qos"),
            opex: a.take("opex"),
            point: Point::from_xml(point_el)?,
            detail: el.child("detail").map(Detail::from_xml).unwrap_or_default(),
            extra: a.into_extra(),
        };
        event.validate()?;
        Ok(event)
    }

    /// Converts to an `<event>` element.
    pub fn to_element(&self) -> Result<XmlElement> {
        self.validate()?;
        let mut el = XmlElement::new("event")
            .with_attr("version", self.version.clone())
            .with_attr("uid", self.uid.clone())
            .with_attr("type", self.cot_type.as_str())
            .with_attr("how", self.how.clone())
            .with_attr("time", timestamp::format(self.time)?)
            .with_attr("start", timestamp::format(self.start)?)
            .with_attr("stale", timestamp::format(self.stale)?)
            .with_opt_attr("access", self.access.clone())
            .with_opt_attr("qos", self.qos.clone())
            .with_opt_attr("opex", self.opex.clone())
            .with_extra_attrs(&self.extra)
            .with_child(self.point.to_xml());
        if !self.detail.is_empty() {
            el = el.with_child(self.detail.to_xml());
        }
        Ok(el)
    }

    /// Serialises to compact XML (no XML declaration).
    pub fn to_xml_string(&self) -> Result<String> {
        Ok(self.to_element()?.to_xml())
    }
}

impl FromStr for Event {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        Self::from_xml_str(s)
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.to_xml_string() {
            Ok(s) => f.write_str(&s),
            Err(_) => Err(fmt::Error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::detail::{Contact, Group, Link, Remarks, Takv};

    const SAMPLE: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<event version="2.0" uid="ANDROID-abc123" type="a-f-G-U-C" how="m-g"
       time="2025-03-01T01:02:03.456Z" start="2025-03-01T01:02:03.456Z"
       stale="2025-03-01T01:07:03.456Z" access="UNCLASSIFIED">
  <point lat="-27.4689" lon="153.0235" hae="12.5" ce="9999999.0" le="9999999.0"/>
  <detail>
    <takv device="Pixel 7" platform="ATAK-CIV" os="34" version="5.2.0"/>
    <contact endpoint="*:-1:stcp" callsign="ALPHA-1"/>
    <__group name="Cyan" role="Team Member"/>
    <remarks>hello &amp; goodbye</remarks>
    <vendor:thing xmlns:vendor="urn:x" a="1"><inner>keep me</inner></vendor:thing>
  </detail>
</event>"#;

    #[test]
    fn parses_real_world_event() {
        let e: Event = SAMPLE.parse().unwrap();
        assert_eq!(e.uid, "ANDROID-abc123");
        assert_eq!(e.point.hae, Some(12.5));
        assert_eq!(e.point.ce, None);
        assert_eq!(e.access.as_deref(), Some("UNCLASSIFIED"));
        assert_eq!(
            e.detail.get::<Contact>().unwrap().callsign.as_deref(),
            Some("ALPHA-1")
        );
        assert_eq!(
            e.detail.get::<Group>().unwrap().name.as_deref(),
            Some("Cyan")
        );
        assert_eq!(e.detail.get::<Takv>().unwrap().os.as_deref(), Some("34"));
        assert_eq!(e.detail.get::<Remarks>().unwrap().text, "hello & goodbye");
    }

    #[test]
    fn round_trip_is_lossless_including_unknown_detail() {
        let e: Event = SAMPLE.parse().unwrap();
        let again: Event = e.to_xml_string().unwrap().parse().unwrap();
        assert_eq!(e, again);
        let raw = again.detail.raw("vendor:thing").unwrap();
        assert_eq!(raw.attr("xmlns:vendor"), Some("urn:x"));
        assert_eq!(raw.child("inner").unwrap().text(), "keep me");
    }

    #[test]
    fn builder_round_trip() {
        let e = Event::new(
            Event::random_uid(),
            "b-m-p-w".parse().unwrap(),
            Point::new(1.0, 2.0)
                .unwrap()
                .with_hae(3.0)
                .with_errors(4.0, 5.0),
        )
        .with_how(how::HUMAN_GIS)
        .with(Contact::new("WP1"))
        .with(Link {
            uid: Some("parent".into()),
            relation: Some("p-p".into()),
            ..Link::default()
        });
        let again: Event = e.to_string().parse().unwrap();
        assert_eq!(e, again);
    }

    #[test]
    fn rejects_bad_events() {
        let cases = [
            "<notevent/>",
            r#"<event uid="a" type="a-f-G" how="m-g" time="2025-01-01T00:00:00Z" start="2025-01-01T00:00:00Z" stale="2025-01-01T00:00:00Z"/>"#,
            r#"<event uid="a" type="a-f-G" how="m-g" time="bogus" start="2025-01-01T00:00:00Z" stale="2025-01-01T00:00:00Z"><point lat="0" lon="0"/></event>"#,
            r#"<event uid="a" type="a-f-G" how="m-g" time="2025-01-01T00:00:00Z" start="2025-01-01T00:00:00Z" stale="2025-01-01T00:00:00Z"><point lat="95" lon="0"/></event>"#,
            r#"<event uid="" type="a-f-G" how="m-g" time="2025-01-01T00:00:00Z" start="2025-01-01T00:00:00Z" stale="2025-01-01T00:00:00Z"><point lat="0" lon="0"/></event>"#,
            r#"<event uid="a" type="a-f-G" how="m-g" time="2025-01-01T00:00:00Z" start="2025-01-01T00:00:10Z" stale="2025-01-01T00:00:00Z"><point lat="0" lon="0"/></event>"#,
        ];
        for c in cases {
            assert!(c.parse::<Event>().is_err(), "{c}");
        }
    }

    #[test]
    fn staleness() {
        let e = Event::new("u", "a-f-G".parse().unwrap(), Point::new(0.0, 0.0).unwrap());
        assert!(!e.is_stale_at(e.start));
        assert!(e.is_stale_at(e.stale + Duration::seconds(1)));
    }

    #[test]
    fn text_that_xml_cannot_carry_is_an_error_not_silently_changed() {
        let mut e = Event::new("u", "a-f-G".parse().unwrap(), Point::new(0.0, 0.0).unwrap());
        assert!(e.validate().is_ok());
        e.uid = "bad\u{c}uid".into();
        assert!(e.validate().is_err());
        assert!(e.to_xml_string().is_err());

        let mut e = Event::new("u", "a-f-G".parse().unwrap(), Point::new(0.0, 0.0).unwrap());
        e.detail
            .push_raw(crate::xml::XmlElement::new("remarks").with_text("a\u{1}b"));
        assert!(e.to_xml_string().is_err());
        e.extra.push(("no spaces".into(), "v".into()));
        assert!(e.validate().is_err());
    }
}
