//! A tiny TAK Server: accepts clients on 127.0.0.1:8087 and routes their
//! events to each other. Stop it with Ctrl-C.
//!
//! ```sh
//! cargo run -p cottak --example mock_server --features server
//! ```

use cottak::server::{Server, ServerConfig, ServerEvent};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8087".to_owned());
    let server = Server::start(ServerConfig::new().listen_tcp(addr)).await?;
    println!("listening on {:?}", server.local_addrs());

    let mut events = server.subscribe();
    loop {
        tokio::select! {
            _ = tokio::signal::ctrl_c() => break,
            item = events.recv() => match item {
                Ok(ServerEvent::Connected(c)) => println!("+ {} ({})", c.peer, c.id),
                Ok(ServerEvent::Disconnected(c)) => println!("- {} ({})", c.peer, c.id),
                Ok(ServerEvent::Event { from, event }) => {
                    println!("{} {} from {from:?}", event.cot_type, event.uid);
                }
                Ok(_) => {}
                Err(_) => break,
            },
        }
    }
    server.shutdown().await;
    Ok(())
}
