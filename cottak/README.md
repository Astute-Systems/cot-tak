# CoT TAK Transmission

This project sends Cursor on Target (CoT) messages to a specified UDP port.

## Prerequisites

- Rust programming language: [Install Rust](https://www.rust-lang.org/tools/install)
- `uuid` crate: Add `uuid = "1.0"` to your `Cargo.toml`
- `ctrlc` crate: Add `ctrlc = "3.2"` to your `Cargo.toml`

## Config file

The config file default location is ```/etc/cot/cot/config.toml```

```.toml
[general]
uuid = "d0775617-6907-4538-be0e-12de8755f86f"
stale = 120
filestore = "/tmp"
http_server = true
http_address = "localhost"
http_port = 8080
callsign = "Red Five"

[chat]
address = "224.10.10.1"
port = 17012

[udp]
address = "239.2.3.1"
port = 6969

[tcp]
address = "192.168.1.100"
port = 7979

[ssl]
address = "192.168.1.101"
port = 443

```

## Publish CoT message (UDP)

``` .rust
use cottak::config::Config;
use cottak::cot::Cot;
use cottak::cot_base::{CursorOnTarget, Point};
use cottak::cot_types::{CoTType, CotClassification};
use cottak::proto_sender::ProtoSender;

// Brisbane City Hall
const LONGITUDE: f64 = 153.02351661489064;
const LATITUDE: f64 = -27.46891737509902;

fn main() {
    // Read the defaults from the configuration
    let config = Config::new();

    // Setup classification of the CoT message
    let cot_type =
        cottak::cot_types::lookup_cot_type(CoTType::GndBuilding, CotClassification::Friend);

    // Brisbane City Hall
    let building = Cot::new(
        CursorOnTarget::new(
            "CITY_#89436".to_string(),
            None,
            Some(cot_type.to_string()),
            "h-e".to_string(),
            Some("UNCLASSIFIED".to_string()),
            Point {
                latitude: LATITUDE,
                longitude: LONGITUDE,
                hae: CursorOnTarget::HAE_NONE,
                ce: CursorOnTarget::CE_NONE,
                le: CursorOnTarget::LE_NONE,
            },
        ),
        None,
        "Brisbane City Hall".to_string(),
        false,
        None,
        None,
        Some(0.0),
        None,
        None,
        None,
        None,
        None,
        None,
    );

    let mut sender = ProtoSender::new(&config.udp.address as &str, config.udp.port)
        .expect("Couldn't setup multicast");

    sender.send(&building);
}
```
