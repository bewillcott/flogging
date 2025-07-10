/*
 * File Name:    builder.rs
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
 * # LoggerBuilder
 */

use super::*;

pub(super) struct LoggerBuilder {
    name: String,
    level: Level,
    handlers: Vec<Box<dyn HandlerTrait>>,
}

impl LoggerBuilder {
    /**
     * Create new Logger instance.
     *
     * Logging level is set to it's default setting (INFO).\
     * No `handlers` are are set.
     */
    pub(super) fn new(name: String) -> Self {
        LoggerBuilder {
            name: name,
            level: Level::default(),
            handlers: Vec::new(),
        }
    }

    pub fn set_level(&mut self, level: Level) -> &mut Self {
        self.level = level;
        self
    }

    pub fn set_handler(&mut self, handler: impl HandlerTrait + 'static) -> &mut Self {
        self.handlers.push(Box::new(handler));
        self
    }
}
