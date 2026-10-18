//! KML 2.2 → CoT.
//!
//! Real-world KML is messy, so the importer is lenient about *content* and
//! strict about *safety*:
//!
//! * each `<Placemark>` that cannot be represented (multi-geometries, tracks,
//!   polygons with holes, bad coordinates, ...) is **skipped and reported**
//!   in [`KmlImport::skipped`] instead of failing the whole file;
//! * malformed XML, DTDs/entities, oversized documents and anything beyond
//!   [`ImportLimits`] fail the import.
//!
//! KMZ is a zip archive; extract its `doc.kml` first.

use crate::color;
use crate::error::{Error, Result};
use crate::geojson::ImportLimits;
use cottak_core::detail::{Color, Contact, FillColor, Remarks, StrokeColor};
use cottak_core::messages::draw::{self, Style};
use cottak_core::xml::XmlElement;
use cottak_core::{timestamp, CotType, Event, Point};
use std::collections::HashMap;
use time::Duration;

/// Longest remarks text kept from a `<description>`.
const MAX_REMARKS: usize = 4096;
/// Longest UID accepted from an `id` attribute or `ExtendedData`.
const MAX_UID: usize = 256;

/// A placemark that was not converted, and why.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Skipped {
    /// The placemark's `<name>`, if it has one.
    pub name: Option<String>,
    /// Why it was skipped.
    pub reason: String,
}

/// The result of [`events_from_kml`].
#[derive(Debug, Clone, Default)]
pub struct KmlImport {
    /// Events for the placemarks that could be converted, in document order.
    pub events: Vec<Event>,
    /// Placemarks that could not be converted.
    pub skipped: Vec<Skipped>,
}

/// The part of a name after any namespace prefix (`gx:Track` → `Track`).
fn local(name: &str) -> &str {
    name.rsplit_once(':').map_or(name, |(_, l)| l)
}

fn child<'a>(el: &'a XmlElement, name: &str) -> Option<&'a XmlElement> {
    el.elements().find(|c| local(&c.name) == name)
}

fn children<'a>(el: &'a XmlElement, name: &'a str) -> impl Iterator<Item = &'a XmlElement> {
    el.elements().filter(move |c| local(&c.name) == name)
}

fn text_of(el: &XmlElement, name: &str) -> Option<String> {
    child(el, name)
        .map(|c| c.text().trim().to_owned())
        .filter(|t| !t.is_empty())
}

/// Visits every `<Placemark>` under `el`, in document order.
fn collect_placemarks<'a>(
    el: &'a XmlElement,
    out: &mut Vec<&'a XmlElement>,
    limits: &ImportLimits,
) -> Result<()> {
    for c in el.elements() {
        match local(&c.name) {
            "Placemark" => {
                if out.len() >= limits.max_features {
                    return Err(Error::Limit("too many placemarks"));
                }
                out.push(c);
            }
            "Document" | "Folder" => collect_placemarks(c, out, limits)?,
            _ => {}
        }
    }
    Ok(())
}

// ----- styles -------------------------------------------------------------------------------

/// What we take from a KML style.
#[derive(Debug, Clone, Copy, Default)]
struct StyleInfo {
    icon: Option<i32>,
    stroke: Option<i32>,
    fill: Option<i32>,
    width: Option<f64>,
}

fn style_info(style: &XmlElement) -> StyleInfo {
    let color_in = |group: &str| {
        child(style, group)
            .and_then(|g| text_of(g, "color"))
            .and_then(|c| color::parse_kml(&c))
    };
    let width = child(style, "LineStyle")
        .and_then(|l| text_of(l, "width"))
        .and_then(|w| w.parse::<f64>().ok())
        .filter(|w| w.is_finite() && *w > 0.0 && *w <= 100.0);
    StyleInfo {
        icon: color_in("IconStyle"),
        stroke: color_in("LineStyle"),
        fill: color_in("PolyStyle"),
        width,
    }
}

