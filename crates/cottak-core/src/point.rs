//! The `<point>` element: location and its error estimates.

use crate::error::{Error, Result};
use crate::xml::{Attrs, XmlElement};

/// Wire value meaning "unknown" for `hae`, `ce` and `le`.
pub const UNKNOWN: f64 = 9_999_999.0;

/// A WGS-84 position with optional height and error estimates.
///
/// `hae`, `ce` and `le` are `None` when unknown; they are encoded as the
/// CoT sentinel `9999999.0` on the wire.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Point {
    /// Latitude in decimal degrees, `-90..=90`.
    pub lat: f64,
    /// Longitude in decimal degrees, `-180..=180`.
    pub lon: f64,
    /// Height above the WGS-84 ellipsoid in metres.
    pub hae: Option<f64>,
    /// Circular error (horizontal) in metres.
    pub ce: Option<f64>,
    /// Linear error (vertical) in metres.
    pub le: Option<f64>,
}

impl Point {
    /// Creates a validated point with unknown height and errors.
    pub fn new(lat: f64, lon: f64) -> Result<Self> {
        let p = Self {
            lat,
            lon,
            hae: None,
            ce: None,
            le: None,
        };
        p.validate()?;
        Ok(p)
    }

    /// Sets the height above ellipsoid (metres).
    #[must_use]
    pub fn with_hae(mut self, hae: f64) -> Self {
        self.hae = Some(hae);
        self
    }

    /// Sets the circular and linear error estimates (metres).
    #[must_use]
    pub fn with_errors(mut self, ce: f64, le: f64) -> Self {
        self.ce = Some(ce);
        self.le = Some(le);
        self
    }

    /// Checks that all values are finite and within range.
    pub fn validate(&self) -> Result<()> {
        check("lat", self.lat, -90.0, 90.0)?;
        check("lon", self.lon, -180.0, 180.0)?;
        // The wire format reserves 9999999 (and above) to mean "unknown", so real
        // values must be strictly inside that range or they would not survive a round trip.
        for (name, v) in [("hae", self.hae), ("ce", self.ce), ("le", self.le)] {
            if let Some(v) = v {
                if !(v.is_finite() && v.abs() < UNKNOWN) {
                    return Err(Error::OutOfRange {
                        field: name,
                        value: v,
                    });
                }
            }
        }
        Ok(())
    }

    pub(crate) fn from_xml(el: &XmlElement) -> Result<Self> {
        let mut a = Attrs::new(el);
        let req = |a: &mut Attrs, name: &'static str| -> Result<f64> {
            let s = a.take(name).ok_or(Error::Missing {
                what: "attribute",
                name,
                element: "point",
            })?;
            s.trim()
                .parse::<f64>()
                .map_err(|_| Error::invalid("point", name, &s, "not a number"))
        };
        let opt = |a: &mut Attrs, name: &'static str| -> Result<Option<f64>> {
            let Some(s) = a.take(name) else {
                return Ok(None);
            };
            let v = s
                .trim()
                .parse::<f64>()
                .map_err(|_| Error::invalid("point", name, &s, "not a number"))?;
            Ok(if v >= UNKNOWN { None } else { Some(v) })
        };
        let p = Self {
            lat: req(&mut a, "lat")?,
            lon: req(&mut a, "lon")?,
            hae: opt(&mut a, "hae")?,
            ce: opt(&mut a, "ce")?,
            le: opt(&mut a, "le")?,
        };
        p.validate()?;
        Ok(p)
    }

    pub(crate) fn to_xml(self) -> XmlElement {
        let f = |v: Option<f64>| v.unwrap_or(UNKNOWN).to_string();
        XmlElement::new("point")
            .with_attr("lat", self.lat.to_string())
            .with_attr("lon", self.lon.to_string())
            .with_attr("hae", f(self.hae))
            .with_attr("ce", f(self.ce))
            .with_attr("le", f(self.le))
    }
}

fn check(field: &'static str, v: f64, lo: f64, hi: f64) -> Result<()> {
    if v.is_finite() && (lo..=hi).contains(&v) {
        Ok(())
    } else {
        Err(Error::OutOfRange { field, value: v })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_ranges() {
        assert!(Point::new(91.0, 0.0).is_err());
        assert!(Point::new(0.0, -181.0).is_err());
        assert!(Point::new(f64::NAN, 0.0).is_err());
        assert!(Point::new(-27.4689, 153.0235).is_ok());
    }

    #[test]
    fn sentinel_maps_to_none() {
        let el =
            XmlElement::parse(r#"<point lat="1" lon="2" hae="9999999.0" ce="5" le="9999999"/>"#)
                .unwrap();
        let p = Point::from_xml(&el).unwrap();
        assert_eq!(p.hae, None);
        assert_eq!(p.ce, Some(5.0));
        assert_eq!(p.le, None);
        assert_eq!(Point::from_xml(&p.to_xml()).unwrap(), p);
    }

    #[test]
    fn requires_lat_lon() {
        let el = XmlElement::new("point").with_attr("lat", "1");
        assert!(matches!(
            Point::from_xml(&el),
            Err(Error::Missing { name: "lon", .. })
        ));
    }

    #[test]
    fn values_at_or_above_the_sentinel_are_invalid() {
        // Found by fuzzing: such a height would be read back as "unknown".
        for v in [UNKNOWN, UNKNOWN + 1.0, 1.2e129, -UNKNOWN, f64::INFINITY] {
            assert!(
                Point::new(0.0, 0.0)
                    .unwrap()
                    .with_hae(v)
                    .validate()
                    .is_err(),
                "{v}"
            );
            assert!(
                Point::new(0.0, 0.0)
                    .unwrap()
                    .with_errors(v, 1.0)
                    .validate()
                    .is_err(),
                "{v}"
            );
        }
        assert!(Point::new(0.0, 0.0)
            .unwrap()
            .with_hae(9_999_998.0)
            .validate()
            .is_ok());
    }
}
