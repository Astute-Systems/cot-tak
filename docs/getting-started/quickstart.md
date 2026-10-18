# Quick start

This page takes you from nothing to a program that connects to a TAK Server, announces itself on
the map and prints what it receives. Every snippet is compiled in CI.

## 1. Build an event

An `Event` is one CoT message: an identity (`uid`), a type, a position and optional details.

```rust
use cottak::core::detail::Contact;
use cottak::core::{Event, Point};

let me = Event::new("MY-UID", "a-f-G-U-C".parse()?, Point::new(-27.4689, 153.0235)?)
    .with(Contact::new("Rusty"));
assert_eq!(me.cot_type.as_str(), "a-f-G-U-C"); // friendly ground unit
# Ok::<(), cottak::core::Error>(())
```

`Point::new` rejects latitudes and longitudes out of range and non-finite numbers, and
`"a-f-G-U-C".parse()` rejects malformed type strings, so an `Event` is always well formed.

## 2. Connect and send

`TakClient::start` returns immediately. The client connects in the background, upgrades to
TAK Protocol v1 when the server offers it, sends keepalives and reconnects with backoff.

```rust,no_run
use cottak::core::detail::Contact;
use cottak::core::{Event, Point};
use cottak::net::{ClientConfig, ClientEvent, TakClient};

# async fn run() -> Result<(), Box<dyn std::error::Error>> {
// Port 8087 is TAK Server's plain-TCP streaming port. Use TLS for anything real.
let (handle, mut events) = TakClient::start(ClientConfig::new("127.0.0.1:8087", "MY-UID"))?;

let me = Event::new("MY-UID", "a-f-G-U-C".parse()?, Point::new(-27.4689, 153.0235)?)
    .with(Contact::new("Rusty"));
handle.send(me).await?;

while let Some(item) = events.recv().await {
    if let ClientEvent::Event(ev) = item {
        println!("{} {}", ev.cot_type, ev.uid);
    }
}
# Ok(()) }
```

!!! tip "Keep draining the receiver"
    The client stops reading from the server while its receive queue is full. Always keep a task
    calling `events.recv()`.

## 3. Use mutual TLS

Real TAK Servers use mutual TLS on port 8089. Trust is explicit: you must give a CA file or a
pinned fingerprint, because there is no "skip verification" switch.

```rust,no_run
use cottak::net::{ClientConfig, TlsConfig};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let tls = TlsConfig::from_files(
    "ca.pem",
    Some(("client.pem".as_ref(), "client.key".as_ref())),
)?;
let config = ClientConfig::new("tak.example.com:8089", "MY-UID").with_tls(tls);
# let _ = config;
# Ok(()) }
```

Don't have a certificate yet? See [Enrollment](../guide/enrollment.md).

## Where next

- [Events and details](../guide/events.md): the data model in depth.
- [Message builders](../guide/messages.md): GeoChat, alerts, shapes, routes, MEDEVAC.
- [TAK Server client](../guide/client.md): configuration, events and error handling.
- [Embedded server](../guide/server.md): test your application without a real server.
