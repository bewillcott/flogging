//!
//! Log
//!

#![allow(unused)]

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

const REPORT_HEADER: &str = "Log Report\n=========\n";

pub struct Log {
    db: Arc<sled::Db>,
    level: Level,
    tx: Arc<Sender<LogEntry>>,
}

impl Log {
    /// Create new Log instance, opening the log file (name as supplied).\
    /// Logging level is set to it's default setting (INFO).
    pub fn new(log_file_name: &str) -> Result<Log, Error> {
        let (sender, receiver) = mpsc::channel(10);
        let db_open = Arc::new(open_db(log_file_name)?);

        let log = Log {
            db: db_open.clone(),
            level: Level::default(),
            tx: Arc::new(sender),
        };

        task::spawn(LogEntry::process_logs(receiver, db_open.clone()));
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
    /// See [Log::set_level]
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

        let mut log_strings = LogEntry::read_log(self.db.clone());

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

/// Open or create a database at the given path.
fn open_db(path: &str) -> Result<sled::Db, Error> {
    Config::new()
        .path(path)
        .open()
        .context("Failed to open database")
}
