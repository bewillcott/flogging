//
// File Name:    main.rs
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
//! # Main
//!

use flogging::*;
use std::{error::Error, result::Result};

// Setting up the module level logger.
static_logger!({
    Logger::builder(module_path!())
        .add_console_handler()
        .add_file_handler("test.log")
        .set_level(Level::FINEST)
        .build()
});

#[logger]
fn do_something() {
    entering!();

    // do some work worth noting
    let result = "Just something to log.";
    info!("Did some work here. {result}");

    // ...

    fine!("Bit more detail.");

    if let Err(e) = error_prone() {
        warning!("Error: {}", e);
    }

    exiting!();
}

#[logger]
fn error_prone() -> Result<(), Box<dyn Error>> {
    entering!();
    let rtn = Err(Box::from("Bad day!"));
    exiting!();
    rtn
}

#[allow(dead_code)]
mod my_mod {
    use flogging::*;
    use std::cell::{LazyCell, RefCell};

    // Setting up the module level logger.
    const LOGGER: LazyCell<RefCell<Logger>> = LazyCell::new(|| {
        RefCell::new({
            Logger::builder(module_path!())
                .add_console_handler()
                .set_level(Level::FINEST)
                .build()
        })
    });

    // #[test]
    // fn test_my_func() {
    //     my_func("Some data");
    // }

    pub(crate) fn my_func(data: &str) -> bool {
        let binding = LOGGER;
        let mut log = binding.borrow_mut();
        log.set_fn_name("my_func");

        log.entering();
        log.entering_with(&format!("data: \"{data}\""));

        // ...

        let rtn = true;

        log.exiting();
        log.exiting_with(&format!("rtn: {rtn}"));
        rtn
    }
}

#[logger]
fn main() {
    // entering!();
    // info!(
    //     "All logging macros, except: `entering` and `exiting`, accept the same parameters as `format!(...)`"
    // );
    // warning!("Those same macros (info, etc.) MUST have atleast the format string.");
    // config!("This is running on Fedora Linux 42.");
    // do_something();
    // info!("Job's done.");
    // exiting!();

    // let data = "Some data";
    // my_mod::my_func(data);

    extern crate flogging;
    use flogging::*;

    let mut log = Logger::string_logger(module_path!());
    log.set_fn_name("main");

    log.info("It is cloudy today.");

    log.get_handler(StringHandler)
        .unwrap()
        .set_formatter(Iso8601);

    log.warning("Rain is wet!");

    log.get_handler(StringHandler)
        .unwrap()
        .set_formatter(UnixTimestamp);

    log.severe("Hurricanes are windy!");

    let log_str = log.get_handler(StringHandler).unwrap().get_log();
    println!("log_str:\n{log_str}");
}
