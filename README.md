# cottak

[![CI](https://github.com/Astute-Systems/cot-tak/actions/workflows/ci.yml/badge.svg)](https://github.com/Astute-Systems/cot-tak/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](#license)

An open Rust toolkit for **Cursor on Target (CoT)** and **TAK** (ATAK, WinTAK, iTAK, TAK Server).

> **Status: 0.2 development.** The crate has been rebuilt as a multi-crate workspace; the 0.1 API
> is gone.

## Crates

| Crate | Status | Purpose |
|---|---|---|
| `cottak-core` | usable | Typed `Event`, `Point`, `CotType`, `Detail`; hardened XML parse/write; lossless round-trip; builders for GeoChat, alerts, shapes, routes, MEDEVAC, sensors, ping/delete |
| `cottak` | usable | Facade re-exporting the family by feature |
| `cottak-proto` | usable | TAK Protocol v1 (protobuf): event mapping, streaming and mesh framing, XML negotiation, optional `tokio_util` codec. No `protoc` needed |
| `cottak-net` | usable | Async TAK Server client over TCP/mTLS (rustls): protobuf upgrade, ping, dead-peer detection, jittered reconnect; mesh UDP multicast |
| `cottak-enroll` | usable | `tak://` onboarding URLs, CSR + `signClient/v2` enrollment (ECDSA generated, or bring your own key for RSA), PEM/PKCS#12 identities (ATAK-compatible export), server pinning |
| `cottak-marti` | usable | Marti REST client (mTLS): missions and content, subscriptions, change log, file upload/download, contacts, groups. Endpoints verified against the TAK Server source |
| `cottak-server` | usable | Embeddable TAK Server for tests and small isolated networks: TCP/mTLS listeners, XML and protobuf negotiation, ping/pong, broadcast, callsign and mission routing, latest-position replay, and a Marti HTTPS mock (missions, files, contacts) for testing `cottak-marti`. No accounts, groups, persistence or federation |
| `cottak-convert` | usable | Events ⇄ GeoJSON, events ⇄ KML/KMZ (import reports the placemarks it skips; KMZ needs the `kmz` feature), CoT type ⇄ MIL-STD-2525B/C symbol codes, type descriptions from a user-supplied `CoTtypes.xml` |

## Example

```rust
use cottak::prelude::*;
use cottak::core::detail::{Contact, Track};

let event = Event::new("ALPHA-1", "a-f-G-U-C".parse()?, Point::new(-27.4689, 153.0235)?)
    .with(Contact::new("ALPHA-1"))
    .with(Track::new(90.0, 1.5));

let xml = event.to_xml_string()?;
let parsed: Event = xml.parse()?;
assert_eq!(parsed, event);
# Ok::<(), cottak::core::Error>(())
```

### Messages

```rust
use cottak::core::messages::{chat::GeoChat, alert::{self, EmergencyKind}, draw};
use cottak::core::Point;

let chat = GeoChat::direct("U1", "ALPHA", "U2", "BRAVO", "On my way").to_event()?;
let sos = alert::raise("U1", "ALPHA", EmergencyKind::Alert911, Point::new(-27.47, 153.02)?)?;
let ring = draw::circle("c1", "Exclusion", Point::new(-27.47, 153.02)?, 250.0, draw::Style::default())?;
# Ok::<(), cottak::core::Error>(())
```

### TAK Server client

```rust,no_run
use cottak::net::{ClientConfig, ClientEvent, TakClient, TlsConfig};

let tls = TlsConfig::from_files("ca.pem", Some(("client.pem".as_ref(), "client.key".as_ref())))?;
let (handle, mut events) = TakClient::start(ClientConfig::new("tak.example.com:8089", "ALPHA-1").with_tls(tls))?;
handle.send(event).await?;
while let Some(ClientEvent::Event(ev)) = events.recv().await { println!("{}", ev.uid); }
```

### Enrollment

```rust,no_run
use cottak::enroll::{Credentials, EnrollConfig, Enroller};
use cottak::net::TlsConfig;

let trust = TlsConfig::from_files("server-ca.pem", None)?;
let enroller = Enroller::new(EnrollConfig::new("tak.example.com", trust)).await?;
let identity = enroller.enroll(&Credentials::new("alice", "password"), "ANDROID-1234").await?;
let tls = identity.tls_config()?; // use with `ClientConfig::with_tls`
```

For a self-signed server, `cottak::net::probe_server_certificate` returns its fingerprint to
confirm with the operator, then pin it with `TlsConfig::with_pinned_sha256`.

### Marti (missions and files)

```rust,no_run
use cottak::marti::{MartiClient, MartiConfig, MissionContent, NewMission};

let marti = MartiClient::new(MartiConfig::new("tak.example.com", tls)).await?;
marti.create_mission("RECON-1", "ALPHA-1", &NewMission::default()).await?;
let file = marti.upload_file("plan.zip", &bytes, &Default::default()).await?;
let content = MissionContent { hashes: vec![file["Hash"].as_str().unwrap().into()], uids: vec![] };
marti.add_content("RECON-1", &content, "ALPHA-1").await?;
```

### Mock server (tests, simulators)

```rust,no_run
use cottak::server::{Server, ServerConfig};

let server = Server::start(ServerConfig::new().listen_tcp("127.0.0.1:0")).await?;
let addr = server.local_addrs()[0]; // point a `TakClient` here
let mut seen = server.subscribe();  // observe every routed event
// `ServerConfig::listen_marti(addr, Some(tls))` adds the HTTPS API for `MartiClient`.
```

### Conversion

```rust
use cottak::convert::{geojson, kml, Sidc};

let doc = geojson::events_to_collection(&events, &Default::default());  // RFC 7946 FeatureCollection
let events = geojson::events_from_geojson(&doc, &Default::default())?;  // and back
let kml_text = kml::events_to_kml(&events, &Default::default());
let imported = kml::events_from_kml(&kml_text, &Default::default())?;      // lenient: see `imported.skipped`
let sidc = Sidc::from_cot(&"a-f-G-U-C".parse()?);                       // Some("SFGPUC---------")
```

### TAK Protocol v1

```rust
use bytes::BytesMut;
use cottak::proto::{frame, TakFrame};

let mut wire = BytesMut::new();
TakFrame::from_event(event).encode_stream(&mut wire)?; // 0xbf <len> <protobuf>
let decoded = TakFrame::decode_stream(&mut wire, frame::DEFAULT_MAX_FRAME_LEN)?;
```

## Design goals

* **Lossless**: unknown `<detail>` children and unmodelled attributes survive a parse/serialise round trip.
* **Safe by default**: no `unsafe`; DOCTYPE and custom entities rejected; bounded XML size and depth; no panics on untrusted input (property tested).
* **No OpenSSL**: TLS is rustls only (enforced by `cargo deny`).
* **Extensible**: implement `DetailItem` to add typed views of your own detail elements.

MSRV: Rust 1.88.

## License

Licensed under either of [Apache License 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.

**Exception: the TAK Protocol schema.** `cottak-proto` bundles `.proto` files derived from
TAK Server, which are licensed under the GPL-3.0 (see
[`crates/cottak-proto/proto/NOTICE.md`](crates/cottak-proto/proto/NOTICE.md)). Anything that links
`cottak-proto` (directly, via `cottak-net`, `cottak-enroll`, `cottak-marti`, `cottak-server`, or the
default `proto` feature of `cottak`) forms a combined work and must comply with the GPL-3.0 for it.
`cottak-core` and `cottak-convert` do not depend on `cottak-proto` and stay MIT OR Apache-2.0 only.
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this work shall be dual licensed as above, without any additional terms or conditions.
