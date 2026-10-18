//! Extracting map geometry from a CoT event.
//!
//! CoT has no single geometry field. This module applies these rules, which
//! match what TAK clients draw:
//!
//! * `b-m-r` routes are lines through their `<link point>` vertices.
//! * `u-d-r` rectangles are polygons through their vertices.
//! * `u-d-f` shapes are polygons when they have a fill colour or a repeated
//!   first vertex, and lines otherwise.
//! * Circles (`<shape><ellipse>` with equal radii; `u-d-c-c`, `u-r-b-c-c`)
//!   become polygons approximating the circle on a sphere. Ellipses with
//!   different radii stay points, because their orientation convention is not
//!   reliable across clients.
//! * `u-rb-a` range and bearing lines become a two-point line.
//! * Everything else is a point.

use cottak_core::detail::{Bearing, Ellipse, FillColor, Link, Range, Shape};
use cottak_core::Event;

/// Mean Earth radius in metres. Great-circle maths on a sphere is accurate to
/// about 0.3 % for the distances CoT shapes cover.
const EARTH_RADIUS_M: f64 = 6_371_008.8;

/// A position with optional height above ellipsoid.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coord {
    /// Latitude in degrees.
    pub lat: f64,
    /// Longitude in degrees.
    pub lon: f64,
    /// Height above ellipsoid in metres.
    pub hae: Option<f64>,
}

/// Geometry of an event.
#[derive(Debug, Clone, PartialEq)]
pub enum Geometry {
    /// A single position.
    Point(Coord),
    /// An open line of at least two vertices.
    Line(Vec<Coord>),
    /// A polygon ring of at least three vertices. The ring is **not** closed
    /// (the first vertex is not repeated at the end).
    Polygon(Vec<Coord>),
}

/// Options for geometry extraction.
#[derive(Debug, Clone, Copy)]
pub struct GeometryOptions {
    /// Number of vertices used to approximate a circle (clamped to 8..=720).
    pub circle_segments: usize,
}

impl Default for GeometryOptions {
    fn default() -> Self {
        Self {
            circle_segments: 64,
        }
    }
}

/// The point reached by travelling `distance_m` from `(lat, lon)` along an
/// initial `bearing_deg` (clockwise from north) on a sphere.
pub fn destination(lat: f64, lon: f64, bearing_deg: f64, distance_m: f64) -> (f64, f64) {
    let (phi1, lam1) = (lat.to_radians(), lon.to_radians());
    let (theta, delta) = (bearing_deg.to_radians(), distance_m / EARTH_RADIUS_M);
    let phi2 = (phi1.sin() * delta.cos() + phi1.cos() * delta.sin() * theta.cos()).asin();
    let lam2 = lam1
        + (theta.sin() * delta.sin() * phi1.cos()).atan2(delta.cos() - phi1.sin() * phi2.sin());
    let lon2 = (lam2.to_degrees() + 540.0).rem_euclid(360.0) - 180.0;
    (phi2.to_degrees(), lon2)
}

/// Great-circle distance in metres between two points (haversine).
pub fn distance_m(a: (f64, f64), b: (f64, f64)) -> f64 {
    let (p1, p2) = (a.0.to_radians(), b.0.to_radians());
    let (dp, dl) = (p2 - p1, (b.1 - a.1).to_radians());
    let h = (dp / 2.0).sin().powi(2) + p1.cos() * p2.cos() * (dl / 2.0).sin().powi(2);
    2.0 * EARTH_RADIUS_M * h.sqrt().asin()
}

fn circle(center: Coord, radius_m: f64, segments: usize) -> Vec<Coord> {
    let n = segments.clamp(8, 720);
    // n <= 720, so these conversions are exact.
    let n_f = f64::from(u16::try_from(n).unwrap_or(720));
    (0..n)
        .map(|i| {
            let bearing = 360.0 * f64::from(u16::try_from(i).unwrap_or(0)) / n_f;
            let (lat, lon) = destination(center.lat, center.lon, bearing, radius_m);
            Coord {
                lat,
                lon,
                hae: center.hae,
            }
        })
        .collect()
}

/// A circle's radius, if `ellipse` is one.
fn circular_radius(e: &Ellipse) -> Option<f64> {
    let (major, minor) = (e.major?, e.minor?);
    let ok = major.is_finite() && major > 0.0 && (major - minor).abs() <= 1e-6 * major;
    ok.then_some(major)
}

/// Radius in metres of the circle drawn by `ev`, if it is a circle (the
/// outermost one for range rings).
pub fn circle_radius(ev: &Event) -> Option<f64> {
    let shape = ev.detail.get::<Shape>()?;
    shape
        .ellipses
        .iter()
        .filter_map(circular_radius)
        .reduce(f64::max)
}

fn vertices(ev: &Event) -> Vec<Coord> {
    ev.detail
        .get_all::<Link>()
        .into_iter()
        .filter(|l| l.relation.as_deref() != Some("p-p"))
        .filter_map(|l| l.point)
        .map(|p| Coord {
            lat: p.lat,
            lon: p.lon,
            hae: p.hae,
        })
        .collect()
}

