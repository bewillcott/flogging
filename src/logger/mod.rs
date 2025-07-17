//
// File Name:    mod.rs
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
//! # Logger
//!

#![allow(unused)]

mod builder;
mod level;
mod log_entry;

use anyhow::{Context, Error, Result};
use std::cell::{LazyCell, RefCell};
use std::collections::hash_map::IterMut;
use std::collections::{HashMap, HashSet};
use std::f32::consts;
use std::fmt;
use std::fs::{File, exists};
use std::io::Write;
use std::marker::PhantomData;
use std::module_path;
use std::ops::DerefMut;
use std::path::Path;
use std::sync::mpsc::Sender;
use std::sync::{Arc, MutexGuard, PoisonError, mpsc};
use std::thread;

use crate::handlers::handler::{self, Handler, HandlerTrait};
pub use crate::logger::builder::*;
pub use crate::logger::level::Level;
pub(crate) use crate::logger::log_entry::LogEntry;

pub struct Logger {
    ///
    /// Identify the source of log messages passed to this logger.
    ///
    /// This would ideally be the **mod** path.
    ///
    mod_path: String,

    ///
    /// The name of the function/method inside which the log message
    /// is generated.
    ///
    fn_name: String,

    ///
    /// Default level used by `log(msg)`.
    ///
    level: Level,

    ///
    /// Holds the handlers associated with this logger.
    ///
    handlers: RefCell<HashMap<Handler, Box<dyn HandlerTrait>>>,
}

impl Logger {
    ///
    /// Create new Logger instance.
    ///
    /// Logging level is set to it's default setting (INFO).\
    /// No `handlers` are set.
    ///
    pub fn builder(mod_path: &str) -> LoggerBuilder {
        LoggerBuilder::create(mod_path.to_string())
    }

    ///
    /// Log a CONFIG message.
    ///
    /// If the logger is currently enabled for the CONFIG message level
    /// then the given message is forwarded to all the registered output
    /// Handler objects.
    ///
    /// ## Parameters
    /// `msg` - The string message.
    ///
    /// ## Examples
    /// ```
    /// extern crate flogging;
    /// use flogging::Logger;
    ///
    /// let mut log = Logger::console_logger(module_path!());
    /// log.config("Some text to store.");
    /// ```
    ///
    ///
    pub fn config(&mut self, msg: &str) {
        self.log(Level::CONFIG, &self.fn_name(), msg);
    }

    ///
    /// Create new Logger instance, with a `ConsoleHandler`.
    ///
    /// Logging level is set to it's default setting (INFO).
    ///
    /// ## Parameters
    /// `mod_path`- The module path. Suggest using [`module_path`].
    ///
    pub fn console_logger(mod_path: &str) -> Logger {
        Logger::builder(mod_path).add_console_handler().build()
    }

    ///
    /// Log a method entry.
    ///
    /// This is a convenience method that can be used to log entry to a method.
    /// A `LogEntry` with message "Entry" and log level FINER, is logged.
    ///
    pub fn entering(&mut self) {
        self.log(Level::FINER, &self.fn_name(), "Entry");
    }

    ///
    /// Log a method return.
    ///
    /// This is a convenience method that can be used to log returning from a method.
    /// A `LogEntry` with message "Return" and log level FINER, is logged.
    ///
    pub fn exiting(&mut self) {
        self.log(Level::FINER, &self.fn_name(), "Return");
    }

    ///
    /// Create new Logger instance, with a `FileHandler`.
    ///
    /// Logging level is set to it's default setting (INFO).
    ///
    /// ## Parameters
    /// `mod_path`- The module path. Suggest using [`std::module_path`][mp].\
    /// `filename` - The name of the log file to use. Will be created
    /// if it doesn't exist.
    ///
    /// ## Examples
    /// ```
    /// extern crate flogging;
    /// use flogging::Logger;
    ///
    /// let mut log = Logger::file_logger(module_path!(), "test.log");
    /// log.info("Some text to store.");
    /// ```
    ///
    /// [mp]: https://doc.rust-lang.org/std/macro.module_path.html
    pub fn file_logger(mod_path: &str, filename: &str) -> Logger {
        Logger::builder(mod_path).add_file_handler(filename).build()
    }

    ///
    /// Log a FINE message.
    ///
    /// If the logger is currently enabled for the FINE message level
    /// then the given message is forwarded to all the registered output
    /// Handler objects.
    ///
    /// ## Parameters
    /// `msg` - The string message.
    /// was called.
    ///
    pub fn fine(&mut self, msg: &str) {
        self.log(Level::FINE, &self.fn_name(), msg);
    }

    ///
    /// Log a FINER message.
    ///
    /// If the logger is currently enabled for the FINER message level
    /// then the given message is forwarded to all the registered output
    /// Handler objects.
    ///
    /// ## Parameters
    /// `msg` - The string message.
    ///
    pub fn finer(&mut self, msg: &str) {
        self.log(Level::FINER, &self.fn_name(), msg);
    }

