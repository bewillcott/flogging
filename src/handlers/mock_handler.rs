//
// File Name:    mock_handler.rs
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
//! # Mock Handler
//!
//! Returned as Default for Custom handler that is missing.
//!

use std::{fmt, io::Error};
use crate::*;

///
/// This is used as a _fake_ or _mock_ handler.
///
/// It is a filler for `Handler::Custom(label).create()`. It is also used
/// in examples for custom handlers.
///
#[derive(Debug, Default)]
pub struct MockHandler {}

impl fmt::Display for MockHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MockHandler!!")
    }
}

impl HandlerTrait for MockHandler {
    fn create(name: &str) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Default::default())
    }

    ///
    /// This is a 'NoOp' fn.
    ///
    fn close(&mut self) {}

    ///
    /// This is a 'NoOp' fn.
    ///
    fn flush(&mut self) {}

    fn get_formatter(&self) -> Formatter {
        Default::default()
    }

    fn get_log(&self) -> String {
        Default::default()
    }

    ///
    /// This is always `false`.
    ///
    fn is_open(&self) -> bool {
        false
    }

    #[allow(private_interfaces)]
    fn publish(&mut self, log_entry: &LogEntry) {}

    fn set_formatter(&mut self, formatter: Formatter) {}

    ///
    /// This is a 'NoOp' fn.
    ///
    fn set_test_mode(&mut self, state: bool) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{Handler, Logger, logger};

    #[test]
    fn handler_trait() {
        let mut log = Logger::custom_logger(
            module_path!(),
            "Mock",
            Box::new(MockHandler::create(module_path!()).unwrap()),
        );

        log.info("trait methods");

        let handler = log
            .get_handler(crate::Handler::Custom("Mock".to_string()))
            .unwrap();

        assert!(!handler.is_open());

        handler.set_formatter(FormatType::Simple.create(None));

        assert_eq!(
            handler.get_formatter().to_string(),
            "dt_fmt: \"\" - fmt_string: \"{mod_path}->{fn_name} [{level:7}] {message}\""
                .to_string()
        );

        assert_eq!(handler.get_log(), "".to_string());

        handler.flush();
        assert_eq!(handler.get_log(), "".to_string());
        handler.close();
    }
}
