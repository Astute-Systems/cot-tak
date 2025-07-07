//
// Copyright (c) 2025, Astute Systems PTY LTD
//
// This file is part of the VivoeX SDK project developed by Astute Systems.
//
// See the commercial LICENSE file in the project root for full license details.
//
//! Send data using TCP connection

use std::io::{Read, Result, Write};
use std::net::TcpStream;

/// Send data using TCP connection
/// * `addr` - IP address
/// * `port` - Port number
///
pub struct TcpSender {
    stream: TcpStream,
}

#[allow(dead_code)]
impl TcpSender {
    pub fn new(addr: &str, port: u16) -> Result<Self> {
        let stream = TcpStream::connect(format!("{}:{}", addr, port))?;
        Ok(TcpSender { stream })
    }

    /// Send data
    /// # Arguments
    /// * `message` - Data to send
    /// # Returns
    /// * `Result` - Result of the operation
    ///
    pub fn send(&mut self, message: &[u8]) -> Result<()> {
        self.stream.write_all(message)?;
        self.stream.flush()?;
        Ok(())
    }

    /// Receive data
    /// # Arguments
    /// * `buffer` - Buffer to store received data
    /// # Returns
    /// * `Result` - Result of the operation
    ///
    pub fn receive(&mut self, buffer: &mut [u8]) -> Result<()> {
        self.stream.read(buffer)?;
        Ok(())
    }

    /// Close the connection
    /// # Returns
    /// * `Result` - Result of the operation
    ///
    pub fn close(&mut self) -> Result<()> {
        self.stream.shutdown(std::net::Shutdown::Both)?;
        Ok(())
    }
}
