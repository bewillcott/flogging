//
// File Name:    formatters.rs
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
//! Formatter
//!

#![allow(unused)]

use std::fmt;

use crate::logger::log_entry::LogEntry;

#[derive(Debug, Clone)]
pub(crate) enum Formatter {
    ///
    Iso8601,
    SimpleFormatter,
    UnixTimestamp,
}

impl Formatter {
    pub(crate) fn format(&self, log_entry: LogEntry) -> String {
        let fmt = match self {
            Formatter::Iso8601 => "%+".to_string(),
            Formatter::SimpleFormatter => "".to_string(), //"%c".to_string()
            Formatter::UnixTimestamp => "%s".to_string(),
        };

        let dt = log_entry.timestamp().format(&fmt);

        match self {
            Formatter::Iso8601 => format!("{dt} [{}] {}", log_entry.level(), log_entry.message()),
            Formatter::SimpleFormatter => {
                format!("[{}] {}", log_entry.level(), log_entry.message())
            }
            Formatter::UnixTimestamp => {
                format!("{dt}: [{}] {}", log_entry.level(), log_entry.message())
            }
        }
    }
}

impl fmt::Display for Formatter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::logger::level::Level;

    #[test]
    fn iso8601() {
        let le = LogEntry::new(Level::INFO, "This is a test message".to_string());
        let f = Formatter::Iso8601;
        let fs = f.format(le);
        println!("\n{fs}\n");
    }
    #[test]
    fn simple_formatter() {
        let le = LogEntry::new(Level::INFO, "This is a test message".to_string());
        let f = Formatter::SimpleFormatter;
        let fs = f.format(le);
        println!("\n{fs}\n");
    }
    #[test]
    fn unix_timestamp() {
        let le = LogEntry::new(Level::INFO, "This is a test message".to_string());
        let f = Formatter::UnixTimestamp;
        let fs = f.format(le);
        println!("\n{fs}\n");
    }
}
