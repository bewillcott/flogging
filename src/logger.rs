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

mod utils;

use super::level::Level;
use anyhow::{Context, Error, Result};
use sled::Config;
use std::fs::{File, exists};
use std::io::Write;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::task::{self, JoinHandle};

use crate::log_entry::LogEntry;
use crate::logger::utils::{open_db, process_logs, read_log};

const REPORT_HEADER: &str = "Log Report\n=========\n";

pub struct Logger {
    db: Arc<sled::Db>,
    level: Level,
    tx: Arc<Sender<LogEntry>>,
}

impl Logger {
    /// Create new Log instance, opening the log file (name as supplied).\
    /// Logging level is set to it's default setting (INFO).
    pub fn new(log_file_name: &str) -> Result<Logger, Error> {
        let (sender, receiver) = mpsc::channel(10);
        let db_open = Arc::new(open_db(log_file_name)?);

        let log = Logger {
            db: db_open.clone(),
            level: Level::default(),
            tx: Arc::new(sender),
        };

        task::spawn(process_logs(receiver, db_open.clone()));
        Ok(log)
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
    pub async fn log(&mut self, message: &str) -> Result<(), Error> {
        let timestamp = chrono::Local::now().to_rfc3339();
        let mut msg = message.to_string();

        if msg.ends_with('\n') {
            msg.remove(msg.len() - 1);
        }

        // build LogEntry
        let entry = LogEntry::build(timestamp, self.level.clone(), msg);
        // Send LogEntry
        self.tx.send(entry).await?;
        Ok(())
    }

    /// Clear the log database of all records.
    pub fn clear(&self) -> Result<(), Error> {
        self.db.clear()?;
        Ok(())
    }

    /// Process the log database, producing both an external text file (`filename`)\
    /// and a `String` which is returned.
    pub fn report(&self, filename: &str) -> Result<Option<Vec<String>>, Error> {
        if self.db.is_empty() {
            return Ok(None);
        }

        let mut log_strings = read_log(self.db.clone());

        if !log_strings.is_empty() {
            let mut file: File;

            if exists(filename)? {
                file = File::options().write(true).truncate(true).open(filename)?;
            } else {
                file = File::options().write(true).create(true).open(filename)?;
            }

            file.write(REPORT_HEADER.as_bytes());

            for s in &mut log_strings {
                s.push('\n');
                file.write(s.as_bytes());
            }

            file.flush();

            Ok(Some(log_strings))
        } else {
            Ok(None)
        }
    }
}
