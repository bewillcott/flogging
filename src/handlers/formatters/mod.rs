//
// File Name:    mod.rs
// Directory:    src/handlers/formatters
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

// #![allow(unused)]

mod format_trait;
mod format_type;
mod formatter;
mod iso8601_formatter;
mod mock_formatter;
mod simple_formatter;
mod unixtimestamp_formatter;

use crate::LogEntry;
pub use format_trait::FormatTrait;
pub use format_type::FormatType;
pub use formatter::Formatter;
pub use iso8601_formatter::Iso8601Formatter;
pub use mock_formatter::MockFormatter;
pub use simple_formatter::SimpleFormatter;
pub use unixtimestamp_formatter::UnixTimestampFormatter;

#[cfg(test)]
mod test {
    use std::io::{Result, Write};
    // use super::*;
    use crate::{Level::*, *};
    use regex::Regex;

    #[test]
    fn iso8601() {
        let re_str = "^
dt_fmt: \"%\\+\" - fmt_string: \"\\{dt:35} \\{mod_path}->\\{fn_name} \\[\\{level:7}] \\{message}\"
(?:\\d{4}-\\d{2}-\\d{2}T\\d{2}:\\d{2}:\\d{2}.\\d{9}\\+\\d{2}:\\d{2}) ->iso8601 \\[INFO   ] This is a test message
$";

        let re = Regex::new(re_str).unwrap();
        let mut buf = Vec::new();

        let le = LogEntry::create(
            INFO,
            "iso8601".to_string(),
            "This is a test message".to_string(),
        );

        let f = FormatType::Iso8601.create(None);
        let fs = f.format(&le);
        // println!("\n{f:width$}\n{fs}", width = f.width());
        writeln!(&mut buf, "\n{f:width$}\n{fs}", width = f.width());
        let result = String::from_utf8(buf).unwrap();
        // println!("{result}",);

        assert!(re.is_match(&result));
    }

    #[test]
    fn simple_formatter() {
        let expected = "
dt_fmt: \"\" - fmt_string: \"{mod_path}->{fn_name} [{level:7}] {message}\"
->simple_formatter [INFO   ] This is a test message
"
        .to_string();

        let mut buf = Vec::new();

        let le = LogEntry::create(
            INFO,
            "simple_formatter".to_string(),
            "This is a test message".to_string(),
        );

        let f = FormatType::Simple.create(None);
        let fs = f.format(&le);
        // println!("\n{f:width$} {fs}\n", width = f.width());
        writeln!(&mut buf, "\n{f:width$}\n{fs}", width = f.width());

        assert_eq!(expected, String::from_utf8(buf).unwrap());
    }

    #[test]
    fn unix_timestamp() {
        let re_str = "^
dt_fmt: \"%s\\.%f\" - fmt_string: \"\\{dt} \\{mod_path}->\\{fn_name} \\[\\{level:7}] \\{message}\"
(?:\\d{10}\\.\\d{9}) ->unix_timestamp \\[INFO   ] This is a test message
$";

        let re = Regex::new(re_str).unwrap();
        let mut buf = Vec::new();

        let le = LogEntry::create(
            INFO,
            "unix_timestamp".to_string(),
            "This is a test message".to_string(),
        );

        let f = FormatType::UnixTimestamp.create(None);
        let fs = f.format(&le);
        // println!("\n{f:width$}\n{fs}", width = f.width());
        writeln!(&mut buf, "\n{f:width$}\n{fs}", width = f.width());
        let result = String::from_utf8(buf).unwrap();
        // println!("{result}",);

        assert!(re.is_match(&result));
    }

    #[test]
    fn custom() {
        let expected = "\nMockFormatter
MockFormatter
"
        .to_string();

        let mut buf = Vec::new();

        let le = LogEntry::create(
            INFO,
            "custom".to_string(),
            "Testing MockFormatter".to_string(),
        );
        let f = FormatType::Custom.create(Some(Box::new(MockFormatter::new())));
        let fs = f.format(&le);
        // println!("\n{f:width$}\n{fs}", width = f.width());
        writeln!(&mut buf, "\n{f:width$}\n{fs}", width = f.width());

        assert_eq!(expected, String::from_utf8(buf).unwrap());
    }
}
