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
        formatter::{FormatTrait, FormatType, Formatter},
        string_handler::StringHandler,
    },
};
use std::{cell::RefCell, collections::HashMap};

///
/// Used by [`Logger`] to provide more flexibility in the configuration of the
/// final logger.
///
pub struct LoggerBuilder {
    mod_path: String,
    level: Level,
    handlers: RefCell<HashMap<Handler, Box<dyn HandlerTrait>>>,
}

#[allow(unused)]
impl LoggerBuilder {
    pub(super) fn create(mod_path: String) -> Self {
        LoggerBuilder {
            mod_path,
            level: Level::default(),
            handlers: RefCell::new(HashMap::new()),
        }
    }

    ///
    /// Adds a [`ConsoleHandler`] with the default formatter,
    /// with output to: [`std::io::stdout`].
    ///
    /// ## Examples
    /// ```
    /// extern crate flogging;
    /// use flogging::*;
    ///
    /// let mut log = Logger::builder(module_path!())
    ///     .add_console_handler()
    ///     .build();
    /// ```
    ///
    pub fn add_console_handler(self) -> Self {
        self.add_handler_with(Handler::Console, None, None, None, None)
    }

    ///
    /// Adds a [`ConsoleHandler`] with the default formatter,
    /// with output to: [`std::io::stderr`].
    ///
    /// ## Examples
    /// ```
    /// extern crate flogging;
    /// use flogging::*;
    ///
    /// let mut log = Logger::builder(module_path!())
    ///     .add_econsole_handler()
    ///     .build();
    /// ```
    ///
    pub fn add_econsole_handler(self) -> Self {
        self.add_handler_with(Handler::EConsole, None, None, None, None)
    }

    ///
    /// Adds a [`ConsoleHandler`] with the required formatter,
    /// with output to: [`std::io::stdout`].
    ///
    /// ## Parameters
    /// - `format_type` - The format type used to produce the required formatter.
    /// - `custom_formatter` - The optional boxed custom formatter.
    ///   Used by the [`FormatType::Custom`] to produce a [`Formatter::Custom`].
    ///
    /// ## Examples
    /// First, using a provided formatter:
    /// ```
    /// extern crate flogging;
    /// use flogging::*;
    ///
    /// let mut log = Logger::builder(module_path!())
    ///     .add_console_handler_with(FormatType::Iso8601, None)
    ///     .build();
    /// ```
    /// Now using a custom formatter:
    /// ```
    /// extern crate flogging;
    /// use flogging::*;
    ///
    /// let mut log = Logger::builder(module_path!())
    ///     .add_console_handler_with(
    ///         FormatType::Custom("MockFormatter".to_string()),
    ///         Some(Box::new(MockFormatter::new())),
    ///     )
    ///     .build();
    /// ```
    ///
    pub fn add_console_handler_with(
        self,
        format_type: FormatType,
        custom_formatter: Option<Box<dyn FormatTrait>>,
    ) -> Self {
        self.add_handler_with(
            Handler::Console,
            None,
            None,
            Some(format_type),
            custom_formatter,
        )
    }

    ///
    /// Adds a [`ConsoleHandler`] with the required formatter,
    /// with output to: [`std::io::stderr`].
    ///
    /// ## Parameters
    /// - `format_type` - The format type used to produce the required formatter.
    /// - `custom_formatter` - The optional boxed custom formatter.
    ///   Used by the [`FormatType::Custom`] to produce a [`Formatter::Custom`].
    ///
    /// ## Examples
    /// First, using a provided formatter:
    /// ```
    /// extern crate flogging;
    /// use flogging::*;
    ///
    /// let mut log = Logger::builder(module_path!())
    ///     .add_econsole_handler_with(FormatType::Iso8601, None)
    ///     .build();
    /// ```
    /// Now using a custom formatter:
    /// ```
    /// extern crate flogging;
    /// use flogging::*;
    ///
    /// let mut log = Logger::builder(module_path!())
    ///     .add_console_handler_with(
    ///         FormatType::Custom("MockFormatter".to_string()),
    ///         Some(Box::new(MockFormatter::new())),
    ///     )
    ///     .build();
    /// ```
    ///
    pub fn add_econsole_handler_with(
        self,
        format_type: FormatType,
        custom_formatter: Option<Box<dyn FormatTrait>>,
    ) -> Self {
        self.add_handler_with(
            Handler::EConsole,
            None,
            None,
            Some(format_type),
            custom_formatter,
        )
    }

