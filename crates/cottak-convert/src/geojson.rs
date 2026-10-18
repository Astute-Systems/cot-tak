//! CoT ⇄ GeoJSON (RFC 7946).
//!
//! Export turns each event into a `Feature` whose properties carry the CoT
//! identity (`uid`, `type`, `how`, times), human fields (`callsign`,
//! `remarks`, `team`, `role`), motion (`course`, `speed`) and styling in the
//! widely supported *simplestyle* vocabulary (`stroke`, `stroke-width`,
//! `stroke-opacity`, `fill`, `fill-opacity`, `marker-color`).
//!
//! Import accepts a `Feature`, a `FeatureCollection` or a bare geometry of
//! type `Point`, `LineString` or `Polygon` (without holes). Multi-part
//! geometries are rejected rather than silently dropped.

use crate::color;
use crate::error::{Error, Result};
use crate::geometry::{circle_radius, geometry_of, Coord, Geometry, GeometryOptions};
use crate::sidc::Sidc;
use cottak_core::detail::{
    Color, Contact, FillColor, Group, Remarks, Status, StrokeColor, StrokeWeight, Track,
};
use cottak_core::messages::draw::{self, Style};
use cottak_core::{timestamp, CotType, Event, Point};
use serde_json::{json, Map, Value};
use time::Duration;

/// Options for export.
#[derive(Debug, Clone, Copy)]
pub struct GeoJsonOptions {
    /// How shapes are turned into geometry.
    pub geometry: GeometryOptions,
    /// Add a `sidc` property (MIL-STD-2525 code) for representable atom types.
    pub include_sidc: bool,
}

impl Default for GeoJsonOptions {
    fn default() -> Self {
        Self {
            geometry: GeometryOptions::default(),
            include_sidc: true,
        }
    }
}

/// Limits applied when importing untrusted GeoJSON.
#[derive(Debug, Clone, Copy)]
pub struct ImportLimits {
    /// Most features accepted in one collection.
    pub max_features: usize,
    /// Most vertices accepted in one geometry.
    pub max_vertices: usize,
}

impl Default for ImportLimits {
    fn default() -> Self {
        Self {
            max_features: 10_000,
            max_vertices: 10_000,
        }
    }
}

// ---------------------------------------------------------------------------------------------
// Export
// ---------------------------------------------------------------------------------------------

fn position(c: &Coord) -> Value {
    match c.hae {
        Some(h) => json!([c.lon, c.lat, h]),
        None => json!([c.lon, c.lat]),
    }
}

/// A ring closed for GeoJSON (first position repeated at the end).
fn closed_ring(vs: &[Coord]) -> Value {
    let mut ring: Vec<Value> = vs.iter().map(position).collect();
    if let Some(first) = vs.first() {
        ring.push(position(first));
    }
    Value::Array(ring)
}

fn geometry_json(g: &Geometry) -> Value {
    match g {
        Geometry::Point(c) => json!({"type": "Point", "coordinates": position(c)}),
        Geometry::Line(vs) => {
            json!({"type": "LineString", "coordinates": vs.iter().map(position).collect::<Vec<_>>()})
        }
        Geometry::Polygon(vs) => json!({"type": "Polygon", "coordinates": [closed_ring(vs)]}),
    }
}

