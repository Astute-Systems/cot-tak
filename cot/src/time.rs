//
// Copyright (c) 2025, Astute Systems PTY LTD
//
// This file is part of the VivoeX SDK project developed by Astute Systems.
//
// See the commercial LICENSE file in the project root for full license details.
//
//! Time utility and helper functions

use chrono::prelude::*;
use chrono::{DateTime, Utc};

pub struct Time {}

impl Time {
    /// Get the current time in seconds since the Unix epoch
    /// # Returns
    /// * `i64` - Current time in seconds since the Unix epoch
    /// # Example
    /// ```
    /// use cot::time::Time;
    /// let current_time = Time::now();
    /// ```
    ///
    pub fn now() -> i64 {
        let current_utc: DateTime<Utc> = Utc::now();
        current_utc.timestamp() as i64
    }

    pub fn now_millis() -> i64 {
        let current_utc: DateTime<Utc> = Utc::now();
        current_utc.timestamp_millis()
    }

    /// Get the current time in milliseconds since the Unix epoch
    /// # Returns
    /// * `i64` - Current time in milliseconds since the Unix epoch
    /// # Example
    /// ```
    /// use cot::time::Time;
    /// let current_time = Time::now_millis();
    /// ```
    ///
    pub fn stale_time() -> i64 {
        let stale: i64 = Time::now() + 120;
        stale
    }

    pub fn stale_time_millis() -> i64 {
        // Add 120 seconds to the current time
        let stale: i64 = Time::now_millis() + 120000;
        stale
    }

    /// Convert time in seconds since the Unix epoch to a string
    /// # Arguments
    /// * `time` - Time in seconds since the Unix epoch
    /// # Returns
    /// * `String` - Time in string format
    /// # Example
    /// ```
    /// use cot::time::Time;
    /// let time = 1611770000;
    /// let time_string = Time::time_to_string(time);
    /// ```
    ///
    pub fn time_to_string(time: i64) -> String {
        let current_utc: DateTime<Utc> = DateTime::from_timestamp(time, 0).unwrap();
        let formatted = current_utc.format("%Y-%m-%dT%H:%M:%S%.6fZ");
        formatted.to_string()
    }

    /// Get the current time in string format
    /// # Returns
    /// * `String` - Current time in string format
    /// # Example
    /// ```
    /// use cot::time::Time;
    /// let current_time = Time::now_string();
    /// ```
    ///
    pub fn now_string() -> String {
        let current_utc: DateTime<Local> = Local::now();
        // Local as 2025-01-27T06:18:18Z

        let formatted = current_utc.format("%Y-%m-%dT%H:%M:%S%.6fZ");
        formatted.to_string()
    }
}
