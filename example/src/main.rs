#![doc(
    html_logo_url = "https://i0.wp.com/astutesys.com/wp-content/uploads/2024/02/cropped-img_5153.png"
)]
//
// Copyright (c) 2025, Astute Systems PTY LTD
//
// This file is part of the VivoeX SDK project developed by Astute Systems.
//
// See the commercial LICENSE file in the project root for full license details.
//
//! Really simple example of a CoT publisher
//!

mod basic;
mod bullseye;
mod casevac;
mod chat;
mod circle;
mod data_package;
mod measure;
mod sensor;

use cottak::config::Config;

use cottak::http_file_server::FileServer;

fn send_cot_items(animate: bool, send_data_package: bool) {
    // Create some CoT messages
    basic::building();
    bullseye::bullseye();
    circle::building();
    data_package::fileshare(send_data_package);
    chat::chat();
    measure::measure();
    sensor::camera(animate);
}

fn main() {
    println!("Example cursor on target (CoT) messages");
    let config = Config::new();
    println!(
        "  UDP SA Address {}:{}",
        config.udp.address, config.udp.port
    );
    // Print CoT library version
    println!(
        "  CoT Library Version: {}\n",
        cottak::host::get_cot_version()
    );

    // Create some CoT messages
    send_cot_items(true, true);

    // Send casuality evacuation message
    casevac::casevac();

    // Start the http server
    let address = format!(
        "{}:{}",
        config.general.http_address, config.general.http_port
    );
    let server = FileServer::new(address.as_str());

    println!(
        "HTTP data package server started on http://{}",
        config.get_address()
    );
    server.start().expect("Failed to start server");

    // Update at 1/2 stale time
    loop {
        std::thread::sleep(std::time::Duration::from_secs(
            config.general.stale as u64 / 2,
        ));
        send_cot_items(false, false);
        println!("Sent CoT messages");
    }
}
