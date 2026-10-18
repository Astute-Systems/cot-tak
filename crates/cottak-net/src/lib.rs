//! Async networking for TAK: a resilient TAK Server client and mesh (UDP) transport.
//!
//! * [`TakClient`] connects to a TAK Server over TCP or mutual TLS (rustls),
//!   upgrades from legacy XML to TAK Protocol v1 when the server offers it,
//!   sends keepalive pings, detects dead connections, and reconnects with
//!   jittered exponential backoff.
//! * [`mesh`] sends and receives Situational Awareness traffic over UDP
//!   multicast without a server.
//!
//! ```no_run
//! use cottak_core::{Event, Point};
//! use cottak_net::{ClientConfig, ClientEvent, TakClient, TlsConfig};
//!
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! let tls = TlsConfig::from_files(
//!     "ca.pem",
//!     Some(("client-cert.pem".as_ref(), "client-key.pem".as_ref())),
//! )?;
//! let config = ClientConfig::new("tak.example.com:8089", "ALPHA-1").with_tls(tls);
//! let (handle, mut events) = TakClient::start(config)?;
//!
//! handle
//!     .send(Event::new("ALPHA-1", "a-f-G-U-C".parse()?, Point::new(-27.47, 153.02)?))
//!     .await?;
//!
//! while let Some(event) = events.recv().await {
//!     if let ClientEvent::Event(ev) = event {
//!         println!("{} {}", ev.uid, ev.cot_type);
//!     }
//! }
//! # Ok(()) }
//! ```

mod client;
mod config;
mod connection;
pub mod error;
pub mod mesh;
mod tls;

pub use client::{ClientEvent, ClientEvents, ClientHandle, TakClient};
pub use config::{Backoff, ClientConfig, ProtocolPreference};
pub use connection::{Connection, Incoming, Protocol};
pub use error::{Error, Result};
pub use tls::{probe_server_certificate, ServerCertInfo, TlsConfig};
