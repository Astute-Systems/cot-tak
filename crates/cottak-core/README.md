# cottak-core

Typed Cursor on Target (CoT) / TAK events with lossless, hardened XML.

Part of [cottak](https://github.com/Astute-Systems/cot-tak), the Rust toolkit for Cursor on Target and TAK.

## Features

- Validated `Event`, `Point` and `CotType`; typed details (contact, track, group, link, shapes, chat, MEDEVAC, sensors, ...)
- **Lossless**: unknown `<detail>` elements and attributes survive a parse and write round trip
- **Hardened XML**: DTDs and custom entities rejected; size, depth and attribute limits
- Message builders: GeoChat, alerts, drawings and routes, MEDEVAC, sensors, ping/pong/delete
- No I/O and no async runtime; optional `serde`

## Example

```rust
use cottak_core::detail::{Contact, Track};
use cottak_core::{Event, Point};

let event = Event::new("ALPHA-1", "a-f-G-U-C".parse()?, Point::new(-27.4689, 153.0235)?)
    .with(Contact::new("ALPHA-1"))
    .with(Track::new(90.0, 1.5));

let parsed: Event = event.to_xml_string()?.parse()?;
assert_eq!(parsed, event);
# Ok::<(), cottak_core::Error>(())
```

## Documentation

- [Guide](https://astute-systems.github.io/cot-tak/guide/events/) (every Rust snippet in it is compiled in CI)
- [API reference](https://astute-systems.github.io/cot-tak/api/cottak_core/)
- [Repository](https://github.com/Astute-Systems/cot-tak) and the [`cottak`](https://crates.io/crates/cottak) facade crate

## Licence

Licensed under either of Apache-2.0 or MIT at your option.
