//! Drawing and measurement messages: circles, polygons, geofences, range
//! rings, bullseyes, range and bearing lines and routes.

use super::{base, how};
use crate::detail::{
    Archive, Bearing, BearingUnits, Bullseye, Color, Contact, Ellipse, FillColor, GeoFence,
    Inclination, LabelsOn, Link, LinkAttr, LinkPoint, NorthRef, Range, RangeUnits, Shape,
    StrokeColor, StrokeWeight,
};
use crate::error::{Error, Result};
use crate::event::Event;
use crate::point::Point;
use time::Duration;

/// Drawn shapes are retained for a day unless the caller changes `stale`.
const SHAPE_TTL: Duration = Duration::days(1);

/// Outline and fill styling for drawn shapes.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    /// Outline colour.
    pub stroke: StrokeColor,
    /// Fill colour.
    pub fill: FillColor,
    /// Outline width.
    pub weight: f64,
}

impl Default for Style {
    /// Opaque red outline, translucent red fill, weight 3.
    fn default() -> Self {
        Self {
            stroke: StrokeColor::from_rgba(255, 0, 0, 255),
            fill: FillColor::from_rgba(255, 0, 0, 64),
            weight: 3.0,
        }
    }
}

fn styled(ev: Event, style: Style) -> Event {
    ev.with(style.stroke)
        .with(StrokeWeight {
            value: style.weight,
        })
        .with(style.fill)
}

/// A drawn circle (`u-d-c-c`) of `radius_m` metres.
pub fn circle(
    uid: impl Into<String>,
    callsign: &str,
    center: Point,
    radius_m: f64,
    style: Style,
) -> Result<Event> {
    if !radius_m.is_finite() || radius_m <= 0.0 {
        return Err(Error::OutOfRange {
            field: "radius",
            value: radius_m,
        });
    }
    let ev = base(uid, "u-d-c-c", how::HUMAN_GIS, center, SHAPE_TTL)?
        .with(Contact::new(callsign))
        .with(Shape {
            ellipses: vec![Ellipse::circle(radius_m)],
            other: Vec::new(),
        })
        .with(LabelsOn::new(false))
        .with(Archive);
    Ok(styled(ev, style))
}

/// Concentric range rings (`u-r-b-c-c`): `rings` circles spaced `spacing_m` apart.
pub fn range_rings(
    uid: impl Into<String>,
    callsign: &str,
    center: Point,
    rings: u8,
    spacing_m: f64,
    style: Style,
) -> Result<Event> {
    if rings == 0 || rings > 50 {
        return Err(Error::OutOfRange {
            field: "rings",
            value: f64::from(rings),
        });
    }
    if !spacing_m.is_finite() || spacing_m <= 0.0 {
        return Err(Error::OutOfRange {
            field: "spacing",
            value: spacing_m,
        });
    }
    let ellipses = (1..=rings)
        .map(|i| Ellipse::circle(f64::from(i) * spacing_m))
        .collect();
    let ev = base(uid, "u-r-b-c-c", how::HUMAN_GIS, center, SHAPE_TTL)?
        .with(Contact::new(callsign))
        .with(RangeUnits::new(1))
        .with(Shape {
            ellipses,
            other: Vec::new(),
        })
        .with(Color::from_rgba(0, 255, 255, 255))
        .with(Archive);
    Ok(styled(ev, style))
}

/// Mean position of `vertices` computed on the unit sphere, so shapes that
/// straddle the antimeridian are handled correctly.
fn centroid(vertices: &[Point]) -> Result<Point> {
    let (mut x, mut y, mut z) = (0.0_f64, 0.0_f64, 0.0_f64);
    for v in vertices {
        let (lat, lon) = (v.lat.to_radians(), v.lon.to_radians());
        x += lat.cos() * lon.cos();
        y += lat.cos() * lon.sin();
        z += lat.sin();
    }
    let n = f64::from(u32::try_from(vertices.len()).unwrap_or(u32::MAX));
    let (x, y, z) = (x / n, y / n, z / n);
    Point::new(z.atan2(x.hypot(y)).to_degrees(), y.atan2(x).to_degrees())
}

fn vertex_links(vertices: &[Point]) -> Vec<Link> {
    vertices
        .iter()
        .map(|v| Link {
            point: Some(LinkPoint {
                lat: v.lat,
                lon: v.lon,
                hae: None,
            }),
            ..Link::default()
        })
        .collect()
}

fn check_polygon(vertices: &[Point]) -> Result<()> {
    if vertices.len() < 3 {
        return Err(Error::TooFew {
            what: "polygon",
            min: 3,
            got: vertices.len(),
        });
    }
    vertices.iter().try_for_each(Point::validate)
}