/// Shared styles by id, with `<StyleMap>` resolved to its `normal` style.
fn collect_styles(root: &XmlElement) -> HashMap<String, StyleInfo> {
    fn walk(
        el: &XmlElement,
        styles: &mut HashMap<String, StyleInfo>,
        maps: &mut Vec<(String, String)>,
    ) {
        for c in el.elements() {
            match local(&c.name) {
                "Style" => {
                    if let Some(id) = c.attr("id") {
                        styles.insert(id.to_owned(), style_info(c));
                    }
                }
                "StyleMap" => {
                    let normal = children(c, "Pair")
                        .find(|p| text_of(p, "key").as_deref() == Some("normal"))
                        .and_then(|p| text_of(p, "styleUrl"));
                    if let (Some(id), Some(url)) = (c.attr("id"), normal) {
                        maps.push((id.to_owned(), url.trim_start_matches('#').to_owned()));
                    }
                }
                "Document" | "Folder" => walk(c, styles, maps),
                _ => {}
            }
        }
    }
    let (mut styles, mut maps) = (HashMap::new(), Vec::new());
    walk(root, &mut styles, &mut maps);
    for (id, target) in maps {
        if let Some(s) = styles.get(&target).copied() {
            styles.insert(id, s);
        }
    }
    styles
}

/// The style of a placemark: its inline `<Style>`, else the one its `<styleUrl>` names.
fn placemark_style(pm: &XmlElement, shared: &HashMap<String, StyleInfo>) -> StyleInfo {
    if let Some(inline) = child(pm, "Style") {
        return style_info(inline);
    }
    text_of(pm, "styleUrl")
        .and_then(|u| shared.get(u.trim_start_matches('#')).copied())
        .unwrap_or_default()
}

// ----- geometry -----------------------------------------------------------------------------

/// One `lon,lat[,alt]` tuple.
fn parse_tuple(s: &str, absolute: bool) -> Option<Point> {
    let mut parts = s.split(',');
    let lon: f64 = parts.next()?.parse().ok()?;
    let lat: f64 = parts.next()?.parse().ok()?;
    let alt = match parts.next() {
        Some(a) => Some(a.parse::<f64>().ok()?),
        None => None,
    };
    if parts.next().is_some() || !lon.is_finite() || !lat.is_finite() {
        return None;
    }
    let mut p = Point::new(lat, lon).ok()?;
    // Altitude is only a height above the ellipsoid when it is absolute.
    if absolute {
        p.hae = alt.filter(|a| a.is_finite());
    }
    Some(p)
}

/// Reads the `<coordinates>` of `el`. `absolute` says whether altitudes are heights above the ellipsoid.
fn parse_coordinates(
    el: &XmlElement,
    absolute: bool,
    limits: &ImportLimits,
) -> std::result::Result<Vec<Point>, Skip> {
    let text = text_of(el, "coordinates").ok_or(Skip::Reason("geometry has no coordinates"))?;
    // Hand-written files often put spaces around the commas inside a tuple.
    let text = text.split(',').map(str::trim).collect::<Vec<_>>().join(",");
    let mut points = Vec::new();
    for tuple in text.split_whitespace() {
        if points.len() >= limits.max_vertices {
            return Err(Skip::Fatal(Error::Limit("too many vertices")));
        }
        points.push(parse_tuple(tuple, absolute).ok_or(Skip::Reason("invalid coordinates"))?);
    }
    Ok(points)
}

