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

use crate::handlers::formatter::Formatter;

use super::{Handler, HandlerTrait, Level, Logger};

pub struct LoggerBuilder {
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
    pub(super) fn create(name: String) -> Self {
        LoggerBuilder {
            name,
            level: Level::default(),
            handlers: Vec::new(),
        }
    }

    pub fn set_level(mut self, level: Level) -> Self {
        self.level = level;
        self
    }

    pub fn add_handler(mut self, handler: Handler, filename: Option<&str>) -> Self {
        self.add_handler_with(handler, filename, None)
    }

    pub fn add_handler_with(
        mut self,
        handler: Handler,
        filename: Option<&str>,
        formatter: Option<Formatter>,
    ) -> Self {
        let name = filename.unwrap_or(&self.name);
        let mut h = handler.create(name).unwrap();

        if let Some(f)= formatter{
            h.set_formatter(f);
        }

        self.handlers.push(h);
        self
    }

    pub fn build(self) -> Logger {
        Logger {
            name: self.name.clone(),
            level: self.level.clone(),
            handlers: self.handlers,
        }
    }
}
