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
//! There is a single global `LogManager` object that is used to maintain a set of
//! shared state about `Logger`s and log services.
//!

#![allow(unused)]

use crate::{
    handlers::handler::{Handler, HandlerTrait},
    logger::Logger,
};
use std::{
    cell::LazyCell,
    collections::HashMap,
    marker::PhantomData,
    ops::DerefMut,
    ptr::NonNull,
    rc::Rc,
    sync::{Arc, LazyLock, Mutex},
};

pub(crate) struct LogManager<'a> {
    loggers: HashMap<String, Box<Logger<'a>>>,
    handlers: HashMap<String, Box<dyn HandlerTrait>>,
    properties: HashMap<String, String>,
}

impl<'a> LogManager<'a> {
    pub(crate) fn new() -> LogManager<'a> {
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
    /// The `Logger` factory methods call this method to register each newly created Logger.
  pub(crate)  fn add_logger(&mut self, name: &'a String, logger: Logger<'a>) -> bool {
        if !self.loggers.contains_key(name) {
            unsafe {
                let log = Box::new(logger);
                self.loggers.insert(name.clone(), log);
            }
            true
        } else {
            false
        }
    }

    /// Method to find a named logger.
    ///
    /// Returns matching logger or `None`.
    pub(crate) fn get_logger(&'a mut self, name: &'a String) -> Option<&'a mut Logger<'a>> {
        unsafe {
            match self.loggers.get_mut(name) {
                Some(logger) => unsafe {
                    let mut r = logger.as_mut();
                    Some(r)
                },
                None => None,
            }
        }
    }

    /// Add a new handler.
    ///
    /// This does nothing and returns false if a logger with the same name is already registered.
    ///
    /// The `Logger` factory methods call this method to register each newly created Logger.
    pub(crate) fn add_handler(
        &mut self,
        name: String,
        handler: Box<dyn HandlerTrait + 'static>,
    ) -> bool {
        if !self.handlers.contains_key(&name) {
            self.handlers.insert(name.clone(), handler);
            true
        } else {
            false
        }
    }

    /// Method to find a named handler.
    ///
    /// Returns matching handler or `None`.
    pub(crate) fn get_handler(&mut self, name: String) -> Option<&Box<dyn HandlerTrait + 'static>> {
        self.handlers.get(&name)
    }
}

impl<'a> Default for LogManager<'a> {
    fn default() -> Self {
        Self::new()
    }
}
