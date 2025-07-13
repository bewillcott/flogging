//
// File Name:    log_entry.rs
// Project Name: flogging
//
// Copyright (C) 2025 Bradley Willcott
//
// SPDX-License-Identifier: GPL-2.0-or-later
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
//! # Log Entry
//!

use chrono::{DateTime, Local};

use super::level::Level;
use std::{fmt, time::Instant};

#[derive(Debug)]
pub(crate) struct LogEntry {
    timestamp: DateTime<Local>,
    mod_path: String,
    ///
    /// This is the name of the function/method inside which this
    /// log message was generated.
    ///
    fn_name: String,
    level: Level,
    message: String,
}

impl fmt::Display for LogEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} : {}->{} ({}) {}",
            self.timestamp, self.mod_path, self.fn_name, self.level, self.message
        )
    }
}

impl LogEntry {
    pub(crate) fn create(level: Level, fn_name: String, message: String) -> LogEntry {
        LogEntry {
            timestamp: Local::now(),
            mod_path: String::new(),
            fn_name,
            level,
            message,
        }
    }

    pub(crate) fn level(&self) -> Level {
        self.level.clone()
    }

    pub(crate) fn message(&self) -> String {
        self.message.clone()
    }

    pub(crate) fn fn_name(&self) -> String {
        self.fn_name.clone()
    }

    pub(crate) fn mod_path(&self) -> String {
        self.mod_path.clone()
    }

    pub(crate) fn set_fn_name(&mut self, fn_name: String) {
        self.fn_name = fn_name.clone();
    }

    pub(crate) fn set_mod_path(&mut self, mod_path: String) {
        self.mod_path = mod_path.clone();
    }

    pub(crate) fn timestamp(&self) -> DateTime<Local> {
        self.timestamp
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string() {
        let log_entry =
            LogEntry::create(Level::INFO, "to_string".to_string(), "message".to_string());

        let output = log_entry.to_string();
        println!("\noutput: {output}\n");
    }
}