/// Why a placemark did not convert.
enum Skip {
    /// Skip this placemark and carry on.
    Reason(&'static str),
    /// Abort the import.
    Fatal(Error),
}

impl From<cottak_core::Error> for Skip {
    fn from(_: cottak_core::Error) -> Self {
        Skip::Reason("not representable as a CoT event")
    }
}

enum Shape {
    Point(Point),
    Line(Vec<Point>),
    Polygon(Vec<Point>),
}

fn shape_of(pm: &XmlElement, limits: &ImportLimits) -> std::result::Result<Shape, Skip> {
    let geometries: Vec<&XmlElement> = pm
        .elements()
        .filter(|c| {
            matches!(
                local(&c.name),
                "Point"
                    | "LineString"
                    | "LinearRing"
                    | "Polygon"
                    | "MultiGeometry"
                    | "Track"
                    | "MultiTrack"
                    | "Model"
            )
        })
        .collect();
    let [geom] = geometries.as_slice() else {
        return Err(Skip::Reason(if geometries.is_empty() {
            "no supported geometry"
        } else {
            "more than one geometry"
        }));
    };
    let absolute = text_of(geom, "altitudeMode").as_deref() == Some("absolute");
    match local(&geom.name) {
        "Point" => {
            let mut pts = parse_coordinates(geom, absolute, limits)?;
            match (pts.pop(), pts.is_empty()) {
                (Some(p), true) => Ok(Shape::Point(p)),
                _ => Err(Skip::Reason("a Point needs exactly one coordinate")),
            }
        }
        "LineString" | "LinearRing" => {
            let pts = parse_coordinates(geom, absolute, limits)?;
            if pts.len() < 2 {
                return Err(Skip::Reason("a line needs at least two coordinates"));
            }
            Ok(Shape::Line(pts))
        }
        "Polygon" => {
            if child(geom, "innerBoundaryIs").is_some() {
                return Err(Skip::Reason("polygons with holes are not supported"));
            }
            let ring = child(geom, "outerBoundaryIs")
                .and_then(|b| child(b, "LinearRing"))
                .ok_or(Skip::Reason("polygon has no outer boundary"))?;
            let mut pts = parse_coordinates(ring, absolute, limits)?;
            if pts.len() >= 2 && pts.first() == pts.last() {
                pts.pop();
            }
            if pts.len() < 3 {
                return Err(Skip::Reason(
                    "a polygon needs at least three distinct coordinates",
                ));
            }
            Ok(Shape::Polygon(pts))
        }
        "MultiGeometry" => Err(Skip::Reason("multi-geometries are not supported")),
        "Track" | "MultiTrack" => Err(Skip::Reason("tracks are not supported")),
        _ => Err(Skip::Reason("3-D models are not supported")),
    }
}

// ----- placemarks ---------------------------------------------------------------------------

fn valid_uid(s: &str) -> bool {
    !s.is_empty() && s.len() <= MAX_UID && !s.chars().any(char::is_control)
}

/// Data from our own export (`<ExtendedData><Data name=..><value>`), by name.
fn extended(pm: &XmlElement, key: &str) -> Option<String> {
    let data = child(pm, "ExtendedData")?;
    children(data, "Data")
        .find(|d| d.attr("name") == Some(key))
        .and_then(|d| text_of(d, "value"))
}

fn remarks_of(pm: &XmlElement) -> Option<String> {
    let raw = text_of(pm, "description")?;
    let cleaned: String = raw
        .chars()
        .filter(|c| !c.is_control() || matches!(c, '\n' | '\t'))
        .take(MAX_REMARKS)
        .collect();
    (!cleaned.is_empty()).then_some(cleaned)
}

fn placemark_to_event(
    pm: &XmlElement,
    shared: &HashMap<String, StyleInfo>,
    limits: &ImportLimits,
) -> std::result::Result<Event, Skip> {
    let shape = shape_of(pm, limits)?;
    let uid = extended(pm, "uid")
        .or_else(|| pm.attr("id").map(str::to_owned))
        .filter(|u| valid_uid(u))
        .unwrap_or_else(Event::random_uid);
    let callsign = text_of(pm, "name").unwrap_or_else(|| uid.clone());
    let style = placemark_style(pm, shared);

    let mut event = match shape {
        Shape::Point(point) => {
            // Our own export records the CoT type; honour it for atoms only.
            let cot_type = extended(pm, "type")
                .filter(|t| t.starts_with("a-"))
                .and_then(|t| t.parse::<CotType>().ok())
                .map_or_else(|| "b-m-p-s-m".parse::<CotType>(), Ok)?;
            let mut ev = Event::new(uid, cot_type, point)
                .with_how("h-g-i-g-o")
                .with_stale_in(Duration::days(1))
                .with(Contact::new(callsign));
            if let Some(argb) = style.icon {
                ev = ev.with(Color { argb });
            }
            ev
        }
        Shape::Line(pts) => draw::polyline(uid, &callsign, &pts, draw_style(style))?,
        Shape::Polygon(pts) => draw::polygon(uid, &callsign, &pts, draw_style(style))?,
    };

    if let Some(r) = remarks_of(pm) {
        event = event.with(Remarks::new(r));
    }
    if let Some(t) = child(pm, "TimeStamp")
        .and_then(|t| text_of(t, "when"))
        .and_then(|w| timestamp::parse(&w).ok())
    {
        event.time = t;
        event.start = t;
    }
    event.validate()?;
    Ok(event)
}

fn draw_style(info: StyleInfo) -> Style {
    let mut style = Style::default();
    if let Some(argb) = info.stroke {
        style.stroke = StrokeColor { argb };
    }
    if let Some(argb) = info.fill {
        style.fill = FillColor { argb };
    }
    if let Some(w) = info.width {
        style.weight = w;
    }
    style
}

/// Converts a KML document to events.
///
/// Accepts a `<kml>` root, or a bare `<Document>`, `<Folder>` or `<Placemark>`.
/// Points become markers, lines and polygons become drawings. Styles (inline,
/// shared via `<styleUrl>`, and `<StyleMap>` normal styles) give colours and
/// line width; `<description>` becomes the remarks.
pub fn events_from_kml(text: &str, limits: &ImportLimits) -> Result<KmlImport> {
    let root = XmlElement::parse(text)?;
    let mut placemarks = Vec::new();
    match local(&root.name) {
        "kml" | "Document" | "Folder" => collect_placemarks(&root, &mut placemarks, limits)?,
        "Placemark" => placemarks.push(&root),
        _ => {
            return Err(Error::InvalidKml(
                "expected a <kml>, <Document>, <Folder> or <Placemark> root".into(),
            ))
        }
    }
    let shared = collect_styles(&root);

    let mut out = KmlImport::default();
    for pm in placemarks {
        match placemark_to_event(pm, &shared, limits) {
            Ok(ev) => out.events.push(ev),
            Err(Skip::Reason(reason)) => out.skipped.push(Skipped {
                name: text_of(pm, "name"),
                reason: reason.to_owned(),
            }),
            Err(Skip::Fatal(e)) => return Err(e),
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kml::{events_to_kml, KmlOptions};
    use cottak_core::detail::StrokeWeight;

    fn import(kml: &str) -> KmlImport {
        events_from_kml(kml, &ImportLimits::default()).unwrap()
    }

    fn p(lat: f64, lon: f64) -> Point {
        Point::new(lat, lon).unwrap()
    }

    #[test]
    fn our_own_export_round_trips() {
        let v = [p(0.0, 0.0), p(0.0, 1.0), p(1.0, 1.0)];
        let marker = Event::new(
            "MARK-1",
            "a-f-G-U-C".parse().unwrap(),
            p(-27.5, 153.0).with_hae(10.0),
        )
        .with(Contact::new("ALPHA & <Co>"))
        .with(Remarks::new("line1\nline2"))
        .with(Color::from_rgba(255, 0, 0, 255));
        let poly = draw::polygon("POLY-1", "AO", &v, Style::default()).unwrap();
        let line = draw::polyline("LINE-1", "L", &v, Style::default()).unwrap();
        let kml = events_to_kml([&marker, &poly, &line], &KmlOptions::default());

        let got = import(&kml);
        assert!(got.skipped.is_empty(), "{:?}", got.skipped);
        assert_eq!(got.events.len(), 3);

        let m = &got.events[0];
        assert_eq!(
            (m.uid.as_str(), m.cot_type.as_str()),
            ("MARK-1", "a-f-G-U-C")
        );
        assert_eq!(
            m.detail.get::<Contact>().unwrap().callsign.as_deref(),
            Some("ALPHA & <Co>")
        );
        assert_eq!(m.detail.get::<Remarks>().unwrap().text, "line1\nline2");
        assert_eq!(
            m.detail.get::<Color>().unwrap().argb,
            Color::from_rgba(255, 0, 0, 255).argb
        );
        assert!((m.point.lat + 27.5).abs() < 1e-9 && (m.point.lon - 153.0).abs() < 1e-9);
        assert_eq!(m.point.hae, Some(10.0), "absolute altitude survives");

        assert_eq!(got.events[1].uid, "POLY-1");
        assert_eq!(got.events[1].cot_type.as_str(), "u-d-f");
        assert_eq!(got.events[2].uid, "LINE-1");
        // Vertices come back as links: the polygon's closing vertex is dropped on import.
        let links = |e: &Event| e.detail.get_all::<cottak_core::detail::Link>().len();
        assert_eq!((links(&got.events[1]), links(&got.events[2])), (3, 3));
    }

    const REAL_WORLD: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<kml xmlns="http://www.opengis.net/kml/2.2" xmlns:gx="http://www.google.com/kml/ext/2.2">
 <Document>
  <name>Mixed</name>
  <Style id="red"><IconStyle><color>ff0000ff</color></IconStyle>
    <LineStyle><color>ff0000ff</color><width>4</width></LineStyle>
    <PolyStyle><color>7f00ff00</color></PolyStyle></Style>
  <StyleMap id="map"><Pair><key>normal</key><styleUrl>#red</styleUrl></Pair>
    <Pair><key>highlight</key><styleUrl>#other</styleUrl></Pair></StyleMap>
  <Folder>
   <name>f</name>
   <Placemark id="p1"><name>Pin</name><styleUrl>#map</styleUrl>
     <description><![CDATA[<b>bold</b> text]]></description>
     <TimeStamp><when>2025-01-31T12:00:00Z</when></TimeStamp>
     <Point><coordinates>153.02,-27.47,100</coordinates></Point></Placemark>
   <Placemark><name>Ground level</name><Point><altitudeMode>clampToGround</altitudeMode>
     <coordinates> 10 , 20 </coordinates></Point></Placemark>
   <Placemark><name>Track</name><LineString><coordinates>
       0,0 1,1
       2,2,50
   </coordinates></LineString><Style><LineStyle><width>2</width></LineStyle></Style></Placemark>
   <Placemark><name>Area</name><Polygon><outerBoundaryIs><LinearRing><coordinates>
       0,0 0,1 1,1 1,0 0,0</coordinates></LinearRing></outerBoundaryIs></Polygon></Placemark>
   <Placemark><name>Prefixed</name><gx:Track><gx:coord>1 2 3</gx:coord></gx:Track></Placemark>
   <Placemark><name>Multi</name><MultiGeometry><Point><coordinates>1,1</coordinates></Point></MultiGeometry></Placemark>
   <Placemark><name>Holes</name><Polygon><outerBoundaryIs><LinearRing><coordinates>0,0 0,1 1,1 0,0</coordinates></LinearRing></outerBoundaryIs>
     <innerBoundaryIs><LinearRing><coordinates>0,0 0,1 1,1 0,0</coordinates></LinearRing></innerBoundaryIs></Polygon></Placemark>
   <Placemark><name>Bad lat</name><Point><coordinates>10,95</coordinates></Point></Placemark>
   <Placemark><name>Nothing</name></Placemark>
   <NetworkLink><name>ignored</name><Link><href>http://example.com/x.kml</href></Link></NetworkLink>
  </Folder>
 </Document>
</kml>"#;

    #[test]
    fn real_world_document_converts_what_it_can_and_reports_the_rest() {
        let got = import(REAL_WORLD);
        let names: Vec<_> = got
            .events
            .iter()
            .map(|e| e.detail.get::<Contact>().unwrap().callsign.unwrap())
            .collect();
        assert_eq!(names, ["Pin", "Ground level", "Track", "Area"]);

        let skipped: Vec<_> = got
            .skipped
            .iter()
            .map(|s| (s.name.as_deref().unwrap(), s.reason.as_str()))
            .collect();
        assert_eq!(
            skipped,
            [
                ("Prefixed", "tracks are not supported"),
                ("Multi", "multi-geometries are not supported"),
                ("Holes", "polygons with holes are not supported"),
                ("Bad lat", "invalid coordinates"),
                ("Nothing", "no supported geometry"),
            ]
        );

        // Style came through the StyleMap, and the description kept its text.
        let pin = &got.events[0];
        assert_eq!(pin.uid, "p1");
        assert_eq!(pin.cot_type.as_str(), "b-m-p-s-m");
        assert_eq!(
            pin.detail.get::<Color>().unwrap().argb,
            Color::from_rgba(255, 0, 0, 255).argb
        );
        assert_eq!(
            pin.detail.get::<Remarks>().unwrap().text,
            "<b>bold</b> text"
        );
        assert_eq!(pin.point.hae, None, "altitude is ignored unless absolute");
        assert_eq!(
            timestamp::format(pin.time).unwrap(),
            "2025-01-31T12:00:00.000Z"
        );

        // Whitespace around coordinates is tolerated, ground-level altitude is dropped.
        let ground = &got.events[1];
        assert!((ground.point.lon - 10.0).abs() < 1e-9 && (ground.point.lat - 20.0).abs() < 1e-9);

        // Inline style on a line, width taken from it.
        let track = &got.events[2];
        assert_eq!(track.detail.get::<StrokeWeight>().unwrap().value, 2.0);
        assert_eq!(track.cot_type.as_str(), "u-d-f");
        // The ring's closing vertex is dropped.
        assert_eq!(
            got.events[3]
                .detail
                .get_all::<cottak_core::detail::Link>()
                .len(),
            4
        );
    }

    #[test]
    fn accepts_bare_roots() {
        let pm =
            r"<Placemark><name>x</name><Point><coordinates>1,2</coordinates></Point></Placemark>";
        assert_eq!(import(pm).events.len(), 1);
        assert_eq!(
            import(&format!("<Document>{pm}{pm}</Document>"))
                .events
                .len(),
            2
        );
        let prefixed = r#"<k:kml xmlns:k="x"><k:Document><k:Placemark><k:Point><k:coordinates>1,2</k:coordinates></k:Point></k:Placemark></k:Document></k:kml>"#;
        assert_eq!(import(prefixed).events.len(), 1);
    }

    #[test]
    fn unsafe_and_oversized_input_is_rejected() {
        let limits = ImportLimits {
            max_features: 2,
            max_vertices: 3,
        };
        let pm = r"<Placemark><Point><coordinates>1,2</coordinates></Point></Placemark>";
        assert!(matches!(
            events_from_kml(&format!("<kml>{pm}{pm}{pm}</kml>"), &limits),
            Err(Error::Limit("too many placemarks"))
        ));
        assert!(matches!(
            events_from_kml(
                "<kml><Placemark><LineString><coordinates>0,0 1,1 2,2 3,3</coordinates></LineString></Placemark></kml>",
                &limits
            ),
            Err(Error::Limit("too many vertices"))
        ));
        for bad in [
            "",
            "not xml",
            "<html/>",
            "<!DOCTYPE kml [<!ENTITY x SYSTEM \"file:///etc/passwd\">]><kml>&x;</kml>",
            "<!DOCTYPE kml [<!ENTITY a \"aaaa\"><!ENTITY b \"&a;&a;&a;&a;\">]><kml>&b;</kml>",
            "<kml><Placemark>",
        ] {
            assert!(
                events_from_kml(bad, &ImportLimits::default()).is_err(),
                "{bad}"
            );
        }
    }

    #[test]
    fn hostile_values_are_neutralised() {
        let kml = "<kml><Placemark id=\"a\u{7}b\"><name>n</name>\
            <description>x\u{1}y&#10;z</description>\
            <Point><coordinates>NaN,0 inf,0</coordinates></Point></Placemark>\
            <Placemark id=\"ok\"><description>x&#1;y</description>\
            <Point><coordinates>1,2</coordinates></Point></Placemark></kml>";
        // Control characters are not even legal in XML, so the document is refused or cleaned.
        let result = events_from_kml(kml, &ImportLimits::default());
        if let Ok(got) = result {
            for e in &got.events {
                assert!(!e.uid.chars().any(char::is_control));
            }
        }

        let long_id = "x".repeat(MAX_UID + 1);
        let got = import(&format!(
            "<kml><Placemark id=\"{long_id}\"><Point><coordinates>1,2</coordinates></Point></Placemark></kml>"
        ));
        assert_ne!(got.events[0].uid, long_id, "an over-long id is replaced");
        let nan = import(
            "<kml><Placemark><Point><coordinates>NaN,0</coordinates></Point></Placemark></kml>",
        );
        assert_eq!(nan.skipped[0].reason, "invalid coordinates");
    }
}
