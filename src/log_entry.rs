//
// File Name:    log_entry.rs
// Project Name: logging
//
// Copyright (C) 2025 Bradley Willcott
//
// This library (crate) is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This library (crate) is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this library (crate).  If not, see <https://www.gnu.org/licenses/>.
//
//!
//! Log Entry
//!

// #![allow(dead_code)]

use super::level::Level;
// use anyhow::{Context, Error};
// use std::cell::{RefCell, RefMut};
use std::fmt;
// use std::ops::Deref;
// use std::str::FromStr;
// use std::sync::Arc;
// use tokio::sync::mpsc;

#[derive(Debug)]
pub(super) struct LogEntry {
    timestamp: String,
    level: Level,
    message: String,
}

impl fmt::Display for LogEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} : ({}) {}", self.timestamp, self.level, self.message)
    }
}

impl LogEntry {
    pub(super) fn build(timestamp: String, level: Level, message: String) -> LogEntry {
        LogEntry {
            timestamp,
            level,
            message,
        }
    }

    pub(super) fn timestamp(&self) -> String {
        self.timestamp.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string() {
        let log_entry =
            LogEntry::build("timestamp".to_string(), Level::INFO, "message".to_string());

        let output = log_entry.to_string();
        println!("\noutput: {output}\n");
    }
}
