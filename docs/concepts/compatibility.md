# Compatibility and limits

An honest list of what has been checked, what has not, and what is missing.

## Platforms and toolchains

- **Rust 1.88+**, edition 2021. CI tests stable, beta and 1.88 on Linux, and stable on macOS and
  Windows.
- Pure Rust; no system libraries (see [Installation](../getting-started/installation.md)).

## Protocol coverage

| Area | Status |
|---|---|
| CoT 2.0 XML events | Complete round trip, including unknown elements |
| TAK Protocol v1 streaming and mesh framing | Implemented; typed protobuf details for `contact`, `__group`, `precisionlocation`, `status`, `takv`, `track`, everything else as `xmlDetail` |
| Streaming negotiation (`t-x-takp-v/q/r`) | Client and server sides |
| Keepalive ping/pong (`t-x-c-t`, `t-x-c-t-r`) | Client and server sides |
| UDP mesh | Send and receive; **static** protocol selection only |
| Enrollment (`/Marti/api/tls/signClient/v2`) | ECDSA generated, any supplied PKCS#8 key |
| Marti REST | Missions, contents, subscriptions, change log, files, contacts, clients, groups |
| GeoJSON / KML / KMZ / MIL-STD-2525B/C | Import and export as described in [Conversion](../guide/convert.md) |

## Verification status

!!! warning "Not yet validated against live systems"
    The test suite uses in-process mock servers and clients written against the TAK Server source
    and ATAK's documented behaviour. The following have **not** been checked against real ATAK,
    WinTAK or TAK Server builds:

    - the message builders (GeoChat, 911/alerts, deletes, shapes, routes, MEDEVAC, sensors);
    - ECDSA and RSA enrollment against a real enrollment service;
    - Marti response shapes for uploads, contacts and subscriptions;
    - protocol negotiation against every server release.

    Endpoint paths and request parameters for Marti and enrollment *were* verified against the
    TAK Server source. If you find a mismatch, please open an issue with a packet capture or
    server log.

## Not implemented

- **Groups and channels** (server side) and **federation**.
- **Username/password authentication** on the streaming protocol.
- **Dynamic per-contact protocol-version tracking** on the mesh.
- **`signClient` v1** (PKCS#12 response) enrollment.
- **RSA key generation** (supply your own key).
- **MIL-STD-2525D** numeric symbol codes.
- KML: polygons with holes, multi-geometries and tracks are skipped on import; GeoJSON import
  rejects multi-part geometries and holes.

## Behavioural limits to know

| Limit | Default |
|---|---|
| XML document | 8 MiB, 64 levels deep, 1024 attributes per element |
| Streaming frame | 8 MiB |
| UDP datagram | 65,507 bytes (IPv4) |
| GeoJSON/KML import | 10,000 features / vertices |
| KMZ archive | 64 MiB, 10,000 entries, 8 MiB of KML |
| Client queues | 1024 events each way |
| Embedded server | 1024 clients, 10,000 stored entities, 1000 missions, 1000 files, 256 MiB of files |

All limits that make sense to tune are configurable; see each guide page.

## Versioning

The workspace is at **2.0.0-rc2**, a release candidate. The API may still change before 2.0.0
final if feedback or live-system testing demands it; from 2.0.0 on, [semantic
versioning](https://semver.org) applies. Error and event enums are `#[non_exhaustive]`, so adding
variants is not a breaking change. The 0.1 API was replaced entirely and has no compatibility
shim.
