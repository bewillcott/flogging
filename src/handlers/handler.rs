//
// File Name:    handler.rs
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

pub use handler_trait::HandlerTrait;
use std::{fmt, hash::Hash, io::Error};

use crate::{
    handlers::{
        console_handler::ConsoleHandler,
        file_handler::FileHandler,
        formatter::{FormatType, Formatter},
        mock_handler::MockHandler,
        string_handler::StringHandler,
    },
    logger::{Level, LogEntry},
};

///
/// Available handlers.
///
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq)]
pub enum Handler {
    ///
    /// Refers to the `ConsoleHandler`.
    ///
    #[default]
    Console,
    ///
    /// Refers to the `FileHandler`.
    ///
    File,
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
            Handler::File => "File",
            Handler::String => "String",
            Handler::Custom(label) => &format!("Custom({label})"),
        };

        write!(f, "Handler::{text}")
    }
}

impl Handler {
    ///
    /// Creates default instances of the selected handler.
    ///
    pub fn new(&self) -> Box<dyn HandlerTrait> {
        match &self {
            Handler::Console => Box::new(ConsoleHandler::default()),
            Handler::File => Box::new(FileHandler::default()),
            Handler::String => Box::new(StringHandler::default()),
            Handler::Custom(label) => Box::new(MockHandler::default()),
        }
    }
    ///
    /// Creates default instances of the selected handler.
    ///
    pub fn create(&self, name: &str) -> Box<dyn HandlerTrait> {
        match &self {
            Handler::Console => Box::new(ConsoleHandler::create(name).unwrap()),
            Handler::File => Box::new(FileHandler::create(name).unwrap()),
            Handler::String => Box::new(StringHandler::create(name).unwrap()),
            Handler::Custom(label) => Box::new(MockHandler::create(name).unwrap()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Handler;

    #[test]
    fn handlers() {
        let console = Handler::Console;
        let file = Handler::File;
        let string = Handler::String;
        let custom = Handler::Custom("MyCustom".to_string());

        assert_eq!(console.to_string(), "Handler::Console".to_string());
        assert_eq!(file.to_string(), "Handler::File".to_string());
        assert_eq!(string.to_string(), "Handler::String".to_string());
        assert_eq!(custom.to_string(), "Handler::Custom(MyCustom)".to_string());

        let mod_path = module_path!();

        let ch = console.new();
        let ch2 = console.create(mod_path);
        let fh = file.new();
        let fh2 = file.create("test.log");
        let sh = string.new();
        let sh2 = string.create(mod_path);
        let cuh = custom.new();
        let cuh2 = custom.create(mod_path);

        println!("ch: {ch}");
        println!("ch2: {ch2}");
        println!("fh: {fh}");
        println!("fh2: {fh2}");
        println!("sh: {sh}");
        println!("sh2: {sh2}");
        println!("cuh: {cuh}");
        println!("cuh2: {cuh2}");
    }
}