    ///
    /// Adds a custom handler with the default formatter.
    ///
    /// ## Parameters
    /// - `label` - Unique identifier for this custom handler. Used when attempting to
    ///   retrieve this handler: [`has_handler()`][Logger::has_handler], [`get_handler()`][Logger::get_handler]
    /// - `custom_handler` - The boxed custom handler.
    ///
    /// ## Examples
    /// ```
    /// extern crate flogging;
    /// use flogging::*;
    ///
    /// let mut log = Logger::builder(module_path!())
    ///     .add_custom_handler(
    ///         "MockHandler",
    ///         Box::new(MockHandler::create("What ever you need").unwrap()),
    ///     )
    ///     .build();
    /// ```
    ///
    pub fn add_custom_handler(self, label: &str, custom_handler: Box<dyn HandlerTrait>) -> Self {
        self.add_handler_with(
            Handler::Custom(label.to_string()),
            Some(custom_handler),
            None,
            None,
            None,
        )
    }

    ///
    /// Adds a custom handler with the required formatter.
    ///
    /// ## Parameters
    /// - `label` - Unique identifier for this custom handler. Used when attempting to
    ///   retrieve this handler: [`has_handler()`][Logger::has_handler], [`get_handler()`][Logger::get_handler]
    /// - `custom_handler` - The boxed custom handler.
    /// - `format_type` - The format type used to produce the required formatter.
    /// - `custom_formatter` - The optional boxed custom formatter.
    ///   Used by the [`FormatType::Custom`] to produce a [`Formatter::Custom`].
    ///
    /// ## Examples
    /// First, using a provided formatter:
    /// ```
    /// extern crate flogging;
    /// use flogging::*;
    ///
    /// let mut log = Logger::builder(module_path!())
    ///     .add_custom_handler_with(
    ///         "MockHandler",
    ///         Box::new(MockHandler::create("What ever you need").unwrap()),
    ///         FormatType::Simple,
    ///         None,
    ///     )
    ///     .build();
    /// ```
    /// Now using a custom formatter:
    /// ```
    /// extern crate flogging;
    /// use flogging::*;
    ///
    /// let mut log = Logger::builder(module_path!())
    ///     .add_custom_handler_with(
    ///         "MockHandler",
    ///         Box::new(MockHandler::create("What ever you need").unwrap()),
    ///         FormatType::Custom("MockFormatter".to_string()),
    ///         Some(Box::new(MockFormatter::new())),
    ///     )
    ///     .build();
    /// ```
    ///
    pub fn add_custom_handler_with(
        self,
        label: &str,
        custom_handler: Box<dyn HandlerTrait>,
        format_type: FormatType,
        custom_formatter: Option<Box<dyn FormatTrait>>,
    ) -> Self {
        self.add_handler_with(
            Handler::Custom(label.to_string()),
            Some(custom_handler),
            None,
            Some(format_type),
            custom_formatter,
        )
    }

    ///
    /// Adds a [`FileHandler`] with the default formatter.
    ///
    /// ## Parameters
    /// - `filename` - The name of the output log file. Must include any relevant
    ///   path (relative or absolute).
    ///
    /// ## Examples
    /// ```
    /// extern crate flogging;
    /// use flogging::*;
    ///
    /// let mut log = Logger::builder(module_path!())
    ///     .add_file_handler("mylog.txt")
    ///     .build();
    /// ```
    ///
    pub fn add_file_handler(self, filename: &str) -> Self {
        self.add_handler_with(Handler::File, None, Some(filename), None, None)
    }

    ///
    /// Adds a [`FileHandler`] with the required formatter.
    ///
    /// ## Parameters
    /// - `filename` - The name of the output log file. Must include any relevant
    ///   path (relative or absolute).
    /// - `format_type` - The format type used to produce the required formatter.
    /// - `custom_formatter` - The optional boxed custom formatter.
    ///   Used by the [`FormatType::Custom`] to produce a [`Formatter::Custom`].
    ///
    /// ## Examples
    /// First, using a provided formatter:
    /// ```
    /// extern crate flogging;
    /// use flogging::*;
    ///
    /// let mut log = Logger::builder(module_path!())
    ///     .add_file_handler_with("mylog.txt", FormatType::Iso8601, None)
    ///     .build();
    /// ```
    /// Now using a custom formatter:
    /// ```
    /// extern crate flogging;
    /// use flogging::*;
    ///
    /// let mut log = Logger::builder(module_path!())
    ///     .add_file_handler_with(
    ///         "mylog.txt",
    ///         FormatType::Custom("MockFormatter".to_string()),
    ///         Some(Box::new(MockFormatter::new())),
    ///     )
    ///     .build();
    /// ```
    ///
    pub fn add_file_handler_with(
        self,
        filename: &str,
        format_type: FormatType,
        custom_formatter: Option<Box<dyn FormatTrait>>,
    ) -> Self {
        self.add_handler_with(
            Handler::File,
            None,
            Some(filename),
            Some(format_type),
            custom_formatter,
        )
    }

