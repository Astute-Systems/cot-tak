# cottak-net

Async TAK Server client (TCP and mutual TLS) and UDP mesh transport.

Part of [cottak](https://github.com/Astute-Systems/cot-tak), the Rust toolkit for Cursor on Target and TAK.

## Features

- TCP and **mutual TLS** (rustls, no OpenSSL); server authentication is mandatory (CA and/or SHA-256 pinning)
- Automatic **upgrade from XML to TAK Protocol v1**, keepalive, dead-peer detection, **jittered reconnect**
- Bounded queues and size limits; stale events dropped
- UDP multicast **mesh** for situational awareness and GeoChat

## Example

```rust,no_run
use cottak_core::{Event, Point};
use cottak_net::{ClientConfig, ClientEvent, TakClient};

# async fn run() -> Result<(), Box<dyn std::error::Error>> {
let (handle, mut events) = TakClient::start(ClientConfig::new("tak.example.com:8087", "ALPHA-1"))?;
handle.send(Event::new("ALPHA-1", "a-f-G-U-C".parse()?, Point::new(-27.47, 153.02)?)).await?;
while let Some(ClientEvent::Event(ev)) = events.recv().await {
    println!("{} {}", ev.cot_type, ev.uid);
}
# Ok(()) }
```

## Documentation

- [Guide](https://astute-systems.github.io/cot-tak/guide/client/) (every Rust snippet in it is compiled in CI)
- [API reference](https://astute-systems.github.io/cot-tak/api/cottak_net/)
- [Repository](https://github.com/Astute-Systems/cot-tak) and the [`cottak`](https://crates.io/crates/cottak) facade crate

## Licence

Licensed under either of Apache-2.0 or MIT at your option. **Note:** this crate depends on `cottak-proto`, whose bundled TAK Protocol schema is GPL-3.0, so programs that link it are a combined work; see the [licensing page](https://astute-systems.github.io/cot-tak/reference/licensing/).
