//
// File Name:    mock_formatter.rs
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
//! # Mock Formatter
//!

use super::format_trait::FormatTrait;
use std::fmt;

///
/// Mock Formatter.
///
/// Used as a filler. It does not have a proper format string. It is also used
/// in examples for custom formatters.
///
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct MockFormatter {
    dt_fmt: String,
    fmt_string: String,
}

impl MockFormatter {
    pub fn new() -> Self {
        Self {
            dt_fmt: "".to_string(),
            fmt_string: "MockFormatter".to_string(),
        }
    }

    pub fn dt_fmt(&self) -> String {
        self.dt_fmt.clone()
    }

    pub fn fmt_string(&self) -> String {
        self.fmt_string.clone()
    }
}

impl Default for MockFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for MockFormatter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "MockFormatter".fmt(f)
    }
}

impl FormatTrait for MockFormatter {
    fn format(&self, log_entry: &crate::LogEntry) -> String {
        "MockFormatter".to_string()
    }
}
