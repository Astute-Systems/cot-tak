//
// Copyright (c) 2025, Astute Systems PTY LTD
//
// This file is part of the VivoeX SDK project developed by Astute Systems.
//
// See the commercial LICENSE file in the project root for full license details.
//
//! Send and receive serial data using a serial connection

use serialport::{DataBits, FlowControl, Parity, SerialPort, SerialPortBuilder, StopBits};
use std::io::Result;

/// Serial port connection
/// * `port` - Serial port name
/// * `baud` - Baud rate
///
pub struct Serial {
    // Serial port settings
    serial_port: SerialPortBuilder,
    open_port: Option<Box<dyn SerialPort>>,
}

impl Serial {
    pub fn new(
        port: String,
        baud: u32,
        data_bits: u8,
        parity: u8,
        stop_bits: u8,
        flow_control: u8,
    ) -> Result<Self> {
        let serial_port = serialport::new(&port, baud)
            .data_bits(match data_bits {
                5 => DataBits::Five,
                6 => DataBits::Six,
                7 => DataBits::Seven,
                8 => DataBits::Eight,
                _ => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "Invalid data bits",
                    ))
                }
            })
            .parity(match parity {
                0 => Parity::None,
                1 => Parity::Odd,
                2 => Parity::Even,
                _ => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "Invalid parity",
                    ))
                }
            })
            .stop_bits(match stop_bits {
                1 => StopBits::One,
                2 => StopBits::Two,
                _ => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "Invalid stop bits",
                    ))
                }
            })
            .flow_control(match flow_control {
                0 => FlowControl::None,
                1 => FlowControl::Software,
                2 => FlowControl::Hardware,
                _ => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "Invalid flow control",
                    ))
                }
            });

        Ok(Serial {
            serial_port,
            open_port: None,
        })
    }

    /// Open the serial port
    /// # Arguments
    /// * `self` - The serial port
    /// # Returns
    /// * `Result<()>` - Result of opening the serial port
    /// # Example
    /// ```
    /// let mut serial = Serial::new("/dev/ttyUSB0".to_string(), 9600, 8, 0, 1, 0).unwrap();
    /// serial.open().unwrap();
    /// ```
    ///
    pub fn open(&mut self) -> Result<()> {
        self.open_port = Some(self.serial_port.clone().open()?);
        Ok(())
    }

    /// Send data over the serial port
    /// # Arguments
    /// * `self` - The serial port
    /// * `message` - The message to send
    /// # Returns
    /// * `Result<usize>` - Result of sending the message
    /// # Example
    /// ```
    /// let mut serial = Serial::new("/dev/ttyUSB0".to_string(), 9600, 8, 0, 1, 0).unwrap();
    /// serial.open().unwrap();
    /// serial.send(b"Hello World").unwrap();
    /// ```
    ///
    pub fn send(&mut self, message: &[u8]) -> Result<usize> {
        if self.open_port.is_none() {
            self.open()?;
        }
        if let Some(ref mut port) = self.open_port {
            port.write(message)
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to open port",
            ))
        }
    }

    /// Receive data over the serial port
    /// # Arguments
    /// * `self` - The serial port
    /// * `message` - The message to receive
    /// # Returns
    /// * `Result<usize>` - Result of receiving the message
    /// # Example
    /// ```
    /// let mut serial = Serial::new("/dev/ttyUSB0".to_string(), 9600, 8, 0, 1, 0).unwrap();
    /// serial.open().unwrap();
    /// let mut message = [0; 1024];
    /// serial.receive(&mut message).unwrap();
    /// ```
    ///
    pub fn receive(&mut self, message: &mut [u8]) -> Result<usize> {
        if self.open_port.is_none() {
            self.open()?;
        }
        if let Some(ref mut port) = self.open_port {
            port.read(message)
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to open port",
            ))
        }
    }
}
