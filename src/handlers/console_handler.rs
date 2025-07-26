//
// File Name:    console_handler.rs
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
//! # ConsoleHandler
//!
//! Publishes log entries to the console: `[std::io::stderr]`.
//!

use std::{fmt, io::Error};

use crate::{
    handlers::{formatter::{FormatType, Formatter}, handler::HandlerTrait},
    logger::{Level, LogEntry},
};

///
/// Publishes log entries to the console: [`std::io::stdout`].
///
#[derive(Debug, Default)]
pub struct ConsoleHandler {
    ///
    /// `name` is the `mod_path`.
    ///
    name: String,
    formatter: Formatter,
}

impl ConsoleHandler {
    fn create(name: &str) -> Self {
        ConsoleHandler {
            name: name.to_string(),
            formatter: FormatType::Simple.create(None),
        }
    }
}

impl fmt::Display for ConsoleHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} : {}", self.name, self.formatter)
    }
}

impl HandlerTrait for ConsoleHandler {
    fn create(name: &str) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(ConsoleHandler::create(name))
    }

    fn close(&mut self) {}

    fn flush(&mut self) {}

    fn get_formatter(&self) -> Formatter {
        self.formatter.clone()
    }

    fn get_log(&self) -> String {
        String::new()
    }

    fn is_open(&self) -> bool {
        true
    }

    fn publish(&mut self, log_entry: &LogEntry) {
        println!("{}", self.formatter.format(log_entry));
    }

    fn set_formatter(&mut self, formatter: Formatter) {
        self.formatter = formatter;
    }
}
