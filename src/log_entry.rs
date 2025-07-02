//
// File Name:    log_entry.rs
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
//! Log Entry
//!

#![allow(dead_code)]

use super::level::Level;
use anyhow::{Context, Error};
use std::cell::{RefCell, RefMut};
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::mpsc;

#[derive(Debug, serde::Serialize)]
pub(super) struct LogEntry {
    timestamp: String,
    level: Level,
    message: String,
}

impl fmt::Display for LogEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} : ({}) {}", self.timestamp, self.level, self.message)
    }
}

impl LogEntry {
    pub(super) fn build(timestamp: String, level: Level, message: String) -> LogEntry {
        LogEntry {
            timestamp: timestamp,
            level: level,
            message: message,
        }
    }

    pub(super) async fn process_logs(
        mut receiver: mpsc::Receiver<LogEntry>,
        log_db: Arc<sled::Db>,
    ) -> Result<(), Error> {
        while let Some(log_entry) = receiver.recv().await {
            LogEntry::write_log(log_db.clone(), log_entry).await?;
        }

        Ok(())
    }

    pub(super) fn read_log(db: Arc<sled::Db>) -> Vec<String> {
        db.into_iter()
            .values()
            .map(|v| String::from_utf8(v.unwrap().to_vec()).unwrap())
            .collect()
    }

    async fn write_log(db: Arc<sled::Db>, log_entry: LogEntry) -> Result<(), Error> {
        let key = format!("log_entry:{}", log_entry.timestamp);
        let value = serde_json::to_string(&log_entry).context("Failed to serialize log entry")?;

        db.insert(&key, value.as_bytes())
            .context("Failed to write log entry")
            .map(|_| ())
    }
}
