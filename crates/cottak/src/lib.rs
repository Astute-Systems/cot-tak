//! Cursor on Target (CoT) and TAK Server toolkit for Rust.
//!
//! This is the facade crate: it re-exports the building blocks of the
//! `cottak` family behind cargo features.
//!
//! | Module | Crate | Purpose |
//! |---|---|---|
//! | [`core`] | `cottak-core` | Typed events, details, message builders, XML round-tripping |
//! | `proto` (feature, default) | `cottak-proto` | TAK Protocol v1 protobuf mapping, framing, negotiation |
//! | `net` (feature) | `cottak-net` | Async TAK Server client (TCP/mTLS), keepalive, reconnect, mesh UDP |
//! | `enroll` (feature) | `cottak-enroll` | `tak://` onboarding and certificate enrollment |
//! | `marti` (feature) | `cottak-marti` | Marti REST client: missions, file sync, contacts, groups |
//! | `server` (feature) | `cottak-server` | Embeddable TAK Server: mock for tests, small broker |
//! | `convert` (feature) | `cottak-convert` | GeoJSON and KML conversion, MIL-STD-2525 symbol codes |
//!
//! ```
//! use cottak::prelude::*;
//!
//! let event = Event::new("ALPHA-1", "a-f-G-U-C".parse()?, Point::new(1.0, 2.0)?);
//! assert!(event.to_xml_string()?.starts_with("<event"));
//! # Ok::<(), cottak::core::Error>(())
//! ```

pub use cottak_core as core;

// The guide's code blocks are compiled as doctests; see the module docs.
#[cfg(doctest)]
mod guide_tests;

#[cfg(feature = "proto")]
pub use cottak_proto as proto;

#[cfg(feature = "net")]
pub use cottak_net as net;

#[cfg(feature = "enroll")]
pub use cottak_enroll as enroll;

#[cfg(feature = "marti")]
pub use cottak_marti as marti;

#[cfg(feature = "server")]
pub use cottak_server as server;

#[cfg(feature = "convert")]
pub use cottak_convert as convert;

/// Commonly used types.
pub mod prelude {
    pub use cottak_core::detail::{Detail, DetailItem};
    pub use cottak_core::{CotType, Event, Point};
}
