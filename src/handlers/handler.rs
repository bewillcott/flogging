//
// File Name:    handler.rs
// Directory:    src/handlers
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
//! # Handler
//!

// #![allow(unused)]
pub mod handler_trait;

use std::{fmt, hash::Hash, io::Error};
pub use handler_trait::HandlerTrait;

///
/// Available handlers.
///
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq)]
pub enum Handler {
    ///
    /// Refers to the `ConsoleHandler` => `ConsoleType::StdOut`.
    ///
    #[default]
    Console,
    ///
    /// Refers to the `ConsoleHandler` => `ConsoleType::StdErr`.
    ///
    EConsole,
    ///
    /// Refers to the `FileHandler`.
    ///
    File,
    ///
    /// Refers to the `ConsoleHandler` => `ConsoleType::Production`.
    ///
    PConsole,
    ///
    /// Refers to the `StringHandler`.
    ///
    String,
    ///
    /// Refers to a custom handler; by default: `MockHandler`.
    ///
    Custom(String),
}

impl fmt::Display for Handler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match &self {
            Handler::Console => "Console",
            Handler::EConsole => "EConsole",
            Handler::File => "File",
            Handler::PConsole => "PConsole",
            Handler::String => "String",
            Handler::Custom(label) => &format!("Custom({label})"),
        };

        write!(f, "Handler::{text}")
    }
}

#[cfg(test)]
mod test {
    use super::Handler;

    #[test]
    fn handlers() {
        let console = Handler::Console;
        let econsole = Handler::EConsole;
        let pconsole = Handler::PConsole;
        let file = Handler::File;
        let string = Handler::String;
        let custom = Handler::Custom("MyCustom".to_string());

        assert_eq!(console.to_string(), "Handler::Console".to_string());
        assert_eq!(econsole.to_string(), "Handler::EConsole".to_string());
        assert_eq!(pconsole.to_string(), "Handler::PConsole".to_string());
        assert_eq!(file.to_string(), "Handler::File".to_string());
        assert_eq!(string.to_string(), "Handler::String".to_string());
        assert_eq!(custom.to_string(), "Handler::Custom(MyCustom)".to_string());
    }
}
