//! Format conversion and symbology for Cursor on Target events.
//!
//! | Module | What it does |
//! |---|---|
//! | [`geojson`] | Events ⇄ GeoJSON (RFC 7946) with simplestyle properties |
//! | [`kml`] | Events ⇄ KML 2.2 (import reports what it skips); KMZ with the `kmz` feature |
//! | [`sidc`] | CoT atom types ⇄ MIL-STD-2525B/C 15-character symbol codes |
//! | [`catalog`] | Human-readable type descriptions from a `CoTtypes.xml` file you supply |
//! | [`geometry`] | The shared rules for turning a CoT event into a point, line or polygon |
//! | [`color`] | ARGB, CSS hex and KML colour conversions |
//!
//! ```
//! use cottak_convert::geojson::{events_to_collection, GeoJsonOptions};
//! use cottak_core::{Event, Point};
//!
//! let event = Event::new("ALPHA-1", "a-f-G-U-C".parse()?, Point::new(-27.47, 153.02)?);
//! let doc = events_to_collection([&event], &GeoJsonOptions::default());
//! assert_eq!(doc["features"][0]["geometry"]["coordinates"][0], 153.02);
//! assert_eq!(doc["features"][0]["properties"]["sidc"], "SFGPUC---------");
//! # Ok::<(), cottak_core::Error>(())
//! ```

pub mod catalog;
pub mod color;
pub mod error;
pub mod geojson;
pub mod geometry;
pub mod kml;
mod kml_import;
#[cfg(feature = "kmz")]
mod kmz;
pub mod sidc;

pub use error::{Error, Result};
pub use sidc::Sidc;
