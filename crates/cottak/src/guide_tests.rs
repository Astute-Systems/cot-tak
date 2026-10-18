//! Compiles and runs the Rust code blocks of the guide (`docs/**/*.md`) as doctests.
//!
//! This module only exists under `cargo test --doc`. Each page is attached to an empty item as
//! its documentation, so rustdoc extracts the code blocks and tests them exactly as it does for
//! `///` comments. A page is included only when the features its examples need are enabled.

macro_rules! guide_page {
    ($cfg:meta, $name:ident, $path:literal) => {
        #[cfg($cfg)]
        #[doc = include_str!($path)]
        pub struct $name;
    };
}

guide_page!(all(), Index, "../../../docs/index.md");
guide_page!(
    feature = "net",
    Quickstart,
    "../../../docs/getting-started/quickstart.md"
);
guide_page!(all(), Events, "../../../docs/guide/events.md");
guide_page!(all(), Messages, "../../../docs/guide/messages.md");
guide_page!(
    feature = "codec",
    Protocol,
    "../../../docs/guide/protocol.md"
);
guide_page!(feature = "net", Client, "../../../docs/guide/client.md");
guide_page!(feature = "net", Mesh, "../../../docs/guide/mesh.md");
guide_page!(
    feature = "enroll",
    Enrollment,
    "../../../docs/guide/enrollment.md"
);
guide_page!(feature = "marti", Marti, "../../../docs/guide/marti.md");
guide_page!(feature = "server", Server, "../../../docs/guide/server.md");
guide_page!(feature = "kmz", Convert, "../../../docs/guide/convert.md");
