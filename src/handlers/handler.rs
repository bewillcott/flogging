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
    #[default]
    Console,
    File,
    String,
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
    pub fn create(&self) -> Box<dyn HandlerTrait> {
        match &self {
            Handler::Console => Box::new(ConsoleHandler::default()),
            Handler::File => Box::new(FileHandler::default()),
            Handler::String => Box::new(StringHandler::default()),
            Handler::Custom(label) => Box::new(MockHandler::default()),
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

        println!("console: {console}");
        println!("file: {file}");
        println!("string: {string}");
        println!("custom: {custom}");
    }
}
