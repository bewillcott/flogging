//
// File Name:    file_handler.rs
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
//! # FileHandler
//!

#![allow(unused)]

use std::{
    fmt,
    fs::{File, exists},
    io::{Error, ErrorKind::InvalidInput, Write},
};

use crate::{
    handlers::{
        formatter::{FormatType, Formatter},
        handler::HandlerTrait,
    },
    logger::{Level, LogEntry},
};

///
/// Publishes log entries to the file whose name was provided during
/// initialization.
///
#[derive(Debug, Default)]
pub struct FileHandler {
    filename: String,
    formatter: Formatter,
    file: Option<File>,
}

impl FileHandler {
    fn create(filename: &str) -> Result<Self, Error> {
        if filename.is_empty() {
            return Err(Error::new(InvalidInput, "'filename' must not be empty"));
        }

        let fh = FileHandler {
            filename: filename.to_string(),
            formatter: FormatType::Iso8601.create(None),
            file: {
                let f = File::options().append(true).create(true).open(filename)?;

                Some(f)
            },
        };

        Ok(fh)
    }
}

impl fmt::Display for FileHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} : {}", self.filename, self.formatter)
    }
}

impl HandlerTrait for FileHandler {
    ///
    /// Create a new handler instance.
    ///
    /// ## Parameters
    /// - `name` - This the `filename` of the log file.
    ///
    fn create(name: &str) -> Result<Self, Error> {
        FileHandler::create(name)
    }

    fn close(&mut self) {
        self.flush();
        self.file = None;
    }

    fn flush(&mut self) {
        if let Some(f) = &self.file {
            f.sync_all();
        }
    }

    fn get_formatter(&self) -> Formatter {
        self.formatter.clone()
    }

    fn get_log(&self) -> String {
        String::new()
    }

    fn is_open(&self) -> bool {
        self.file.is_some()
    }

    #[allow(private_interfaces)]
    fn publish(&mut self, log_entry: &LogEntry) {
        if self.is_open() {
            let mut buf = self.formatter.format(log_entry);
            buf.push('\n');

            self.file.as_mut().unwrap().write_all(buf.as_bytes());
        }
    }

    fn set_formatter(&mut self, formatter: Formatter) {
        self.formatter = formatter;
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    use crate::{logger, Logger};

    #[test]
    fn handler_trait(){
        let mut log = Logger::file_logger(module_path!(), "test.log");

        log.info("trait methods");

        let handler = log.get_handler(crate::Handler::File).unwrap();
        assert!(handler.is_open());
        assert_eq!(handler.get_formatter().to_string(), "dt_fmt: \"%+\" - fmt_string: \"{dt:35} |{mod_path}->{fn_name}| [{level:7}] {message}\"".to_string());
        assert_eq!(handler.get_log(),"".to_string());
        handler.flush();
        handler.close();
    }

    #[test]
    #[should_panic(expected = "'filename' must not be empty")]
    fn filename_empty(){
        let mut log = Logger::file_logger(module_path!(), "");
    }


}
