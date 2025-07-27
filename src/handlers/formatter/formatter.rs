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

use super::*;

///
/// Provides wrappers for holding each type of formatter.
///
#[derive(Debug, Clone)]
pub enum Formatter {

    ///
    ///  ISO 8601 / RFC 3339 date & time format.
    ///
    Iso8601(Iso8601Formatter),

    ///
    ///  Simple format.
    ///
    Simple(SimpleFormatter),

    ///
    ///  Unix Timestamp format.
    ///
    UnixTimestamp(UnixTimestampFormatter),

    ///
    /// Custom format.
    ///
    /// Used to hold user provided formatter.
    ///
    Custom(Box<dyn FormatTrait>),
}

impl Formatter {
    ///
    /// Format the text of the `log_entry`, in accordance with the formatting
    /// for this [formatter](enum.Formatter.html).
    ///
    /// ## Parameters
    /// - `log_entry` - The log entry to be formatted.
    ///
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
            Formatter::Custom(formatter) => formatter.fmt(f),
        }
    }
}
