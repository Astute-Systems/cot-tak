#![doc(html_favicon_url = "https://astutesys.com/favicon.ico")]
#![doc(
    html_logo_url = "https://i0.wp.com/astutesys.com/wp-content/uploads/2024/02/cropped-img_5153.png"
)]
#![doc(html_no_source)]
//
// Copyright (c) 2025, Astute Systems PTY LTD
//
// This file is part of the VivoeX SDK project developed by Astute Systems.
//
// See the commercial LICENSE file in the project root for full license details.
//
//! ![TAK Logo](https://astute-systems.github.io/CotX/images/astute_tak_logo.png)
//!
//! # CoT Library
//!
//! CoT is a library for sending Cursor on Target (CoT) messages over various transport protocols.
//!
//! ![TAK Brisbane](https://astute-systems.github.io/CotX/images/tak_brisbane.png)
//! **Example TAK displaying GTFS Realtime data for Brisbane**
//!
//! ## Cursor on Target (CoT) Messages
//!
//! Cursor on Target (CoT) is a data format used for real-time data exchange between systems.
//! It is primarily used in military and emergency response applications to share situational
//! awareness information. CoT messages are XML-based and contain information about events,
//! such as the location, type, and time of the event.
//!
//! ## Usage with TAK Servers and Clients
//!
//! TAK (Tactical Assault Kit) servers and clients use CoT messages to communicate and share
//! situational awareness data. TAK servers aggregate CoT messages from various sources and
//! distribute them to connected clients. TAK clients, such as ATAK (Android Team Awareness Kit),
//! display the received CoT messages on a map, providing users with real-time situational awareness.
//!
//! This library provides the necessary tools to create, send, and receive CoT messages over
//! different transport protocols, enabling seamless integration with TAK servers and clients.
//!
//! ## Example
//!
//! To send a CoT message using this library, you can follow the example below:
//!
//! ### XML
//!
//! ``` rust
//! use cot::config::Config;
//! use cot::cot::{CursorOnTarget, Point};
//! use cot::cot_types::{CoTType, CotClassification};
//! use cot::udp_sender::UdpSender;
//!
//! // Brisbane City Hall
//! const LONGITUDE: f64 = 153.02351661489064;
//! const LATITUDE: f64 = -27.46891737509902;
//!
//! // ...
//!
//! // Read the configuration file
//! let config = Config::from_file(&Config::get_config_file_path()).expect("Failed to read config file");
//!
//! // Setup classification of the CoT message
//! let cot_type = cot::cot_types::lookup_cot_type(CoTType::GndBuilding, CotClassification::Friend);
//!
//! // Brisbane City Hall
//! let cot = CursorOnTarget::new(
//!     Some("No Remarks".to_string()),                    // remarks
//!     Some("CITY_#89436".to_string()),                   // uid
//!     Some("BRISBANE_CITY_HALL".to_string()),            // callsign
//!     Some(cot_type.to_string()),                        // type
//!     Point {
//!        longitude: LATITUDE,                            // latitude
//!        latitude: LONGITUDE,                            // longitude
//!        hae: cot::cot::HAE_NONE,                        // hae
//!        ce: cot::cot::CE_NONE,                          // ce
//!        le: cot::cot::LE_NONE,                          // le
//!     },
//!     false,                                             // military
//!     Some("".to_string()),                              // squawk
//!     Some(0.0),                                         // speed
//!     Some(0.0),                                         // battery
//!     None,                                              // link
//!     Some(0.0),                                         // heading
//!     Some(0.0),                                         // vertical_rate
//!     Some(0),                                           // last_seen
//!     None,                                              // video
//! );
//!
//! // Create a UDP sender
//! let xml = UdpSender::new(&config.udp.address as &str, config.udp.port)
//!     .expect("Couldn't setup multicast");
//!
//! // Send the CoT message
//! xml.send(&cot.get_xml_bytes()).expect("Failed to send CoT message");
//! ```
//! UDP sender is shown above but it is also possible to use [tcp_sender::TcpSender] for TCL or [ssl_sender::SslSender] as the transport protocol.
//! XML is compliant to version 1 of the CoT schema. You can use version 2 (protobuf) by
//! requesting the protobuf method on the cot object and sending the bytes to the proto_sender.
//!
//! ### Protobuf
//!
//! Example using protobuf sender is the same as for XML example above except we use the [proto_sender::ProtoSender] instead of the UDP sender.
//!
//! ``` compile_fail ignore
//! // Create a Protobuf sender
//! let mut proto = ProtoSender::new(&config.udp.address as &str, config.udp.port)
//!     .expect("Couldn't setup multicast");
//!
//! // Send the CoT message
//! proto.send(&cot).expect("Failed to send CoT message");
//! ```
//! ## More information
//!
//! * WinTAK [download](https://www.civtak.org/download-atak) for civilian applications (Windows)
//! * ATAK-CIV for [Android devices](https://play.google.com/store/apps/details?id=com.atakmap.app.civ)
//! * iTAK for [iOS devices](https://apps.apple.com/us/app/itak/id1561656396)
//!

pub mod chat;
pub mod config;
pub mod cot;
pub mod cot_base;
pub mod cot_bullseye;
pub mod cot_casevac;
pub mod cot_circle;
pub mod cot_geofence;
pub mod cot_measure;
pub mod cot_navigation;
pub mod cot_sensor;
pub mod cot_types;
pub mod host;
pub mod http_file_server;
pub mod nmea;
pub mod proto_sender;
pub mod serial_port;
pub mod ssl_sender;
pub mod tcp_sender;
mod tests;
pub mod time;
pub mod udp_receiver;
pub mod udp_sender;
pub mod version;
// pub mod push_cot;
pub mod data_package;

include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));
