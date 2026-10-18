# cottak-convert

Convert CoT to and from GeoJSON, KML/KMZ and MIL-STD-2525 symbol codes.

Part of [cottak](https://github.com/Astute-Systems/cot-tak), the Rust toolkit for Cursor on Target and TAK.

## Features

- GeoJSON (RFC 7946, simplestyle) import and export
- KML 2.2 import and export; import skips what it cannot represent and reports it
- KMZ read and write (feature `kmz`), zip-bomb safe
- MIL-STD-2525B/C symbol codes; type descriptions from a `CoTtypes.xml` you supply

## Example

```rust
use cottak_convert::geojson::{events_to_collection, GeoJsonOptions};
use cottak_core::{Event, Point};

let event = Event::new("ALPHA-1", "a-f-G-U-C".parse()?, Point::new(-27.47, 153.02)?);
let doc = events_to_collection([&event], &GeoJsonOptions::default());
assert_eq!(doc["features"][0]["properties"]["sidc"], "SFGPUC---------");
# Ok::<(), cottak_core::Error>(())
```

## Documentation

- [Guide](https://astute-systems.github.io/cot-tak/guide/convert/) (every Rust snippet in it is compiled in CI)
- [API reference](https://astute-systems.github.io/cot-tak/api/cottak_convert/)
- [Repository](https://github.com/Astute-Systems/cot-tak) and the [`cottak`](https://crates.io/crates/cottak) facade crate

## Licence

Licensed under either of Apache-2.0 or MIT at your option.
