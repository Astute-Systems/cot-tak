//! High-level builders for well-known TAK messages.
//!
//! Each function returns an ordinary [`Event`], so the result can be
//! inspected, modified, serialised to XML or (with `cottak-proto`) encoded for
//! TAK Protocol v1.
//!
//! The wire shapes follow what ATAK-CIV and WinTAK emit. The structure is
//! covered by unit tests, but interoperability with every TAK client version
//! should be verified against your own deployment.
//!
//! | Module | Messages |
//! |---|---|
//! | [`chat`] | GeoChat direct and all-room messages |
//! | [`alert`] | 911, ring-the-bell, in-contact, cancel |
//! | [`draw`] | circles, polygons, geofences, range rings, bullseye, range and bearing, routes |
//! | [`medevac`] | 9-line MEDEVAC / CASEVAC request |
//! | [`sensor`] | sensor field-of-view with video link |
//! | [`control`] | ping, pong, delete |

pub mod alert;
pub mod chat;
pub mod control;
pub mod draw;
pub mod medevac;
pub mod sensor;

use crate::cot_type::CotType;
use crate::error::Result;
use crate::event::Event;
use crate::point::Point;
use time::Duration;

/// Common `how` values used by the builders.
pub(crate) use crate::event::how;

/// Creates an event with the given type string, `how` and time-to-live.
pub(crate) fn base(
    uid: impl Into<String>,
    cot_type: &str,
    how: &str,
    point: Point,
    ttl: Duration,
) -> Result<Event> {
    Ok(Event::new(uid, CotType::new(cot_type)?, point)
        .with_how(how)
        .with_stale_in(ttl))
}

/// A point at 0,0 with unknown height and errors, used by events whose
/// position is irrelevant (chat, ping).
pub(crate) fn null_point() -> Result<Point> {
    Point::new(0.0, 0.0)
}
