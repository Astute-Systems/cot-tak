# cottak

**cottak** is an open Rust toolkit for **Cursor on Target (CoT)** and **TAK**: ATAK, WinTAK, iTAK
and TAK Server. It is a family of focused crates behind one facade, built for applications that
need to *produce, consume, route or convert* CoT safely.

![TAK map](assets/tak_brisbane.png){ width="640" }

## What you get

| You want to... | Use | Crate |
|---|---|---|
| Build, parse and validate CoT events, shapes, chat, alerts, MEDEVAC | [Events](guide/events.md), [Messages](guide/messages.md) | `cottak-core` |
| Speak TAK Protocol v1 (protobuf) and negotiate it | [TAK Protocol v1](guide/protocol.md) | `cottak-proto` |
| Connect to a TAK Server over TCP or mutual TLS, with reconnect | [TAK Server client](guide/client.md) | `cottak-net` |
| Share situational awareness without a server (UDP multicast) | [Mesh networking](guide/mesh.md) | `cottak-net` |
| Get a client certificate from a server (`tak://` onboarding) | [Enrollment](guide/enrollment.md) | `cottak-enroll` |
| Manage missions and files through the Marti REST API | [Marti API](guide/marti.md) | `cottak-marti` |
| Run a TAK Server inside your tests or on an isolated network | [Embedded server](guide/server.md) | `cottak-server` |
| Convert between CoT and GeoJSON, KML/KMZ, MIL-STD-2525 symbols | [Conversion](guide/convert.md) | `cottak-convert` |

## Design goals

- **Safe on untrusted input.** Every parser has size, depth and count limits, rejects DTDs and
  custom entities, and is fuzzed. See the [security model](concepts/security.md).
- **Lossless.** Elements the library does not model are preserved verbatim, so a parse and write
  round trip does not drop data you did not ask it to touch.
- **No OpenSSL, no C.** TLS is [rustls](https://github.com/rustls/rustls) with the `ring`
  provider. There is deliberately no way to skip server verification.
- **Small pieces.** Depend only on the crates you need; the `cottak` facade re-exports them
  behind [Cargo features](reference/features.md).
- **Documented by compilation.** Rust snippets in this guide are compiled and run as doctests in
  CI, so they cannot drift from the API. See [Documentation](development/documentation.md).

## A first look

```rust
use cottak::core::detail::{Contact, Track};
use cottak::core::{Event, Point};

let event = Event::new("ALPHA-1", "a-f-G-U-C".parse()?, Point::new(-27.4689, 153.0235)?)
    .with(Contact::new("ALPHA-1"))
    .with(Track::new(90.0, 1.5));

let xml = event.to_xml_string()?;
let parsed: Event = xml.parse()?;
assert_eq!(parsed, event);
# Ok::<(), cottak::core::Error>(())
```

Continue with [Installation](getting-started/installation.md) and the
[Quick start](getting-started/quickstart.md), or jump straight to the
[API reference](reference/api.md).

!!! warning "Status: 0.2 development"
    The workspace has been rebuilt for 0.2; the 0.1 API is gone. Message builders follow the
    behaviour of ATAK but have not all been validated against live ATAK/WinTAK/TAK Server
    builds. See [Compatibility and limits](concepts/compatibility.md) for exactly what is and is
    not verified.
