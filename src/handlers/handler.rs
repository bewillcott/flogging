//
// File Name:    handler.rs
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
//! # Handler
//!

// #![allow(unused)]

use std::{fmt::Display, io::Error};

use crate::{
    handlers::{
        console_handler::ConsoleHandler, file_handler::FileHandler, formatter::Formatter,
        string_handler::StringHandler,
    },
    logger::{Level, LogEntry},
};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Handler {
    ConsoleHandler,
    FileHandler,
    StringHandler,
}

impl Handler {
    pub(crate) fn create(&self, name: &str) -> Result<Box<dyn HandlerTrait>, Error> {
        let r: Box<dyn HandlerTrait> = match self {
            Handler::ConsoleHandler => Box::new(ConsoleHandler::create(name)?),
            Handler::FileHandler => Box::new(FileHandler::create(name)?),
            Handler::StringHandler => Box::new(StringHandler::create(name)?),
        };

        Ok(r)
    }
}

#[allow(private_interfaces)]
pub trait HandlerTrait: Display + Send + Sync {
    ///
    /// Create a new handler instance.
    ///
    /// **name**: Used to identify handler.
    ///
    fn create(name: &str) -> Result<Self, Error>
    where
        Self: Sized;

    ///
    /// Close the Handler and free all associated resources.
    ///
    /// The close method will perform a flush and then close the Handler.
    /// After close has been called this Handler should no longer be used.
    /// Method calls may either be silently ignored or may return `Error`s.
    ///
    fn close(&mut self);

    ///
    /// Flush any buffered output.
    ///
    fn flush(&mut self);

    ///
    /// Return a copy of the internal buffer as a `String`.
    ///
    fn get_log(&self) -> String;

    ///
    /// Return the format String for this Handler.
    ///
    fn get_formatter(&self) -> &Formatter;

    ///
    /// Check status of this handler.
    ///
    fn is_open(&self) -> bool;

    ///
    /// Publish a LogEntry.
    ///
    /// The logging request was made initially to a Logger object, which initialized
    /// the LogEntry and forwarded it here.
    ///
    /// The Handler is responsible for formatting the message, when and if necessary.
    ///
    fn publish(&mut self, log_entry: &LogEntry);

    ///
    /// Set a Format.
    ///
    fn set_formatter(&mut self, format: Formatter);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn file_handler() {
        let name = "temp.txt";
        let h = Handler::FileHandler;
        let fh = h.create(name);

        println!("\n{}\n", fh.unwrap());
    }

    #[test]
    fn file_handler_error() {
        let name = "";
        let h = Handler::FileHandler;
        let fh = h.create(name);

        assert!(fh.is_err());
    }
}
