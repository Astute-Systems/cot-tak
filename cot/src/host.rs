//
// Copyright (c) 2025, Astute Systems PTY LTD
//
// This file is part of the VivoeX SDK project developed by Astute Systems.
//
// See the commercial LICENSE file in the project root for full license details.
//
//! This module contains system information functions.

use crate::version::{MAJOR, MINOR, PATCH, SUFFIX};
use hostname::get;
use whoami;

/// Get the host computer name and user as user@host
/// # Example
/// ```
/// use cot::host::get_known_host_id;
/// let host_id = get_known_host_id();
/// ```
///
pub fn get_known_host_id() -> String {
    // Get the host computer name and user as user@host
    let host = get()
        .expect("Failed to get hostname")
        .into_string()
        .expect("Failed to convert hostname to string");
    let user = whoami::username();
    format!("{}@{}", user, host)
}

/// Get the host computer name
/// # Example
/// ```
/// use cot::host::get_host_name;
/// let host_name = get_host_name();
/// ```
///
pub fn get_os() -> String {
    // Check if windows, macOS, or other Unix-like systems
    if cfg!(windows) {
        return "Windows".to_string();
    }
    if cfg!(target_os = "macos") {
        return "macOS".to_string();
    }
    if cfg!(unix) {
        return "Unix".to_string();
    }
    "Unknown".to_string()
}

/// Get the host computer name
/// # Example
/// ```
/// use cot::host::get_host_name;
/// let host_name = get_host_name();
/// ```
///
pub fn get_os_version() -> String {
    // Read /etc/os-release to get the OS version

    // Extract VERSION_ID=? from /etc/os-release
    let os_release =
        std::fs::read_to_string("/etc/os-release").expect("Failed to read /etc/os-release");
    let version_id = os_release
        .lines()
        .find(|line| line.starts_with("VERSION_ID="))
        .expect("Failed to find VERSION_ID in /etc/os-release");

    version_id.to_string()
}

/// Get the host computer name
/// # Example
/// ```
/// use cot::host::get_host_name;
/// let host_name = get_host_name();
/// ```
///
pub fn get_cot_version() -> String {
    // Read the version file
    let version = format!("{}.{}.{}{}", MAJOR, MINOR, PATCH, SUFFIX);
    version
}
