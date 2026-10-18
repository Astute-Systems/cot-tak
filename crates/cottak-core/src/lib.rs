//! Typed Cursor on Target (CoT) events for TAK, with lossless XML round-tripping.
//!
//! This crate is the dependency-light foundation of the `cottak` family. It
//! knows nothing about networks or TAK Server; it only models, parses and
//! writes CoT.
//!
//! * [`Event`] – the `<event>` with [`Point`], [`CotType`] and [`Detail`].
//! * [`detail`] – typed views of `<detail>` children; unknown elements are
//!   preserved losslessly.
//! * [`messages`] – builders for GeoChat, alerts, shapes, routes, MEDEVAC, sensors and
//!   control messages.
//! * [`xml`] – the hardened XML tree used underneath (no DTD, bounded size and
//!   depth).
//!
//! ```
//! use cottak_core::{Event, detail::Contact};
//!
//! let xml = r#"<event version="2.0" uid="U1" type="a-f-G-U-C" how="m-g"
//!     time="2025-01-01T00:00:00Z" start="2025-01-01T00:00:00Z" stale="2025-01-01T00:05:00Z">
//!     <point lat="1" lon="2" hae="0" ce="10" le="10"/>
//!     <detail><contact callsign="ALPHA"/></detail></event>"#;
//!
//! let event: Event = xml.parse()?;
//! assert_eq!(event.detail.get::<Contact>().unwrap().callsign.as_deref(), Some("ALPHA"));
//! # Ok::<(), cottak_core::Error>(())
//! ```
//!
//! # Features
//!
//! * `serde` – derive `Serialize`/`Deserialize` for the model.

pub mod cot_type;
pub mod detail;
pub mod error;
pub mod event;
pub mod messages;
pub mod point;
pub mod timestamp;
pub mod xml;

pub use cot_type::{Affiliation, CotType, Dimension, Kind};
pub use detail::{Detail, DetailItem};
pub use error::{Error, Result};
pub use event::Event;
pub use point::Point;
