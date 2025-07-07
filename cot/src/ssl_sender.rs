//
// Copyright (c) 2025, Astute Systems PTY LTD
//
// This file is part of the VivoeX SDK project developed by Astute Systems.
//
// See the commercial LICENSE file in the project root for full license details.
//
//! Send data using SSL secure connection

use native_tls::{TlsConnector, TlsStream};
use std::io::{Result, Write};
use std::net::TcpStream;

/// Send data using SSL secure connection
/// * `addr` - IP address
pub struct SslSender {
    stream: TlsStream<TcpStream>,
}

#[allow(dead_code)]
impl SslSender {
    pub fn new(addr: &str, port: u16) -> Result<Self> {
        let connector = TlsConnector::new().expect("Couldn't create TLS connector");
        let stream = TcpStream::connect(format!("{}:{}", addr, port))?;
        let stream = connector
            .connect(addr, stream)
            .expect("Couldn't establish TLS connection");

        Ok(SslSender { stream })
    }

    /// Send data
    /// # Arguments
    /// * `message` - Data to send
    ///
    pub fn send(&mut self, message: &[u8]) -> Result<()> {
        self.stream.write_all(message)?;
        self.stream.flush()?;
        Ok(())
    }
}
