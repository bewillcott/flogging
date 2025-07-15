//
// File Name:    macros.rs
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
//! # Macros
//!

use crate::Logger;

///
/// Setup module level logger access.
///
/// The basic macro syntax is:
///
/// ```text
/// static_logger!({/* the block of Rust code to build a Logger goes here */});
/// ```
/// Notice there are curly braces "`{}`" wrapping the inner Rust code. They
/// are required.
///
/// The code you put in here will depend on what configuration of `Logger` you
/// need to setup.
///
/// # Examples
/// ```
/// use flogging::*;
///
/// static_logger!({
///     Logger::builder(module_path!())
///         .set_level(Level::FINEST)
///         .add_console_handler()
///         .add_file_handler_with("rdb.log", Formatter::Iso8601)
///         .build()
/// });
/// ```
#[macro_export]
macro_rules! static_logger {
    ($block:block) => {
        use flogging::Logger as FLogger;
        use std::cell::{LazyCell as FLazyCell, RefCell as FRefCell};

        // Setup module level logger access.
        const LOGGER: FLazyCell<FRefCell<FLogger>> = FLazyCell::new(|| FRefCell::new({ $block }));
    };
}
