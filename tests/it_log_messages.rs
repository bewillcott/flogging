//
// File Name:    it_log_messages.rs
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
//! # Integrated Testing of Log Messages
//!

#[allow(unused_variables, unused_imports)]
#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use flogging::{Level::*, *};
    use flogging_macros::*;
    use regex::{Regex, RegexBuilder};

    const_logger!({
        Logger::builder(module_path!())
            // .add_console_handler()
            .add_custom_handler_with(
                "console",
                Box::new(ConsoleHandler::create("false").unwrap()),
                FormatType::Iso8601,
                None,
            )
            // .add_file_handler("test.log")
            .set_level(FINEST)
            .build()
    });

    ///
    /// This is a test of adding log messages.
    ///
    #[test]
    // #[show_streams]
    #[logger]
    fn add_log_messages() {
        config!("Some testing config stuff.");
        entering!();

        config!("Just some config stuff.");
        println!("module_path!(): {}", module_path!());

        fine!("This a fine test.");
        finer!("This a finer test.");
        finest!("This the finest test.");
        warning!("Lookout - the sky's falling!");
        severe!("Oh Shit! We're done for now!!");

        exiting!();
    }

    #[test]
    #[logger]
    fn shared_log_file() {
        // Added testing of 'remove_file()'.
        let mut logger1 = Logger::builder("logger1")
            .add_console_handler()
            .remove_file("test.log")
            .add_file_handler("test.log")
            .set_level(FINE)
            .build();

        // Will quietly fail 'remove_file()' because "test1.log"
        // does not exist.
        let logger1b = Logger::builder("logger1")
            .add_console_handler()
            .remove_file("test1.log")
            .add_file_handler("test.log")
            .set_level(FINE)
            .build();

        logger1.set_fn_name("shared_log_file");

        let mut logger2 = Logger::builder("logger2")
            .add_econsole_handler()
            .add_file_handler("test.log")
            .set_level(FINE)
            .build();

        logger2.set_fn_name("shared_log_file");

        logger1.fine("This a fine test.");
        logger2.warning("This a warning test.");
        logger1.severe("This a severe test.");
        logger2.fine("This another fine test.");

        println!("\n{}", logger1);
    }

    #[test]
    fn string_handler() {
        let mut log = Logger::string_logger(module_path!());
        log.set_level(FINEST).set_fn_name("string_handler");

        log.entering();
        log.info("Believe it or not!");
        log.fine("Something is happening.");
        log.finest("We're now down with the worms!");
        log.exiting();

        println!("\n{}", log);
        // print!("{}", log.get_handler(Handler::StringHandler).unwrap().get_log());
    }

    #[test]
    #[ignore = "for manual testing ONLY!"]
    fn regex() {
        let item = "///
/// This is a test of adding log messages.
///
fn add_a_log_message(fmt: &str, msg: &str)
{
    let mut logger =
    Logger::builder(module_path!()).add_console_handler().add_file_handler(\"test.log\").set_level(FINE).build();
    logger.fine(\"add_a_log_message\", \"This a just a test.\"); let mut logger =
    Logger::console_logger(module_path!()); logger.set_level(SEVERE);
    logger.reset_level();
    logger.warning(\"add_a_log_message\", \"This is test two!\");
}";

        let re = RegexBuilder::new(
            r"(?<head>.*)fn\s+(?<fn_name>[_]*[a-z][_\w]*)\((?<attrs>[^\{]*)\).*\{(?<body>.*)\}$",
        )
        .dot_matches_new_line(true)
        .build()
        .unwrap();
        let binding = item.to_string();
        let caps = re.captures(&binding).unwrap();
        let head = caps["head"].to_string();
        let fn_name = caps["fn_name"].to_string();

        let tmp = caps["attrs"].to_string();
        let attrs: Vec<&str> = tmp.split(',').collect();

        let body = caps["body"].to_string();

        println!("head: {head}");
        println!("fn_name: {fn_name}");
        println!("attrs: {attrs:?}");
        println!("body: {body}");
        println!()
    }

    #[test]
    #[ignore = "for manual testing ONLY!"]
    fn regex_params() {
        let t = "\"{}\", arg";
        let t2 = "\"Some text{:?}\n\", arg";
        let t1 = "arg";
        let t1a = "arg, arg1";
        let s = "\"Lookout, the sky's falling!\"";
        let s1 = "\"Lookout, the sky's {falling}!\"";

        let regex_str = "(?<fmt>\".*\\{.*\\}.*\")(,[\\s]*(?<attrs>.*)*)";

        let text = s1;

        if text.starts_with('\"') && text.ends_with('\"') {
            println!("{text}");
            return;
        }

        let re = RegexBuilder::new(regex_str)
            .dot_matches_new_line(true)
            .build()
            .unwrap();

        if re.is_match(text) {
            println!("{}", text);
        } else {
            println!("None");
            let count = text.split(',').count();

            let mut buf = String::new();

            if count == 1 {
                buf.push_str("\"{}\", ");
            } else {
                buf.push_str("\"{}");

                for _i in 1..count {
                    buf.push_str(", {}");
                }

                buf.push_str("\", ");
            }

            buf.push_str(text);
            println!("buf: {buf}");
        }
    }
}

#[cfg(test)]
mod temp {
    use flogging::*;
    use std::{error::Error, result::Result};

    // Setting up the module level logger.
    const_logger!({
        Logger::builder(module_path!())
            .add_console_handler()
            .add_file_handler("test.log")
            .set_level(Level::FINEST)
            .build()
    });

    ///
    /// ~~~
    /// use flogging::*;
    ///
    /// const_logger!({
    ///     Logger::console_logger(module_path!())
    /// });
    ///
    /// #[logger]
    /// fn my_func(){
    ///     config!("Some text to store.");
    /// }
    /// ~~~
    #[test]
    #[logger]
    fn do_something() {
        entering!();

        // do some work worth noting
        info!("Did some work here.");

        // ...

        fine!("Bit more detail.");

        if let Err(e) = error_prone() {
            warning!(e);
        }
        set_level!(Level::INFO);
        exiting!();
    }

    fn error_prone() -> Result<(), Box<dyn Error>> {
        Err(Box::from("Bad day!"))
    }

    #[test]
    #[logger(alt text)]
    fn my_func() {
        entering!();
        entering!("Testing");

        exiting!();
        exiting!("Done!");
    }
}

#[cfg(test)]
mod my_mod {
    use flogging::*;

    // Setting up the module level logger.
    const_logger!({
        Logger::builder(module_path!())
            .add_console_handler()
            .add_file_handler("test.log")
            .set_level(Level::FINEST)
            .build()
    });

    #[test]
    #[logger]
    fn test_my_func() {
        entering!();
        my_func("Some data");
        exiting!();
    }

    #[logger]
    fn my_func(data: &str) -> bool {
        entering!();
        entering!("data: \"{data}\"");
        entering!(data);

        // ...

        let rtn = true;

        exiting!();
        exiting!("rtn: {rtn}");
        rtn
    }
}
