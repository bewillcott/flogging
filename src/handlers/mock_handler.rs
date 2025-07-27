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

use crate::{handlers::formatter::Formatter, FormatType, HandlerTrait, LogEntry};

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

    fn close(&mut self) {}

    fn flush(&mut self) {}

    fn get_formatter(&self) -> Formatter {
        Default::default()
    }

    fn get_log(&self) -> String {
        Default::default()
    }

    fn is_open(&self) -> bool {
        false
    }

    #[allow(private_interfaces)]
    fn publish(&mut self, log_entry: &LogEntry) {}

    fn set_formatter(&mut self, formatter: Formatter) {}
}
