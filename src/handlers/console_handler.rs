//
// File Name:    console_handler.rs
// Directory:    src/handlers
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

pub mod console_type;

use std::{fmt, io::Error};
use crate::*;
use console_type::ConsoleType;

///
/// Publishes log entries to the console.
///
/// If `console_type` is:
///
/// - `ConsoleType::StdOut` - print to `stdout`,
/// - `ConsoleType::StdErr` - print to `stderr`,
/// - `ConsoleType::Production`:\
///   If `log_entry.level` is `LeveL::INFO`, then\
///   prints unformatted `log_entry.msg` to `stdout`, else\
///   prints formatted `log_entry.msg` to `stderr`.
///
#[derive(Debug, Default)]
pub struct ConsoleHandler {
    console_type: ConsoleType,
    formatter: Formatter,
}

impl ConsoleHandler {
    fn create(console_type: ConsoleType) -> Self {
        ConsoleHandler {
            console_type,
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
    fn create(console_type: &str) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(ConsoleHandler::create(console_type.parse().unwrap()))
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
        // if self.stderr {
        //     eprintln!("{}", self.formatter.format(log_entry));
        // } else {
        //     println!("{}", self.formatter.format(log_entry));
        // }
        match self.console_type {
            ConsoleType::StdOut => println!("{}", self.formatter.format(log_entry)),
            ConsoleType::StdErr => eprintln!("{}", self.formatter.format(log_entry)),
            ConsoleType::Production => production(&self.formatter, log_entry),
        }
    }

    fn set_formatter(&mut self, formatter: Formatter) {
        self.formatter = formatter;
    }
}

fn production(formatter: &Formatter, log_entry: &LogEntry) {
    if log_entry.level() == Level::INFO {
        println!("{}", log_entry.message());
    } else {
        eprintln!("{}", formatter.format(log_entry))
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
            "dt_fmt: \"\" - fmt_string: \"{mod_path}->{fn_name} [{level:7}] {message}\""
                .to_string()
        );
        assert_eq!(handler.get_log(), "".to_string());
        handler.flush();
        handler.close();
    }
}
