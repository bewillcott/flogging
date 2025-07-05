//
// File Name:    file_handler.rs
// Project Name: logging
//
// Copyright (C) 2025 Bradley Willcott
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
//! File Handler
//!

#![allow(unused)]

use std::{
    fmt,
    fs::{File, exists},
    io::{Error, ErrorKind::InvalidInput},
};

use crate::{
    handlers::{
        formatter::Formatter::{self, Iso8601},
        handler::HandlerTrait,
    },
    logger::{level::Level, log_entry::LogEntry},
};

#[derive(Debug)]
pub(crate) struct FileHandler {
    name: String,
    format: Formatter,
    level: Level,
    file: Option<File>,
}

impl FileHandler {
    fn new(filename: String) -> Result<Self, Error> {
        if filename.is_empty() {
            return Err(Error::new(InvalidInput, "'filename' must not be empty"));
        }

        let fh = FileHandler {
            name: filename.clone(),
            format: Iso8601,
            level: Level::default(),
            file: {
                let f: File;

                if exists(&filename)? {
                    f = File::options().write(true).truncate(true).open(&filename)?;
                } else {
                    f = File::options().write(true).create(true).open(&filename)?;
                }

                Some(f)
            },
        };

        Ok(fh)
    }
}
impl fmt::Display for FileHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name: String;

        if self.name.is_empty() {
            name = "<name>".to_string();
        } else {
            name = self.name.clone();
        };

        write!(f, "{} [{}]: {}", name, self.level, self.format)
    }
}

impl HandlerTrait for FileHandler {
    fn new(name: String) -> Result<Self, Error> {
        FileHandler::new(name)
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
        self.format.clone()
    }

    fn get_level(&self) -> Level {
        self.level.clone()
    }

    fn is_loggable(&self, log_entry: LogEntry) -> bool {
        log_entry.level() >= self.level
    }

    fn publish(&mut self, log_entry: LogEntry) {
        todo!()
    }

    fn set_formatter(&mut self, format: Formatter) {
        self.format = format;
    }

    fn set_level(&mut self, level: Level) {
        self.level = level;
    }
}
//     let mut file: File;

//     if exists(filename)? {
//         file = File::options().write(true).truncate(true).open(filename)?;
//     } else {
//         file = File::options().write(true).create(true).open(filename)?;
//     }

//     file.write(REPORT_HEADER.as_bytes());

//     for s in &mut log_strings {
//         s.push('\n');
//         file.write(s.as_bytes());
//     }

//     file.flush();
