# Final Code

Here is the complete source code for the custom formatter: `CsvFormatter`.

```rust, no_run
//
// File Name:    confile_handler.rs
// Directory:    src/handlers
// Project Name: my_project
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
//! # ConfileHandler
//!

#![allow(unused)]

use std::{
    fmt,
    fs::{File, exists},
    io::{Error, ErrorKind::InvalidInput, Write},
};

use flogging::*;

///
/// Publishes log entries to the file whose name was provided during
/// initialization.
///
#[derive(Debug, Default)]
pub struct ConfileHandler {
    filename: String,
    file_fmt: Formatter,
    con_fmt: Formatter,
    file: Option<File>,
}

impl ConfileHandler {
    fn create(filename: &str) -> Result<Self, Error> {
        if filename.is_empty() {
            return Err(Error::new(InvalidInput, "'filename' must not be empty"));
        }

        let fh = ConfileHandler {
            filename: filename.to_string(),
            file_fmt: FormatType::Iso8601.create(None),
            con_fmt: FormatType::Simple.create(None),
            file: {
                let f = File::options().append(true).create(true).open(filename)?;

                Some(f)
            },
        };

        Ok(fh)
    }
}

impl fmt::Display for ConfileHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} : {}\nConsole: {}",
            self.filename, self.file_fmt, self.con_fmt
        )
    }
}

impl HandlerTrait for ConfileHandler {
    ///
    /// Create a new handler instance.
    ///
    /// ## Parameters
    /// - `name` - This the `filename` of the log file.
    ///
    fn create(name: &str) -> Result<Self, Error> {
        ConfileHandler::create(name)
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
        self.file_fmt.clone()
    }

    fn get_log(&self) -> String {
        String::new()
    }

    fn is_open(&self) -> bool {
        self.file.is_some()
    }

    fn publish(&mut self, log_entry: &LogEntry) {
        if self.is_open() {
            let mut buf = self.file_fmt.format(log_entry);
            buf.push('\n');

            self.file.as_mut().unwrap().write_all(buf.as_bytes());
            println!("{}", self.con_fmt.format(log_entry));
        }
    }

    fn set_formatter(&mut self, _formatter: Formatter) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handler_trait() {
        let mut log = Logger::custom_logger(
            module_path!(),
            "ConfileHandler",
            Box::new(ConfileHandler::create("example.log").unwrap()),
        );

        log.set_fn_name("handler_trait");
        log.info("trait methods");

        let handler = log
            .get_handler(crate::Handler::Custom("ConfileHandler".to_string()))
            .unwrap();
        assert!(handler.is_open());
        assert_eq!(handler.get_formatter().to_string(), "dt_fmt: \"%+\" - fmt_string: \"{dt:35} |{mod_path}->{fn_name}| [{level:7}] {message}\"".to_string());
        assert_eq!(handler.get_log(), "".to_string());
        handler.flush();
        handler.close();
    }

    #[test]
    #[should_panic(expected = "'filename' must not be empty")]
    fn filename_empty() {
        let mut log = Logger::custom_logger(
            module_path!(),
            "ConfileHandler",
            Box::new(ConfileHandler::create("").unwrap()),
        );
    }
}
```
