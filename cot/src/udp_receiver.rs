//
// Copyright (c) 2025, Astute Systems PTY LTD
//
// This file is part of the VivoeX SDK project developed by Astute Systems.
//
// See the commercial LICENSE file in the project root for full license details.
//
//! Receive data using UDP connection

use std::io::Result;
use std::net::{Ipv4Addr, UdpSocket};

/// Receive data using UDP connection
/// * `multicast_addr` - Multicast address
/// * `local_addr` - Local address
///
pub struct UdpReceiver {
    socket: UdpSocket,
    multicast_addr: Ipv4Addr,
}

impl UdpReceiver {
    /// Create a new UDP receiver
    /// # Arguments
    /// * `multicast_addr` - Multicast address
    /// * `local_addr` - Local address
    /// * `port` - Port number
    /// # Returns
    /// * `Result` - Result of the operation
    /// # Example
    /// ```
    /// use cot::udp_receiver::UdpReceiver;
    /// let receiver = UdpReceiver::new("239.2.3.1", "0.0.0.0", 6969);
    /// ```
    ///
    pub fn new(multicast_addr: &str, local_addr: &str, port: u16) -> Result<Self> {
        let socket = UdpSocket::bind((local_addr, port))?;
        let multicast_addr: Ipv4Addr = multicast_addr.parse().expect("Invalid multicast address");
        let local_addr: Ipv4Addr = local_addr.parse().expect("Invalid local address");

        socket.join_multicast_v4(&multicast_addr, &local_addr)?;

        Ok(UdpReceiver {
            socket,
            multicast_addr,
        })
    }

    /// Receive data
    /// # Arguments
    /// * `message` - Buffer to store received data
    /// # Returns
    /// * `Result` - Result of the operation
    ///
    pub fn receive(&self, message: &mut [u8]) -> Result<usize> {
        let (size, _src) = self.socket.recv_from(message).expect(
            "Couldn't receive message, do you have a default gw set for the multicast address?",
        );
        Ok(size)
    }

    pub fn multicast_addr(&self) -> Ipv4Addr {
        self.multicast_addr
    }
}
