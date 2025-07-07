//
// Copyright (c) 2025, Astute Systems PTY LTD
//
// This file is part of the VivoeX SDK project developed by Astute Systems.
//
// See the commercial LICENSE file in the project root for full license details.
//

use cottak::chat;
use cottak::config::Config;
use cottak::proto_sender::ProtoSender;

pub fn chat() {
    let config = Config::new();

    let chat = chat::Chat::new(
        "Red Leader".to_string(),
        "All Chat Rooms".to_string(),
        "Hello World".to_string(),
    );

    let mut sender = ProtoSender::new(&config.chat.address as &str, config.chat.port)
        .expect("Couldn't setup multicast");

    sender.send(&chat);

    // println!("{}", chat.get_xml());
}
