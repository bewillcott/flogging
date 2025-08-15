//
// File Name:    mod.rs
// Directory:    src/logger
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
//! `Logger` is the work-horse of the crate.
//!
//! It contains the primary functions to both create a logger instance, and has
//! the methods to add log messages at various log levels.
//!

#![allow(clippy::needless_doctest_main)]

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

pub use builder::*;
pub use level::Level;
pub use log_entry::LogEntry;

use crate::{Handler, HandlerTrait};

///
/// This is the work-horse, providing the primary methods of the crate.
///
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
    /// Create a new Logger instance.
    ///
    /// Logging level is set to it's default setting (INFO).
    ///
    /// No `handlers` are set. Use the various methods of
    /// [`LoggerBuilder`] to configure the new `Logger`.
    ///
    /// ## Parameters
    /// - `mod_path` - The module path. Can be set with: [`module_path!()`]
    ///
    /// Returns a `LoggerBuilder` for further configuring.
    ///
    /// ## Examples
    /// ```
    /// extern crate flogging;
    /// use flogging::*;
    ///
    /// let mut log = Logger::builder(module_path!())
    ///     .add_console_handler()
    ///     .build();
    /// ```
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
    /// - `msg` - The string message.
    ///
    /// ## Examples
    /// ```
    /// extern crate flogging;
    /// use flogging::*;
    ///
    /// let mut log = Logger::console_logger(module_path!());
    /// log.set_level(Level::CONFIG);
    /// log.config("Some text to store.");
    /// ```
    ///
    pub fn config(&mut self, msg: &str) {
        self.log(Level::CONFIG, &self.fn_name(), msg);
    }

    ///
    /// Create new Logger instance, with a `ConsoleHandler`, output
    /// set to: [`std::io::stdout`].
    ///
    /// Logging level is set to it's default setting (INFO).
    ///
    /// ## Parameters
    /// - `mod_path`- The module path. Suggest using [`module_path!()`].
    ///
    /// Returns a configured `Logger`.
    ///
    /// ## Examples
    /// ```
    /// extern crate flogging;
    /// use flogging::*;
    ///
    /// let mut log = Logger::console_logger(module_path!());
    /// log.set_fn_name("main");
    ///
    /// log.warning("Don't over do it.");
    /// ```
    /// Output to stdout:
    /// ```text
    /// |flogging->main| [WARNING] Don't over do it.
    /// ```
    ///
    pub fn console_logger(mod_path: &str) -> Logger {
        Logger::builder(mod_path).add_console_handler().build()
    }

    ///
    /// Create new Logger instance, with a `ConsoleHandler`, output
    /// set to: [`std::io::stderr`].
    ///
    /// Logging level is set to it's default setting (INFO).
    ///
    /// ## Parameters
    /// - `mod_path`- The module path. Suggest using [`module_path!()`].
    ///
    /// Returns a configured `Logger`.
    ///
    /// ## Examples
    /// ```
    /// extern crate flogging;
    /// use flogging::*;
    ///
    /// let mut log = Logger::econsole_logger(module_path!());
    /// log.set_fn_name("main");
    ///
    /// log.warning("Don't over do it.");
    /// ```
    /// Output to stderr:
    /// ```text
    /// |flogging->main| [WARNING] Don't over do it.
    /// ```
    ///
    pub fn econsole_logger(mod_path: &str) -> Logger {
        Logger::builder(mod_path).add_econsole_handler().build()
    }

    ///
    /// Create new Logger instance, with a custom handler.
    ///
    /// ## Parameters
    /// - `mod_path`- The module path. Suggest using [`module_path!()`].
    /// - `label` - Unique label for this custom handler.
    /// - `custom` - The boxed custom handler.
    ///
    /// Returns a configured `Logger`.
    ///
    /// ## Examples
    /// ```
    /// extern crate flogging;
    /// use flogging::*;
    ///
    /// let mut log = Logger::custom_logger(
    ///     module_path!(),
    ///     "MockHandler",
    ///     Box::new(MockHandler::create("Some text").unwrap()),
    /// );
    /// log.set_fn_name("main");
    ///
    /// log.warning("Don't over do it.");
    /// ```
    /// [`MockHandler`] doesn't publish anything, as [`publish()`][MockHandler::publish()] is a **NoOp** method.
    /// It is used here to make the example work.
    ///
    /// However, it is expected that _your_ custom handler will do a little more.
    ///
    pub fn custom_logger(mod_path: &str, label: &str, custom: Box<dyn HandlerTrait>) -> Logger {
        Logger::builder(mod_path)
            .add_custom_handler(label, custom)
            .build()
    }

    ///
    /// Log a method entry.
    ///
    /// This is a convenience method that can be used to log entry to a method.
    /// A `LogEntry` with message "Entry" and log level FINER, is logged.
    ///
    /// ## Examples
    /// ```
    /// mod my_mod {
    ///     extern crate flogging;
    ///     use flogging::*;
    ///     use std::cell::{LazyCell, RefCell};
    ///
    ///     // Setting up the module level logger.
    ///     const LOGGER: LazyCell<RefCell<Logger>> = LazyCell::new(|| {
    ///         RefCell::new({
    ///             Logger::builder(module_path!())
    ///                 .add_console_handler()
    ///                 .set_level(Level::FINEST)
    ///                 .build()
    ///         })
    ///     });
    ///
    ///     pub fn my_func(data: &str) {
    ///         let binding = LOGGER;
    ///         let mut log = binding.borrow_mut();
    ///         log.set_fn_name("my_func");
    ///
    ///         log.entering();
    ///     }
    /// }
    ///
    /// fn main() {
    ///     let data = "Some data";
    ///     my_mod::my_func(data);
    /// }
    /// ```
    /// Output:
    /// ```text
    /// |flogging::my_mod->my_func| [FINER  ] Entry
    /// ```
    ///
    pub fn entering(&mut self) {
        self.log(Level::FINER, &self.fn_name(), "Entry");
    }

    ///
    /// Log a method entry.
    ///
    /// This is a convenience method that can be used to log entry to a method.
    /// A `LogEntry` with message "Entry" and log level FINER, is logged.
    ///
    /// ## Parameters
    /// - `msg` - The string message.
    ///
    /// ## Examples
    /// ```
    /// mod my_mod {
    ///     extern crate flogging;
    ///     use flogging::*;
    ///     use std::cell::{LazyCell, RefCell};
    ///
    ///     // Setting up the module level logger.
    ///     const LOGGER: LazyCell<RefCell<Logger>> = LazyCell::new(|| {
    ///         RefCell::new({
    ///             Logger::builder(module_path!())
    ///                 .add_console_handler()
    ///                 .set_level(Level::FINEST)
    ///                 .build()
    ///         })
    ///     });
    ///
    ///     pub fn my_func(data: &str) {
    ///         let binding = LOGGER;
    ///         let mut log = binding.borrow_mut();
    ///         log.set_fn_name("my_func");
    ///
    ///         log.entering_with(&format!("data: \"{data}\""));
    ///     }
    /// }
    ///
    /// fn main() {
    ///     let data = "Some data";
    ///     my_mod::my_func(data);
    /// }
    /// ```
    /// Output:
    /// ```text
    /// |flogging::my_mod->my_func| [FINER  ] Entry: (data: "Some data")
    /// ```
    ///
    pub fn entering_with(&mut self, msg: &str) {
        self.log(
            Level::FINER,
            &self.fn_name(),
            &("Entry: (".to_string() + msg + ")"),
        );
    }

    ///
    /// Log a method return.
    ///
    /// This is a convenience method that can be used to log returning from a method.
    /// A `LogEntry` with message "Return" and log level FINER, is logged.
    ///
    /// ## Examples
    /// ```
    /// mod my_mod {
    ///     extern crate flogging;
    ///     use flogging::*;
    ///     use std::cell::{LazyCell, RefCell};
    ///
    ///     // Setting up the module level logger.
    ///     const LOGGER: LazyCell<RefCell<Logger>> = LazyCell::new(|| {
    ///         RefCell::new({
    ///             Logger::builder(module_path!())
    ///                 .add_console_handler()
    ///                 .set_level(Level::FINEST)
    ///                 .build()
    ///         })
    ///     });
    ///
    ///     pub fn my_func(data: &str) {
    ///         let binding = LOGGER;
    ///         let mut log = binding.borrow_mut();
    ///         log.set_fn_name("my_func");
    ///
    ///         log.exiting();
    ///     }
    /// }
    ///
    /// fn main() {
    ///     let data = "Some data";
    ///     my_mod::my_func(data);
    /// }
    /// ```
    /// Output:
    /// ```text
    /// |flogging::my_mod->my_func| [FINER  ] Return
    /// ```
    ///
    pub fn exiting(&mut self) {
        self.log(Level::FINER, &self.fn_name(), "Return");
    }

    ///
    /// Log a method return.
    ///
    /// This is a convenience method that can be used to log returning from a method.
    /// A `LogEntry` with message "Return" and log level FINER, is logged.
    ///
    /// ## Parameters
    /// - `msg` - The string message.
    ///
    /// ## Examples
    /// ```
    /// mod my_mod {
    ///     extern crate flogging;
    ///     use flogging::*;
    ///     use std::cell::{LazyCell, RefCell};
    ///
    ///     // Setting up the module level logger.
    ///     const LOGGER: LazyCell<RefCell<Logger>> = LazyCell::new(|| {
    ///         RefCell::new({
    ///             Logger::builder(module_path!())
    ///                 .add_console_handler()
    ///                 .set_level(Level::FINEST)
    ///                 .build()
    ///         })
    ///     });
    ///
    ///     pub fn my_func(data: &str) -> bool {
    ///         let binding = LOGGER;
    ///         let mut log = binding.borrow_mut();
    ///         log.set_fn_name("my_func");
    ///
    ///         let rtn = true;
    ///         log.exiting_with(&format!("rtn: {rtn}"));
    ///         rtn
    ///     }
    /// }
    ///
    /// fn main() {
    ///     let data = "Some data";
    ///     my_mod::my_func(data);
    /// }
    /// ```
    /// Output:
    /// ```text
    /// |flogging::my_mod->my_func| [FINER  ] Return: (rtn: true)
    /// ```
    ///
    pub fn exiting_with(&mut self, msg: &str) {
        self.log(
            Level::FINER,
            &self.fn_name(),
            &("Return: (".to_string() + msg + ")"),
        );
    }

    ///
    /// Create new Logger instance, with a `FileHandler`.
    ///
    /// Logging level is set to it's default setting (INFO).
    ///
    /// ## Parameters
    /// - `mod_path`- The module path. Suggest using [`std::module_path`][mp].
    /// - `filename` - The name of the log file to use. Will be created
    ///   if it doesn't exist.
    ///
    /// Returns a configured `Logger`.
    ///
    /// ## Examples
    /// ```
    /// extern crate flogging;
    /// use flogging::Logger;
    ///
    /// let mut log = Logger::file_logger(module_path!(), "test.log");
    /// log.set_fn_name("main");
    ///
    /// log.info("Some text to store.");
    /// ```
    /// Output:
    /// ```text
    /// 2025-07-18T12:14:47.322720683+08:00 |flogging->main| [INFO   ] Some text to store.
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
    /// - `msg` - The string message.
    ///
    /// ## Examples
    /// ```
    /// extern crate flogging;
    /// use flogging::*;
    ///
    /// let mut log = Logger::console_logger(module_path!());
    /// log.set_level(Level::FINEST);
    /// log.set_fn_name("main");
    ///
    /// log.fine("Some text to store.");
    /// ```
    /// Output:
    /// ```text
    /// |flogging->main| [FINE   ] Some text to store.
    /// ```
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
    /// - `msg` - The string message.
    ///
    /// ## Examples
    /// ```
    /// extern crate flogging;
    /// use flogging::*;
    ///
    /// let mut log = Logger::console_logger(module_path!());
    /// log.set_level(Level::FINEST);
    /// log.set_fn_name("main");
    ///
    /// log.finer("Some text to store.");
    /// ```
    /// Output:
    /// ```text
    /// |flogging->main| [FINER  ] Some text to store.
    /// ```
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
    /// - `msg` - The string message.
    ///
    /// ## Examples
    /// ```
    /// extern crate flogging;
    /// use flogging::*;
    ///
    /// let mut log = Logger::console_logger(module_path!());
    /// log.set_level(Level::FINEST);
    /// log.set_fn_name("main");
    ///
    /// log.finest("Some text to store.");
    /// ```
    /// Output:
    /// ```text
    /// |flogging->main| [FINEST ] Some text to store.
    /// ```
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
    /// ## Parameters
    /// - `handler` - The enum of the required handler.
    ///
    /// Returns Some boxed handler, or None.
    ///
    /// ## Examples
    /// ```
    /// extern crate flogging;
    /// use flogging::*;
    ///
    /// let mut log = Logger::string_logger(module_path!());
    /// log.set_fn_name("get_handler");
    ///
    /// log.info("Some text to store.");
    ///
    /// let h = log.get_handler(Handler::String).unwrap();
    /// println!("{h}");
    /// ```
    pub fn get_handler(&mut self, handler: Handler) -> Option<Box<&mut dyn HandlerTrait>> {
        match self.handlers.get_mut().get_mut(&handler) {
            Some(val) => Some(Box::new(&mut **val)),
            None => None,
        }
    }

    ///
    /// Check if the required `Handler` has been added to this `Logger`.
    ///
    /// ## Parameters
    /// - `handler` - The enum of the required handler.
    ///
    /// Returns `true` if it exists, `false` otherwise.
    ///
    /// ## Examples
    /// ```
    /// extern crate flogging;
    /// use flogging::*;
    ///
    /// let mut log = Logger::string_logger(module_path!());
    /// log.info("Some text to store.");
    ///
    /// println!("This logger has a 'StringHandler': {}", log.has_handler(Handler::String));
    /// ```
    pub fn has_handler(&self, handler: Handler) -> bool {
        self.handlers.borrow().contains_key(&handler)
    }
    ///
    /// Log a INFO message.
    ///
    /// If the logger is currently enabled for the INFO message level
    /// then the given message is forwarded to all the registered output
    /// Handler objects.
    ///
    /// ## Parameters
    /// - `msg` - The string message.
    ///
    /// ## Examples
    /// ```
    /// extern crate flogging;
    /// use flogging::*;
    ///
    /// let mut log = Logger::console_logger(module_path!());
    /// log.set_level(Level::FINEST);
    /// log.set_fn_name("main");
    ///
    /// log.info("Some text to store.");
    /// ```
    /// Output:
    /// ```text
    /// |flogging->main| [INFO   ] Some text to store.
    /// ```
    ///
    pub fn info(&mut self, msg: &str) {
        self.log(Level::INFO, &self.fn_name(), msg);
    }

    ///
    /// Check if a message of the given level would actually be logged by this logger.
    ///
    /// ## Parameters
    /// - `level` - The level to compare with.
    ///
    /// Returns `true` if it is loggable, `false` if not.
    ///
    fn is_loggable(&self, level: &Level) -> bool {
        *level >= self.level
    }

    ///
    /// Checks whether or not this logger is processing log requests.
    ///
    /// Returns `true` if it is, `false` if not.
    ///
    /// ## Examples
    /// ```
    /// extern crate flogging;
    /// use flogging::*;
    ///
    /// let mut log = Logger::console_logger(module_path!());
    /// log.set_level(Level::OFF);
    /// log.set_fn_name("main");
    ///
    /// let msg = "The program might become unstable.";
    /// log.warning(msg);
    ///
    /// if !log.is_logging() {
    ///     eprintln!{"{msg}"};
    /// }
    /// ```
    /// Output to [`std::io::stderr`]:
    /// ```text
    /// The program might become unstable.
    /// ```
    ///
    pub fn is_logging(&self) -> bool {
        self.level() != &Level::OFF
    }

    ///
    /// Obtain the current logging level for this Log instance.
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
    /// - `entry` - The `LogEntry` to be published.
    ///
    fn _log(&mut self, entry: &mut LogEntry) {
        entry.set_mod_path(self.mod_path.clone());

        for handler in self.handlers.get_mut() {
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
    /// - `level` - One of the message level identifiers, e.g., SEVERE.
    /// - `fn_name` - The name of the function/method from-which this method
    ///   was called.
    /// - `msg` - The string message.
    ///
    fn log(&mut self, level: Level, fn_name: &str, msg: &str) {
        if !self.is_loggable(&level) {
            return;
        }

        // build LogEntry
        let mut log_entry = LogEntry::create(level, fn_name.to_string(), msg.to_string());
        // Send LogEntry
        self._log(&mut log_entry);
    }

    ///
    /// Reset this `Logger` instance's logging level.
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
    /// ## Parameters
    /// - `fn_name` - The name of the function/method in which you are
    ///   logging.
    ///
    /// Returns itself for chaining purposes.
    ///
    pub fn set_fn_name(&mut self, fn_name: &str) -> &mut Self {
        self.fn_name = fn_name.to_string();
        self
    }

    ///
    /// Set logging level for this Log instance.
    ///
    /// ## Parameters
    /// - `level` - The new logging level to set.
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
    /// - `msg` - The string message.
    ///
    /// ## Examples
    /// ```
    /// extern crate flogging;
    /// use flogging::*;
    ///
    /// let mut log = Logger::console_logger(module_path!());
    /// log.set_level(Level::FINEST);
    /// log.set_fn_name("main");
    ///
    /// log.severe("Some text to store.");
    /// ```
    /// Output:
    /// ```text
    /// |flogging->main| [SEVERE ] Some text to store.
    /// ```
    ///
    pub fn severe(&mut self, msg: &str) {
        self.log(Level::SEVERE, &self.fn_name(), msg);
    }

    ///
    /// Create new Logger instance, with a `ConsoleHandler`.
    ///
    /// Logging level is set to it's default setting (INFO).
    ///
    /// I expect this will be primarily used during unit testing of
    /// the logging output. Though, any requirement to pass-on the log entry,
    /// perhaps for further processing, would also be a valid use case.
    ///
    /// ## Parameters
    /// - `mod_path`- The module path. Suggest using [`module_path!()`].
    ///
    /// Returns a configured `Logger`.
    ///
    /// ## Examples
    /// ```
    /// extern crate flogging;
    /// use flogging::*;
    ///
    /// let mut log = Logger::string_logger(module_path!());
    /// log.set_fn_name("main");
    ///
    /// log.warning("Don't over do it.");
    ///
    /// let log_str = log.get_handler(Handler::String).unwrap().get_log();
    ///
    /// println!("{log_str}");
    /// ```
    /// Output:
    /// ```text
    /// |flogging->main| [WARNING] Don't over do it.
    /// ```
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
    /// - `msg` - The string message.
    ///
    /// ## Examples
    /// ```
    /// extern crate flogging;
    /// use flogging::*;
    ///
    /// let mut log = Logger::console_logger(module_path!());
    /// log.set_level(Level::FINEST);
    /// log.set_fn_name("main");
    ///
    /// log.warning("Some text to store.");
    /// ```
    /// Output:
    /// ```text
    /// |flogging->main| [WARNING] Some text to store.
    /// ```
    ///
    pub fn warning(&mut self, msg: &str) {
        self.log(Level::WARNING, &self.fn_name(), msg);
    }
}

impl fmt::Display for Logger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buf = String::new();

        for elem in self.handlers.borrow().iter() {
            let s = format!("{}: {}\n", elem.0, elem.1);
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
