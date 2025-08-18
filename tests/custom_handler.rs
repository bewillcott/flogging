//
// File Name:    custom_handler.rs
// Directory:    tests
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
//! # CustomHandler for Testing
//!

#![allow(dead_code)]

use std::{fmt, io::Error};

use flogging::*;

#[derive(Debug, Default)]
pub(crate) struct CustomHandler {
    name: String,
    formatter: Formatter,
    log: Vec<String>,
}

impl CustomHandler {
    fn new(name: &str) -> Self {
        CustomHandler {
            name: name.to_string(),
            formatter: FormatType::Simple.create(None),
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

impl fmt::Display for CustomHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let len = self.name.len() + self.formatter.to_string().len();
        let line = "-".repeat(len + 3);

        write!(
            f,
            "{} : {}\n{line}\n{}",
            self.name,
            self.formatter,
            self.log()
        )
    }
}

impl HandlerTrait for CustomHandler {
    fn create(name: &str) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self::new(name))
    }

    fn close(&mut self) {}

    fn flush(&mut self) {
        self.log.clear();
    }

    fn get_formatter(&self) -> Formatter {
        self.formatter.clone()
    }

    fn get_log(&self) -> String {
        self.log()
    }

    fn is_open(&self) -> bool {
        true
    }

    fn publish(&mut self, log_entry: &LogEntry) {
        println!("{}", self.formatter.format(log_entry));
        self.log.push(self.formatter.format(log_entry));
    }

    fn set_formatter(&mut self, format: Formatter) {
        self.formatter = format;
    }

    ///
    /// This is a 'NoOp' fn. Use `get_log()`, as this already
    /// has the required functionality.
    ///
    fn set_test_mode(&mut self, _state: bool) {}
}
