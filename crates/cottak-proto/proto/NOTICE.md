# Licence of the .proto files

The `*.proto` files in this directory define the TAK Protocol v1 wire format
(`atakmap.commoncommo.protobuf.v1`). They are derived from the files of the same
names in the TAK Server repository:

<https://github.com/TAK-Product-Center/Server> (`src/takserver-protobuf/src/main/proto`)

which is licensed under the **GNU General Public License, version 3**. The
files here are therefore distributed under **GPL-3.0** (see
`../LICENSE-GPL-3.0`), separately from the Rust code of this crate, which is
MIT OR Apache-2.0.

Provenance: compared against the upstream `main` branch; comments and message
structure match, with minor local differences (formatting and ordering).
The upstream files carry no per-file licence header, so the repository
licence applies. If you are a copyright holder and want this handled
differently, please open an issue.

The generated Rust code (built from these files by `build.rs`) is a derivative
of them. A program that links `cottak-proto` must therefore comply with the
GPL-3.0 for the combined work.
