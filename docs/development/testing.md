# Testing and fuzzing

## Layers of tests

| Layer | Where | What it covers |
|---|---|---|
| Unit tests | `src/**` next to the code | Parsing, conversion, limits, edge cases |
| Property tests | `tests/props.rs` (`proptest`) | Round trips and "never panics on arbitrary input" |
| Integration tests | `crates/*/tests/` | Real components against each other |
| Doc tests | `///` examples and `docs/**/*.md` | Documented behaviour stays true |
| Fuzz targets | `fuzz/` | Coverage-guided search for panics and invariant violations |

### Integration tests worth knowing

- `cottak-net/tests/client.rs` and `tls.rs`: the real client against in-process mock servers,
  including mutual TLS with generated certificates and negative cases (untrusted CA, wrong
  server name, pinning).
- `cottak-enroll/tests/enroll.rs`: the whole enrollment flow against a mock enrollment service,
  including wrong keys, redirects, oversized responses and RSA keys.
- `cottak-marti/tests/marti.rs`: the REST client against a mock of the API.
- `cottak-server/tests/`: the **embedded server driven by the real clients**: protocol upgrade,
  routing, replay, mission routing, mutual TLS, and raw-HTTP framing attacks against the Marti
  mock.

The embedded server is also the best way to test your own application: see
[Embedded server](../guide/server.md).

## Running tests

```sh
cargo test --workspace --all-features            # everything, including the doc tests
cargo test -p cottak-server                      # one crate
cargo test -p cottak --doc --all-features        # just the documentation tests
```

Tests bind to `127.0.0.1:0` (an ephemeral port) and never need external network access.

## Fuzzing

The fuzz crate is a separate workspace so that nightly-only tooling does not affect normal builds.

| Target | Input | Invariant |
|---|---|---|
| `xml_parse` | Arbitrary bytes | No panics; successful parses re-serialise |
| `event_roundtrip` | Arbitrary bytes | Parse, write and re-parse give an equal event |
| `proto_stream` | Arbitrary bytes | Stream decoding never panics |
| `mesh_decode` | Arbitrary bytes | Datagram decoding never panics |
| `geojson_import` | Arbitrary text | Imported events are valid and round-trip |
| `kml_import` | Arbitrary text | Imported events are valid and round-trip |
| `kmz_import` | Arbitrary bytes | No panics, bounded work, valid events |
| `text_formats` | Arbitrary text | Small text parsers (types, times, symbol codes) never panic |
| `marti_models` | Arbitrary JSON | Tolerant decoding never panics |

```sh
cd fuzz
cargo install cargo-fuzz --locked
cargo +nightly fuzz run kmz_import seeds/kmz_import -- -max_total_time=60
```

Seed inputs live in `fuzz/seeds/<target>/`; add a seed for every bug a fuzzer finds, plus a
regression test in the owning crate. CI runs each target for 30 seconds on every change.

## Testing your own code against cottak

- Use `cottak-server` as a dev-dependency: start it on port 0, point your client at
  `server.local_addrs()`, and observe traffic with `server.subscribe()`.
- Build events with the [message builders](../guide/messages.md) and assert on
  `event.to_xml_string()` or the typed details.
- For protocol-level tests, `TakFrame` and `TakCodec` encode and decode in memory without sockets.