    ///
    /// Log a FINEST message.
    ///
    /// If the logger is currently enabled for the FINEST message level
    /// then the given message is forwarded to all the registered output
    /// Handler objects.
    ///
    /// ## Parameters
    /// `msg` - The string message.
    ///
    pub fn finest(&mut self, msg: &str) {
        self.log(Level::FINEST, &self.fn_name(), msg);
    }

    ///
    /// Get the current function/method name.
    ///
    pub fn fn_name(&self) -> String {
        self.fn_name.clone()
    }

    ///
    /// Get required `Handler`.
    ///
    /// ## Examples
    /// ```
    /// extern crate flogging;
    /// use flogging::{Logger,Handler};
    ///
    /// let mut log = Logger::string_logger(module_path!());
    /// log.info("Some text to store.");
    ///
    /// let h = log.get_handler(Handler::StringHandler);
    /// ```
    pub fn get_handler(&mut self, handler: Handler) -> Option<&dyn HandlerTrait> {
        match self.handlers.get_mut().get(&handler) {
            Some(val) => Some(&**val),
            None => None,
        }
    }

    ///
    /// Log a INFO message.
    ///
    /// If the logger is currently enabled for the INFO message level
    /// then the given message is forwarded to all the registered output
    /// Handler objects.
    ///
    /// ## Parameters
    /// `msg` - The string message.
    ///
    pub fn info(&mut self, msg: &str) {
        self.log(Level::INFO, &self.fn_name(), msg);
    }

    ///
    /// Check if a message of the given level would actually be logged by this logger.
    ///
    fn is_loggable(&self, level: &Level) -> bool {
        *level >= self.level
    }

    ///
    /// Obtain the current default logging level for this Log instance.
    ///
    pub fn level(&self) -> &Level {
        &self.level
    }

    ///
    /// Log a `LogEntry`.
    ///
    /// All the other logging methods in this class call through this method to actually
    /// perform any logging.
    ///
    /// ## Parameters
    /// `entry` - The `LogEntry` to be published.
    ///
    fn _log(&mut self, entry: &mut LogEntry) {
        entry.set_mod_path(self.mod_path.clone());

        for mut handler in self.handlers.get_mut() {
            handler.1.publish(entry);
        }
    }

    ///
    /// Log a message, with no arguments.
    ///
    /// If the logger is currently enabled for the given message level then the given
    /// message is forwarded to all the registered output `Handler` objects.
    ///
    /// ## Parameters
    /// `level` - One of the message level identifiers, e.g., SEVERE.\
    /// `fn_name` - The name of the function/method from-which this method
    /// was called.\
    /// `msg` - The string message.
    ///
    pub fn log(&mut self, level: Level, fn_name: &str, msg: &str) {
        if !self.is_loggable(&level) {
            return;
        }

        let mut msg = msg.to_string();

        if msg.ends_with('\n') {
            msg.remove(msg.len() - 1);
        }

        // build LogEntry
        let mut log_entry = LogEntry::create(level, fn_name.to_string(), msg);
        // Send LogEntry
        self._log(&mut log_entry);
    }

    ///
    /// Reset this `Logger` instance's default logging level.
    ///
    /// Returns itself for chaining purposes.
    ///
    /// See [Level]
    ///
    pub fn reset_level(&mut self) -> &mut Self {
        self.level = Level::default();
        self
    }

    ///
    /// Set the current function/method name.
    ///
    pub fn set_fn_name(&mut self, fn_name: &str) -> &mut Self {
        self.fn_name = fn_name.to_string();
        self
    }

    ///
    /// Set default logging level for this Log instance.
    ///
    /// Returns itself for chaining purposes.
    ///
    pub fn set_level(&mut self, level: Level) -> &mut Self {
        self.level = level;
        self
    }

    ///
    /// Log a SEVERE message.
    ///
    /// If the logger is currently enabled for the SEVERE message level
    /// then the given message is forwarded to all the registered output
    /// Handler objects.
    ///
    /// ## Parameters
    /// `msg` - The string message.
    ///
    pub fn severe(&mut self, msg: &str) {
        self.log(Level::SEVERE, &self.fn_name(), msg);
    }

    ///
    /// Create new Logger instance, with a `ConsoleHandler`.
    ///
    /// Logging level is set to it's default setting (INFO).
    ///
    /// ## Parameters
    /// `mod_path`- The module path. Suggest using [`module_path`].
    ///
    pub fn string_logger(mod_path: &str) -> Logger {
        Logger::builder(mod_path).add_string_handler().build()
    }

    ///
    /// Log a WARNING message.
    ///
    /// If the logger is currently enabled for the WARNING message level
    /// then the given message is forwarded to all the registered output
    /// Handler objects.
    ///
    /// ## Parameters
    /// `msg` - The string message.
    ///
    pub fn warning(&mut self, msg: &str) {
        self.log(Level::WARNING, &self.fn_name(), msg);
    }
}

impl fmt::Display for Logger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buf = String::new();

        for elem in self.handlers.borrow().iter() {
            let s = format!("{:?}: {}\n", elem.0, elem.1);
            buf.push_str(&s);
        }

        writeln!(
            f,
            "{}::{} - [{}]\n\n{}",
            self.mod_path, self.fn_name, self.level, buf
        )
    }
}

#[cfg(test)]
mod tests;
