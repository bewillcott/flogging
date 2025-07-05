//
// File Name:    level.rs
// Project Name: logging
//
// Copyright (C) 2025 Bradley Willcott
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
//! Log Entry Level
//!
//! This module provides the enum containing the possible log entry levels.
//! The order in which they are listed within the enum, enables logging at that level
//! and at all higher levels.
//!
//! The levels in descending order are:
//!
//! - SEVERE (highest level)
//! - WARNING
//! - INFO
//! - CONFIG
//! - FINE
//! - FINER
//! - FINEST (lowest level)
//!
//! In addition there is a level **OFF** that can be used to turn off logging.

use std::fmt;

/// Log entry level setting.\
/// Default level: INFO.
#[allow(unused)]
#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
pub enum Level {
    /// FINEST indicates a highly detailed tracing message.
    FINEST,
    /// FINER indicates a fairly detailed tracing message.
    /// Suggest logging calls for entering, returning,
    /// or throwing an exception are traced at this level.
    FINER,
    /// FINE is a message level providing tracing information.
    ///
    /// All of FINE, FINER, and FINEST are intended for relatively
    /// detailed tracing. The exact meaning of the three levels will
    /// vary between subsystems, but in general, FINEST should be
    /// used for the most voluminous detailed output, FINER for somewhat
    /// less detailed output, and FINE for the lowest volume (and most
    /// important) messages.
    ///
    /// In general the FINE level should be used for information that
    /// will be broadly interesting to developers who do not have a
    /// specialized interest in the specific subsystem.
    ///
    /// FINE messages might include things like minor (recoverable)
    /// failures. Issues indicating potential performance problems are
    /// also worth logging as FINE.
    FINE,
    /// CONFIG is a message level for static configuration messages.
    ///
    /// CONFIG messages are intended to provide a variety of static
    /// configuration information, to assist in debugging problems
    /// that may be associated with particular configurations.
    ///
    /// For example, a CONFIG message might include the CPU type, the
    /// graphics depth, the GUI look-and-feel, etc.
    CONFIG,
    /// INFO is a message level for informational messages.
    ///
    /// Typically INFO messages will be written to the console or its
    /// equivalent. So the INFO level should only be used for reasonably
    /// significant messages that will make sense to end users and system
    /// administrators.
    ///
    /// \[default level]
    #[default]
    INFO,
    /// WARNING is a message level indicating a potential problem.
    ///
    /// In general WARNING messages should describe events that will be
    /// of interest to end users or system managers, or which indicate
    /// potential problems.
    WARNING,
    /// SEVERE is a message level indicating a serious failure.
    ///
    /// In general SEVERE messages should describe events that are of
    /// considerable importance and which will prevent normal program
    /// execution. They should be reasonably intelligible to end users
    /// and to system administrators.
    SEVERE,
    /// OFF is a special level that can be used to turn off logging.
    OFF,
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_levels() {
        let log_level = Level::default();
        let b = Level::FINE;

        println!("\n{log_level}\n");

        assert!(b < log_level);
    }
}
