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
    handlers::{
        formatter::{FormatType, Formatter},
        handler::HandlerTrait,
    },
    logger::{Level, LogEntry},
};

///
/// Publishes log entries to the console.
///
/// If parameter `stderr` is `true`, then: \
/// - [`std::io::stderr`],
///
/// else the default is:
/// - [`std::io::stdout`].
///
#[derive(Debug, Default)]
pub struct ConsoleHandler {
    stderr: bool,
    formatter: Formatter,
}

impl ConsoleHandler {
    fn create(stderr: bool) -> Self {
        ConsoleHandler {
            stderr,
            formatter: FormatType::Simple.create(None),
        }
    }
}

impl fmt::Display for ConsoleHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.formatter.fmt(f)
    }
}

impl HandlerTrait for ConsoleHandler {
    fn create(stderr: &str) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let val = "true".eq_ignore_ascii_case(stderr);
        Ok(ConsoleHandler::create(val))
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
        if self.stderr {
            eprintln!("{}", self.formatter.format(log_entry));
        } else {
            println!("{}", self.formatter.format(log_entry));
        }
    }

    fn set_formatter(&mut self, formatter: Formatter) {
        self.formatter = formatter;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{Logger, logger};

    #[test]
    fn handler_trait() {
        let mut log = Logger::console_logger(module_path!());

        log.info("trait methods");

        let handler = log.get_handler(crate::Handler::Console).unwrap();
        assert!(handler.is_open());
        assert_eq!(
            handler.get_formatter().to_string(),
            "dt_fmt: \"\" - fmt_string: \"|{mod_path}->{fn_name}| [{level:7}] {message}\""
                .to_string()
        );
        assert_eq!(handler.get_log(), "".to_string());
        handler.flush();
        handler.close();
    }
}
