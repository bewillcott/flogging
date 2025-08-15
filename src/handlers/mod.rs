//
// File Name:    mod.rs
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
//! # Handlers
//!

#![allow(unused)]

mod console_handler;
mod file_handler;
mod formatters;
mod handler;
mod mock_handler;
mod string_handler;

pub use console_handler::{ConsoleHandler, console_type::*};
pub use file_handler::FileHandler;
pub use formatters::*;
pub use handler::{Handler, handler_trait::*};
pub use mock_handler::MockHandler;
pub use string_handler::StringHandler;
