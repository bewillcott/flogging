//
// File Name:    console_handler.rs
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
//! Console Handler
//!

use std::{fmt, io::Error};

use crate::{
    handlers::{formatter::Formatter, handler::HandlerTrait},
    logger::{level::Level, log_entry::LogEntry},
};

pub(crate) struct ConsoleHandler {
    name: String,
    level: Level,
    format: Formatter,
}

impl ConsoleHandler {
    fn new(name: String) -> Self {
        ConsoleHandler {
            name,
            level: Level::default(),
            format: Formatter::SimpleFormatter,
        }
    }
}

impl fmt::Display for ConsoleHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} [{}]: {}", self.name, self.level, self.format)
    }
}

impl HandlerTrait for ConsoleHandler {
    fn new(name: String) -> Result<Self, Error>
    where
        Self: Sized,
    {
        todo!()
    }

    fn close(&mut self) {
        todo!()
    }

    fn flush(&mut self) {
        todo!()
    }

    fn get_formatter(&self) -> Formatter {
        todo!()
    }

    fn get_level(&self) -> Level {
        todo!()
    }

    fn is_loggable(&self, log_entry: LogEntry) -> bool {
        todo!()
    }

    fn publish(&mut self, log_entry: LogEntry) {
        todo!()
    }

    fn set_formatter(&mut self, format: Formatter) {
        todo!()
    }

    fn set_level(&mut self, level: Level) {
        todo!()
    }
}