fn properties(ev: &Event, geometry: &Geometry, opts: &GeoJsonOptions) -> Map<String, Value> {
    let mut p = Map::new();
    let mut put = |k: &str, v: Value| {
        p.insert(k.to_owned(), v);
    };
    put("uid", json!(ev.uid));
    put("type", json!(ev.cot_type.as_str()));
    put("how", json!(ev.how));
    for (key, t) in [("time", ev.time), ("start", ev.start), ("stale", ev.stale)] {
        if let Ok(s) = timestamp::format(t) {
            put(key, json!(s));
        }
    }
    if let Some(v) = ev.point.ce {
        put("ce", json!(v));
    }
    if let Some(v) = ev.point.le {
        put("le", json!(v));
    }

    if let Some(c) = ev.detail.get::<Contact>().and_then(|c| c.callsign) {
        put("callsign", json!(c));
    }
    if let Some(r) = ev.detail.get::<Remarks>().filter(|r| !r.text.is_empty()) {
        put("remarks", json!(r.text));
    }
    if let Some(g) = ev.detail.get::<Group>() {
        if let Some(n) = g.name {
            put("team", json!(n));
        }
        if let Some(r) = g.role {
            put("role", json!(r));
        }
    }
    if let Some(t) = ev.detail.get::<Track>() {
        if let Some(c) = t.course {
            put("course", json!(c));
        }
        if let Some(s) = t.speed {
            put("speed", json!(s));
        }
    }
    if let Some(b) = ev.detail.get::<Status>().and_then(|s| s.battery) {
        put("battery", json!(b));
    }

    if let Some(c) = ev.detail.get::<StrokeColor>() {
        put("stroke", json!(color::hex(c.argb)));
        put("stroke-opacity", json!(color::opacity(c.argb)));
    }
    if let Some(w) = ev.detail.get::<StrokeWeight>() {
        put("stroke-width", json!(w.value));
    }
    if let Some(c) = ev.detail.get::<FillColor>() {
        put("fill", json!(color::hex(c.argb)));
        put("fill-opacity", json!(color::opacity(c.argb)));
    }
    if let Some(c) = ev.detail.get::<Color>() {
        put("marker-color", json!(color::hex(c.argb)));
    }

    if let Some(r) = circle_radius(ev) {
        put("shape", json!("circle"));
        put("radius", json!(r));
    }
    if matches!(geometry, Geometry::Polygon(_)) {
        put("closed", json!(true));
    }
    if opts.include_sidc {
        if let Some(s) = Sidc::from_cot(&ev.cot_type) {
            put("sidc", json!(s.as_str()));
        }
    }
    p
}

/// Converts one event to a GeoJSON `Feature`.
pub fn event_to_feature(ev: &Event, opts: &GeoJsonOptions) -> Value {
    let geometry = geometry_of(ev, &opts.geometry);
    json!({
        "type": "Feature",
        "id": ev.uid,
        "geometry": geometry_json(&geometry),
        "properties": Value::Object(properties(ev, &geometry, opts)),
    })
}

/// Converts events to a GeoJSON `FeatureCollection`.
pub fn events_to_collection<'a>(
    events: impl IntoIterator<Item = &'a Event>,
    opts: &GeoJsonOptions,
) -> Value {
    json!({
        "type": "FeatureCollection",
        "features": events.into_iter().map(|e| event_to_feature(e, opts)).collect::<Vec<_>>(),
    })
}

// ---------------------------------------------------------------------------------------------
// Import
// ---------------------------------------------------------------------------------------------

fn bad(msg: impl Into<String>) -> Error {
    Error::InvalidGeoJson(msg.into())
}

fn number(v: &Value, what: &str) -> Result<f64> {
    v.as_f64()
        .filter(|n| n.is_finite())
        .ok_or_else(|| bad(format!("{what} must be a finite number")))
}

fn parse_position(v: &Value) -> Result<Coord> {
    let arr = v
        .as_array()
        .filter(|a| a.len() >= 2)
        .ok_or_else(|| bad("position needs [lon, lat]"))?;
    let lon = number(&arr[0], "longitude")?;
    let lat = number(&arr[1], "latitude")?;
    let hae = arr.get(2).map(|a| number(a, "altitude")).transpose()?;
    Point::new(lat, lon)?; // range check
    Ok(Coord { lat, lon, hae })
}

fn parse_positions(v: &Value, limits: &ImportLimits) -> Result<Vec<Coord>> {
    let arr = v
        .as_array()
        .ok_or_else(|| bad("coordinates must be an array of positions"))?;
    if arr.len() > limits.max_vertices {
        return Err(Error::Limit("too many vertices"));
    }
    arr.iter().map(parse_position).collect()
}

fn to_points(vs: &[Coord]) -> Result<Vec<Point>> {
    vs.iter()
        .map(|c| {
            let mut p = Point::new(c.lat, c.lon)?;
            p.hae = c.hae;
            Ok(p)
        })
        .collect()
}

