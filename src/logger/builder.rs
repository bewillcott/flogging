//
// File Name:    builder.rs
// Project Name: flogging
//
// Copyright (C) 2025 Bradley Willcott
//
// SPDX-License-Identifier: GPL-2.0-or-later
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
//! # LoggerBuilder
//!

use std::{cell::RefCell, collections::HashMap};

use crate::handlers::{formatter::Formatter, string_handler::StringHandler};

use super::{Handler, HandlerTrait, Level, Logger};

pub struct LoggerBuilder {
    name: String,
    level: Level,
    handlers: RefCell<HashMap<Handler, Box<dyn HandlerTrait>>>,
}

impl LoggerBuilder {
    pub(super) fn create(name: String) -> Self {
        LoggerBuilder {
            name,
            level: Level::default(),
            handlers: RefCell::new(HashMap::new()),
        }
    }

    pub fn add_console_handler(mut self) -> Self {
        self.add_handler_with(Handler::ConsoleHandler, None, None)
    }

    pub fn add_console_handler_with(mut self, formatter: Formatter) -> Self {
        self.add_handler_with(Handler::ConsoleHandler, None, Some(formatter))
    }

    pub fn add_file_handler(mut self, filename: &str) -> Self {
        self.add_handler_with(Handler::FileHandler, Some(filename), None)
    }

    pub fn add_file_handler_with(mut self, filename: &str, formatter: Formatter) -> Self {
        self.add_handler_with(Handler::FileHandler, Some(filename), Some(formatter))
    }

    fn add_handler_with(
        mut self,
        handler: Handler,
        filename: Option<&str>,
        formatter: Option<Formatter>,
    ) -> Self {
        let name = filename.unwrap_or(&self.name);
        let mut h = handler.create(name).unwrap();

        if let Some(f) = formatter {
            h.set_formatter(f);
        }

        let map = self.handlers.get_mut();
        map.insert(handler, h);

        self
    }

    pub fn add_string_handler(mut self) -> Self {
        self.add_handler_with(Handler::StringHandler, None, None)
    }

    pub fn add_string_handler_with(mut self, formatter: Formatter) -> Self {
        self.add_handler_with(Handler::StringHandler, None, Some(formatter))
    }

    pub fn build(self) -> Logger {
        Logger {
            mod_path: self.name.clone(),
            fn_name: String::new(),
            level: self.level.clone(),
            handlers: self.handlers,
        }
    }

    pub fn set_level(mut self, level: Level) -> Self {
        self.level = level;
        self
    }
}
