/*
 * File Name:    file_handler.rs
 * Project Name: logging
 *
 * Copyright (C) 2025 Bradley Willcott
 *
 * This library (crate) is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This library (crate) is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this library (crate).  If not, see <https://www.gnu.org/licenses/>.
 */

/*!
 * # FileHandler
 */

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

#[derive(Debug)]
pub struct FileHandler {
    name: String,
    format: Formatter,
    file: Option<File>,
}

impl FileHandler {
    pub fn create(filename: &str) -> Result<Self, Error> {
        if filename.is_empty() {
            return Err(Error::new(InvalidInput, "'filename' must not be empty"));
        }

        let fh = FileHandler {
            name: filename.to_string(),
            format: Iso8601,
            file: {
                let f = File::options()
                    .write(true)
                    .append(true)
                    .create(true)
                    .open(&filename)?;

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

        write!(f, "{} : {}", name, self.format)
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

    fn get_formatter(&self) -> &Formatter {
        &self.format
    }

    fn is_open(&self) -> bool {
        self.file.is_some()
    }

    fn publish(&mut self, log_entry: &LogEntry) {
        if self.is_open() {
            let mut buf = self.format.format(log_entry);
            buf.push_str("\n");

            self.file
                .as_mut()
                .unwrap()
                .write_all(buf.as_bytes());
        }
    }

    fn set_formatter(&mut self, format: Formatter) {
        self.format = format;
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
