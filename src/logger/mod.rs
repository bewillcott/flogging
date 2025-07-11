/*
 * File Name:    mod.rs
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
 * # Logger
 */

#![allow(unused)]

mod builder;
mod level;
mod log_entry;
mod utils;

use anyhow::{Context, Error, Result};
use std::collections::HashSet;
use std::collections::hash_map::IterMut;
use std::fmt::Debug;
use std::fs::{File, exists};
use std::io::Write;
use std::marker::PhantomData;
use std::ops::DerefMut;
use std::path::Path;
use std::sync::mpsc::Sender;
use std::sync::{Arc, MutexGuard, PoisonError, mpsc};
use std::thread;

use crate::handlers::handler::{self, Handler, HandlerTrait};
use crate::logger::builder::LoggerBuilder;
pub use crate::logger::level::Level;
pub(crate) use crate::logger::log_entry::LogEntry;

const REPORT_HEADER: &str = "Log Report\n=========\n";

pub struct Logger {
    /**
     * Identify the source of log messages passed to this logger.
     */
    name: String,
    /**
     * Default level used by `log(msg)`.
     */
    level: Level,
    /**
     * Holds the handlers associated with this logger.
     */
    handlers: Vec<Box<dyn HandlerTrait>>,
}

#[allow(private_interfaces)]
impl Logger {
    /**
     * Create new Logger instance.
     *
     * Logging level is set to it's default setting (INFO).\
     * No `handlers` are are set.
     */
    pub fn builder(name: &str) -> LoggerBuilder {
        LoggerBuilder::create(name.to_string())
    }

    /**
     * Create new Logger instance, with a `ConsoleHandler`.
     *
     * Logging level is set to it's default setting (INFO).\
     * No `handlers` are are set.
     */
    pub fn console_logger(name: &str) -> Logger {
        Logger::builder("Test")
            .add_handler(Handler::ConsoleHandler, None)
            .build()
    }

    /**
     * Check if a message of the given level would actually be logged by this logger.
     */
    fn is_loggable(&self, level: &Level) -> bool {
        *level >= self.level
    }

    /**
     * Obtain the current default logging level for this Log instance.
     */
    pub fn level(&self) -> &Level {
        &self.level
    }

    /**
     * Log a `LogEntry`.
     *
     * All the other logging methods in this class call through this method to actually
     * perform any logging.
     *
     * ## Parameters
     * `entry` - The `LogEntry` to be published.
     */
    fn _log(&mut self, entry: &mut LogEntry) {
        entry.set_name(self.name.clone());

        for mut handler in &mut self.handlers {
            handler.publish(entry);
        }
    }

    /**
     * Log a message, with no arguments.
     *
     * If the logger is currently enabled for the given message level then the given
     * message is forwarded to all the registered output `Handler` objects.
     *
     * ## Parameters
     * `level` - One of the message level identifiers, e.g., SEVERE.\
     * `msg` - The string message.
     */
    pub fn log(&mut self, level: Level, msg: &str) {
        if !self.is_loggable(&level) {
            return;
        }

        let mut msg = msg.to_string();

        if msg.ends_with('\n') {
            msg.remove(msg.len() - 1);
        }

        // build LogEntry
        let mut log_entry = LogEntry::new(level, msg);
        // Send LogEntry
        self._log(&mut log_entry);
    }

    /**
     * Reset this `Logger` instance's default logging level.
     *
     * Returns itself for chaining purposes.
     *
     * See [Level]
     */
    pub fn reset_level(&mut self) -> &mut Self {
        self.level = Level::default();
        self
    }

    /**
     * Set default logging level for this Log instance.
     *
     * Returns itself for chaining purposes.
     */
    pub fn set_level(&mut self, level: Level) -> &mut Self {
        self.level = level;
        self
    }
}
