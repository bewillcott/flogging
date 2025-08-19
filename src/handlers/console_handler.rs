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

use crate::*;
use console_type::ConsoleType;
use std::{
    fmt,
    io::{self, Error, Write},
};

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
    writer: Option<Vec<u8>>,
}

impl ConsoleHandler {
    fn create(console_type: ConsoleType) -> Self {
        ConsoleHandler {
            console_type,
            formatter: FormatType::Simple.create(None),
            writer: None,
        }
    }

    fn log(&self) -> String {
        if let Some(w) = self.writer.to_owned() {
            String::from_utf8(w).unwrap()
        } else {
            String::new()
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

    ///
    /// Removes the internal buffer, if in `test_mode`.\
    /// Will therefore, no longer be *in* `test_mode`.
    ///
    fn close(&mut self) {
        if self.writer.is_some() {
            self.writer = None;
        }
    }

    ///
    /// Clears the internal buffer, if in `test_mode`.
    ///
    fn flush(&mut self) {
        if let Some(w) = self.writer.as_mut() {
            w.clear()
        };
    }

    fn get_formatter(&self) -> Formatter {
        self.formatter.clone()
    }

    fn get_log(&self) -> String {
        self.log()
    }

    ///
    /// `ConsoleHandler` is *always* open.
    ///
    fn is_open(&self) -> bool {
        true
    }

    fn publish(&mut self, log_entry: &LogEntry) {
        match self.writer.as_mut() {
            Some(w) => {
                let _ = match self.console_type {
                    ConsoleType::StdOut => writeln!(w, "{}", self.formatter.format(log_entry)),
                    ConsoleType::StdErr => writeln!(w, "{}", self.formatter.format(log_entry)),
                    ConsoleType::Production => production_test(w, &self.formatter, log_entry),
                };
            }
            None => match self.console_type {
                ConsoleType::StdOut => println!("{}", self.formatter.format(log_entry)),
                ConsoleType::StdErr => eprintln!("{}", self.formatter.format(log_entry)),
                ConsoleType::Production => production(&self.formatter, log_entry),
            },
        }
    }

    fn set_formatter(&mut self, formatter: Formatter) {
        self.formatter = formatter;
    }

    ///
    /// Sets the test mode to `state`.
    ///
    /// If set to `true`, use `get_log()` to obtain the
    /// log.
    ///
    fn set_test_mode(&mut self, state: bool) {
        if state {
            // true
            self.writer = Some(Vec::new());
        } else {
            self.writer = None;
        }
    }
}

fn production(formatter: &Formatter, log_entry: &LogEntry) {
    if log_entry.level() == Level::INFO {
        println!("{}", log_entry.message());
    } else {
        eprintln!("{}", formatter.format(log_entry));
    }
}

fn production_test(
    writer: &mut Vec<u8>,
    formatter: &Formatter,
    log_entry: &LogEntry,
) -> io::Result<()> {
    if log_entry.level() == Level::INFO {
        writeln!(writer, "{}", log_entry.message())?;
    } else {
        writeln!(writer, "{}", formatter.format(log_entry))?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn stdout_handler() {
        let mut log = Logger::console_logger(module_path!());

        log.info("trait methods");
        log.warning("The sky is falling!");

        let handler = log.get_handler(crate::Handler::Console).unwrap();
        handler.set_test_mode(false);

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

    #[test]
    fn stdout_handler_test_mode() {
        let expected = "flogging::handlers::console_handler::tests-> [INFO   ] trait methods
flogging::handlers::console_handler::tests-> [WARNING] The sky is falling!
"
            .to_string();

        let mut log = Logger::console_logger(module_path!());

        let h = log.get_handler(crate::Handler::Console).unwrap();
        h.set_test_mode(true);

        assert!(h.is_open());
        assert_eq!(
            h.get_formatter().to_string(),
            "dt_fmt: \"\" - fmt_string: \"{mod_path}->{fn_name} [{level:7}] {message}\""
                .to_string()
        );

        log.info("trait methods");
        log.warning("The sky is falling!");

        let h = log.get_handler(crate::Handler::Console).unwrap();
        let buf = h.get_log();

        assert_eq!(expected, buf);

        h.flush();
        h.close();
    }

    #[test]
    fn stderr_handler() {
        let mut log = Logger::econsole_logger(module_path!());

        log.info("trait methods");
        log.warning("The sky is falling!");

        let handler = log.get_handler(crate::Handler::EConsole).unwrap();
        handler.set_test_mode(false);

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

    #[test]
    fn stderr_handler_test_mode() {
        let expected = "flogging::handlers::console_handler::tests-> [INFO   ] trait methods
flogging::handlers::console_handler::tests-> [WARNING] The sky is falling!\n"
            .to_string();

        let mut log = Logger::econsole_logger(module_path!());

        let h = log.get_handler(crate::Handler::EConsole).unwrap();
        h.set_test_mode(true);

        assert!(h.is_open());
        assert_eq!(
            h.get_formatter().to_string(),
            "dt_fmt: \"\" - fmt_string: \"{mod_path}->{fn_name} [{level:7}] {message}\""
                .to_string()
        );

        log.info("trait methods");
        log.warning("The sky is falling!");

        let h = log.get_handler(crate::Handler::EConsole).unwrap();
        let buf = h.get_log();

        assert_eq!(expected, buf);

        h.flush();
        h.close();
    }

    #[test]
    fn production_handler() {
        let mut log = Logger::pconsole_logger(module_path!());

        log.info("trait methods");
        log.warning("The sky is falling!");

        let handler = log.get_handler(crate::Handler::PConsole).unwrap();
        handler.set_test_mode(false);

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

    #[test]
    fn production_handler_test_mode() {
        let expected = "trait methods
flogging::handlers::console_handler::tests-> [WARNING] The sky is falling!\n"
            .to_string();

        let mut log = Logger::pconsole_logger(module_path!());

        let h = log.get_handler(crate::Handler::PConsole).unwrap();
        h.set_test_mode(true);

        assert!(h.is_open());
        assert_eq!(
            h.get_formatter().to_string(),
            "dt_fmt: \"\" - fmt_string: \"{mod_path}->{fn_name} [{level:7}] {message}\""
                .to_string()
        );

        log.info("trait methods");
        log.warning("The sky is falling!");

        let h = log.get_handler(crate::Handler::PConsole).unwrap();
        let buf = h.get_log();

        assert_eq!(expected, buf);

        h.flush();
        h.close();
    }
}
