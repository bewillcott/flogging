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
use crate::*;
use std::{fmt, io::Error};

///
/// Publishes log entries to an internal list.
///
/// The list can then be accessed via: [get_log()][StringHandler::get_log()].
///
#[derive(Debug, Default)]
pub struct StringHandler {
    formatter: Formatter,
    log: Vec<String>,
}

impl StringHandler {
    fn new() -> Self {
        StringHandler {
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

impl fmt::Display for StringHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let len = self.formatter.to_string().len();
        let line = "-".repeat(len + 3);

        write!(f, "{}\n{line}\n{}", self.formatter, self.log())
    }
}

impl HandlerTrait for StringHandler {
    fn create(_name: &str) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(StringHandler::new())
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

    #[allow(private_interfaces)]
    fn publish(&mut self, log_entry: &LogEntry) {
        self.log.push(self.formatter.format(log_entry));
    }

    fn set_formatter(&mut self, formatter: Formatter) {
        self.formatter = formatter;
    }

    ///
    /// This is a 'NoOp' fn. Use `get_log()`, as this already
    /// has the required functionality.
    ///
    fn set_test_mode(&mut self, _state: bool) {}
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn string_handler() {
        let expected = "flogging::handlers::string_handler::tests-> [INFO   ] trait methods
flogging::handlers::string_handler::tests-> [WARNING] The sky is falling!\n"
            .to_string();

        let mut log = Logger::string_logger(module_path!());

        log.info("trait methods");
        log.warning("The sky is falling!");

        let handler = log.get_handler(crate::Handler::String).unwrap();
        handler.set_test_mode(true);
        assert!(handler.is_open());
        assert_eq!(
            handler.get_formatter().to_string(),
            "dt_fmt: \"\" - fmt_string: \"{mod_path}->{fn_name} [{level:7}] {message}\""
                .to_string()
        );
        assert_eq!(expected, handler.get_log());
        handler.flush();

        assert_eq!(handler.get_log(), "".to_string());
        handler.close();
    }
}
