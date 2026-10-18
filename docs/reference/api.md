# API reference

The authoritative API reference is the **rustdoc** documentation generated from the source on every
change to `main`. It is published alongside this guide and cross-linked from it.

[Open the full API reference :material-open-in-new:](api:cottak/index.html){ .md-button .md-button--primary }

## Crates

| Crate | API docs | Purpose |
|---|---|---|
| `cottak` | [cottak](api:cottak/index.html) | Facade; re-exports the family behind features |
| `cottak-core` | [cottak_core](api:cottak_core/index.html) | Events, details, XML, message builders |
| `cottak-proto` | [cottak_proto](api:cottak_proto/index.html) | TAK Protocol v1 mapping, framing, negotiation |
| `cottak-net` | [cottak_net](api:cottak_net/index.html) | TAK Server client, TLS, mesh |
| `cottak-enroll` | [cottak_enroll](api:cottak_enroll/index.html) | Certificate enrollment |
| `cottak-marti` | [cottak_marti](api:cottak_marti/index.html) | Marti REST client |
| `cottak-server` | [cottak_server](api:cottak_server/index.html) | Embedded TAK Server and Marti mock |
| `cottak-convert` | [cottak_convert](api:cottak_convert/index.html) | GeoJSON, KML/KMZ, MIL-STD-2525 |

## Useful entry points

- Events: [`Event`](api:cottak_core/event/struct.Event.html),
  [`Detail`](api:cottak_core/detail/struct.Detail.html),
  [`CotType`](api:cottak_core/cot_type/struct.CotType.html),
  [message builders](api:cottak_core/messages/index.html)
- Networking: [`TakClient`](api:cottak_net/struct.TakClient.html),
  [`ClientConfig`](api:cottak_net/struct.ClientConfig.html),
  [`TlsConfig`](api:cottak_net/struct.TlsConfig.html),
  [`MeshSocket`](api:cottak_net/mesh/struct.MeshSocket.html)
- Services: [`Enroller`](api:cottak_enroll/struct.Enroller.html),
  [`MartiClient`](api:cottak_marti/struct.MartiClient.html),
  [`Server`](api:cottak_server/struct.Server.html)
- Conversion: [`geojson`](api:cottak_convert/geojson/index.html),
  [`kml`](api:cottak_convert/kml/index.html),
  [`Sidc`](api:cottak_convert/sidc/struct.Sidc.html)

## Reading the docs offline

```sh
cargo doc --workspace --no-deps --all-features --open
```

This builds exactly what is published (CI uses `RUSTDOCFLAGS="-D warnings"`, so broken links in
the API docs fail the build). Once 2.0.0 is published on [crates.io](https://crates.io/crates/cottak), the same documentation will also
be available on docs.rs.

## Why rustdoc and not Doxygen?

Doxygen does not understand Rust; third-party filters exist but lose traits, generics, lifetimes
and intra-doc links. Rust's own tool is the standard, produces searchable HTML, tests the
examples and powers docs.rs. This site therefore embeds rustdoc output rather than a second,
lower-fidelity reference. See [Documentation](../development/documentation.md).
