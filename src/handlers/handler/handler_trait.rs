//
// File Name:    handler_trait.rs
// Directory:    src/handlers/handler
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
//! # HandlerTrait
//!

use std::{fmt, io::Error};

use crate::{Formatter, LogEntry};


///
/// Provides common methods required for all handlers.
///
pub trait HandlerTrait: fmt::Display + Send + Sync {
    ///
    /// Create a new handler instance.
    ///
    /// ## Parameters
    /// - `name` - Can be used as needed.
    ///
    fn create(name: &str) -> Result<Self, Error>
    where
        Self: Sized;

    ///
    /// Close the Handler and free all associated resources.
    ///
    /// The close method will perform a flush and then close the Handler.
    /// After `close` has been called, this Handler should no longer be used.
    /// Method calls will be silently ignored.
    ///
    fn close(&mut self);

    ///
    /// Flush any buffered output.
    ///
    fn flush(&mut self);

    ///
    /// Return a copy of the internal buffer as a `String`.
    ///
    fn get_log(&self) -> String;

    ///
    /// Return the Formatter for this Handler.
    ///
    fn get_formatter(&self) -> Formatter;

    ///
    /// Check status of this handler.
    ///
    fn is_open(&self) -> bool;

    ///
    /// Publish a LogEntry.
    ///
    /// The logging request was made initially to a Logger object, which initialized
    /// the LogEntry and forwarded it here.
    ///
    /// The Handler is responsible for formatting the message, when and if necessary.
    ///
    /// ## Parameters
    /// - `log_entry` - The `LogEntry` to be published.
    ///
    fn publish(&mut self, log_entry: &LogEntry);

    ///
    /// Set a Formatter.
    ///
    /// ## Parameters
    /// - `formatter` The `Formatter` to use.
    ///
    fn set_formatter(&mut self, formatter: Formatter);
}