/// A drawn polygon (`u-d-f`). The shape is implicitly closed; do not repeat
/// the first vertex. The event position is the centroid.
pub fn polygon(
    uid: impl Into<String>,
    callsign: &str,
    vertices: &[Point],
    style: Style,
) -> Result<Event> {
    check_polygon(vertices)?;
    let mut ev = base(uid, "u-d-f", how::HUMAN_GIS, centroid(vertices)?, SHAPE_TTL)?
        .with(Contact::new(callsign));
    for l in vertex_links(vertices) {
        ev.detail.push(l);
    }
    Ok(styled(ev.with(Archive), style))
}

/// A geofence: a polygon plus trigger rules.
pub fn geofence(
    uid: impl Into<String>,
    callsign: &str,
    vertices: &[Point],
    rules: GeoFence,
    style: Style,
) -> Result<Event> {
    Ok(polygon(uid, callsign, vertices, style)?.with(rules))
}

/// A drawn polyline (`u-d-f` without a fill colour): an open line through at least two vertices.
/// The event position is the first vertex.
pub fn polyline(
    uid: impl Into<String>,
    callsign: &str,
    vertices: &[Point],
    style: Style,
) -> Result<Event> {
    if vertices.len() < 2 {
        return Err(Error::TooFew {
            what: "polyline",
            min: 2,
            got: vertices.len(),
        });
    }
    vertices.iter().try_for_each(Point::validate)?;
    let mut ev =
        base(uid, "u-d-f", how::HUMAN_GIS, vertices[0], SHAPE_TTL)?.with(Contact::new(callsign));
    for l in vertex_links(vertices) {
        ev.detail.push(l);
    }
    // No fill colour: that is what distinguishes a line from a polygon.
    Ok(ev.with(Archive).with(style.stroke).with(StrokeWeight {
        value: style.weight,
    }))
}

/// A bullseye reference (`u-r-b-bullseye`).
pub fn bullseye(
    uid: impl Into<String>,
    callsign: &str,
    center: Point,
    bullseye: Bullseye,
) -> Result<Event> {
    Ok(
        base(uid, "u-r-b-bullseye", how::HUMAN_GIS, center, SHAPE_TTL)?
            .with(Archive)
            .with(Contact::new(callsign))
            .with(bullseye),
    )
}

/// A range and bearing line (`u-rb-a`) from `start`.
///
/// Uses ATAK display codes: metric range units, degree bearing units, true north.
pub fn range_bearing_line(
    uid: impl Into<String>,
    callsign: &str,
    start: Point,
    range_m: f64,
    bearing_deg: f64,
    style: Style,
) -> Result<Event> {
    if !range_m.is_finite() || range_m < 0.0 {
        return Err(Error::OutOfRange {
            field: "range",
            value: range_m,
        });
    }
    if !bearing_deg.is_finite() || !(0.0..360.0).contains(&bearing_deg) {
        return Err(Error::OutOfRange {
            field: "bearing",
            value: bearing_deg,
        });
    }
    let ev = base(uid, "u-rb-a", how::HUMAN, start, SHAPE_TTL)?
        .with(Contact::new(callsign))
        .with(Range::new(range_m))
        .with(Bearing::new(bearing_deg))
        .with(Inclination::new(0.0))
        .with(RangeUnits::new(1))
        .with(BearingUnits::new(0))
        .with(NorthRef::new(1))
        .with(Archive);
    Ok(styled(ev, style))
}

/// A named point on a [`route`].
#[derive(Debug, Clone, PartialEq)]
pub struct Waypoint {
    /// Name shown on the map.
    pub name: String,
    /// Position.
    pub point: Point,
    /// `true` for a checkpoint (`b-m-p-c`), `false` for a waypoint (`b-m-p-w`).
    pub checkpoint: bool,
}

impl Waypoint {
    /// A plain waypoint.
    pub fn new(name: impl Into<String>, point: Point) -> Self {
        Self {
            name: name.into(),
            point,
            checkpoint: false,
        }
    }
}

