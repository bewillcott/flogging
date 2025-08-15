//
// File Name:    console_type.rs
// Directory:    src/handlers/console_handler
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
//! # ConsoleType
//!

use std::{fmt, str::FromStr};

///
/// `ConsoleType` configures the `ConsoleHandler`'s output.
///
#[derive(Debug, Default)]
pub enum ConsoleType {
    #[default]
    ///
    /// Prints to `stdout`.
    ///
    StdOut,
    ///
    /// Print to `stderr`.
    ///
    StdErr,
    ///
    /// If `log_entry.level` is `LeveL::INFO`, then
    /// prints unformatted `log_entry.msg` to `stdout`, else
    /// prints formatted `log_entry.msg` to `stderr`.
    ///
    Production,
}

impl ConsoleType {
    ///
    /// Converts a console type to its string version.
    ///
    pub const fn as_str(&self) -> &'static str {
        match self {
            ConsoleType::StdOut => "stdout",
            ConsoleType::StdErr => "stderr",
            ConsoleType::Production => "production",
        }
    }
}

impl fmt::Display for ConsoleType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

///
/// Returned from `FromStr::from_str()` when an unknown string
/// is passed-in.
///
#[derive(Debug)]
pub struct ConsoleTypeError {
    msg: String,
}

impl FromStr for ConsoleType {
    type Err = ConsoleTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "stdout" => Ok(ConsoleType::StdOut),
            "stderr" => Ok(ConsoleType::StdErr),
            "production" => Ok(ConsoleType::Production),
            _ => Err(ConsoleTypeError {
                msg: format!("Unknown console type: {s}"),
            }),
        }
    }
}
