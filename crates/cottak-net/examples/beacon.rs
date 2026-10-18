//! Sends a position report every 10 seconds and prints everything received.
//!
//! ```text
//! cargo run -p cottak-net --example beacon -- tak.example.com:8087
//! cargo run -p cottak-net --example beacon -- tak.example.com:8089 ca.pem client.pem client.key
//! ```

use cottak_core::detail::{Contact, Track};
use cottak_core::{Event, Point};
use cottak_net::{ClientConfig, ClientEvent, TakClient, TlsConfig};
use std::path::Path;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let addr = args
        .next()
        .ok_or("usage: beacon <host:port> [ca.pem client.pem client.key]")?;
    let uid = "cottak-example-1";

    let mut config = ClientConfig::new(addr, uid);
    if let (Some(ca), Some(cert), Some(key)) = (args.next(), args.next(), args.next()) {
        config = config.with_tls(TlsConfig::from_files(
            &ca,
            Some((Path::new(&cert), Path::new(&key))),
        )?);
    }
    let (handle, mut events) = TakClient::start(config)?;

    let sender = handle.clone();
    tokio::spawn(async move {
        let mut tick = tokio::time::interval(Duration::from_secs(10));
        loop {
            tick.tick().await;
            let Ok(point) = Point::new(-27.4698, 153.0251) else {
                return;
            };
            let Ok(cot_type) = "a-f-G-U-C".parse() else {
                return;
            };
            let event = Event::new(uid, cot_type, point)
                .with(Contact::new("COTTAK-EXAMPLE"))
                .with(Track::new(0.0, 0.0));
            if sender.send(event).await.is_err() {
                return;
            }
        }
    });

    while let Some(event) = events.recv().await {
        match event {
            ClientEvent::Connected { protocol } => println!("connected ({protocol:?})"),
            ClientEvent::ProtocolChanged(p) => println!("upgraded to {p:?}"),
            ClientEvent::Event(ev) => println!("{} {} from {}", ev.cot_type, ev.uid, ev.how),
            ClientEvent::Invalid { reason } => eprintln!("ignored invalid message: {reason}"),
            ClientEvent::Disconnected { reason, retry_in } => {
                eprintln!("disconnected: {reason}; retrying in {retry_in:?}");
            }
            _ => {}
        }
    }
    Ok(())
}
