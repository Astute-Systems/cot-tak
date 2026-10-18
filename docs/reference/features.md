# Cargo features

The `cottak` facade crate enables optional functionality through Cargo features. Each feature
pulls in only what it needs.

| Feature | Default | Enables | Implies |
|---|:-:|---|---|
| `proto` | yes | `cottak::proto`: TAK Protocol v1 mapping, framing, negotiation | |
| `codec` | | `TakCodec`, a `tokio_util` `Encoder`/`Decoder` | `proto` |
| `net` | | `cottak::net`: TAK Server client, TLS, mesh UDP | `proto` |
| `enroll` | | `cottak::enroll`: onboarding URLs and certificate enrollment | `net` |
| `marti` | | `cottak::marti`: Marti REST client | `net` |
| `server` | | `cottak::server`: embedded TAK Server and Marti mock | `net` |
| `convert` | | `cottak::convert`: GeoJSON, KML, MIL-STD-2525 | |
| `kmz` | | KMZ reading and writing | `convert` |
| `pkcs12` | | PKCS#12 import/export of identities | `enroll` |
| `serde` | | `Serialize`/`Deserialize` for core types | |

`cottak::core` is always available.

## Choosing features

| I am building... | Use |
|---|---|
| A parser, gateway or data pipeline | `default-features = false` (core only) |
| A TAK client | `net` (add `enroll` to onboard with `tak://` links) |
| A service that manages missions | `marti` |
| Integration tests for any of the above | `server` as a dev-dependency |
| A GIS import/export tool | `convert` (plus `kmz` for `.kmz` files) |

```toml
[dependencies]
cottak = { version = "2.0.0-rc2", default-features = false, features = ["convert", "kmz"] }
```

!!! note
    `proto` is on by default and carries the GPL-3.0 schema files, as do `net`, `enroll`, `marti`
    and `server`. For a permissively licensed build use `default-features = false` with only
    `convert`, `kmz` and `serde`. See [Licensing](licensing.md).

## Crate-level features

If you depend on a member crate directly:

| Crate | Feature | Meaning |
|---|---|---|
| `cottak-core` | `serde` | Serde support for the model |
| `cottak-proto` | `codec` | `tokio_util` codec |
| `cottak-enroll` | `pkcs12` | PKCS#12 import/export |
| `cottak-convert` | `kmz` | KMZ support (adds `zip` and `flate2` with the pure-Rust backend) |

## Runtime requirements

The `net`, `enroll`, `marti` and `server` features need a Tokio runtime with the multi-threaded or
current-thread scheduler and its timer and I/O drivers enabled (`#[tokio::main]` provides them).
