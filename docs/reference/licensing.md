# Licensing

## The code

All Rust source in this repository is dual-licensed under either of

- [Apache License, Version 2.0](https://github.com/Astute-Systems/cot-tak/blob/main/LICENSE-APACHE)
- [MIT license](https://github.com/Astute-Systems/cot-tak/blob/main/LICENSE-MIT)

at your option. Unless you explicitly state otherwise, any contribution intentionally submitted for
inclusion in this work shall be dual licensed as above, without any additional terms or conditions.

## The exception: the TAK Protocol schema

`cottak-proto` bundles the `.proto` files that define TAK Protocol v1. They are derived from the
files of the same names in the [TAK Server](https://github.com/TAK-Product-Center/Server)
repository, which is licensed under the **GPL-3.0**. They are therefore distributed under the
GPL-3.0, separately from the crate's Rust code. The generated Rust code is built from them, so
the crate as a whole is:

```text
(MIT OR Apache-2.0) AND GPL-3.0-only
```

The provenance and terms are recorded in `crates/cottak-proto/proto/NOTICE.md`, with the licence
text in `crates/cottak-proto/LICENSE-GPL-3.0`.

**Consequence.** A program that links `cottak-proto` is a combined work and must comply with the
GPL-3.0 for it. That covers, directly or indirectly:

- `cottak-net`, `cottak-enroll`, `cottak-marti` (it uses `cottak-net`'s TLS configuration) and
  `cottak-server`, and
- the `cottak` facade with its default `proto` feature or any feature that pulls those crates in
  (`codec`, `net`, `enroll`, `marti`, `server`, `pkcs12`).

**Not affected:** `cottak-core` and `cottak-convert` do not depend on `cottak-proto` and remain
MIT OR Apache-2.0 only. For a permissive build, depend on those two crates directly, or on the
facade with `default-features = false` and only the `convert`, `kmz` and `serde` features.

!!! info "Not legal advice"
    This is a description of how the repository is set up, not legal advice. If your use of the
    GPL-3.0 parts matters to you, consult a lawyer.

## The CoT type catalogue

MITRE's `CoTtypes.xml` carries a copyright notice without a licence grant, so it is **not**
included. `cottak-convert` can read a copy you supply (`TypeCatalog::from_xml`).

## Dependencies

CI runs `cargo deny`, which only allows these licences for dependencies: MIT, Apache-2.0
(including with the LLVM exception), BSD-3-Clause, ISC, Zlib, CC0-1.0, Unicode-3.0 and
CDLA-Permissive-2.0. Copyleft dependencies are rejected, with the single reviewed exception of
`cottak-proto` itself.
