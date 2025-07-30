//
// File Name:    tests.rs
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
//! # Logger Tests
//!

use super::*;

#[test]
fn config() {
    mod helper {
        use super::*;

        pub(super) fn help() {
            let mut log = Logger::console_logger(module_path!());
            log.set_fn_name("help");
            log.info("Some text to store.");
        }
    }

    helper::help();
}

#[test]
fn has_handler() {
    use super::*;

    let mut log = Logger::string_logger(module_path!());
    log.info("Some text to store.");

    println!(
        "This logger has a 'StringHandler': {}",
        log.has_handler(Handler::String)
    );
}

#[test]
fn get_handler() {
    use super::*;

    let mut log = Logger::string_logger(module_path!());
    log.set_fn_name("get_handler");

    log.info("Some text to store.");

    assert!(log.get_handler(Handler::String).is_some());
    assert!(log.get_handler(Handler::Console).is_none());
}

#[test]
fn file_logger() {
    use super::*;
    use std::fs;

    let filename = "test.log";
    let path = Path::new(filename);

    if path.try_exists().unwrap() {
        fs::remove_file(path).unwrap();
    }

    let mut log = Logger::file_logger(module_path!(), path.to_str().unwrap());
    log.set_fn_name("file_logger").set_level(Level::ALL);

    log.config("Running on Fedora Linux v42");
    log.fine("This is some fine work!");

    assert_eq!(log.level(), &Level::ALL);
    log.reset_level();
    assert_eq!(log.level(), &Level::default());
}

#[test]
fn is_logging() {
    let mut log = Logger::console_logger(module_path!());
    log.set_fn_name("is_logging");

    assert!(log.is_logging());
    assert!(log.is_loggable(&Level::WARNING));

    log.set_level(Level::OFF);
    assert!(!log.is_logging());
    assert!(!log.is_loggable(&Level::WARNING));
}
