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
//! The `Level` module provides the enum containing the possible log entry levels.
//! As they are ordered within the enum, enabling logging at a given level also
//! enables logging at all higher levels.
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
#[derive(Debug, serde::Serialize, Clone)]
pub enum Level {
    FINEST,
    FINER,
    FINE,
    CONFIG,
    INFO,
    WARNING,
    SEVERE,
    OFF,
}

impl Level {
    pub fn value(&self) -> u8 {
        match self {
            Level::FINEST => 1,
            Level::FINER => 2,
            Level::FINE => 3,
            Level::CONFIG => 4,
            Level::INFO => 5,
            Level::WARNING => 6,
            Level::SEVERE => 7,
            Level::OFF => 8,
        }
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Level::FINEST => "FINEST",
            Level::FINER => "FINER",
            Level::FINE => "FINE",
            Level::CONFIG => "CONFIG",
            Level::INFO => "INFO",
            Level::WARNING => "WARNING",
            Level::SEVERE => "SEVERE",
            Level::OFF => "OFF",
        };

        write!(f, "{label}")?;
        Ok(())
    }
}

impl Default for Level {
    fn default() -> Self {
        Level::INFO
    }
}
