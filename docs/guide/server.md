# Embedded server

`cottak-server` (`cottak::server`, feature `server`) is a TAK Server you can start from Rust. Use
it for **integration tests**, **simulators** and **small isolated networks** where installing the
official server is overkill.

!!! warning "Not a replacement for TAK Server"
    There are no user accounts, no groups or channels, no persistence and no federation. A TLS
    listener can authenticate clients by certificate, but every authenticated client sees every
    other client's traffic, and identities a client claims (UID, callsign, mission `creatorUid`)
    are taken at their word. All state is in memory and bounded by configuration limits.

## What it does

- **Listeners** for plain TCP, TLS (optionally requiring client certificates) and the Marti HTTPS
  API, any number at once, sharing one state.
- **Protocols**: legacy XML and [TAK Protocol v1](protocol.md), including the standard
  negotiation, so clients of both kinds interoperate.
- **Keepalive**: answers pings and disconnects clients that go silent.
- **Routing**: every event goes to every other client, or only to the callsigns in
  `<marti><dest callsign="..."/></marti>`, or to the subscribers of
  `<marti><dest mission="..."/></marti>`.
- **Replay**: the latest position of every entity is kept and sent to newly connected clients; a
  delete message (`t-x-d-d`) removes entities.
- **Observation and injection**: watch every connection and routed event, or send events as the
  server.

## Running it in a test

This example starts a server on an ephemeral port, connects a real [client](client.md), and
observes the event on the server side. It runs as part of the documentation tests.

```rust
use cottak::core::{Event, Point};
use cottak::net::{ClientConfig, ProtocolPreference, TakClient};
use cottak::server::{Server, ServerConfig, ServerEvent};
use std::time::Duration;

# #[tokio::main]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
// Port 0 lets the OS choose; ask the server where it landed.
let server = Server::start(ServerConfig::new().listen_tcp("127.0.0.1:0")).await?;
let addr = server.local_addrs()[0];
let mut observer = server.subscribe();

let config = ClientConfig::new(addr.to_string(), "TEST-1").with_protocol(ProtocolPreference::XmlOnly);
let (client, _events) = TakClient::start(config)?;
client
    .send(Event::new("TEST-1", "a-f-G-U-C".parse()?, Point::new(1.0, 2.0)?))
    .await?;

// Wait until the server has routed our event.
let seen = tokio::time::timeout(Duration::from_secs(10), async {
    loop {
        if let ServerEvent::Event { event, .. } = observer.recv().await.unwrap() {
            break event;
        }
    }
})
.await?;
assert_eq!(seen.uid, "TEST-1");
assert_eq!(server.stored_events().len(), 1);

client.shutdown().await;
server.shutdown().await;
# Ok(()) }
```

`Server::shutdown` stops the listeners, disconnects every client and waits for all tasks.
Dropping the `Server` stops it as well.

## Configuration

Build a `ServerConfig` with `ServerConfig::new()` and add at least one listener:

```rust
use cottak::server::{ServerConfig, ServerTlsConfig};

let config = ServerConfig::new()
    .listen_tcp("127.0.0.1:8087")                       // like TAK Server's 8087
    .with_protocol_versions(&[1])                        // offer protocol v1; &[] = XML only
    .with_read_timeout(std::time::Duration::from_secs(90));
# let _ = config;

// TLS with client certificates required (like port 8089):
# fn tls() -> Result<(), Box<dyn std::error::Error>> {
let tls = ServerTlsConfig::mutual_from_files("server.pem", "server.key", "client-ca.pem")?;
let config = ServerConfig::new().listen_tls("0.0.0.0:8089", tls);
# let _ = config;
# Ok(()) }
```

`ServerTlsConfig::mutual` requires client certificates. `server_only` does not authenticate
clients and is meant for tests and trusted networks; the choice is explicit so client
authentication can never be dropped by accident (an empty client CA is an error).

Other settings are public fields on `ServerConfig`:

| Field | Default | Meaning |
|---|---|---|
| `server_uid` | `cottak-server` | UID used in the control events the server sends |
| `protocol_versions` | `[1]` | Versions offered to clients |
| `max_clients` | 1024 | Connections allowed, including ones still in TLS handshake |
| `max_message_len` | 8 MiB | Largest accepted message |
| `client_queue` | 1024 | Events buffered per slow client; beyond that, events for it are dropped |
| `read_timeout` | 90 s | Silent clients are disconnected |
| `write_timeout` / `handshake_timeout` | 10 s | |
| `replay_on_connect` | `true` | Send latest positions to new clients |
| `max_stored_events` | 10,000 | Entities kept for replay |

## The Marti mock

Add `listen_marti(addr, tls)` to serve a mock of the [Marti API](marti.md):

```rust,no_run
use cottak::server::{BasicAuth, ServerConfig, ServerTlsConfig};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let tls = ServerTlsConfig::mutual_from_files("server.pem", "server.key", "client-ca.pem")?;
let config = ServerConfig::new()
    .listen_tcp("127.0.0.1:8087")
    .listen_marti("127.0.0.1:8443", Some(tls))
    .with_marti_basic_auth(BasicAuth::new("alice", "s3cret")); // optional
# let _ = config;
# Ok(()) }
```

It implements missions (with contents, change log, subscriptions and passwords), file upload and
download, `contacts/all`, `clientEndPoints`, `groups/all` and the version endpoints, using the
same JSON shapes as TAK Server. `MartiClient` works against it.

It is connected to the streaming side:

- **Contacts and endpoints** come from the connected clients (UID, callsign and team learned from
  their position reports).
- **Mission routing**: an event sent with `<marti><dest mission="ops"/></marti>` is recorded in
  mission `ops` and delivered to its subscribers (matched by client UID).
- **Latest state**: the mission remembers the newest event for each of its UIDs and serves them
  from `GET /missions/{name}/cot`.

Access rules are simple: only a mission's creator may delete it; the creator and subscribers (unless
the mission's default role is read-only) may change its content; password-protected missions
require the password to read or join.

Memory use is bounded by `max_missions` (1000), `max_files` (1000), `max_store_bytes` (256 MiB) and
`marti_max_body` (16 MiB per request). The HTTP implementation is deliberately strict: it frames
bodies by `Content-Length` only (rejecting `Transfer-Encoding`), limits header and body sizes and
times out slow requests.

## Observing and injecting

```rust,no_run
use cottak::server::{Server, ServerEvent};
# async fn run(server: Server) -> Result<(), Box<dyn std::error::Error>> {
for client in server.clients() {
    println!("{} {:?} via {:?}", client.peer, client.callsign, client.protocol);
}

let mut events = server.subscribe();
while let Ok(item) = events.recv().await {
    match item {
        ServerEvent::Connected(c) => println!("+ {}", c.peer),
        ServerEvent::Disconnected(c) => println!("- {}", c.peer),
        ServerEvent::Event { from, event } => println!("{:?}: {}", from, event.uid),
        _ => {}
    }
}
# Ok(()) }
```

`server.broadcast(event)` injects an event as if the server sent it (honouring `<marti>` routing).
A subscriber that falls behind skips events rather than slowing the server.

The repository includes a ready-to-run example:

```sh
cargo run -p cottak --example mock_server --features server
```
