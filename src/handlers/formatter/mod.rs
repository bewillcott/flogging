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

pub mod format_trait;
pub mod iso8601_formatter;
pub mod mock_formatter;
pub mod simple_formatter;
pub mod unixtimestamp_formatter;

use crate::{
    handlers::formatter::{
        format_trait::FormatTrait, iso8601_formatter::Iso8601Formatter, mock_formatter::MockFormatter, simple_formatter::SimpleFormatter, unixtimestamp_formatter::UnixTimestampFormatter
    },
    logger::LogEntry,
};
use std::{fmt, sync::Arc};

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq)]
pub enum FormatType {
    Iso8601,
    #[default]
    Simple,
    UnixTimestamp,
    Custom(String),
}

#[derive(Debug, Clone)]
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
    Iso8601(Iso8601Formatter),

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
    Simple(SimpleFormatter),

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
    UnixTimestamp(UnixTimestampFormatter),
    Custom(Box<dyn FormatTrait>),
}

impl FormatType {
    pub fn create(&self, custom: Option<Box<dyn FormatTrait>>) -> Formatter {
        match &self {
            FormatType::Iso8601 => Formatter::Iso8601(Default::default()),
            FormatType::Simple => Formatter::Simple(Default::default()),
            FormatType::UnixTimestamp => Formatter::UnixTimestamp(Default::default()),
            FormatType::Custom(label) => match custom {
                Some(f) => Formatter::Custom(f),
                None => Formatter::Custom(Box::new(MockFormatter::default())),
            },
        }
    }
}

impl Formatter {
    /// Format the text of the `log_entry`, in accordance with the formatting
    /// for this [formatter](enum.Formatter.html).
    pub fn format(&self, log_entry: &LogEntry) -> String {
        match self {
            Formatter::Iso8601(f) => f.format(log_entry),
            Formatter::Simple(f) => f.format(log_entry),
            Formatter::UnixTimestamp(f) => f.format(log_entry),
            Formatter::Custom(f) => f.format(log_entry),
        }
    }

    pub(crate) fn width(&self) -> usize {
        15
    }
}

impl fmt::Display for FormatType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            FormatType::Iso8601 => "Iso8601",
            FormatType::Simple => "SimpleFormatter",
            FormatType::UnixTimestamp => "UnixTimestamp",
            FormatType::Custom(label)=> &format!("Custom({label})"),
        };

        label.fmt(f)
    }
}

impl Default for Formatter {
    fn default() -> Self {
        FormatType::default().create(None)
    }
}

impl fmt::Display for Formatter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Formatter::Iso8601(formatter) => formatter.fmt(f),
            Formatter::Simple(formatter) => formatter.fmt(f),
            Formatter::UnixTimestamp(formatter) => formatter.fmt(f),
            Formatter::Custom(formatter)=> formatter.fmt(f),
        }
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
        let f = FormatType::Iso8601.create(None);
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
        let f = FormatType::Simple.create(None);
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
        let f = FormatType::UnixTimestamp.create(None);
        let fs = f.format(&le);
        println!("\n{f:width$} {fs}\n", width = f.width());
    }
}
