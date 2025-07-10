/*
 * File Name:    handler.rs
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
 * # Handler
 */

#![allow(unused)]

use std::{fmt::Display, io::Error};

use crate::{
    handlers::{console_handler::ConsoleHandler, file_handler::FileHandler, formatter::Formatter},
    logger::{level::Level, log_entry::LogEntry},
};

#[derive(Debug, Clone)]
pub(crate) enum Handler {
    ConsoleHandler,
    FileHandler,
}

impl Handler {
    pub(crate) fn new(&self, name: String) -> Result<Box<dyn HandlerTrait>, Error> {
        let r: Box<dyn HandlerTrait + 'static> = match self {
            Handler::ConsoleHandler => Box::new({
                match ConsoleHandler::new(name) {
                    Ok(h) => h,
                    Err(e) => return Err(e),
                }
            }),
            Handler::FileHandler => Box::new({
                match FileHandler::new(name) {
                    Ok(h) => h,
                    Err(e) => return Err(e),
                }
            }),
        };

        Ok(r)
    }
}

pub(crate) trait HandlerTrait: Display + Send + Sync {
    /**
     * Create a new handler instance.
     *
     * **name**: Used to identify handler.
     */
    fn new(name: String) -> Result<Self, Error>
    where
        Self: Sized;

    /**
     * Close the Handler and free all associated resources.
     *
     * The close method will perform a flush and then close the Handler.
     * After close has been called this Handler should no longer be used.
     * Method calls may either be silently ignored or may return `Error`s.
     */
    fn close(&mut self);

    /**
     * Flush any buffered output.
     */
    fn flush(&mut self);

    /**
     * Return the format String for this Handler.
     */
    fn get_formatter(&self) -> Formatter;

    /**
     * Get the log level specifying which messages will be logged by this Handler.
     */
    fn get_level(&self) -> Level;

    /**
     * Check if this Handler would actually log a given LogEntry.
     */
    fn is_loggable(&self, log_entry: LogEntry) -> bool;

    /**
     * Publish a LogEntry.
     *
     * The logging request was made initially to a Logger object, which initialized
     * the LogEntry and forwarded it here.
     *
     * The Handler is responsible for formatting the message, when and if necessary.
     */
    fn publish(&mut self, log_entry: LogEntry);

    /**
     * Set a Format.
     */
    fn set_formatter(&mut self, format: Formatter);

    /**
    * Set the log level specifying which message levels will be logged by this Handler.
     */
    fn set_level(&mut self, level: Level);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn file_handler() {
        let name = "temp.txt".to_string();
        let h = Handler::FileHandler;
        let fh = h.new(name);

        println!("\n{}\n", fh.unwrap());
    }

    #[test]
    fn file_handler_error() {
        let name = "".to_string();
        let h = Handler::FileHandler;
        let fh = h.new(name);

        assert!(fh.is_err());
    }
}
