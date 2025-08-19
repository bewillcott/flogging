//
// File Name:    format_type.rs
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
//! # FormatType
//!

use std::fmt;
use super::*;

///
/// Used as a simple way to obtain the various [`Formatter`]s.
///
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq)]
pub enum FormatType {
    ///
    ///  ISO 8601 / RFC 3339 date & time format.
    ///
    Iso8601,

    ///
    ///  Simple format.
    ///
    #[default]
    Simple,

    ///
    ///  Unix Timestamp format.
    ///
    UnixTimestamp,

    ///
    /// Custom format.
    ///
    /// Used to hold user provided formatter.
    ///
    Custom,
}

impl FormatType {
    ///
    /// Provides the requisite [`Formatter`].
    ///
    /// ## Parameters
    /// - `custom` - Optional boxed implementor of [`FormatTrait`].
    ///   Used by [`FormatType::Custom`] to provide a
    ///   [`Formatter::Custom`] formatter. If `None` provided, then the default
    ///   is to use a [`MockFormatter`].
    pub fn create(&self, custom: Option<Box<dyn FormatTrait>>) -> Formatter {
        match &self {
            FormatType::Iso8601 => Formatter::Iso8601(Default::default()),
            FormatType::Simple => Formatter::Simple(Default::default()),
            FormatType::UnixTimestamp => Formatter::UnixTimestamp(Default::default()),
            FormatType::Custom => match custom {
                Some(f) => Formatter::Custom(f),
                None => Formatter::Custom(Box::new(MockFormatter::default())),
            },
        }
    }
}

impl fmt::Display for FormatType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            FormatType::Iso8601 => "Iso8601",
            FormatType::Simple => "SimpleFormatter",
            FormatType::UnixTimestamp => "UnixTimestamp",
            FormatType::Custom => "Custom",
        };

        label.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Result, Write};

    #[test]
    fn display() {
        let expected = "ft: Custom
fmt: MockFormatter
Iso8601: Iso8601
Simple: SimpleFormatter
UnixTimestamp: UnixTimestamp
".to_string();
        let mut buf = Vec::new();

        let ft = FormatType::Custom;
        let fmt = ft.create(None);

        writeln!(&mut buf, "ft: {ft}");
        writeln!(&mut buf, "fmt: {fmt}");
        writeln!(&mut buf, "Iso8601: {}", FormatType::Iso8601);
        writeln!(&mut buf, "Simple: {}", FormatType::Simple);
        writeln!(&mut buf, "UnixTimestamp: {}", FormatType::UnixTimestamp);

        assert_eq!(expected, String::from_utf8(buf).unwrap());
    }
}
