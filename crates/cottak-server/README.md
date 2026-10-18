# cottak-server

Embeddable TAK Server and Marti HTTPS mock for tests, simulators and small networks.

Part of [cottak](https://github.com/Astute-Systems/cot-tak), the Rust toolkit for Cursor on Target and TAK.

## Features

- TCP and (mutual) TLS listeners, XML and TAK Protocol v1 with negotiation, ping/pong
- Routing: broadcast, callsign (`<marti><dest callsign>`) and mission (`<dest mission>`); latest-position replay
- A Marti HTTPS mock: missions, files, contacts, endpoints and groups, so `cottak-marti` can be tested end to end
- Bounded clients, queues, stored events and uploads

Not a replacement for TAK Server: no accounts, groups, persistence or federation.

## Example

```rust,no_run
use cottak_server::{Server, ServerConfig};

# async fn run() -> Result<(), Box<dyn std::error::Error>> {
let server = Server::start(ServerConfig::new().listen_tcp("127.0.0.1:0")).await?;
let addr = server.local_addrs()[0];   // point a client here
let mut seen = server.subscribe();    // observe every routed event
# let _ = (addr, &mut seen);
server.shutdown().await;
# Ok(()) }
```

## Documentation

- [Guide](https://astute-systems.github.io/cot-tak/guide/server/) (every Rust snippet in it is compiled in CI)
- [API reference](https://astute-systems.github.io/cot-tak/api/cottak_server/)
- [Repository](https://github.com/Astute-Systems/cot-tak) and the [`cottak`](https://crates.io/crates/cottak) facade crate

## Licence

Licensed under either of Apache-2.0 or MIT at your option. **Note:** this crate depends on `cottak-proto`, whose bundled TAK Protocol schema is GPL-3.0, so programs that link it are a combined work; see the [licensing page](https://astute-systems.github.io/cot-tak/reference/licensing/).
