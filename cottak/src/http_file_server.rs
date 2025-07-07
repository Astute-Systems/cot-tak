//
// Copyright (c) 2025, Astute Systems PTY LTD
//
// This file is part of the VivoeX SDK project developed by Astute Systems.
//
// See the commercial LICENSE file in the project root for full license details.
//
//! This module contains a simple HTTP file server.
//!

use crate::config::Config;
use std::fs::File;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let config = Config::get_instance();

    let get = b"GET /";
    if buffer.starts_with(get) {
        let request_line = String::from_utf8_lossy(&buffer[..]);
        let file_path = request_line.split_whitespace().nth(1).unwrap();
        let file_path = &file_path[1..]; // Remove the leading '/'

        // Prepend the config path to file_path
        let file_path = format!("{}/{}", config.lock().unwrap().general.filestore, file_path);
        let file_path = file_path.as_str();

        println!("Request for file: {}", file_path);
        // Strip ? onwards
        let stripped_path = file_path.split('?').next().unwrap();
        if Path::new(stripped_path).exists() {
            let mut file = File::open(stripped_path).unwrap();
            let mut contents = Vec::new();
            file.read_to_end(&mut contents).unwrap();

            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n",
                contents.len()
            );
            stream.write(response.as_bytes()).unwrap();
            stream.write(&contents).unwrap();
        } else {
            let response = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
            stream.write(response.as_bytes()).unwrap();
        }
    } else {
        let response = "HTTP/1.1 400 BAD REQUEST\r\n\r\n";
        stream.write(response.as_bytes()).unwrap();
    }

    stream.flush().unwrap();
}

pub struct FileServer {
    address: String,
    running: Arc<AtomicBool>,
}

impl FileServer {
    pub fn new(address: &str) -> Self {
        FileServer {
            address: address.to_string(),
            running: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn start(&self) -> std::io::Result<()> {
        let listener = TcpListener::bind(&self.address)?;
        let running = self.running.clone();
        running.store(true, Ordering::SeqCst);

        thread::spawn(move || {
            for stream in listener.incoming() {
                if !running.load(Ordering::SeqCst) {
                    break;
                }
                match stream {
                    Ok(stream) => {
                        thread::spawn(|| {
                            handle_client(stream);
                        });
                    }
                    Err(e) => {
                        eprintln!("Connection failed: {}", e);
                    }
                }
            }
        });

        Ok(())
    }

    pub fn stop(&self) {
        self.running.store(false, Ordering::SeqCst);
    }
}
