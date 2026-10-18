# Installation

## Requirements

- **Rust 1.88 or newer** (the workspace `rust-version`; CI tests it).
- A [Tokio](https://tokio.rs) runtime for the networking crates (`net`, `enroll`, `marti`,
  `server`). The core, protocol and conversion crates are synchronous and runtime-free.
- No system libraries: TLS is rustls with `ring`, protobuf schemas are compiled by a pure-Rust
  compiler (`protox`), so neither OpenSSL nor `protoc` is needed.

## Adding the dependency

The [`cottak` crate on crates.io](https://crates.io/crates/cottak) currently carries the old
**0.1.x** line, which has a different API. The 2.0.0 release candidate is tagged `v2.0.0-rc2` and is
not published there yet, so depend on the repository:

```toml
[dependencies]
cottak = { git = "https://github.com/Astute-Systems/cot-tak", tag = "v2.0.0-rc2", features = ["net"] }
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

Pick the [features](../reference/features.md) you need:

| Feature | Adds |
|---|---|
| *(default)* `proto` | TAK Protocol v1 mapping, framing and negotiation (`cottak::proto`) |
| `codec` | `tokio_util` `Encoder`/`Decoder` for protocol v1 streams |
| `net` | TAK Server client and mesh UDP (`cottak::net`) |
| `enroll` | `tak://` onboarding and certificate enrollment (`cottak::enroll`) |
| `marti` | Marti REST client (`cottak::marti`) |
| `server` | Embeddable TAK Server and Marti mock (`cottak::server`) |
| `convert` | GeoJSON, KML and MIL-STD-2525 conversion (`cottak::convert`) |
| `kmz` | KMZ (zipped KML) reading and writing |
| `pkcs12` | Import/export of PKCS#12 identities |
| `serde` | `Serialize`/`Deserialize` for the core types |

Or depend on a single crate directly, for example `cottak-core` for just the event model.

!!! warning "Licensing of `cottak-proto`"
    The protocol schema files bundled in `cottak-proto` are derived from TAK Server and are
    GPL-3.0. Anything that links `cottak-proto` (including the default `proto` feature of the
    facade) is a combined work. `cottak-core` and `cottak-convert` are MIT OR Apache-2.0 only.
    See [Licensing](../reference/licensing.md).

## Building from source

```sh
git clone https://github.com/Astute-Systems/cot-tak
cd cot-tak
cargo build --workspace --all-features
cargo test --workspace --all-features
```

See [Contributing](../development/contributing.md) for the full set of checks CI runs.
