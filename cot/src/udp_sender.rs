//
// Copyright (c) 2025, Astute Systems PTY LTD
//
// This file is part of the VivoeX SDK project developed by Astute Systems.
//
// See the commercial LICENSE file in the project root for full license details.
//
//! Send data using UDP connection

use socket2::{Domain, Socket, Type};
use std::io::{Result, Write};
use std::net::{Ipv4Addr, SocketAddr};

/// Send data using UDP connection
/// * `socket` - UDP socket
/// * `addr` - IP address
pub struct UdpSender {
    socket: socket2::Socket,
    addr: String,
}

impl UdpSender {
    /// Create a new UDP sender
    /// # Arguments
    /// * `multicast_addr` - Multicast address
    /// * `port` - Port number
    /// # Returns
    /// * `Result` - Result of the operation
    /// # Example
    /// ```
    /// use cot::udp_sender::UdpSender;
    /// let sender = UdpSender::new("239.2.3.2", 6969);
    /// ```
    ///
    pub fn new(multicast_addr: &str, port: u16) -> Result<Self> {
        let addr_v4 = multicast_addr
            .parse::<Ipv4Addr>()
            .expect(format!("Invalid multicast address: {}", multicast_addr).as_str());
        let local_v4 = "0.0.0.0"
            .parse::<Ipv4Addr>()
            .expect("Invalid local address");

        let address: SocketAddr = format!("{}:{}", "0.0.0.0", port).parse().unwrap();
        let socket = Socket::new(Domain::IPV4, Type::DGRAM, None)?;

        // Allow the socket to be reused
        socket.set_reuse_address(true)?;
        // socket.set_reuse_port(true)?;
        socket.bind(&address.into())?;

        socket.set_multicast_loop_v4(true)?;
        socket.set_multicast_ttl_v4(1)?;
        socket.join_multicast_v4(&addr_v4, &local_v4)?;

        Ok(UdpSender {
            socket,
            addr: format!("{}:{}", multicast_addr.to_string(), port),
        })
    }

    /// Send data
    /// # Arguments
    /// * `message` - Data to send
    /// # Returns
    /// * `Result` - Result of the operation
    /// # Example
    /// ```
    /// use cot::udp_sender::UdpSender;
    /// let sender = UdpSender::new("239.2.3.1", 6969);
    /// sender.send(b"Hello, world!");
    /// ```
    ///
    pub fn send(&self, message: &[u8]) -> Result<usize> {
        let sock: SocketAddr = self.addr.parse().unwrap();
        self.socket.send_to(message, &sock.into())
    }

    pub fn flush(&mut self) {
        self.socket.flush().expect("Failed to flush UDP port")
    }

    /// Close the connection
    pub fn close(&self) {
        // self.socket
        //     .shutdown(std::net::Shutdown::Both)
        //     .expect("Failed to close socket");
    }
}
