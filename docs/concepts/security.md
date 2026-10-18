# Security model

CoT is exchanged between devices that do not trust each other, often over radio links and shared
networks. `cottak` assumes **every byte it reads is hostile** and **every secret it holds is
precious**.

## Untrusted input

| Surface | Defences |
|---|---|
| Event XML | No DTDs, no custom entities (blocks XXE and entity-expansion bombs); 8 MiB document limit, 64-level depth limit, 1024 attributes per element with a linear duplicate check; control characters rejected |
| TAK Protocol v1 | Frame length limit enforced before buffering; bad magic bytes and over-long varints end the stream; malformed protobuf is an error |
| Mesh datagrams | Exact header check, version check, size bounded by UDP |
| GeoJSON / KML | Feature and vertex limits; coordinates range-checked; non-finite numbers rejected; imported events validated |
| KMZ | Archive size, entry count and KML size limits (declared **and** actual); never extracted to disk |
| Marti and enrollment responses | Response size caps; tolerant decoding that never trusts field presence; redirects never followed |
| Embedded server HTTP | `Content-Length` framing only (`Transfer-Encoding` refused), header/body limits, request timeouts, multipart delimiters anchored to line starts |

All of these parsers are exercised by **property tests and fuzz targets** (`xml_parse`,
`event_roundtrip`, `proto_stream`, `mesh_decode`, `geojson_import`, `kml_import`, `kmz_import`,
`text_formats`, `marti_models`). A short fuzz run is part of every CI build. Bugs the fuzzers
found are pinned by regression tests and seed inputs.

## Transport security

- TLS is rustls with the `ring` provider. **No OpenSSL, no native-tls** (`cargo deny` forbids
  them).
- **Server authentication is mandatory.** A `TlsConfig` without a CA or a pinned fingerprint is an
  error. There is no "insecure" option.
- Pinning uses SHA-256 fingerprints; a pin can be combined with CA validation.
- `probe_server_certificate` is explicitly **trust on first use**: use it to show a fingerprint to a
  human, never to decide trust automatically.
- Enrollment sends credentials only **after** the server has been verified, and does not follow
  redirects.
- Server-side TLS makes client authentication an explicit choice (`mutual` vs `server_only`), and an
  empty client-CA list is an error rather than silently disabling authentication.

## Secrets

- Private keys, passwords and tokens are redacted from `Debug` output.
- `Identity::save` creates key files with owner-only permissions on Unix and refuses to overwrite
  existing files.
- Enrollment never transmits the private key, only a signed certificate request.
- The Marti mock compares credentials and mission passwords in constant time and never returns a
  password.

## Memory safety and resource use

- `#![forbid(unsafe_code)]` across the workspace.
- Every queue is bounded. A slow client or observer loses events; it cannot make the process grow.
- The embedded server bounds clients, stored events, missions, files and total uploaded bytes.

## Supply chain

CI runs `cargo deny` (advisories, licences, banned crates, sources) and `cargo audit` on every
change, and the MSRV resolver is used so dependency versions are what the minimum toolchain can
actually build. Advisories that apply are fixed by upgrading, not ignored.

## Release provenance (SLSA)

Every published release carries **SLSA Level 3 build provenance** for its crate archives
(`*.crate`, produced by `cargo package`). The
[`SLSA provenance`](https://github.com/Astute-Systems/cot-tak/blob/main/.github/workflows/slsa-provenance.yml)
workflow builds the archives, hashes them and calls the
[SLSA GitHub generator](https://github.com/slsa-framework/slsa-github-generator), which runs in
an isolated job and signs a provenance statement with the workflow's OIDC identity. The statement
records the source repository, the tag, the workflow and the SHA-256 of each archive, and is
attached to the GitHub release as `multiple.intoto.jsonl`.

This lets you check that an archive was built by this repository's CI from the tagged source and
was not modified afterwards. It does **not** replace reading the code, and it covers the crate
archives only (not your own build of the library).

### Verifying a release

1. Download the `.crate` file you want and `multiple.intoto.jsonl` from the
   [release page](https://github.com/Astute-Systems/cot-tak/releases).
2. Install [`slsa-verifier`](https://github.com/slsa-framework/slsa-verifier).
3. Verify, naming the source repository and the release tag:

```sh
slsa-verifier verify-artifact cottak-core-2.0.0-rc2.crate \
  --provenance-path multiple.intoto.jsonl \
  --source-uri github.com/Astute-Systems/cot-tak \
  --source-tag v2.0.0-rc2
```

A successful run prints `PASSED: SLSA verification passed`. If the archive, the tag or the
repository does not match the provenance, verification fails and you should not use the file.

!!! note
    Provenance is attached when a GitHub release is *published*. A bare tag push runs the same
    build (to catch problems early) but uploads no provenance. The archives are built with
    `cargo package --no-verify`; their contents are tested by the regular CI.

## What is *not* protected

- **The mesh has no authentication or encryption.** That is a property of TAK's UDP mesh protocol.
- **The embedded server is a test and small-network tool.** It has no accounts and trusts the
  identities clients claim. Do not expose it as a replacement for a TAK Server.
- `cottak` does not validate the *meaning* of events. A well-formed event can still lie about
  where a unit is.

## Reporting a vulnerability

Please report security problems privately through GitHub's
[security advisories](https://github.com/Astute-Systems/cot-tak/security/advisories/new) rather
than a public issue.
