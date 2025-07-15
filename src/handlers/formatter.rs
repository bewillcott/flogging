//
// File Name:    formatter.rs
// Project Name: flogging
//
// Copyright (C) 2025 Bradley Willcott
//
// SPDX-License-Identifier: GPL-3.0-or-later
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
//! # Formatter
//!

#![allow(unused)]

use crate::logger::LogEntry;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Formatter {
    Iso8601,
    Simple,
    UnixTimestamp,
}

impl Formatter {
    pub(crate) fn format(&self, log_entry: &LogEntry) -> String {
        let fmt = match self {
            Formatter::Iso8601 => "%+".to_string(),
            Formatter::Simple => "".to_string(), //"%c".to_string()
            Formatter::UnixTimestamp => "%s.%f".to_string(),
        };

        let dt = log_entry.timestamp().format(&fmt);

        match self {
            Formatter::Iso8601 => format!(
                "{dt:35} |{}->{}| [{:7}] {}",
                log_entry.mod_path(),
                log_entry.fn_name(),
                log_entry.level(),
                log_entry.message()
            ),
            Formatter::Simple => {
                format!(
                    "|{}->{}| [{:7}] {}",
                    log_entry.mod_path(),
                    log_entry.fn_name(),
                    log_entry.level(),
                    log_entry.message()
                )
            }
            Formatter::UnixTimestamp => {
                format!(
                    "{dt} |{}->{}| [{:7}] {}",
                    log_entry.mod_path(),
                    log_entry.fn_name(),
                    log_entry.level(),
                    log_entry.message()
                )
            }
        }
    }

    pub(crate) fn width(&self) -> usize {
        15
    }
}

impl fmt::Display for Formatter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Formatter::Iso8601 => "Iso8601",
            Formatter::Simple => "SimpleFormatter",
            Formatter::UnixTimestamp => "UnixTimestamp",
        };

        label.fmt(f)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Level::*;

    #[test]
    fn iso8601() {
        let le = LogEntry::create(
            INFO,
            "iso8601".to_string(),
            "This is a test message".to_string(),
        );
        let f = Formatter::Iso8601;
        let fs = f.format(&le);
        println!("\n{f:width$} {fs}\n", width = f.width());
    }
    #[test]
    fn simple_formatter() {
        let le = LogEntry::create(
            INFO,
            "simple_formatter".to_string(),
            "This is a test message".to_string(),
        );
        let f = Formatter::Simple;
        let fs = f.format(&le);
        println!("\n{f:width$} {fs}\n", width = f.width());
    }

    #[test]
    fn unix_timestamp() {
        let le = LogEntry::create(
            INFO,
            "unix_timestamp".to_string(),
            "This is a test message".to_string(),
        );
        let f = Formatter::UnixTimestamp;
        let fs = f.format(&le);
        println!("\n{f:width$} {fs}\n", width = f.width());
    }
}
