//
// File Name:    it_customs.rs
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
//! #  Testing Customs
//!
//! Testing both Custom Handler and Custom Formatter
//!

mod custom_formatter;
mod custom_handler;

#[cfg(test)]
mod test {
    use crate::{custom_formatter::CustomFormatter, custom_handler::CustomHandler};
    use flogging::{FormatType, Handler, HandlerTrait, Logger};

    #[test]
    fn custom_handler() {
        let mut log = Logger::custom_logger(
            module_path!(),
            "CustomHandler",
            Box::new(CustomHandler::create("custom_handler").unwrap()),
        );

        log.info("Just testing");

        let list = log
            .get_handler(Handler::Custom("CustomHandler".to_string()))
            .unwrap()
            .as_ref()
            .get_log();

        println!("Log:\n{}", list);

        assert_eq!(&list, "|it_customs::test->| [INFO   ] Just testing\n")
    }

    #[test]
    fn add_console_handler_with_custom_formatter() {
        let mut log = Logger::builder(module_path!())
            .add_console_handler_with(
                FormatType::Custom,
                Some(Box::new(CustomFormatter::new())),
            )
            .build();

        log.info("Testing custom formatter");
    }

    #[test]
    fn add_econsole_handler_with_custom_formatter() {
        let mut log = Logger::builder(module_path!())
            .add_econsole_handler_with(
                FormatType::Custom,
                Some(Box::new(CustomFormatter::new())),
            )
            .build();

        log.info("Testing custom formatter");
    }
}