    fn add_handler_with(
        mut self,
        handler: Handler,
        custom_handler: Option<Box<dyn HandlerTrait>>,
        filename: Option<&str>,
        format_type: Option<FormatType>,
        custom_formatter: Option<Box<dyn FormatTrait>>,
    ) -> Self {
        let name = filename.unwrap_or(&self.mod_path);
        let mut h: Box<dyn HandlerTrait> = match handler {
            Handler::Console => Box::new(ConsoleHandler::create("false").unwrap()),
            Handler::EConsole => Box::new(ConsoleHandler::create("true").unwrap()),
            Handler::File => Box::new(FileHandler::create(name).unwrap()),
            Handler::String => Box::new(StringHandler::create(name).unwrap()),
            Handler::Custom(_) => custom_handler.unwrap(),
        };

        if let Some(f) = format_type {
            h.set_formatter(match f {
                FormatType::Iso8601 => f.create(None),
                FormatType::Simple => f.create(None),
                FormatType::UnixTimestamp => f.create(None),
                FormatType::Custom => f.create(custom_formatter),
            });
        }

        let map = self.handlers.get_mut();
        map.insert(handler, h);

        self
    }

    ///
    /// Adds a [`StringHandler`] with the default formatter.
    ///
    /// ## Examples
    /// ```
    /// extern crate flogging;
    /// use flogging::*;
    ///
    /// let mut log = Logger::builder(module_path!())
    ///     .add_string_handler()
    ///     .build();
    /// ```
    ///
    pub fn add_string_handler(self) -> Self {
        self.add_handler_with(Handler::String, None, None, None, None)
    }

    ///
    /// Adds a [`StringHandler`] with the required formatter.
    ///
    /// ## Parameters
    /// - `format_type` - The format type used to produce the required formatter.
    /// - `custom_formatter` - The optional boxed custom formatter.
    ///   Used by the [`FormatType::Custom`] to produce a [`Formatter::Custom`].
    ///
    /// ## Examples
    /// First, using a provided formatter:
    /// ```
    /// extern crate flogging;
    /// use flogging::*;
    ///
    /// let mut log = Logger::builder(module_path!())
    ///     .add_string_handler_with(FormatType::Iso8601, None)
    ///     .build();
    /// ```
    /// Now using a custom formatter:
    /// ```
    /// extern crate flogging;
    /// use flogging::*;
    ///
    /// let mut log = Logger::builder(module_path!())
    ///     .add_string_handler_with(
    ///         FormatType::Custom("MockFormatter".to_string()),
    ///         Some(Box::new(MockFormatter::new())),
    ///     )
    ///     .build();
    /// ```
    ///
    pub fn add_string_handler_with(
        self,
        format_type: FormatType,
        custom_formatter: Option<Box<dyn FormatTrait>>,
    ) -> Self {
        self.add_handler_with(
            Handler::String,
            None,
            None,
            Some(format_type),
            custom_formatter,
        )
    }

    ///
    /// Complete the build process and produce the final [`Logger`] instance.
    ///
    /// ## Examples
    /// ```
    /// extern crate flogging;
    /// use flogging::*;
    ///
    /// let mut log = Logger::builder(module_path!())
    ///     .add_console_handler()
    ///     .build();
    /// ```
    ///
    pub fn build(self) -> Logger {
        Logger {
            mod_path: self.mod_path.clone(),
            fn_name: String::new(),
            level: self.level.clone(),
            handlers: self.handlers,
        }
    }

    ///
    /// Set the logging level for the [`Logger`] instance being configured.
    ///
    /// ## Parameters
    /// - `level` - The new level to set.
    ///
    /// ## Examples
    /// ```
    /// extern crate flogging;
    /// use flogging::*;
    ///
    /// let mut log = Logger::builder(module_path!())
    ///     .add_console_handler()
    ///     .set_level(Level::ALL)
    ///     .build();
    /// ```
    ///
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
    fn add_custom_handler() {
        let mut log = Logger::builder(module_path!())
            .add_custom_handler("Console", Box::new(ConsoleHandler::create("test").unwrap()))
            .build();

        log.info("We begin!");
    }

    #[test]
    fn add_file_handler_with() {
        let mut log = Logger::builder(module_path!())
            .add_file_handler_with("test.log", crate::FormatType::UnixTimestamp, None)
            .build();

        log.info("We begin!");
    }

    #[test]
    fn add_string_handler_with() {
        let mut log = Logger::builder(module_path!())
            .add_string_handler_with(crate::FormatType::Simple, None)
            .build();

        log.info("We begin!");
    }
}
