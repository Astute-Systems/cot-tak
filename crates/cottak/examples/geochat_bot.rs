//! A GeoChat bot: answers "ping" with "pong" and echoes everything else.
//!
//! ```text
//! cargo run -p cottak --example geochat_bot --features net -- tak.example.com:8087 BOT-1 "Echo Bot"
//! ```
//! Add `ca.pem client.pem client.key` after the callsign to use mutual TLS on 8089.

use cottak::core::detail::Contact;
use cottak::core::messages::chat::GeoChat;
use cottak::core::{Event, Point};
use cottak::net::{ClientConfig, ClientEvent, TakClient, TlsConfig};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let (Some(addr), Some(uid), Some(callsign)) = (args.next(), args.next(), args.next()) else {
        return Err(
            "usage: geochat_bot <host:port> <uid> <callsign> [ca.pem client.pem client.key]".into(),
        );
    };
    let mut config = ClientConfig::new(addr, uid.clone());
    if let (Some(ca), Some(cert), Some(key)) = (args.next(), args.next(), args.next()) {
        config = config.with_tls(TlsConfig::from_files(
            &ca,
            Some((Path::new(&cert), Path::new(&key))),
        )?);
    }
    let (handle, mut events) = TakClient::start(config)?;

    // Announce ourselves so clients show the bot on the map and can address it.
    let presence = Event::new(uid.clone(), "a-f-G-U-C".parse()?, Point::new(0.0, 0.0)?)
        .with(Contact::new(callsign.clone()));
    handle.send(presence).await?;

    while let Some(event) = events.recv().await {
        let ClientEvent::Event(ev) = event else {
            continue;
        };
        let Some(msg) = GeoChat::from_event(&ev) else {
            continue;
        };
        if msg.sender_uid == uid {
            continue; // our own message coming back
        }
        let text = if msg.text.trim().eq_ignore_ascii_case("ping") {
            "pong".to_owned()
        } else {
            format!("echo: {}", msg.text)
        };
        let reply = if msg.is_broadcast() {
            GeoChat::to_all(&uid, &callsign, text)
        } else {
            GeoChat::direct(&uid, &callsign, &msg.sender_uid, &msg.sender_callsign, text)
        };
        handle.send(reply.to_event()?).await?;
    }
    Ok(())
}
