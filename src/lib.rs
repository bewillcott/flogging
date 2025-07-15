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
//! ## Examples
//!
//!
//!
//! ```
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
//! fn do_something(){
//!     entering!();
//!
//!     // do some work worth noting
//!     info!("Did some work here.");
//!
//!     // ...
//!
//!     fine!("Bit more detail.");
//!
//!     if let Err(e) = error_prone() {
//!         warning!(&e.to_string());
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
//! fn main(){
//!     entering!();
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
//! |flogging->main| [CONFIG ] This is running on Fedora Linux 42.
//! |flogging->do_something| [FINER  ] Entry
//! |flogging->do_something| [INFO   ] Did some work here.
//! |flogging->do_something| [FINE   ] Bit more detail.
//! |flogging->error_prone| [FINER  ] Entry
//! |flogging->error_prone| [FINER  ] Return
//! |flogging->do_something| [WARNING] Bad day!
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
//! - [`static_logger!()`][static_logger] - Setup module level logger access.
//! - [`logger!()`][logger] - Provides for logging within the attributed function/method.
//!
//! - [`entering!()`][entering] - This is a convenience method that can be used to log entry to a method.
//! - [`exiting!()`][exiting] - This is a convenience method that can be used to log returning from a method.
//! - [`finest!()`][finest] - FINEST indicates a highly detailed tracing message.
//! - [`finer!()`][finer] - FINER indicates a fairly detailed tracing message.
//! - [`fine!()`][fine] - FINE is a message level providing tracing information.
//! - [`config!()`][config] - CONFIG is a message level for static configuration messages.
//! - [`info!()`][info] - INFO is a message level for informational messages.
//! - [`warning!()`][warning] - WARNING is a message level indicating a potential problem.
//! - [`severe!()`][severe] - SEVERE is a message level indicating a serious failure.
//!
//!

#![allow(unused_imports)]

mod handlers;
mod logger;
pub mod macros;

pub use flogging_macros::*;
pub use handlers::{formatter::Formatter, handler::Handler};
use logger::LogEntry;
pub use logger::*;
