//
// File Name:    format_trait.rs
// Directory:    src/handlers/formatter
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
//! # Format Trait
//!

use crate::LogEntry;
use chrono;
use dyn_clone::DynClone;
use dyn_fmt::AsStrFormatExt;
use regex::Regex;
use std::{fmt, hash};
use strfmt::strfmt;

///
/// Provides methods for formatting [`LogEntry`]s.
///
pub trait FormatTrait: fmt::Display + DynClone + Send + Sync {
    ///
    /// Use this method to setup the parameters for calling [`ft_fmt()`][FormatTrait::ft_fmt()].
    ///
    /// ## Parameters
    /// - `log_entry` A reference to the `LogEntry` to be formatted.
    ///
    /// ## Examples
    /// ```text
    /// impl FormatTrait for SimpleFormatter {
    ///     fn format(&self, log_entry: &LogEntry) -> String {
    ///         self.ft_fmt(self.dt_fmt(), self.fmt_string(), log_entry)
    ///     }
    /// }
    /// ```
    ///
    fn format(&self, log_entry: &LogEntry) -> String;

    ///
    /// This method does the actual formatting of the `log_entry`.
    ///
    /// ## Parameters
    /// - `dt_fmt` - The [`chrono::DateTime`] format string.
    /// - `fmt` - The primary format string.\
    ///   Available variables:
    ///     - `dt` - The datetime formatted with: `dt_fmt`.
    ///     - `mod_path` - The module path, possibly supplied via: [`module_path!()`][module_path].
    ///     - `fn_name` - The name of the  function/method inside which the log entry
    ///       was generated. Supplied by the [`#[logger]`][crate::logger] macro, or manually with the
    ///       [`set_fn_name()`][crate::Logger::set_fn_name] method.
    ///     - `level` - The log [level][crate::Level] for which the entry was created.
    ///     - `message` - The text of the log entry.
    ///
    fn ft_fmt(&self, dt_fmt: String, fmt: String, log_entry: &LogEntry) -> String {
        let dt = log_entry.timestamp.format(&dt_fmt).to_string();

        strfmt!(
            &fmt,
            dt,
            message => log_entry.message.clone(),
            mod_path =>  log_entry.mod_path.clone(),
            fn_name => log_entry.fn_name.clone(),
            level => log_entry.level.as_str()
        )
        .unwrap()
    }
}

dyn_clone::clone_trait_object!(FormatTrait);

impl fmt::Debug for dyn FormatTrait {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Local;
    use crate::*;

    #[test]
    fn strfmt() {
        let fmt = "test: {text}";
        let text = "Some text";
        let text2 = "Will this work?";

        let output = format!("{}", strfmt!(fmt, text2, text).unwrap());

        println!("{output}");
    }

    #[test]
    fn format() {
        let log_entry = LogEntry {
            timestamp: Local::now(),
            mod_path: module_path!().to_string(),
            fn_name: "format".to_string(),
            level: crate::Level::INFO,
            message: "We are testing log entry formatting.".to_string(),
        };

        println! {"{log_entry}"};

        // let formatter =
        // let text =
    }

    #[test]
    fn debug() {
        let mut fmt = FormatType::Custom.create(Some(Box::new(SimpleFormatter::new())));

        let sf: Box<dyn FormatTrait> = Box::new(SimpleFormatter::new());

        println!("sf: {sf:?}");
    }
}