/// A route (`b-m-r`) through at least two waypoints. The event position is the start.
pub fn route(
    uid: impl Into<String>,
    callsign: &str,
    waypoints: &[Waypoint],
    attrs: LinkAttr,
) -> Result<Event> {
    if waypoints.len() < 2 {
        return Err(Error::TooFew {
            what: "route",
            min: 2,
            got: waypoints.len(),
        });
    }
    waypoints.iter().try_for_each(|w| w.point.validate())?;
    let mut ev = base(uid, "b-m-r", how::HUMAN_GIS, waypoints[0].point, SHAPE_TTL)?
        .with(Contact::new(callsign));
    for w in waypoints {
        ev.detail.push(Link {
            uid: Some(Event::random_uid()),
            cot_type: Some(if w.checkpoint { "b-m-p-c" } else { "b-m-p-w" }.to_owned()),
            relation: Some("c".to_owned()),
            callsign: Some(w.name.clone()),
            point: Some(LinkPoint {
                lat: w.point.lat,
                lon: w.point.lon,
                hae: w.point.hae,
            }),
            ..Link::default()
        });
    }
    Ok(ev.with(attrs).with(Archive))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(lat: f64, lon: f64) -> Point {
        Point::new(lat, lon).unwrap()
    }

    fn round_trips(ev: &Event) {
        assert_eq!(&ev.to_xml_string().unwrap().parse::<Event>().unwrap(), ev);
    }

    #[test]
    fn circle_has_radius_and_style() {
        let ev = circle("c1", "FENCE", p(1.0, 2.0), 250.0, Style::default()).unwrap();
        let shape = ev.detail.get::<Shape>().unwrap();
        assert_eq!(shape.ellipses[0].major, Some(250.0));
        assert_eq!(
            ev.detail.get::<StrokeColor>().unwrap().rgba(),
            (255, 0, 0, 255)
        );
        assert_eq!(ev.cot_type.as_str(), "u-d-c-c");
        round_trips(&ev);
        assert!(circle("c", "x", p(0.0, 0.0), 0.0, Style::default()).is_err());
    }

    #[test]
    fn range_rings_are_concentric() {
        let ev = range_rings("r", "RR", p(0.0, 0.0), 3, 100.0, Style::default()).unwrap();
        let radii: Vec<_> = ev
            .detail
            .get::<Shape>()
            .unwrap()
            .ellipses
            .iter()
            .map(|e| e.major.unwrap())
            .collect();
        assert_eq!(radii, [100.0, 200.0, 300.0]);
        round_trips(&ev);
        assert!(range_rings("r", "RR", p(0.0, 0.0), 0, 100.0, Style::default()).is_err());
    }

    #[test]
    fn polygon_vertices_and_centroid() {
        let v = [p(0.0, 0.0), p(0.0, 2.0), p(2.0, 2.0), p(2.0, 0.0)];
        let ev = polygon("poly", "AO", &v, Style::default()).unwrap();
        assert_eq!(ev.detail.get_all::<Link>().len(), 4);
        assert!((ev.point.lat - 1.0).abs() < 0.01 && (ev.point.lon - 1.0).abs() < 0.01);
        round_trips(&ev);
        assert!(polygon("poly", "AO", &v[..2], Style::default()).is_err());
    }

    #[test]
    fn centroid_handles_antimeridian() {
        let v = [p(0.0, 179.0), p(0.0, -179.0), p(1.0, -179.0), p(1.0, 179.0)];
        let c = centroid(&v).unwrap();
        assert!(c.lon.abs() > 179.0, "lon {}", c.lon);
    }

    #[test]
    fn geofence_adds_rules() {
        let v = [p(0.0, 0.0), p(0.0, 1.0), p(1.0, 1.0)];
        let rules = GeoFence {
            trigger: Some("Entry".into()),
            ..GeoFence::default()
        };
        let ev = geofence("g", "GF", &v, rules, Style::default()).unwrap();
        assert_eq!(
            ev.detail.get::<GeoFence>().unwrap().trigger.as_deref(),
            Some("Entry")
        );
        round_trips(&ev);
    }

    #[test]
    fn range_bearing_validates() {
        let ev =
            range_bearing_line("rb", "RB", p(0.0, 0.0), 1000.0, 45.0, Style::default()).unwrap();
        assert_eq!(ev.detail.get::<Bearing>().unwrap().value, 45.0);
        round_trips(&ev);
        assert!(range_bearing_line("rb", "RB", p(0.0, 0.0), 1.0, 360.0, Style::default()).is_err());
        assert!(range_bearing_line("rb", "RB", p(0.0, 0.0), -1.0, 0.0, Style::default()).is_err());
    }

    #[test]
    fn route_links() {
        let w = [
            Waypoint::new("SP", p(0.0, 0.0)),
            Waypoint::new("RP", p(0.1, 0.1)),
        ];
        let ev = route("rt", "ROUTE", &w, LinkAttr::default()).unwrap();
        let links = ev.detail.get_all::<Link>();
        assert_eq!(links.len(), 2);
        assert_eq!(links[1].callsign.as_deref(), Some("RP"));
        round_trips(&ev);
        assert!(route("rt", "R", &w[..1], LinkAttr::default()).is_err());
    }

    #[test]
    fn bullseye_event() {
        let b = Bullseye {
            title: Some("BE".into()),
            ring_num: Some(3),
            ..Bullseye::default()
        };
        let ev = bullseye("b", "BE", p(1.0, 1.0), b).unwrap();
        assert_eq!(ev.cot_type.as_str(), "u-r-b-bullseye");
        round_trips(&ev);
    }

    #[test]
    fn polyline_has_no_fill() {
        let v = [p(0.0, 0.0), p(0.5, 0.5), p(1.0, 0.0)];
        let ev = polyline("pl", "LINE", &v, Style::default()).unwrap();
        assert_eq!(ev.detail.get_all::<Link>().len(), 3);
        assert!(ev.detail.get::<FillColor>().is_none());
        assert_eq!(ev.point, v[0]);
        round_trips(&ev);
        assert!(polyline("pl", "L", &v[..1], Style::default()).is_err());
    }
}
