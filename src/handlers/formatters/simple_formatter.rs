//
// File Name:    simple_formatter.rs
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
//! # Simple Formatter
//!

use std::fmt;
use crate::FormatTrait;

///
/// Simple format.
///
/// Template:
/// - `mod_path`, `fn_name`, `level`, and `message` all come out of the `LogEntry`
///   provided to the [`format()`][SimpleFormatter::format] method.
/// ```ignore
/// format!("{mod_path}->{fn_name} [{level:7}] {message}");
/// ```
/// Sample output:
/// ```text
/// flogging->main [INFO   ] It is cloudy today.
/// ```
///

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct SimpleFormatter {
    dt_fmt: String,
    fmt_string: String,
}

impl SimpleFormatter {
    ///
    /// Creates a new instance of `SimpleFormatter`.
    ///
    pub fn new() -> Self {
        Self {
            dt_fmt: "".to_string(),
            fmt_string: "{mod_path}->{fn_name} [{level:7}] {message}".to_string(),
        }
    }

    ///
    /// Returns the date/time format string.
    ///
    pub fn dt_fmt(&self) -> String {
        self.dt_fmt.clone()
    }

    ///
    /// Returns the primary format string.
    ///
    pub fn fmt_string(&self) -> String {
        self.fmt_string.clone()
    }
}

impl Default for SimpleFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SimpleFormatter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "dt_fmt: \"{}\" - fmt_string: \"{}\"",
            self.dt_fmt, self.fmt_string
        )
    }
}

impl FormatTrait for SimpleFormatter {
    fn format(&self, log_entry: &crate::LogEntry) -> String {
        self.ft_fmt(self.dt_fmt(), self.fmt_string(), log_entry)
    }
}
