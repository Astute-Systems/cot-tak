//! CoT ⇄ KML 2.2.
//!
//! Export is [`events_to_kml`]; import is [`events_from_kml`].
//!
//! Each event becomes a `<Placemark>` with its geometry, styling taken from
//! the CoT colour details, and the CoT identity in `<ExtendedData>`.
//! Output is built with an escaping XML writer, so callsigns and remarks can
//! contain any text without breaking the document.

use crate::color;
use crate::geometry::{geometry_of, Coord, Geometry, GeometryOptions};
use crate::sidc::Sidc;
use cottak_core::detail::{Color, Contact, FillColor, Remarks, StrokeColor, StrokeWeight};
use cottak_core::xml::XmlElement;
use cottak_core::{timestamp, Event};

pub use crate::kml_import::{events_from_kml, KmlImport, Skipped};
#[cfg(feature = "kmz")]
pub use crate::kmz::{events_from_kmz, events_to_kmz, KmzLimits};

/// Options for [`events_to_kml`].
#[derive(Debug, Clone, Default)]
pub struct KmlOptions {
    /// Name of the `<Document>`.
    pub document_name: Option<String>,
    /// How shapes are turned into geometry.
    pub geometry: GeometryOptions,
}

fn text_element(name: &str, text: impl Into<String>) -> XmlElement {
    XmlElement::new(name).with_text(text)
}

fn coordinate(c: &Coord, with_alt: bool) -> String {
    if with_alt {
        format!("{},{},{}", c.lon, c.lat, c.hae.unwrap_or(0.0))
    } else {
        format!("{},{}", c.lon, c.lat)
    }
}

fn coordinates(cs: &[Coord], with_alt: bool) -> XmlElement {
    text_element(
        "coordinates",
        cs.iter()
            .map(|c| coordinate(c, with_alt))
            .collect::<Vec<_>>()
            .join(" "),
    )
}

fn geometry_element(g: &Geometry) -> XmlElement {
    let alt = |cs: &[Coord]| cs.iter().any(|c| c.hae.is_some());
    let with_mode = |mut el: XmlElement, has_alt: bool| {
        if has_alt {
            el = el.with_child(text_element("altitudeMode", "absolute"));
        }
        el
    };
    match g {
        Geometry::Point(c) => {
            let a = c.hae.is_some();
            with_mode(XmlElement::new("Point"), a)
                .with_child(coordinates(std::slice::from_ref(c), a))
        }
        Geometry::Line(cs) => {
            let a = alt(cs);
            with_mode(XmlElement::new("LineString"), a).with_child(coordinates(cs, a))
        }
        Geometry::Polygon(cs) => {
            let a = alt(cs);
            let mut ring: Vec<Coord> = cs.clone();
            if let Some(first) = cs.first() {
                ring.push(*first);
            }
            let boundary = XmlElement::new("outerBoundaryIs")
                .with_child(XmlElement::new("LinearRing").with_child(coordinates(&ring, a)));
            with_mode(XmlElement::new("Polygon"), a).with_child(boundary)
        }
    }
}

fn style_element(ev: &Event) -> Option<XmlElement> {
    let mut style = XmlElement::new("Style");
    let mut any = false;
    if let Some(c) = ev.detail.get::<Color>() {
        style = style.with_child(
            XmlElement::new("IconStyle").with_child(text_element("color", color::kml(c.argb))),
        );
        any = true;
    }
    let stroke = ev.detail.get::<StrokeColor>();
    let weight = ev.detail.get::<StrokeWeight>();
    if stroke.is_some() || weight.is_some() {
        let mut line = XmlElement::new("LineStyle");
        if let Some(c) = stroke {
            line = line.with_child(text_element("color", color::kml(c.argb)));
        }
        if let Some(w) = weight {
            line = line.with_child(text_element("width", w.value.to_string()));
        }
        style = style.with_child(line);
        any = true;
    }
    if let Some(c) = ev.detail.get::<FillColor>() {
        style = style.with_child(
            XmlElement::new("PolyStyle").with_child(text_element("color", color::kml(c.argb))),
        );
        any = true;
    }
    any.then_some(style)
}

fn placemark(ev: &Event, opts: &KmlOptions) -> XmlElement {
    let name = ev
        .detail
        .get::<Contact>()
        .and_then(|c| c.callsign)
        .unwrap_or_else(|| ev.uid.clone());
    let mut pm = XmlElement::new("Placemark")
        .with_attr("id", ev.uid.clone())
        .with_child(text_element("name", name));

    if let Some(r) = ev.detail.get::<Remarks>().filter(|r| !r.text.is_empty()) {
        pm = pm.with_child(text_element("description", r.text));
    }
    if let Ok(when) = timestamp::format(ev.time) {
        pm = pm.with_child(XmlElement::new("TimeStamp").with_child(text_element("when", when)));
    }
    if let Some(style) = style_element(ev) {
        pm = pm.with_child(style);
    }

    let mut data = XmlElement::new("ExtendedData");
    let mut add = |k: &str, v: String| {
        data = std::mem::take(&mut data).with_child(
            XmlElement::new("Data")
                .with_attr("name", k)
                .with_child(text_element("value", v)),
        );
    };
    add("uid", ev.uid.clone());
    add("type", ev.cot_type.to_string());
    add("how", ev.how.clone());
    if let Ok(stale) = timestamp::format(ev.stale) {
        add("stale", stale);
    }
    if let Some(s) = Sidc::from_cot(&ev.cot_type) {
        add("sidc", s.to_string());
    }
    pm.with_child(geometry_element(&geometry_of(ev, &opts.geometry)))
        .with_child(data)
}

