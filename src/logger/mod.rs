//
// File Name:    logger.rs
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
//!
//! Logger
//!

#![allow(unused)]

pub mod level;
pub(crate) mod log_entry;
mod log_manager;
mod utils;

use anyhow::{Context, Error, Result};
use std::collections::HashSet;
use std::fmt::Debug;
use std::fs::{File, exists};
use std::io::Write;
use std::ops::DerefMut;
use std::path::Path;
use std::sync::mpsc::Sender;
use std::sync::{Arc, MutexGuard, PoisonError, mpsc};
use std::thread;

use crate::handlers::handler::Handler;
use crate::logger::level::Level;
use crate::logger::log_entry::LogEntry;
use crate::logger::log_manager::{LOG_MANAGER, LogManager};

const REPORT_HEADER: &str = "Log Report\n=========\n";

#[derive(Debug, Clone)]
pub struct Logger {
    name: String,
    level: Level,
    handlers: Vec<Box<Handler>>,
}

impl Logger {
    /// Create new Log instance, opening the log file (name as supplied).\
    /// Logging level is set to it's default setting (INFO).
    pub(crate) fn new(name: &str) -> Logger {
        let v: Vec<Box<Handler>> = Vec::new();

        Logger {
            name: name.to_string(),
            level: Level::default(),
            handlers: Vec::new(),
        }
    }

    /// Find or create a logger for a named subsystem.
    ///
    /// If a logger has already been created with the given name it is returned.
    /// Otherwise a new logger is created.
    pub fn get_logger(name: String) -> Logger {
        let mut mgr: MutexGuard<'_, LogManager> = LOG_MANAGER.lock().unwrap();

        match mgr.get_logger(name.clone()) {
            Some(logger) => logger.clone(),
            None => {
                let logger = Logger::new(&name);
                let rl = logger.clone();
                mgr.add_logger(name, logger);
                rl
            }
        }
    }

    /// Reset default logging level, for this Log instance,\
    /// back to it's initial setting (INFO).
    ///
    /// Returns itself for chaining purposes.
    pub fn reset_level(&mut self) -> &mut Self {
        self.level = Level::default();
        self
    }

    /// Set default logging level for this Log instance.\
    /// Returns itself for chaining purposes.
    pub fn set_level(&mut self, level: Level) -> &mut Self {
        self.level = level;
        self
    }

    /// Obtain the current default logging level for this Log instance.
    pub fn level(&self) -> Level {
        self.level.clone()
    }

    /// Log an entry.\
    /// The level is the current default level.
    ///
    /// See [Logger::set_level]
    pub async fn log(&mut self, level: Level, message: &str) -> Result<(), Error> {
        let mut msg = message.to_string();

        if msg.ends_with('\n') {
            msg.remove(msg.len() - 1);
        }

        // build LogEntry
        let entry = LogEntry::new(self.level.clone(), msg);
        // Send LogEntry
        // self.tx.send(entry)?;
        Ok(())
    }

    /// Clear the log database of all records.
    pub fn clear(&self) -> Result<(), Error> {
        // self.db.clear()?;
        Ok(())
    }

    /// Process the log database, producing both an external text file (`filename`)\
    /// and a `String` which is returned.
    pub fn report(&self, filename: &str) -> Result<Option<Vec<String>>, Error> {
        // if self.db.is_empty() {
        //     return Ok(None);
        // }

        // let mut log_strings = read_log(self.db.clone());

        // if !log_strings.is_empty() {
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

        //     Ok(Some(log_strings))
        // } else {
        Ok(None)
        // }
    }
}
