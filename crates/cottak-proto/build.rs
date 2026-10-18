//! Compiles the TAK Protocol v1 `.proto` files with a pure-Rust compiler
//! (`protox`), so building needs no `protoc` binary.

use std::error::Error;

const PROTOS: &[&str] = &[
    "takmessage.proto",
    "takcontrol.proto",
    "cotevent.proto",
    "detail.proto",
    "contact.proto",
    "group.proto",
    "precisionlocation.proto",
    "status.proto",
    "takv.proto",
    "track.proto",
];

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=proto");
    let fds = protox::compile(PROTOS, ["proto"])?;
    prost_build::Config::new().compile_fds(fds)?;
    Ok(())
}
