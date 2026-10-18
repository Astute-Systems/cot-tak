# TAK Server client

`cottak-net` (`cottak::net`, feature `net`) provides an async client for TAK Server's streaming
ports. It is built to run unattended: it reconnects, detects dead peers and upgrades protocols on
its own.

## What the client does for you

- Connects over **TCP** (default 8087) or **mutual TLS** (8089) using rustls.
- Starts in legacy XML and **upgrades to TAK Protocol v1** when the server offers it
  ([how](protocol.md#negotiation)).
- Sends **keepalive pings** and answers the server's, and treats silence as a dead connection.
- **Reconnects** with jittered exponential backoff; events queued while disconnected are sent after
  reconnecting (stale ones are dropped by default).
- Applies **size limits** to every message it reads.

## Starting a client

`TakClient::start` validates the configuration and spawns the connection task on the current
Tokio runtime. Connection problems are *not* errors here; they are reported as events.

```rust,no_run
use cottak::core::{Event, Point};
use cottak::net::{ClientConfig, ClientEvent, TakClient};

# async fn run() -> Result<(), Box<dyn std::error::Error>> {
let config = ClientConfig::new("tak.example.com:8087", "ALPHA-1");
let (handle, mut events) = TakClient::start(config)?;

handle
    .send(Event::new("ALPHA-1", "a-f-G-U-C".parse()?, Point::new(-27.47, 153.02)?))
    .await?;

while let Some(item) = events.recv().await {
    match item {
        ClientEvent::Connected { protocol } => println!("connected using {protocol:?}"),
        ClientEvent::ProtocolChanged(p) => println!("upgraded to {p:?}"),
        ClientEvent::Event(ev) => println!("{} {}", ev.cot_type, ev.uid),
        ClientEvent::Invalid { reason } => eprintln!("ignored a bad message: {reason}"),
        ClientEvent::Disconnected { reason, retry_in } => {
            eprintln!("disconnected ({reason}); retrying in {retry_in:?}");
        }
        _ => {}
    }
}
# Ok(()) }
```

`ClientEvent` is `#[non_exhaustive]`: keep a wildcard arm.

The `ClientHandle` is cheap to clone. `send` waits if the queue is full, `try_send` does not, and
`shutdown` stops the task and waits for it. Dropping the `ClientEvents` receiver also shuts the
client down.

## TLS

TLS settings live in `TlsConfig`. **Server authentication is mandatory**: give a CA, a pinned
SHA-256 fingerprint, or both. There is no option to skip verification.

**CA files (mutual TLS)**

```rust,no_run
use cottak::net::{ClientConfig, TlsConfig};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let tls = TlsConfig::from_files(
    "server-ca.pem",
    Some(("client.pem".as_ref(), "client.key".as_ref())),
)?;
let config = ClientConfig::new("tak.example.com:8089", "ALPHA-1").with_tls(tls);
# let _ = config;
# Ok(()) }
```

**Pinned fingerprint**

```rust,no_run
use cottak::net::{ClientConfig, TlsConfig};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
// For a self-signed server: pin the fingerprint the operator gave you.
let tls = TlsConfig::new()
    .with_pinned_sha256_hex("AB:CD:EF:01:23:45:67:89:AB:CD:EF:01:23:45:67:89:AB:CD:EF:01:23:45:67:89:AB:CD:EF:01:23:45:67:89")?
    .with_client_identity_pem(std::fs::read("client.pem")?, std::fs::read("client.key")?);
let config = ClientConfig::new("tak.example.com:8089", "ALPHA-1").with_tls(tls);
# let _ = config;
# Ok(()) }
```

**In-memory PEM**

```rust,no_run
use cottak::net::{ClientConfig, TlsConfig};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let (ca, cert, key) = (String::new(), String::new(), String::new());
let tls = TlsConfig::new()
    .with_ca_pem(ca)
    .with_client_identity_pem(cert, key)
    .with_server_name("tak.example.com"); // when connecting by IP address
let config = ClientConfig::new("203.0.113.5:8089", "ALPHA-1").with_tls(tls);
# let _ = config;
# Ok(()) }
```

`TlsConfig::validate` checks the configuration eagerly (it is also done by `TakClient::start`).
Debug output never includes key material.

### Learning a self-signed server's fingerprint

`probe_server_certificate` connects *without* authenticating the server and returns its
certificate so you can show the fingerprint to a human. This is trust-on-first-use by design:
compare the fingerprint with the operator's published value, then pin it.

```rust,no_run
use cottak::net::probe_server_certificate;

# async fn run() -> Result<(), Box<dyn std::error::Error>> {
let cert = probe_server_certificate("tak.example.com:8089").await?;
println!("SHA-256: {}", cert.fingerprint());   // compare out of band!
# Ok(()) }
```

## Configuration reference

`ClientConfig::new(addr, uid)` has sensible defaults; all fields are public.

| Field | Default | Meaning |
|---|---|---|
| `addr` | | `host:port` of the server |
| `uid` | | The UID this client uses in keepalives |
| `tls` | `None` (plain TCP) | See above |
| `protocol` | `PreferProtobuf` | `XmlOnly` stays on legacy XML for the whole connection |
| `connect_timeout` | 10 s | Time allowed to connect and complete the TLS handshake |
| `ping_interval` | 30 s | Keepalive period (`with_keepalive` also sets the read timeout to 3x) |
| `read_timeout` | 90 s | Silence longer than this is a dead connection |
| `write_timeout` | 10 s | A write that takes longer fails the connection |
| `negotiate_timeout` | 30 s | Time allowed for the protocol upgrade |
| `backoff` | 1 s to 60 s | Reconnect delay range (`min * 2^attempt`, capped, with jitter) |
| `send_queue` / `recv_queue` | 1024 | Queue sizes between your code and the connection task |
| `max_message_len` | 8 MiB | Largest accepted message |
| `drop_stale` | `true` | Drop queued events that went stale before they could be sent |

## Plain XML or protocol v1?

Leave the default (`PreferProtobuf`) unless you have a reason not to: the client only upgrades if
the server offers it, so it is safe against older servers. Choose `XmlOnly` to debug traffic or to
talk to a server that mishandles the upgrade.

## Errors

`cottak::net::Error` covers I/O, TLS, configuration and limit errors, and
`Error::Shutdown` when you use a handle after the client stopped. Network faults after startup
do not surface as errors from `send`; they arrive as `ClientEvent::Disconnected`.

## Example: a GeoChat bot

The repository ships a complete bot in
[`crates/cottak/examples/geochat_bot.rs`](https://github.com/Astute-Systems/cot-tak/blob/main/crates/cottak/examples/geochat_bot.rs):

```sh
cargo run -p cottak --example geochat_bot --features net -- tak.example.com:8087 BOT-1 "Echo Bot"
```
