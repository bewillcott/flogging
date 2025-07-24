//
// File Name:    format_trait.rs
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
use dyn_fmt::AsStrFormatExt;
use regex::Regex;
use std::{fmt, hash};
use strfmt::strfmt;
use dyn_clone::DynClone;

pub trait FormatTrait: fmt::Display + DynClone + Send + Sync {
    fn format(&self, log_entry: &LogEntry) -> String;

    fn _fmt(&self, dt_fmt: String, fmt: String, log_entry: &LogEntry) -> String {
        let dt = log_entry.timestamp.format(&dt_fmt).to_string();

        let output = format!(
            "{}",
            strfmt!(
                &fmt,
                dt,
                message => log_entry.message.clone(),
                mod_path =>  log_entry.mod_path.clone(),
                fn_name => log_entry.fn_name.clone(),
                level => log_entry.level.as_str()
            )
            .unwrap()
        );

        output
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
    use chrono::Local;

    use crate::handlers::formatter::iso8601_formatter::Iso8601Formatter;

    use super::*;

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
}
