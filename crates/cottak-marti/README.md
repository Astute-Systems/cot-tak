# cottak-marti

Typed client for TAK Server's Marti REST API: missions, file sync, contacts and groups.

Part of [cottak](https://github.com/Astute-Systems/cot-tak), the Rust toolkit for Cursor on Target and TAK.

## Features

- Missions: create, list, fetch, delete, add/remove content, change log, subscriptions, latest CoT per UID
- File sync: upload, data-package upload, download by hash
- Contacts, client endpoints, groups, server version
- Mutual TLS or HTTP Basic; typed errors; size limits; tolerant models that keep unknown fields

## Example

```rust,no_run
use cottak_marti::{MartiClient, MartiConfig, NewMission};
use cottak_net::TlsConfig;

# async fn run() -> Result<(), Box<dyn std::error::Error>> {
let tls = TlsConfig::from_files("ca.pem", Some(("client.pem".as_ref(), "client.key".as_ref())))?;
let marti = MartiClient::new(MartiConfig::new("tak.example.com", tls)).await?;
marti.create_mission("RECON-1", "ALPHA-1", &NewMission::default()).await?;
# Ok(()) }
```

## Documentation

- [Guide](https://astute-systems.github.io/cot-tak/guide/marti/) (every Rust snippet in it is compiled in CI)
- [API reference](https://astute-systems.github.io/cot-tak/api/cottak_marti/)
- [Repository](https://github.com/Astute-Systems/cot-tak) and the [`cottak`](https://crates.io/crates/cottak) facade crate

## Licence

Licensed under either of Apache-2.0 or MIT at your option. **Note:** this crate depends on `cottak-proto`, whose bundled TAK Protocol schema is GPL-3.0, so programs that link it are a combined work; see the [licensing page](https://astute-systems.github.io/cot-tak/reference/licensing/).
