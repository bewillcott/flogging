//
// File Name:    it_log_messages.rs
// Project Name: flogging
//
// Copyright (C) 2025 Bradley Willcott
//
// SPDX-License-Identifier: GPL-2.0-or-later
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
//! # Integrated Testing of Log Messages
//!

#[allow(unused_variables)]
#[cfg(test)]
mod tests {
    use flogging::{Level::*, Logger};

    #[test]
    fn add_a_log_message() {
        let mut logger = Logger::builder(module_path!())
            .add_console_handler()
            .add_file_handler("test.log")
            .set_level(FINE)
            .build();

        // logger.set_level(OFF);
        logger.fine("add_a_log_message", "This a just a test.");

        let mut logger = Logger::console_logger(module_path!());
        logger.set_level(SEVERE);
        logger.reset_level();

        logger.warning("add_a_log_message", "This is test two!");
    }

    #[test]
    fn shared_log_file() {
        let mut logger1 = Logger::builder("logger1")
            .add_console_handler()
            .add_file_handler("test.log")
            .set_level(FINE)
            .build();

        let mut logger2 = Logger::builder("logger2")
            .add_console_handler()
            .add_file_handler("test.log")
            .set_level(FINE)
            .build();

        logger1.fine("shared_log_file", "This a just a test.");
        logger2.warning("shared_log_file", "This a just a test.");
        logger1.severe("shared_log_file", "This a just a test.");
        logger2.fine("shared_log_file", "This a just a test.");
    }
}