fn string_prop<'a>(props: &'a Map<String, Value>, keys: &[&str]) -> Option<&'a str> {
    keys.iter()
        .find_map(|k| props.get(*k)?.as_str())
        .filter(|s| !s.is_empty())
}

fn style_from(props: &Map<String, Value>) -> Style {
    let mut style = Style::default();
    let opacity =
        |key: &str, default: f64| props.get(key).and_then(Value::as_f64).unwrap_or(default);
    if let Some(rgb) = string_prop(props, &["stroke"]).and_then(color::parse_hex) {
        style.stroke = StrokeColor {
            argb: color::pack(rgb, opacity("stroke-opacity", 1.0)),
        };
    }
    if let Some(rgb) = string_prop(props, &["fill"]).and_then(color::parse_hex) {
        style.fill = FillColor {
            argb: color::pack(rgb, opacity("fill-opacity", 0.25)),
        };
    }
    if let Some(w) = props.get("stroke-width").and_then(Value::as_f64) {
        if w.is_finite() && w > 0.0 && w <= 100.0 {
            style.weight = w;
        }
    }
    style
}

fn uid_of(feature: &Map<String, Value>, props: &Map<String, Value>) -> Result<String> {
    let from_id = match feature.get("id") {
        Some(Value::String(s)) => Some(s.clone()),
        Some(Value::Number(n)) => Some(n.to_string()),
        _ => None,
    };
    let uid = string_prop(props, &["uid"])
        .map(str::to_owned)
        .or(from_id)
        .unwrap_or_else(Event::random_uid);
    if uid.len() > 256 || uid.chars().any(char::is_control) {
        return Err(bad("uid must be at most 256 printable characters"));
    }
    Ok(uid)
}

fn feature_to_event(feature: &Map<String, Value>, limits: &ImportLimits) -> Result<Event> {
    let empty = Map::new();
    let props = feature
        .get("properties")
        .and_then(Value::as_object)
        .unwrap_or(&empty);
    let geometry = feature
        .get("geometry")
        .and_then(Value::as_object)
        .ok_or(Error::Unsupported("features without a geometry"))?;
    let kind = geometry
        .get("type")
        .and_then(Value::as_str)
        .ok_or_else(|| bad("geometry has no type"))?;
    let coordinates = geometry
        .get("coordinates")
        .ok_or_else(|| bad("geometry has no coordinates"))?;

    let uid = uid_of(feature, props)?;
    let callsign = string_prop(props, &["callsign", "name", "title"])
        .unwrap_or(&uid)
        .to_owned();
    let remarks = string_prop(props, &["remarks", "description"]);
    let style = style_from(props);

    let mut event = match kind {
        "Point" => {
            let c = parse_position(coordinates)?;
            let mut point = Point::new(c.lat, c.lon)?;
            point.hae = c.hae;
            let cot_type = string_prop(props, &["type"])
                .and_then(|t| t.parse::<CotType>().ok())
                .map_or_else(|| "b-m-p-s-m".parse::<CotType>(), Ok)?;
            let mut ev = Event::new(uid, cot_type, point)
                .with_how("h-g-i-g-o")
                .with_stale_in(Duration::days(1))
                .with(Contact::new(callsign));
            if let Some(rgb) = string_prop(props, &["marker-color"]).and_then(color::parse_hex) {
                ev = ev.with(Color {
                    argb: color::pack(rgb, 1.0),
                });
            }
            ev
        }
        "LineString" => {
            let vs = parse_positions(coordinates, limits)?;
            if vs.len() < 2 {
                return Err(bad("a LineString needs at least two positions"));
            }
            draw::polyline(uid, &callsign, &to_points(&vs)?, style)?
        }
        "Polygon" => {
            let rings = coordinates
                .as_array()
                .ok_or_else(|| bad("Polygon coordinates must be an array of rings"))?;
            match rings.len() {
                0 => return Err(bad("a Polygon needs an outer ring")),
                1 => {}
                _ => return Err(Error::Unsupported("polygons with holes")),
            }
            let mut vs = parse_positions(&rings[0], limits)?;
            if vs.len() >= 2 && vs.first() == vs.last() {
                vs.pop();
            }
            if vs.len() < 3 {
                return Err(bad(
                    "a Polygon ring needs at least three distinct positions",
                ));
            }
            draw::polygon(uid, &callsign, &to_points(&vs)?, style)?
        }
        "MultiPoint" | "MultiLineString" | "MultiPolygon" | "GeometryCollection" => {
            return Err(Error::Unsupported(
                "multi-part geometries and geometry collections",
            ));
        }
        _ => return Err(bad(format!("unknown geometry type `{kind}`"))),
    };

    if let Some(r) = remarks {
        event = event.with(Remarks::new(r));
    }
    event.validate()?;
    Ok(event)
}

