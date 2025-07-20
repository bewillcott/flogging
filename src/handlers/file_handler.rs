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
        formatter::Formatter::{self, Iso8601},
        handler::HandlerTrait,
    },
    logger::{Level, LogEntry},
};

#[derive(Debug, Default)]
pub struct FileHandler {
    name: String,
    formatter: Formatter,
    file: Option<File>,
}

impl FileHandler {
    pub fn create(filename: &str) -> Result<Self, Error> {
        if filename.is_empty() {
            return Err(Error::new(InvalidInput, "'filename' must not be empty"));
        }

        let fh = FileHandler {
            name: filename.to_string(),
            formatter: Iso8601,
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
        let name = if self.name.is_empty() {
            "<name>".to_string()
        } else {
            self.name.clone()
        };

        write!(f, "{} : {}", name, self.formatter)
    }
}

impl HandlerTrait for FileHandler {
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

    fn set_formatter(&mut self, format: Formatter) {
        self.formatter = format;
    }
}
