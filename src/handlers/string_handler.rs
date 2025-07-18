//
// File Name:    string_handler.rs
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
//! # StringHandler
//!
use std::{fmt, io::Error};

use crate::{
    handlers::{formatter::Formatter, handler::HandlerTrait},
    logger::{Level, LogEntry},
};

pub struct StringHandler {
    name: String,
    formatter: Formatter,
    log: Vec<String>,
}

impl StringHandler {
    fn create(name: &str) -> Self {
        StringHandler {
            name: name.to_string(),
            formatter: Formatter::Simple,
            log: Vec::new(),
        }
    }

    fn log(&self) -> String {
        let mut buf = String::new();

        for s in &self.log {
            buf.push_str(s);
            buf.push('\n');
        }

        buf
    }
}

impl fmt::Display for StringHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} : {}\n\n{}", self.name, self.formatter, self.log())
    }
}

impl HandlerTrait for StringHandler {
    fn create(name: &str) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(StringHandler::create(name))
    }

    fn close(&mut self) {}

    fn flush(&mut self) {
        self.log.clear();
    }

    fn get_formatter(&self) -> &Formatter {
        &self.formatter
    }

    fn get_log(&self) -> String {
        self.log()
    }

    fn is_open(&self) -> bool {
        true
    }

    #[allow(private_interfaces)]
    fn publish(&mut self, log_entry: &LogEntry) {
        self.log.push(self.formatter.format(log_entry));
    }

    fn set_formatter(&mut self, format: Formatter) {
        self.formatter = format;
    }
}
