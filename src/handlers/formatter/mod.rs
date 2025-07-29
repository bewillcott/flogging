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

mod format_trait;
mod format_type;
mod formatter;
mod iso8601_formatter;
mod mock_formatter;
mod simple_formatter;
mod unixtimestamp_formatter;

pub use crate::{
    handlers::formatter::{
        format_trait::FormatTrait, format_type::FormatType, formatter::Formatter,
        iso8601_formatter::Iso8601Formatter, mock_formatter::MockFormatter,
        simple_formatter::SimpleFormatter, unixtimestamp_formatter::UnixTimestampFormatter,
    },
    logger::LogEntry,
};
use std::{fmt, sync::Arc};

#[cfg(test)]
mod test {
    use super::*;
    use crate::{const_logger, Level::*, Logger};

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

    #[test]
    fn custom() {
        let le = LogEntry::create(
            INFO,
            "custom".to_string(),
            "Testing MockFormatter".to_string(),
        );
        let f = FormatType::Custom("Mock".to_string()).create(Some(Box::new(MockFormatter::new())));
        let fs = f.format(&le);
        println!("\n{f:width$} {fs}\n", width = f.width());
    }
}