/// Converts a GeoJSON `Feature`, `FeatureCollection` or bare geometry to events.
pub fn events_from_geojson(doc: &Value, limits: &ImportLimits) -> Result<Vec<Event>> {
    let obj = doc
        .as_object()
        .ok_or_else(|| bad("document must be a JSON object"))?;
    match obj.get("type").and_then(Value::as_str) {
        Some("FeatureCollection") => {
            let features = obj
                .get("features")
                .and_then(Value::as_array)
                .ok_or_else(|| bad("no features array"))?;
            if features.len() > limits.max_features {
                return Err(Error::Limit("too many features"));
            }
            features
                .iter()
                .map(|f| {
                    feature_to_event(
                        f.as_object()
                            .ok_or_else(|| bad("feature must be an object"))?,
                        limits,
                    )
                })
                .collect()
        }
        Some("Feature") => Ok(vec![feature_to_event(obj, limits)?]),
        Some(
            "Point" | "LineString" | "Polygon" | "MultiPoint" | "MultiLineString" | "MultiPolygon"
            | "GeometryCollection",
        ) => {
            let mut wrapper = Map::new();
            wrapper.insert("geometry".to_owned(), doc.clone());
            Ok(vec![feature_to_event(&wrapper, limits)?])
        }
        _ => Err(bad("expected a Feature, FeatureCollection or geometry")),
    }
}

