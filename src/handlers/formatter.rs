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

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Formatter {
    /// ISO 8601 / RFC 3339 date & time format.
    ///
    /// Example:
    /// ```text
    /// 2001-07-08T00:34:60.026490+09:30
    /// ```
    /// Template:
    ///
    /// `dt` in the template would be the datetime
    /// string, similar to the above.
    ///
    /// ```text
    /// format!(
    ///     "{dt:35} |{}->{}| [{:7}] {}",
    ///     log_entry.mod_path(),
    ///     log_entry.fn_name(),
    ///     log_entry.level(),
    ///     log_entry.message()
    /// );
    /// ```
    /// Sample output:
    /// ```text
    /// 2025-07-18T14:01:01.051532664+08:00 |flogging->main| [WARNING] Rain is wet!
    /// ```
    ///
    Iso8601,

    /// Simple format.
    ///
    /// Template:
    /// ```text
    /// format!(
    ///     "|{}->{}| [{:7}] {}",
    ///     log_entry.mod_path(),
    ///     log_entry.fn_name(),
    ///     log_entry.level(),
    ///     log_entry.message()
    /// );
    /// ```
    /// Sample output:
    /// ```text
    /// |flogging->main| [INFO   ] It is cloudy today.
    /// ```
    ///
    Simple,

    /// Unix Timestamp format.
    ///
    /// The first part (before the decimal point) is
    /// the number of seconds since 1970-01-01 00:00 UTC.
    ///
    /// The second part is the number of nanoseconds since
    /// the last whole second.
    ///
    /// Example:
    /// ```text
    /// 1752817859.157970496
    /// ```
    /// Template:
    ///
    /// `dt` in the template would be the datetime
    /// string, similar to the above.
    ///
    /// ```text
    /// format!(
    ///     "{dt} |{}->{}| [{:7}] {}",
    ///     log_entry.mod_path(),
    ///     log_entry.fn_name(),
    ///     log_entry.level(),
    ///     log_entry.message()
    /// );
    /// ```
    /// Sample output:
    /// ```text
    /// 1752818461.051538870 |flogging->main| [SEVERE ] Hurricanes are windy!
    /// ```
    ///
    UnixTimestamp,
}

impl Formatter {
    /// Format the text of the `log_entry`, in accordance with the formatting
    /// for this [formatter](enum.Formatter.html).
    #[allow(private_interfaces)]
    pub fn format(&self, log_entry: &LogEntry) -> String {
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
