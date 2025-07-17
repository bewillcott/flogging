//
// File Name:    lib.rs
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
//! # Logging
//!
//! The primary purpose of logging, is to facilitate fault diagnosis through the
//! provision of specific information as, when, and from where it is needed. This could
//! be during development, testing, or even during production runs.
//!
//! ## Setting up
//!
//! You need to add this crate to your project:
//! ```
//! cargo add flogging
//! ```
//! or add this text to the projects `Cargo.toml` file:
//! ```
//! [dependencies]
//! flogging = "0.3.0"
//! ```
//!
//! ## Examples
//!
//! This example demonstrates using the macros.
//!
//! Let's see what is required:
//!
//! 1. At the module level:
//!     - `use flogging::*;`
//!     - `static_logger!({...});`[->][static_logger]
//! 2. On each function/method you want to add logging to:
//!     - `#[logger]`[->][logger]
//! 3. Inside each such attributed function/method:
//!     - Any of the logging [macros]
//!
//! ```
//! extern crate flogging;
//! use flogging::*;
//! use std::{error::Error, result::Result};
//!
//! // Setting up the module level logger.
//! static_logger!({
//!     Logger::builder(module_path!())
//!         .add_console_handler()
//!         .add_file_handler("test.log")
//!         .set_level(Level::FINEST)
//!         .build()
//! });
//!
//! #[logger]
//! fn do_something() {
//!     entering!();
//!
//!     // do some work worth noting
//!     let result = "Just something to log.";
//!     info!("Did some work here.\n  {result}");
//!
//!     // ...
//!
//!     fine!("Bit more detail.");
//!
//!     if let Err(e) = error_prone() {
//!         warning!("Error: {}", e);
//!     }
//!
//!     exiting!();
//! }
//!
//! #[logger]
//! fn error_prone() -> Result<(), Box<dyn Error>> {
//!     entering!();
//!     let rtn = Err(Box::from("Bad day!"));
//!     exiting!();
//!     rtn
//! }
//!
//! #[logger]
//! fn main() {
//!     entering!();
//!     info!(
//!         "All logging macros, except: `entering` and `exiting`, accept the same parameters as `format!(...)`"
//!     );
//!     warning!("Those same macros (info, etc.) MUST have atleast the format string.");
//!     config!("This is running on Fedora Linux 42.");
//!     do_something();
//!     info!("Job's done.");
//!     exiting!();
//! }
//!
//! ```
//! Output:
//! ```code
//! |flogging->main| [FINER  ] Entry
//! |flogging->main| [INFO   ] All logging macros, except: `entering` and `exiting`, accept the same parameters as `format!(...)`
//! |flogging->main| [WARNING] Those same macros (info, etc.) MUST have atleast the format string.
//! |flogging->main| [CONFIG ] This is running on Fedora Linux 42.
//! |flogging->do_something| [FINER  ] Entry
//! |flogging->do_something| [INFO   ] Did some work here. Just something to log.
//! |flogging->do_something| [FINE   ] Bit more detail.
//! |flogging->error_prone| [FINER  ] Entry
//! |flogging->error_prone| [FINER  ] Return
//! |flogging->do_something| [WARNING] Error: Bad day!
//! |flogging->do_something| [FINER  ] Return
//! |flogging->main| [INFO   ] Job's done.
//! |flogging->main| [FINER  ] Return
//! ```
//!
//! ## Key Elements
//!
//! - [`Logger`]: The main entity on which applications make logging calls. A `Logger` instance
//!   is used to log messages for a specific system or application component.
//! - [`LoggerBuilder`]: Used by `Logger::builder(mod_path)` to build a logger with greater control
//!   over its final configuration.
//! - [`Level`]: Defines a set of standard logging levels that can be used to control logging
//!   output. Programs can be configured to output logging for some levels while ignoring
//!   output for others.
//!
//! ## Macros
//!
//! Though this crate has all the methods a developer with time and patience would be happy
//! with, I believe the most efficient way is to use the supplied macros.
//!
//! [macros]: index.html#macros-1
//!

#![allow(unused_imports)]

mod handlers;
mod logger;
mod macros;

pub use flogging_macros::*;
pub use handlers::{formatter::Formatter, handler::Handler};
use logger::LogEntry;
pub use logger::*;
pub use macros::*;