/// Parses GeoJSON text and converts it to events.
pub fn events_from_str(text: &str, limits: &ImportLimits) -> Result<Vec<Event>> {
    let doc: Value = serde_json::from_str(text).map_err(|e| bad(e.to_string()))?;
    events_from_geojson(&doc, limits)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cottak_core::detail::LinkAttr;
    use cottak_core::messages::draw::Waypoint;

    fn p(lat: f64, lon: f64) -> Point {
        Point::new(lat, lon).unwrap()
    }

    #[test]
    fn marker_exports_with_rfc7946_axis_order_and_properties() {
        let ev = Event::new(
            "U1",
            "a-f-G-U-C".parse().unwrap(),
            p(-27.5, 153.0).with_hae(12.0).with_errors(5.0, 6.0),
        )
        .with(Contact::new("ALPHA"))
        .with(Remarks::new("hello"))
        .with(Track::new(90.0, 2.0));
        let f = event_to_feature(&ev, &GeoJsonOptions::default());
        assert_eq!(f["geometry"]["type"], "Point");
        assert_eq!(
            f["geometry"]["coordinates"],
            json!([153.0, -27.5, 12.0]),
            "[lon, lat, alt]"
        );
        assert_eq!(f["id"], "U1");
        let props = &f["properties"];
        assert_eq!(props["callsign"], "ALPHA");
        assert_eq!(props["remarks"], "hello");
        assert_eq!(props["course"], 90.0);
        assert_eq!(props["ce"], 5.0);
        assert_eq!(props["sidc"], "SFGPUC---------");
        assert!(props["time"].as_str().unwrap().ends_with('Z'));
    }

    #[test]
    fn shapes_export_as_closed_rings_and_lines_with_style() {
        let v = [p(0.0, 0.0), p(0.0, 1.0), p(1.0, 1.0)];
        let poly = draw::polygon("poly", "AO", &v, Style::default()).unwrap();
        let f = event_to_feature(&poly, &GeoJsonOptions::default());
        let ring = f["geometry"]["coordinates"][0].as_array().unwrap();
        assert_eq!(ring.len(), 4);
        assert_eq!(ring.first(), ring.last(), "RFC 7946 rings are closed");
        assert_eq!(f["properties"]["stroke"], "#ff0000");
        assert_eq!(f["properties"]["fill-opacity"], 0.25);
        assert_eq!(f["properties"]["stroke-width"], 3.0);

        let route = draw::route(
            "r",
            "R",
            &[Waypoint::new("A", v[0]), Waypoint::new("B", v[1])],
            LinkAttr::default(),
        )
        .unwrap();
        assert_eq!(
            event_to_feature(&route, &GeoJsonOptions::default())["geometry"]["type"],
            "LineString"
        );

        let circle = draw::circle("c", "C", v[0], 100.0, Style::default()).unwrap();
        let f = event_to_feature(&circle, &GeoJsonOptions::default());
        assert_eq!(
            (
                f["geometry"]["type"].as_str(),
                f["properties"]["radius"].as_f64()
            ),
            (Some("Polygon"), Some(100.0))
        );
    }

    #[test]
    fn collection_round_trips_through_import() {
        let events = vec![
            Event::new("M1", "a-h-G".parse().unwrap(), p(1.0, 2.0)).with(Contact::new("HOSTILE")),
            draw::polygon(
                "P1",
                "AO",
                &[p(0.0, 0.0), p(0.0, 1.0), p(1.0, 1.0)],
                Style::default(),
            )
            .unwrap(),
            draw::polyline("L1", "LINE", &[p(0.0, 0.0), p(1.0, 1.0)], Style::default()).unwrap(),
        ];
        let doc = events_to_collection(&events, &GeoJsonOptions::default());
        let back = events_from_geojson(&doc, &ImportLimits::default()).unwrap();
        assert_eq!(back.len(), 3);

        assert_eq!(
            (back[0].uid.as_str(), back[0].cot_type.as_str()),
            ("M1", "a-h-G")
        );
        assert_eq!(
            back[0].detail.get::<Contact>().unwrap().callsign.as_deref(),
            Some("HOSTILE")
        );
        assert_eq!((back[0].point.lat, back[0].point.lon), (1.0, 2.0));

        assert_eq!(back[1].cot_type.as_str(), "u-d-f");
        assert!(back[1].detail.get::<FillColor>().is_some());
        assert_eq!(
            back[1].detail.get_all::<cottak_core::detail::Link>().len(),
            3,
            "closing vertex is not duplicated"
        );

        assert!(back[2].detail.get::<FillColor>().is_none());
        assert_eq!(
            back[2].detail.get_all::<cottak_core::detail::Link>().len(),
            2
        );
        for e in &back {
            // Imported events are valid CoT that survives an XML round trip.
            assert_eq!(&e.to_xml_string().unwrap().parse::<Event>().unwrap(), e);
        }
    }

    #[test]
    fn imports_simplestyle_and_bare_geometries() {
        let text = r##"{"type":"Feature","id":42,
            "geometry":{"type":"Polygon","coordinates":[[[0,0],[1,0],[1,1],[0,0]]]},
            "properties":{"name":"Zone","stroke":"#00ff00","stroke-opacity":0.5,"fill":"#0000ff","stroke-width":5,"description":"keep out"}}"##;
        let ev = &events_from_str(text, &ImportLimits::default()).unwrap()[0];
        assert_eq!(ev.uid, "42");
        assert_eq!(
            ev.detail.get::<Contact>().unwrap().callsign.as_deref(),
            Some("Zone")
        );
        assert_eq!(ev.detail.get::<Remarks>().unwrap().text, "keep out");
        let stroke = ev.detail.get::<StrokeColor>().unwrap();
        assert_eq!(
            (color::hex(stroke.argb), color::opacity(stroke.argb)),
            ("#00ff00".to_owned(), 0.5)
        );
        assert_eq!(ev.detail.get::<StrokeWeight>().unwrap().value, 5.0);

        let bare = events_from_str(
            r#"{"type":"Point","coordinates":[10,20,30]}"#,
            &ImportLimits::default(),
        )
        .unwrap();
        assert_eq!(
            (bare[0].point.lat, bare[0].point.lon, bare[0].point.hae),
            (20.0, 10.0, Some(30.0))
        );
        assert_eq!(bare[0].cot_type.as_str(), "b-m-p-s-m");
    }

    #[test]
    fn rejects_bad_and_unsupported_input() {
        let l = ImportLimits::default();
        for (name, doc) in [
            ("not json", "["),
            ("array root", "[]"),
            ("unknown type", r#"{"type":"Blob"}"#),
            (
                "lat out of range",
                r#"{"type":"Point","coordinates":[0,95]}"#,
            ),
            (
                "lon out of range",
                r#"{"type":"Point","coordinates":[181,0]}"#,
            ),
            (
                "string coordinate",
                r#"{"type":"Point","coordinates":["1",2]}"#,
            ),
            ("short position", r#"{"type":"Point","coordinates":[1]}"#),
            (
                "one-point line",
                r#"{"type":"LineString","coordinates":[[0,0]]}"#,
            ),
            (
                "two-point ring",
                r#"{"type":"Polygon","coordinates":[[[0,0],[1,1],[0,0]]]}"#,
            ),
            (
                "holes",
                r#"{"type":"Polygon","coordinates":[[[0,0],[2,0],[2,2],[0,0]],[[0.5,0.5],[1,0.5],[1,1],[0.5,0.5]]]}"#,
            ),
            ("multi", r#"{"type":"MultiPoint","coordinates":[[0,0]]}"#),
            (
                "no geometry",
                r#"{"type":"Feature","geometry":null,"properties":{}}"#,
            ),
        ] {
            assert!(events_from_str(doc, &l).is_err(), "{name}");
        }
    }

    #[test]
    fn limits_are_enforced() {
        let tight = ImportLimits {
            max_features: 1,
            max_vertices: 2,
        };
        let two = r#"{"type":"FeatureCollection","features":[{"type":"Feature","geometry":{"type":"Point","coordinates":[0,0]}},{"type":"Feature","geometry":{"type":"Point","coordinates":[1,1]}}]}"#;
        assert!(matches!(events_from_str(two, &tight), Err(Error::Limit(_))));
        let long = r#"{"type":"LineString","coordinates":[[0,0],[1,1],[2,2]]}"#;
        assert!(matches!(
            events_from_str(long, &tight),
            Err(Error::Limit(_))
        ));
    }

    #[test]
    fn invalid_uid_is_rejected_and_missing_uid_is_generated() {
        let l = ImportLimits::default();
        let ctl = "{\"type\":\"Feature\",\"properties\":{\"uid\":\"a\\u0007b\"},\"geometry\":{\"type\":\"Point\",\"coordinates\":[0,0]}}";
        assert!(events_from_str(ctl, &l).is_err());
        let none = r#"{"type":"Point","coordinates":[0,0]}"#;
        let a = &events_from_str(none, &l).unwrap()[0];
        let b = &events_from_str(none, &l).unwrap()[0];
        assert_ne!(a.uid, b.uid);
    }

    #[test]
    fn absurd_altitudes_are_rejected() {
        // Found by fuzzing: 1.2e129 collides with CoT's "unknown" sentinel.
        let doc = r#"{"type":"Point","coordinates":[1,2,1.2222222222222221e129]}"#;
        assert!(events_from_str(doc, &ImportLimits::default()).is_err());
    }

    #[test]
    fn control_characters_in_text_are_rejected() {
        // Found by fuzzing: a form feed in a name was dropped when the event was written as XML.
        let doc = "{\"type\":\"Feature\",\"properties\":{\"name\":\"a\\fb\"},\"geometry\":{\"type\":\"Point\",\"coordinates\":[0,0]}}";
        assert!(events_from_str(doc, &ImportLimits::default()).is_err());
    }
}