/// Renders events as a KML 2.2 document.
pub fn events_to_kml<'a>(events: impl IntoIterator<Item = &'a Event>, opts: &KmlOptions) -> String {
    let mut doc = XmlElement::new("Document");
    if let Some(name) = &opts.document_name {
        doc = doc.with_child(text_element("name", name.clone()));
    }
    for ev in events {
        doc = doc.with_child(placemark(ev, opts));
    }
    let kml = XmlElement::new("kml")
        .with_attr("xmlns", "http://www.opengis.net/kml/2.2")
        .with_child(doc);
    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n{}",
        kml.to_xml()
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use cottak_core::messages::draw::{self, Style};
    use cottak_core::Point;

    fn p(lat: f64, lon: f64) -> Point {
        Point::new(lat, lon).unwrap()
    }

    fn parse(kml: &str) -> XmlElement {
        XmlElement::parse(kml).expect("output must be well-formed XML")
    }

    #[test]
    fn marker_placemark() {
        let ev = Event::new(
            "U1",
            "a-f-G-U-C".parse().unwrap(),
            p(-27.5, 153.0).with_hae(10.0),
        )
        .with(Contact::new("ALPHA & <Co>"))
        .with(Remarks::new("line1\nline2 \"quoted\""))
        .with(Color::from_rgba(255, 0, 0, 255));
        let kml = events_to_kml(
            [&ev],
            &KmlOptions {
                document_name: Some("Test".into()),
                ..Default::default()
            },
        );
        assert!(kml.starts_with("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<kml xmlns=\"http://www.opengis.net/kml/2.2\">"));

        let root = parse(&kml);
        let doc = root.child("Document").unwrap();
        assert_eq!(doc.child("name").unwrap().text(), "Test");
        let pm = doc.child("Placemark").unwrap();
        assert_eq!(pm.attr("id"), Some("U1"));
        assert_eq!(
            pm.child("name").unwrap().text(),
            "ALPHA & <Co>",
            "text survives escaping"
        );
        assert_eq!(
            pm.child("description").unwrap().text(),
            "line1\nline2 \"quoted\""
        );
        let point = pm.child("Point").unwrap();
        assert_eq!(point.child("altitudeMode").unwrap().text(), "absolute");
        assert_eq!(
            point.child("coordinates").unwrap().text(),
            "153,-27.5,10",
            "lon,lat,alt order"
        );
        assert_eq!(
            pm.child("Style")
                .unwrap()
                .child("IconStyle")
                .unwrap()
                .child("color")
                .unwrap()
                .text(),
            "ff0000ff"
        );
        let data = pm.child("ExtendedData").unwrap();
        let by_name = |n: &str| {
            data.elements()
                .find(|d| d.attr("name") == Some(n))
                .map(|d| d.child("value").unwrap().text())
        };
        assert_eq!(by_name("type").as_deref(), Some("a-f-G-U-C"));
        assert_eq!(by_name("sidc").as_deref(), Some("SFGPUC---------"));
    }

    #[test]
    fn polygon_and_line_geometry_and_style() {
        let v = [p(0.0, 0.0), p(0.0, 1.0), p(1.0, 1.0)];
        let poly = draw::polygon("poly", "AO", &v, Style::default()).unwrap();
        let line = draw::polyline("line", "L", &v, Style::default()).unwrap();
        let kml = events_to_kml([&poly, &line], &KmlOptions::default());
        let root = parse(&kml);
        let pms: Vec<_> = root.child("Document").unwrap().elements().collect();
        assert_eq!(pms.len(), 2);

        let ring = pms[0]
            .child("Polygon")
            .unwrap()
            .child("outerBoundaryIs")
            .unwrap()
            .child("LinearRing")
            .unwrap();
        let coords = ring.child("coordinates").unwrap().text();
        let tuples: Vec<_> = coords.split(' ').collect();
        assert_eq!(tuples.len(), 4);
        assert_eq!(tuples.first(), tuples.last(), "KML rings are closed");
        assert!(pms[0].child("Style").unwrap().child("PolyStyle").is_some());
        assert_eq!(
            pms[0]
                .child("Style")
                .unwrap()
                .child("LineStyle")
                .unwrap()
                .child("width")
                .unwrap()
                .text(),
            "3"
        );

        assert!(pms[1].child("LineString").is_some());
        assert!(pms[1].child("Style").unwrap().child("PolyStyle").is_none());
    }

    #[test]
    fn empty_input_is_a_valid_document() {
        let kml = events_to_kml(std::iter::empty(), &KmlOptions::default());
        let root = parse(&kml);
        assert_eq!(root.name, "kml");
        assert!(root.child("Document").unwrap().elements().next().is_none());
    }
}
