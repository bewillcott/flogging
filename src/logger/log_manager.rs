//
// File Name:    log_manager.rs
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
//! # Log Manager
//!
//! There is a single global LogManager object that is used to maintain a set of
//! shared state about Loggers and log services.
//!

#![allow(unused)]

use crate::{handlers::handler::HandlerTrait, logger::Logger};
use std::{
    collections::HashMap,
    ops::DerefMut,
    sync::{Arc, LazyLock, Mutex},
};

pub(crate) static LOG_MANAGER: LazyLock<Arc<Mutex<LogManager>>> =
    LazyLock::new(|| Arc::new(Mutex::new(LogManager::new())));

pub(crate) struct LogManager {
    loggers: HashMap<String, Logger>,
    handlers: HashMap<String, Box<dyn HandlerTrait>>,
    properties: HashMap<String, String>,
}

impl LogManager {
    fn new() -> LogManager {
        LogManager {
            loggers: HashMap::new(),
            handlers: HashMap::new(),
            properties: HashMap::new(),
        }
    }

    /// Add a named logger.
    ///
    /// This does nothing and returns false if a logger with the same name is already registered.
    ///
    /// The Logger factory methods call this method to register each newly created Logger.
    pub(crate) fn add_logger(&mut self, name: String, logger: Logger) -> bool {
        if !self.loggers.contains_key(&name) {
            self.loggers.insert(name, logger);
            true
        } else {
            false
        }
    }

    /// Method to find a named logger.
    ///
    /// Returns matching logger or None.
    pub(crate) fn get_logger(&mut self, name: String) -> Option<&Logger> {
        self.loggers.get(&name)
    }
}
