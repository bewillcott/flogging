//
// File Name:    lib.rs
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
//! # Logging
//!
//! The primary purpose of logging, is to facilitate fault diagnosis through the
//! provision of specific information as, when, and from where it is need. This could
//! be during development, testing, or even during production runs.
//!
//! ## Key Elements
//!
//! - `Logger`: The main entity on which applications make logging calls. A Logger instance
//! is used to log messages for a specific system or application component.
//! - `Level`: Defines a set of standard logging levels that can be used to control logging
//! output. Programs can be configured to output logging for some levels while ignoring
//! output for others.
//!

#![allow(unused_imports)]

pub mod level;

pub mod logger;

mod log_entry;
