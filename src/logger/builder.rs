//
// File Name:    builder.rs
// Project Name: flogging
//
// Copyright (C) 2025 Bradley Willcott
//
// SPDX-License-Identifier: GPL-3.0-or-later
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

use super::{Handler, HandlerTrait, Level, Logger};
use crate::{
    ConsoleHandler, FileHandler,
    handlers::{
        formatter::{FormatType, Formatter, format_trait::FormatTrait},
        string_handler::StringHandler,
    },
};
use std::{cell::RefCell, collections::HashMap};

pub struct LoggerBuilder {
    name: String,
    level: Level,
    handlers: RefCell<HashMap<Handler, Box<dyn HandlerTrait>>>,
}

#[allow(unused)]
impl LoggerBuilder {
    pub(super) fn create(name: String) -> Self {
        LoggerBuilder {
            name,
            level: Level::default(),
            handlers: RefCell::new(HashMap::new()),
        }
    }

    pub fn add_console_handler(self) -> Self {
        self.add_handler_with(Handler::Console, None, None, None, None)
    }

    pub fn add_console_handler_with(
        self,
        formatter: FormatType,
        custom_formatter: Option<Box<dyn FormatTrait>>,
    ) -> Self {
        self.add_handler_with(
            Handler::Console,
            None,
            None,
            Some(formatter),
            custom_formatter,
        )
    }

    pub fn add_custom_handler(self, label: &str, custom: Box<dyn HandlerTrait>) -> Self {
        self.add_handler_with(
            Handler::Custom(label.to_string()),
            Some(custom),
            None,
            None,
            None,
        )
    }

    fn add_custom_handler_with(
        self,
        label: &str,
        custom: Box<dyn HandlerTrait>,
        formatter: FormatType,
        custom_formatter: Option<Box<dyn FormatTrait>>,
    ) -> Self {
        self.add_handler_with(
            Handler::Custom(label.to_string()),
            Some(custom),
            None,
            Some(formatter),
            custom_formatter,
        )
    }

    pub fn add_file_handler(self, filename: &str) -> Self {
        self.add_handler_with(Handler::File, None, Some(filename), None, None)
    }

    pub fn add_file_handler_with(
        self,
        filename: &str,
        formatter: FormatType,
        custom_formatter: Option<Box<dyn FormatTrait>>,
    ) -> Self {
        self.add_handler_with(
            Handler::File,
            None,
            Some(filename),
            Some(formatter),
            custom_formatter,
        )
    }

    pub fn add_handler_with(
        mut self,
        handler: Handler,
        custom_handler: Option<Box<dyn HandlerTrait>>,
        filename: Option<&str>,
        formatter: Option<FormatType>,
        custom_formatter: Option<Box<dyn FormatTrait>>,
    ) -> Self {
        let name = filename.unwrap_or(&self.name);
        let mut h: Box<dyn HandlerTrait> = match handler {
            Handler::Console => Box::new(ConsoleHandler::create(name).unwrap()),
            Handler::File => Box::new(FileHandler::create(name).unwrap()),
            Handler::String => Box::new(StringHandler::create(name).unwrap()),
            Handler::Custom(_) => custom_handler.unwrap(),
        };

        if let Some(f) = formatter {
            h.set_formatter(match f {
                FormatType::Iso8601 => Default::default(),
                FormatType::Simple => Default::default(),
                FormatType::UnixTimestamp => Default::default(),
                FormatType::Custom(_) => Formatter::Custom(custom_formatter.unwrap()),
            });
        }

        let map = self.handlers.get_mut();
        map.insert(handler, h);

        self
    }

    pub fn add_string_handler(self) -> Self {
        self.add_handler_with(Handler::String, None, None, None, None)
    }

    pub fn add_string_handler_with(
        self,
        formatter: FormatType,
        custom_formatter: Option<Box<dyn FormatTrait>>,
    ) -> Self {
        self.add_handler_with(
            Handler::String,
            None,
            None,
            Some(formatter),
            custom_formatter,
        )
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

#[cfg(test)]
mod tests {

    use crate::{ConsoleHandler, HandlerTrait, Logger};

    use super::LoggerBuilder;

    #[test]
    fn builder() {
        let mut log = Logger::builder(module_path!())
            .add_custom_handler("Console", Box::new(ConsoleHandler::create("test").unwrap()))
            .build();

        log.info("We begin!");
    }
}
