# Contributing

Thank you for helping. This page covers building, the checks CI runs and the conventions to
follow. Open an issue first for anything substantial.

## Layout

```text
crates/
  cottak-core/      events, details, XML, message builders
  cottak-proto/     TAK Protocol v1 (protobuf, framing, negotiation)
  cottak-net/       TAK Server client, TLS, mesh
  cottak-enroll/    certificate enrollment
  cottak-marti/     Marti REST client
  cottak-server/    embedded server and Marti mock
  cottak-convert/   GeoJSON, KML/KMZ, MIL-STD-2525
  cottak/           facade crate, examples
docs/               this documentation (MkDocs)
fuzz/               cargo-fuzz targets (separate workspace)
```

## Setup

You need Rust 1.88 or newer. Fuzzing needs a nightly toolchain and `cargo install cargo-fuzz`;
the docs need Python 3 (see [Documentation](documentation.md)).

```sh
git clone https://github.com/Astute-Systems/cot-tak
cd cot-tak
cargo build --workspace --all-features
```

## The checks

CI runs all of these; running them locally first saves a round trip.

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-features --all-targets      # pedantic, must be warning-free
cargo test --workspace --all-features
cargo test --workspace --no-default-features
cargo build --workspace --examples --all-features
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --all-features
cargo deny check                                           # advisories, licences, bans
cargo +1.88 test --workspace --all-features                # the MSRV
```

`Cargo.lock` is not committed (the workspace is a library); regenerate it with
`cargo generate-lockfile`. The workspace uses resolver 3, so dependency versions respect the MSRV.

## Conventions

- **No `unsafe`.** The workspace forbids it.
- **Bound everything untrusted.** New parsers need explicit size, depth and count limits, tests
  with hostile input, and (for byte or text formats) a fuzz target.
- **Errors**: `thiserror` enums, `#[non_exhaustive]`, no panics on input. Return errors for
  invalid data; reserve `unwrap`/`expect` for tests and provably-unreachable cases.
- **Secrets** never appear in `Debug` output or error messages.
- **Docs**: every public item has a doc comment (`missing_docs` warns); user-facing behaviour gets
  a guide page whose code is compiled by the documentation tests.
- **Licensing**: new dependencies must satisfy `deny.toml`. Do not edit the `.proto` schema files
  or `cottak-proto`'s licence setup without discussing it first.

## Pull requests

1. Make the change with tests (unit tests next to the code, integration tests in `tests/`).
2. Update the relevant guide page in `docs/` and the rustdoc comments.
3. Run the checks above.
4. Describe *why* in the pull request, and any behaviour that is verified only against mocks.

## Releases

1. Set the version in the workspace `Cargo.toml` (`[workspace.package]` and the
   `[workspace.dependencies]` path pins) and update the version mentioned in the docs.
2. Make sure CI and the Docs workflow pass on `main`.
3. Tag the commit (`git tag -a vX.Y.Z -m "..."`), push the tag, and publish a GitHub release
   (`gh release create vX.Y.Z --prerelease` for release candidates).
4. The `SLSA provenance` workflow packages the crates and attaches signed provenance to the
   release; see [Verifying a release](../concepts/security.md#verifying-a-release).

By contributing you agree that your work is licensed as described in
[Licensing](../reference/licensing.md).
