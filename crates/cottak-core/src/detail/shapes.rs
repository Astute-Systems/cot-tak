//! Shape, drawing, measurement and route detail elements.

use super::DetailItem;
use crate::xml::XmlElement;

value_item!(
    /// `<height value="..."/>`: extrusion height of a shape.
    Height, "height", f64
);
value_item!(
    /// `<height_unit value="..."/>`: unit code for [`Height`].
    HeightUnit, "height_unit", i32
);
value_item!(
    /// `<range value="..."/>`: length of a range and bearing line, in metres.
    Range, "range", f64
);
value_item!(
    /// `<bearing value="..."/>`: bearing of a range and bearing line, in degrees.
    Bearing, "bearing", f64
);
value_item!(
    /// `<inclination value="..."/>`: inclination of a range and bearing line, in degrees.
    Inclination, "inclination", f64
);
value_item!(
    /// `<rangeUnits value="..."/>`: display unit code for ranges.
    RangeUnits, "rangeUnits", i32
);
value_item!(
    /// `<bearingUnits value="..."/>`: display unit code for bearings.
    BearingUnits, "bearingUnits", i32
);
value_item!(
    /// `<northRef value="..."/>`: north reference code (true, magnetic, grid).
    NorthRef, "northRef", i32
);
value_item!(
    /// `<labels_on value="..."/>`: whether the map shows labels for the shape.
    LabelsOn, "labels_on", bool
);

attrs_item!(
    /// `<ellipse>`: a circle or ellipse inside a [`Shape`]. Radii are in metres.
    Ellipse, "ellipse",
    {
        /// Semi-major axis in metres.
        major: f64 => "major",
        /// Semi-minor axis in metres.
        minor: f64 => "minor",
        /// Rotation in degrees (`360` for a circle).
        angle: f64 => "angle",
    }
);

impl Ellipse {
    /// A circle of the given radius in metres.
    pub fn circle(radius_m: f64) -> Self {
        Self {
            major: Some(radius_m),
            minor: Some(radius_m),
            angle: Some(360.0),
            extra: Vec::new(),
        }
    }
}

/// `<shape>`: geometry for drawn circles, ellipses and range rings.
///
/// Polygons and polylines are not carried here; they are expressed as
/// vertex [`Link`](super::Link)s directly in the detail.
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Shape {
    /// Ellipses, innermost first for range rings.
    pub ellipses: Vec<Ellipse>,
    /// Other children (style links and so on), preserved verbatim.
    pub other: Vec<XmlElement>,
}

impl DetailItem for Shape {
    const NAME: &'static str = "shape";

    fn from_xml(el: &XmlElement) -> Option<Self> {
        let mut s = Self::default();
        for c in el.elements() {
            match Ellipse::from_xml(c).filter(|_| c.name == Ellipse::NAME) {
                Some(e) => s.ellipses.push(e),
                None => s.other.push(c.clone()),
            }
        }
        Some(s)
    }

    fn to_xml(&self) -> XmlElement {
        let mut el = XmlElement::new(Self::NAME);
        for e in &self.ellipses {
            el = el.with_child(e.to_xml());
        }
        for o in &self.other {
            el = el.with_child(o.clone());
        }
        el
    }
}

attrs_item!(
    /// `<bullseye>`: a bullseye reference point with optional range rings.
    Bullseye, "bullseye",
    {
        /// Title shown on the map.
        title: String => "title",
        /// Measure from the edge of the bullseye to the centre.
        edge_to_center: bool => "edgeToCenter",
        /// Use mils instead of degrees.
        mils: bool => "mils",
        /// Whether range rings exist.
        has_range_rings: bool => "hasRangeRings",
        /// Distance between range rings.
        ring_dist: f64 => "ringDist",
        /// Number of range rings.
        ring_num: i32 => "ringNum",
        /// Whether the rings are shown.
        range_ring_visible: bool => "rangeRingVisible",
        /// Radius of the bullseye.
        distance: f64 => "distance",
        /// UID of the bullseye reference.
        bullseye_uid: String => "bullseyeUID",
        /// Bearing reference (`M`, `T` or `G`).
        bearing_ref: String => "bearingRef",
    }
);

attrs_item!(
    /// `<__geofence>`: trigger rules for a geofence polygon.
    GeoFence, "__geofence",
    {
        /// Elevation monitoring mode.
        elevation: String => "elevation",
        /// What is tracked.
        tracking: String => "tracking",
        /// Trigger condition, for example `Entry`, `Exit` or `Both`.
        trigger: String => "trigger",
        /// Which entities are monitored, for example `All` or `TeamMembers`.
        monitor: String => "monitor",
        /// Bounding sphere radius.
        bounding_sphere: String => "boundingSphere",
        /// Minimum elevation.
        min_elevation: String => "minElevation",
        /// Maximum elevation.
        max_elevation: String => "maxElevation",
    }
);

attrs_item!(
    /// `<link_attr>`: route presentation and planning attributes.
    LinkAttr, "link_attr",
    {
        /// Line colour.
        color: String => "color",
        /// Travel method, for example `Driving` or `Walking`.
        method: String => "method",
        /// Route direction, for example `Infil` or `Exfil`.
        direction: String => "direction",
        /// Checkpoint name prefix.
        prefix: String => "prefix",
        /// Line style.
        stroke: String => "stroke",
        /// Route type, for example `Primary`.
        route_type: String => "type",
    }
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_items_round_trip() {
        let h = Height::new(12.5);
        assert_eq!(Height::from_xml(&h.to_xml()), Some(h));
        assert_eq!(
            LabelsOn::from_xml(&LabelsOn::new(true).to_xml()),
            Some(LabelsOn::new(true))
        );
        let bad = XmlElement::new("height").with_attr("value", "tall");
        assert_eq!(Height::from_xml(&bad), None);
    }

    #[test]
    fn shape_keeps_style_children() {
        let el = XmlElement::parse(
            r#"<shape><ellipse major="10" minor="10" angle="360"/>
            <link relation="p-c" uid="s.Style" type="b-x-KmlStyle"><Style/></link></shape>"#,
        )
        .unwrap();
        let s = Shape::from_xml(&el).unwrap();
        assert_eq!(s.ellipses, vec![Ellipse::circle(10.0)]);
        assert_eq!(s.other.len(), 1);
        assert_eq!(Shape::from_xml(&s.to_xml()).unwrap(), s);
    }

    #[test]
    fn bullseye_parses_legacy_shape() {
        let el = XmlElement::parse(
            r#"<bullseye title="BE" edgeToCenter="true" mils="false" hasRangeRings="true"
            ringDist="100" ringNum="3" rangeRingVisible="true" distance="300"
            bullseyeUID="u" bearingRef="M"/>"#,
        )
        .unwrap();
        let b = Bullseye::from_xml(&el).unwrap();
        assert_eq!(b.ring_num, Some(3));
        assert_eq!(b.bearing_ref.as_deref(), Some("M"));
        assert_eq!(b.extra, vec![]);
    }
}
