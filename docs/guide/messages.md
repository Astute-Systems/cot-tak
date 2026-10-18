# Message builders

`cottak::core::messages` builds complete, ready-to-send events for the common TAK interactions.
They follow ATAK's output; they return plain `Event`s, so you can inspect or extend them before
sending.

!!! note "Validation status"
    The builders are written from ATAK's behaviour and the original 0.1 implementation and are
    covered by unit and property tests. They have not all been validated against live ATAK/WinTAK
    builds. See [Compatibility and limits](../concepts/compatibility.md).

## GeoChat

```rust
use cottak::core::messages::chat::GeoChat;

// A direct message...
let dm = GeoChat::direct("U1", "ALPHA", "U2", "BRAVO", "On my way");
let event = dm.to_event()?;
assert!(event.uid.starts_with("GeoChat.U1.U2."));
assert!(!dm.is_broadcast());

// ...and one to the "All Chat Rooms" group.
let all = GeoChat::to_all("U1", "ALPHA", "Anyone copy?");
assert!(all.is_broadcast());

// Parsing is the inverse. `None` means "not a GeoChat message".
let back = GeoChat::from_event(&event).unwrap();
assert_eq!(back.text, "On my way");
assert_eq!(back.sender_callsign, "ALPHA");
# Ok::<(), cottak::core::Error>(())
```

Send the resulting event with a [client](client.md); the server routes direct messages by
`<marti><dest callsign=.../>`.

## Alerts

`alert::raise` and `alert::cancel` build 911, ring-the-bell, troops-in-contact and geofence
alerts:

```rust
use cottak::core::messages::alert::{self, EmergencyKind};
use cottak::core::Point;

let here = Point::new(-27.47, 153.02)?;
let sos = alert::raise("U1", "ALPHA", EmergencyKind::Alert911, here)?;
let clear = alert::cancel("U1", "ALPHA", EmergencyKind::Alert911, here)?;
assert_eq!(sos.cot_type.as_str(), "b-a-o-tbl");
assert_eq!(clear.cot_type.as_str(), "b-a-o-can");
# Ok::<(), cottak::core::Error>(())
```

## Control messages

```rust
use cottak::core::messages::control;

let ping = control::ping("MY-UID")?;          // t-x-c-t  (keepalive)
let pong = control::pong("server")?;          // t-x-c-t-r
assert!(control::is_ping(&ping) && control::is_pong(&pong));

// Ask receivers to remove an entity from their maps.
let delete = control::delete("TRACK-1", "a-h-G")?;
assert_eq!(delete.cot_type.as_str(), control::DELETE_TYPE);
# Ok::<(), cottak::core::Error>(())
```

The [client](client.md) sends pings for you.

## Drawings and shapes

`messages::draw` covers circles, range rings, polygons, geofences, polylines, bullseyes,
range-and-bearing lines and routes. They share a `Style` (stroke, fill, weight; the default is a
translucent red).

```rust
use cottak::core::messages::draw::{self, Style};
use cottak::core::Point;

let centre = Point::new(-27.47, 153.02)?;

let exclusion = draw::circle("c1", "Exclusion zone", centre, 250.0, Style::default())?;
assert_eq!(exclusion.cot_type.as_str(), "u-d-c-c");

let area = draw::polygon(
    "p1",
    "Search area",
    &[Point::new(0.0, 0.0)?, Point::new(0.0, 1.0)?, Point::new(1.0, 1.0)?],
    Style::default(),
)?;
let route_line = draw::polyline(
    "l1",
    "Route",
    &[Point::new(0.0, 0.0)?, Point::new(1.0, 1.0)?],
    Style::default(),
)?;
assert_eq!(area.cot_type.as_str(), "u-d-f");
assert_eq!(route_line.cot_type.as_str(), "u-d-f");

// Invalid geometry is an error, not a malformed event.
assert!(draw::circle("c2", "x", centre, -1.0, Style::default()).is_err());
# Ok::<(), cottak::core::Error>(())
```

Routes use `draw::route` with a list of `Waypoint`s; bullseyes, range rings and geofences are
analogous. See the [`messages::draw` API](api:cottak_core/messages/draw/index.html).

## MEDEVAC and sensors

`messages::medevac::request` builds a 9-line MEDEVAC request (`b-r-f-h-c`) from a `Medevac`
detail, and `messages::sensor::field_of_view` builds a sensor cone with an optional video link.
Both take richer detail structs; see the API docs for the fields.

## Writing your own

Nothing stops you from composing events yourself. A builder is just `Event::new` plus typed
details and the right `how`/stale values:

```rust
use cottak::core::detail::{Contact, Remarks};
use cottak::core::event::how;
use cottak::core::{Event, Point};

let marker = Event::new(Event::random_uid(), "b-m-p-s-m".parse()?, Point::new(1.0, 2.0)?)
    .with_how(how::HUMAN_GIS)
    .with(Contact::new("Rally point"))
    .with(Remarks::new("Meet here at 0600"));
marker.validate()?;
# Ok::<(), cottak::core::Error>(())
```
