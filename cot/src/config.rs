//
// Copyright (c) 2025, Astute Systems PTY LTD
//
// This file is part of the VivoeX SDK project developed by Astute Systems.
//
// See the commercial LICENSE file in the project root for full license details.
//
//! Configuration module (read from TOML)

use once_cell::sync::Lazy;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::sync::{Arc, Mutex};

// Default path to the configuration file
pub const DEFAULT_CONFIG_FILE: &str = "/etc/cot/config.toml";

#[derive(Debug, Deserialize, Clone)]
pub struct General {
    pub uuid: String,
    pub stale: i64,
    pub filestore: String,
    pub http_server: bool,
    pub http_address: String,
    pub http_port: u16,
    pub callsign: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub general: General,
    pub chat: Connection,
    pub udp: Connection,
    pub tcp: Connection,
    pub ssl: Connection,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Connection {
    pub address: String,
    pub port: u16,
}

impl Config {
    pub fn new() -> Self {
        // Check if /etc/cot/config.toml exists
        let config_file = Config::get_config_file_path();
        if config_file.is_empty() {
            return Config {
                // Just use the defaults
                general: General {
                    uuid: "test_uuid_5275492".to_string(),
                    stale: 120,
                    filestore: "/tmp".to_string(),
                    http_server: true,
                    http_address: "localhost".to_string(),
                    http_port: 8080,
                    callsign: "test_callsign".to_string(),
                },
                chat: Connection {
                    address: "244.10.10.1".to_string(),
                    port: 17012,
                },
                udp: Connection {
                    address: "239.2.3.1".to_string(),
                    port: 6969,
                },
                tcp: Connection {
                    address: "192.168.1.100".to_string(),
                    port: 7979,
                },
                ssl: Connection {
                    address: "192.168.1.101".to_string(),
                    port: 8989,
                },
            };
        } else {
            return Config::from_file(&config_file).expect("Failed to read config file");
        }
    }

    pub fn from_file(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Check the file exists
        if !std::path::Path::new(file_path).exists() {
            // Include the file path in the error message
            return Err(From::from(format!(
                "Configuration file not found: \"{}\"",
                file_path
            )));
        }
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        // Read from TOML
        let config: Config = toml::from_str(&contents)?;

        Ok(config)
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        println!("UDP: {:?}", self.udp);
        println!("TCP: {:?}", self.tcp);
        println!("SSL: {:?}", self.ssl);
    }

    pub fn clone(&self) -> Self {
        Config {
            general: self.general.clone(),
            chat: self.chat.clone(),
            udp: self.udp.clone(),
            tcp: self.tcp.clone(),
            ssl: self.ssl.clone(),
        }
    }

    pub fn get_config_file_path() -> String {
        // Check env COT_CONFIG
        if let Ok(config_file) = std::env::var("COT_CONFIG") {
            return config_file;
        }

        // Check if the default file exists
        if std::path::Path::new(DEFAULT_CONFIG_FILE).exists() {
            return DEFAULT_CONFIG_FILE.to_string();
        }

        // Get the filename from the default path
        let filename = std::path::Path::new(DEFAULT_CONFIG_FILE)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();

        // Check if the file exists in the current directory
        if std::path::Path::new(filename).exists() {
            return filename.to_string();
        }

        // Check to se eif its in the executable working directory
        if let Ok(mut path) = std::env::current_exe() {
            path.pop();
            path.push(filename);
            if path.exists() {
                return path.to_str().unwrap().to_string();
            }
        }

        // Return error
        "".to_string()
    }

    pub fn get_address(&self) -> String {
        format!("{}:{}", self.general.http_address, self.general.http_port)
    }

    pub fn get_instance() -> Arc<Mutex<Config>> {
        static INSTANCE: Lazy<Arc<Mutex<Config>>> = Lazy::new(|| {
            let config = Config::from_file(&Config::get_config_file_path())
                .expect("Failed to read config file");
            Arc::new(Mutex::new(config))
        });
        INSTANCE.clone()
    }
}
