//
// File Name:    lib.rs
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

#![warn(missing_docs)]

//!
//! # FLogging
//!
//! The primary purpose of logging, is to facilitate fault diagnosis through the
//! provision of specific information as, when, and from where, it is needed. This could
//! be during development, testing, or even during production runs.
//!
//! ## Setting up
//!
//! You need to add this crate to your project:
//! ```text
//! $ cargo add flogging
//! ```
//! or add this text to the projects `Cargo.toml` file:
//! ```text
//! [dependencies]
//! flogging = "0.4.1"
//! ```
//!
//! ## Features
//!
//! - [Levels](enum.Level.html) - There are nine (9) levels of message logging, with two (2) special ones.
//! - [Choice](index.html#choice) - You can use either macros or methods.
//! - [Built-in options](index.html#built-in-options) - A range of handlers and formatters.
//! - [Customization](index.html#customization) - You can create your own handlers and/or formatters.
//!
//! ### Choice
//!
//! - [Macros](index.html#macros)
//! - [Methods](index.html#methods)
//!
//! #### Macros
//!
//! - [Special Note](index.html#special-note)
//!
//! This crate has very easy to use macros. By using them, you remove a lot of the complexity
//! from the process. Thus making it both simpler and less code cluttering, to use.
//!
//! - There is one macro ([`const_logger!()`]) that is used to setup a single module/file for logging.
//! - There is one macro ([`#[logger]`][macro@logger]) that is applied as an attribute to each function/method
//!   that you need to create log entries within.
//! - There are nine macros that are used to actually log the messages:
//!     - [`config!()`]
//!     - [`entering!()`]
//!     - [`exiting!()`]
//!     - [`fine!()`]
//!     - [`finer!()`]
//!     - [`finest!()`]
//!     - [`info!()`]
//!     - [`severe!()`]
//!     - [`warning!()`]
//! - There are two helper macros:
//!     - [`get_handler!()`]
//!     - [`set_level!()`]
//!
//! Check out the [Examples](index.html#examples) below for how easy it is to get started.
//!
//! ##### Special Note
//!
//! For the macros that accept the parameter: `msg`, the following is true:
//!
//! - They accept parameters the same as for [`std::format!`](https://doc.rust-lang.org/std/macro.format.html)
//!     - plain text `&str`: `("It's your time.")`
//!     - format `&str` with interpolated variables: `("Var: {var}")`
//!     - format `&str` with supporting parameters: `("Var: {}", var)`
//!     - Combination of the last two: `("Vars {var1} - {}:{}", var2, var3)`
//! - Additional Feature
//!     - Just one or more variables without a supplied format string: `(var1, var2, var3)`
//!     - In this case, a default format string will be used: `"{}, {}, {}"`
//!     - The number of `"{}"` will depend on the number of parameters.
//!     - Ideal for logging concrete instances that have very good `Display` implementations,
//!       or you just need their data without further explanation.
//! - Special Cases
//!     - [entering!] and [exiting!]
//!     - These two macros have the same features as the others,
//!       but they may also be used _without_ any parameters. In such
//!       a case, their defaults will be used.
//!
//! #### Methods
//!
//! - [Logger](index.html#logger)
//! - [LoggerBuilder](index.html#loggerbuilder)
//!
//! Now for the coding geeks! Yes I didn't forget you lot.
//!
//! Though the macros are the easiest and simplest way to use this crate, those macros are just candy
//! coating over the real workers, the methods. There are two main mods/structs in this crate, [`Logger`](struct.Logger.html)
//! and [`LoggerBuilder`](struct.LoggerBuilder.html).
//!
//! ##### Logger
//!
//! `Logger` is the work-horse of the crate. It has all the methods for initializing each function/method
//! for logging, and all of the message logging methods.
//!
//! Using the "methods" option is more complex, as-in, you have to write a lot more code, and manage it.
//! To see how much more is involved, check-out the [`Logger`](struct.Logger.html)'s methods. There are
//! plenty of examples throughout.
//!
//! - [`builder()`][Logger::builder]
//! - [`config()`][Logger::config]
//! - [`console_logger()`][Logger::console_logger]
//! - [`custom_logger()`][Logger::custom_logger]
//! - [`entering()`][Logger::entering]
//! - [`entering_with()`][Logger::entering_with]
//! - [`exiting()`][Logger::exiting]
//! - [`exiting_with()`][Logger::exiting_with]
//! - [`file_logger()`][Logger::file_logger]
//! - [`fine()`][Logger::fine]
//! - [`finer()`][Logger::finer]
//! - [`finest()`][Logger::finest]
//! - [`fn_name()`][Logger::fn_name]
//! - [`get_handler()`][Logger::get_handler]
//! - [`has_handler()`][Logger::has_handler]
//! - [`info()`][Logger::info]
//! - [`level()`][Logger::level]
//! - [`reset_level()`][Logger::reset_level]
//! - [`set_fn_name()`][Logger::set_fn_name]
//! - [`set_level()`][Logger::set_level]
//! - [`severe()`][Logger::severe]
//! - [`string_logger()`][Logger::string_logger]
//! - [`warning()`][Logger::warning]
//!
//! ##### LoggerBuilder
//!
//! `LoggerBuilder` is used by `Logger` to provide various configuration options for setting up your logger.
//! The available options/methods are:
//!
//! - [`add_console_handler()`][LoggerBuilder::add_console_handler()]
//! - [`add_console_handler_with()`][LoggerBuilder::add_console_handler_with()]
//! - [`add_custom_handler()`][LoggerBuilder::add_custom_handler()]
//! - [`add_custom_handler_with()`][LoggerBuilder::add_custom_handler_with()]
//! - [`add_file_handler()`][LoggerBuilder::add_file_handler()]
//! - [`add_file_handler_with()`][LoggerBuilder::add_file_handler_with()]
//! - [`add_string_handler()`][LoggerBuilder::add_string_handler()]
//! - [`add_string_handler_with()`][LoggerBuilder::add_string_handler_with()]
//! - [`set_level()`][LoggerBuilder::set_level()]
//!
//! And to finish:
//! - [`build()`][LoggerBuilder::build()]
//!
//! These options/methods allow you a lot of flexibility in how you configure your logger. As you will typically
//! have a different logger for each mod/file, you have a lot of control over what is logged, how it is formatted,
//! and where it is stored/viewed. With the [`set_level()`][LoggerBuilder::set_level()] method, you can control
//! this on a mod/file basis. Logging each file differently, or even turning logging off when you no-longer require it.
//!
//! **Note**
//!
//! As of version (0.4.0), you can only set the logging level for the logger. All handlers process every log entry
//! that the logger accepts, based on the logger's current log level setting. This may change in a future version,
//! allowing each handler to have its own logging level.
//!
//! ### Built-in options
//!
//! I have included a number of handlers to get you started:
//!
//! - [`ConsoleHandler`]
//! - [`FileHandler`]
//! - [`MockHandler`]
//! - [`StringHandler`]
//!
//! There are also a number of formatters as well:
//!
//! - [`Iso8601Formatter`]
//! - [`MockFormatter`]
//! - [`SimpleFormatter`]
//! - [`UnixTimestampFormatter`]
//!
//! ### Customization
//!
//! - [Custom Handler](index.html#custom-handler)
//! - [Custom Formatter](index.html#custom-formatter)
//!
//! Now for the fun part - "Doing it _your_ way!!!"
//!
//! Though I have provided some "standard" handlers and formatters, not everyone, or every project,
//! will want to use them. I expect there will be a need for:
//!
//! - sending log entries to remote system log servers,
//! - sending log entries to another program (local or remote) for live analysis, or some other processing,
//! - storing log entries in a specific file format (xml, json, csv),
//! - storing log entries in a database.
//!
//! And I'm sure you'll come-up with more requirements at some time in the future. So, you have the option
//! to create your own custom handlers and custom formatters. Mixing them up with the built-in ones as
//! you need to.
//!
//! OK now, how do you do it. Well this is going to require some work on your part.
//!
//! #### Custom Handler
//!
//! To create a custom handler, I would suggest looking at the source code for the built-in ones, and copying
//! the code from the one that is closest to your requirements. _Make sure that you rename as appropriate!_
//! Then make the necessary changes, adding in your own code, to get it doing what you need.
//!
//! When you are ready to try-out your new custom handler, check-out these methods:
//!
//! - [`Logger::custom_logger()`]
//! - [`LoggerBuilder::add_custom_handler()`]
//! - [`LoggerBuilder::add_custom_handler_with()`]
//!
//! #### Custom Formatter
//!
//! Now for the custom formatter. This may require a bit more investigation on your part, as to the actual
//! formatting options that are available.
//!
//! Firstly, this crate uses [crono] for the date/time functionality. Check
//! out the available [specifiers]. You will need to use the formatting options from this crate for the `dt_fmt` string, of your custom
//! formatter.
//!
//! Secondly, the `fmt_string` uses the format options available in accordance with [std::fmt]. Though I am
//! actually using the [strfmt] crate to do the formatting, because it does
//! _not_ require a 'static' string like `format!()`.
//!
//! Again, check-out the built-in formatters, and copy the code from the one that is closest to your
//! requirements. _As before, renaming as necessary!_ Also, check-out the trait: [`FormatTrait`](trait.FormatTrait.html).
//! You will need to implement it for your custom formatter, as you will notice when you look at the built-in formatters.
//! Also, you will find that the 'provided method', [`ft_fmt()`][FormatTrait::ft_fmt], provides certain variables
//! that you can include, via interpolation, in your `fmt_string`.
//!
//! Once you have got your custom formatter set up, you can then use it with:
//!
//! - [`LoggerBuilder::add_console_handler_with()`]
//! - [`LoggerBuilder::add_custom_handler_with()`]
//! - [`LoggerBuilder::add_file_handler_with()`]
//! - [`LoggerBuilder::add_string_handler_with()`]
//!
//! ## Examples
//!
//! This example demonstrates the use of the macros. The reason I am demoing the macros, is that I expect most
//! people will want to use them, instead of the methods, for ease of use.
//!
//! Let's see what is required:
//!
//! 1. At the module/file level:
//!     - `use flogging::*;`
//!     - `const_logger!({...});`[=>][const_logger]
//! 2. On each function/method you want to add logging to:
//!     - `#[logger]`[=>][logger]
//! 3. Inside each such attributed function/method:
//!     - Any of the logging [macros]
//!
//! ```
//! extern crate flogging;
//! use flogging::*;
//! use std::{error::Error, result::Result};
//!
//! // Setting up the module level logger.
//! const_logger!({
//!     Logger::builder(module_path!())
//!         .add_console_handler()
//!         .add_file_handler("test.log")
//!         .set_level(Level::FINEST)
//!         .build()
//! });
//!
//! #[logger]
//! fn do_something() {
//!     entering!();
//!
//!     // do some work worth noting
//!     let result = "Just something to log.";
//!     info!("Did some work here.\n  {result}");
//!
//!     // ...
//!
//!     fine!("Bit more detail.");
//!
//!     if let Err(e) = error_prone() {
//!         warning!("Error: {}", e);
//!     }
//!
//!     exiting!();
//! }
//!
//! #[logger]
//! fn error_prone() -> Result<(), Box<dyn Error>> {
//!     entering!();
//!     let rtn = Err(Box::from("Bad day!"));
//!     exiting!();
//!     rtn
//! }
//!
//! #[logger]
//! fn main() {
//!     entering!();
//!     info!("All logging macros accept the same parameters as `std::format!(...)`");
//!     warning!("Those same macros (info, etc.) MUST have atleast one parameter.");
//!     config!("This is running on Fedora Linux 42.");
//!     do_something();
//!     info!("Job's done.");
//!     exiting!();
//! }
//!
//! ```
//! Output:
//! ```code
//! |flogging->main| [FINER  ] Entry
//! |flogging->main| [INFO   ] All logging macros accept the same parameters as `std::format!(...)`
//! |flogging->main| [WARNING] Those same macros (info, etc.) MUST have atleast one parameter.
//! |flogging->main| [CONFIG ] This is running on Fedora Linux 42.
//! |flogging->do_something| [FINER  ] Entry
//! |flogging->do_something| [INFO   ] Did some work here.
//!   Just something to log.
//! |flogging->do_something| [FINE   ] Bit more detail.
//! |flogging->error_prone| [FINER  ] Entry
//! |flogging->error_prone| [FINER  ] Return
//! |flogging->do_something| [WARNING] Error: Bad day!
//! |flogging->do_something| [FINER  ] Return
//! |flogging->main| [INFO   ] Job's done.
//! |flogging->main| [FINER  ] Return
//! ```
//!
//! ## Release Notes
//!
//! I am using Semantic Versioning in accordance with the specifications on this site:
//! <https://semver.org/>.
//!
//! #### Version 0.5.0
//!
//! **{Under Development}**
//!
//! Added new macro and method: `is_logging`. Checks to see if the logger is accepting log
//! requests.
//!
//! #### Version 0.4.1
//!
//! This is primarily a documentation update. After the release of (0.4.0), I realized that some
//! of the links were no-longer pointing to where they were supposed to. Further, I found some
//! documentation comments that needed improving/expanding.
//!
//! Additionally, I've set a minimum Rust version: "1.85.1". This is in alinement with the
//! edition = "2024".
//!
//! Further investigation, showed a need to improve the test coverage. So that has also been done.
//!
//! #### Version 0.4.0
//!
//! This is the initial release. It's not (0.1.0) because of the way I progress a project, whilst it
//! is still only internal. However, now that it is public, the numbering will progress as expected.
//!
//! This is my first foray into Rust development, and **crates.io** publishing.
//!
//! [crono]: https://crates.io/crates/chrono
//! [macros]: index.html#macros-1
//! [specifiers]: https://docs.rs/chrono/latest/chrono/format/strftime
//! [strfmt]: https://crates.io/crates/strfmt
//!

#![allow(unused_imports)]

mod handlers;
mod logger;
mod macros;

#[doc(inline)]
pub use flogging_macros::*;
pub use handlers::{
    console_handler::ConsoleHandler,
    file_handler::FileHandler,
    formatter::{
        FormatTrait, FormatType, Formatter, Iso8601Formatter, MockFormatter, SimpleFormatter,
        UnixTimestampFormatter,
    },
    handler::{Handler, HandlerTrait},
    mock_handler::MockHandler,
    string_handler::StringHandler,
};
pub use logger::*;
pub use macros::*;
