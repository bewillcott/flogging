/*
 * File Name:    it_log_messages.rs
 * Project Name: logging
 *
 * Copyright (C) 2025 Bradley Willcott
 *
 * This library (crate) is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This library (crate) is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this library (crate).  If not, see <https://www.gnu.org/licenses/>.
 */

/*!
 * # Integrated Testing of Log Messages
 */

#[allow(unused_variables)]
#[cfg(test)]
mod tests {
    use logging::{Handler, Level::*, Logger};

    #[test]
    fn add_a_log_message() {
        let mut logger = Logger::builder("it_log_messages.rs")
            .add_handler(Handler::ConsoleHandler, None)
            .add_handler(Handler::FileHandler, Some("test.log"))
            .set_level(FINE)
            .build();

        // logger.set_level(OFF);
        logger.log(FINE, "This a just a test.");

        let mut logger = Logger::console_logger("Test 2");
        logger.set_level(SEVERE);
        logger.reset_level();

        logger.log(WARNING, "This is test two!");
    }

    #[test]
    fn shared_log_file() {
        let mut logger1 = Logger::builder("logger1")
            .add_handler(Handler::ConsoleHandler, None)
            .add_handler(Handler::FileHandler, Some("test.log"))
            .set_level(FINE)
            .build();

        let mut logger2 = Logger::builder("logger2")
            .add_handler(Handler::ConsoleHandler, None)
            .add_handler(Handler::FileHandler, Some("test.log"))
            .set_level(FINE)
            .build();

        logger1.log(FINE, "This a just a test.");
        logger2.log(WARNING, "This a just a test.");
        logger1.log(SEVERE, "This a just a test.");
        logger2.log(FINE, "This a just a test.");
    }
}
