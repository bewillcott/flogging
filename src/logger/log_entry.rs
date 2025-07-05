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

use chrono::{DateTime, Local};

use super::level::Level;
// use anyhow::{Context, Error};
// use std::cell::{RefCell, RefMut};
use std::{fmt, time::Instant};
// use std::ops::Deref;
// use std::str::FromStr;
// use std::sync::Arc;
// use tokio::sync::mpsc;

#[derive(Debug)]
pub(crate) struct LogEntry {
    timestamp: DateTime<Local>,
    level: Level,
    message: String,
}

impl fmt::Display for LogEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} : ({}) {}", self.timestamp, self.level, self.message)
    }
}

impl LogEntry {
    pub(crate) fn new(level: Level, message: String) -> LogEntry {
        LogEntry {
            timestamp: Local::now(),
            level,
            message,
        }
    }

    pub(crate) fn timestamp(&self) -> DateTime<Local> {
        self.timestamp
    }

    pub(crate) fn level(&self)-> Level{
        self.level.clone()
    }

    pub(crate) fn message(&self)->String{
        self.message.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string() {
        let log_entry =
            LogEntry::new( Level::INFO, "message".to_string());

        let output = log_entry.to_string();
        println!("\noutput: {output}\n");
    }
}