/// Determines the geometry of `ev` using the rules in the module docs.
pub fn geometry_of(ev: &Event, opts: &GeometryOptions) -> Geometry {
    let centre = Coord {
        lat: ev.point.lat,
        lon: ev.point.lon,
        hae: ev.point.hae,
    };
    let t = ev.cot_type.as_str();

    if let Some(radius) = circle_radius(ev) {
        return Geometry::Polygon(circle(centre, radius, opts.circle_segments));
    }

    if t == "u-rb-a" {
        let range = ev.detail.get::<Range>().map(|r| r.value);
        let bearing = ev.detail.get::<Bearing>().map(|b| b.value);
        if let (Some(range), Some(bearing)) = (range, bearing) {
            if range.is_finite() && range > 0.0 && bearing.is_finite() {
                let (lat, lon) = destination(centre.lat, centre.lon, bearing, range);
                return Geometry::Line(vec![
                    centre,
                    Coord {
                        lat,
                        lon,
                        hae: centre.hae,
                    },
                ]);
            }
        }
        return Geometry::Point(centre);
    }

    let mut vs = vertices(ev);
    match t {
        "b-m-r" if vs.len() >= 2 => Geometry::Line(vs),
        "u-d-r" | "u-d-f" => {
            let closed = vs.len() >= 4 && vs.first() == vs.last();
            let polygon = t == "u-d-r" || closed || ev.detail.get::<FillColor>().is_some();
            if polygon && vs.len() >= 3 {
                if closed {
                    vs.pop();
                }
                Geometry::Polygon(vs)
            } else if vs.len() >= 2 {
                Geometry::Line(vs)
            } else {
                Geometry::Point(centre)
            }
        }
        _ => Geometry::Point(centre),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cottak_core::detail::LinkAttr;
    use cottak_core::messages::draw::{self, Style, Waypoint};
    use cottak_core::Point;

    fn p(lat: f64, lon: f64) -> Point {
        Point::new(lat, lon).unwrap()
    }

    #[test]
    fn destination_and_distance_agree() {
        let (lat, lon) = destination(10.0, 20.0, 90.0, 1000.0);
        assert!((distance_m((10.0, 20.0), (lat, lon)) - 1000.0).abs() < 0.01);
        // Due east at the equator: 1 km is 1000 / 111_194.93 m-per-degree = 0.0089932 degrees.
        let (lat, lon) = destination(0.0, 0.0, 90.0, 1000.0);
        assert!(
            lat.abs() < 1e-9 && (lon - 0.008_993_2).abs() < 1e-6,
            "{lat} {lon}"
        );
        // Crossing the antimeridian stays within range.
        let (_, lon) = destination(0.0, 179.999, 90.0, 5000.0);
        assert!((-180.0..=180.0).contains(&lon) && lon < 0.0, "{lon}");
    }

    #[test]
    fn circles_are_polygons_at_the_right_radius() {
        let ev = draw::circle("c", "C", p(-27.0, 153.0), 500.0, Style::default()).unwrap();
        let Geometry::Polygon(ring) = geometry_of(&ev, &GeometryOptions::default()) else {
            panic!("circle must be a polygon")
        };
        assert_eq!(ring.len(), 64);
        for c in &ring {
            assert!((distance_m((-27.0, 153.0), (c.lat, c.lon)) - 500.0).abs() < 0.5);
        }
    }

    #[test]
    fn polygon_line_route_and_point() {
        let v = [p(0.0, 0.0), p(0.0, 1.0), p(1.0, 1.0)];
        let poly = draw::polygon("a", "A", &v, Style::default()).unwrap();
        assert!(
            matches!(geometry_of(&poly, &GeometryOptions::default()), Geometry::Polygon(r) if r.len() == 3)
        );

        let line = draw::polyline("b", "B", &v, Style::default()).unwrap();
        assert!(
            matches!(geometry_of(&line, &GeometryOptions::default()), Geometry::Line(l) if l.len() == 3)
        );

        let wps = [Waypoint::new("A", v[0]), Waypoint::new("B", v[1])];
        let route = draw::route("r", "R", &wps, LinkAttr::default()).unwrap();
        assert!(
            matches!(geometry_of(&route, &GeometryOptions::default()), Geometry::Line(l) if l.len() == 2)
        );

        let marker = cottak_core::Event::new("m", "a-f-G".parse().unwrap(), v[0]);
        assert!(matches!(
            geometry_of(&marker, &GeometryOptions::default()),
            Geometry::Point(_)
        ));
    }

    #[test]
    fn range_and_bearing_line_has_the_right_length() {
        let ev = draw::range_bearing_line("rb", "RB", p(0.0, 0.0), 2000.0, 90.0, Style::default())
            .unwrap();
        let Geometry::Line(l) = geometry_of(&ev, &GeometryOptions::default()) else {
            panic!()
        };
        assert!((distance_m((l[0].lat, l[0].lon), (l[1].lat, l[1].lon)) - 2000.0).abs() < 0.01);
    }

    #[test]
    fn explicitly_closed_rings_are_not_duplicated() {
        let v = [p(0.0, 0.0), p(0.0, 1.0), p(1.0, 1.0), p(0.0, 0.0)];
        let mut ev = draw::polyline("x", "X", &v, Style::default()).unwrap();
        // A repeated first vertex marks a closed shape even without a fill colour.
        let g = geometry_of(&ev, &GeometryOptions::default());
        assert!(matches!(&g, Geometry::Polygon(r) if r.len() == 3), "{g:?}");
        ev.detail.set(FillColor::from_rgba(1, 2, 3, 4));
        assert!(
            matches!(geometry_of(&ev, &GeometryOptions::default()), Geometry::Polygon(r) if r.len() == 3)
        );
    }

    #[test]
    fn ellipses_stay_points() {
        let mut ev = draw::circle("c", "C", p(1.0, 1.0), 100.0, Style::default()).unwrap();
        let mut shape = ev.detail.get::<Shape>().unwrap();
        shape.ellipses[0].minor = Some(50.0);
        ev.detail.set(shape);
        assert!(matches!(
            geometry_of(&ev, &GeometryOptions::default()),
            Geometry::Point(_)
        ));
    }
}
