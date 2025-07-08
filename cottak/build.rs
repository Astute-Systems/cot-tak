//
// Copyright (c) 2025, Astute Systems PTY LTD
//
// This file is part of the VivoeX SDK project developed by Astute Systems.
//
// See the commercial LICENSE file in the project root for full license details.
//

use std::fs;

pub fn update_cargo_version(filename: &str, version_number: &str) {
    // Update Cargo.toml
    let cargo_toml = fs::read_to_string(filename).expect("Unable to read Cargo.toml file");
    // Replace whole line if it contains version
    let new_cargo_toml = cargo_toml
        .lines()
        .map(|line| {
            // Must be start of line
            if line.starts_with("version =") {
                format!("version = \"{}\"", version_number)
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join("\n");

    // Write back to Cargo.toml
    fs::write(filename, new_cargo_toml).expect("Unable to write to Cargo.toml file");
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=path/to/Cargo.lock");
    protobuf_codegen::Codegen::new()
        .cargo_out_dir("protos")
        .include("src/proto")
        .input("src/proto/takmessage.proto")
        .input("src/proto/takcontrol.proto")
        .input("src/proto/cotevent.proto")
        .input("src/proto/contact.proto")
        .input("src/proto/group.proto")
        .input("src/proto/precisionlocation.proto")
        .input("src/proto/status.proto")
        .input("src/proto/takv.proto")
        .input("src/proto/track.proto")
        .input("src/proto/detail.proto")
        .run_from_script();
}
