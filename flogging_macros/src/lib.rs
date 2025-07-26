//
// File Name:    lib.rs
// Project Name: flogger_macros
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
//! # Flogging Macros
//!
//! This is a **supporting crate** for the `flogging` crate.
//!
//! It is _not_ meant to be used on its own. In fact, it would not work without the
//! other crate. Further, it should not be separately added to your project. Add
//! `flogging` instead, and this will be included as a dependent to that crate.
//!
//! ```text
//! $ cargo add flogging
//! ```
//! Alternatively, add the following to your project's `Cargo.toml` file:
//! ```text
//! [dependencies]
//! flogging = "0.4.0"
//! ```
//!
//! ## Special Note
//!
//! For the macros that accept the parameter: `msg`, the following is true:
//!
//! - They accept parameters the same as for [`std::format!`](https://doc.rust-lang.org/std/macro.format.html)
//!     - plain text `&str`: `("It's your time.")`
//!     - format `&str` with interpolated variables: `("Var: {var}")`
//!     - format `&str` with supporting parameters: `("Var: {}", var)`
//!     - Combination of the last two: `("Vars {var1} - {}:{}", var2, var3)`
//! - Additional Feature
//!     - Just one or more variables: `(var1, var2, var3)`
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

mod format;
mod logger;

extern crate dyn_fmt;
extern crate proc_macro;
extern crate proc_macro_error;

use crate::{format::format_impl, logger::logger_impl};
use proc_macro::TokenStream;
// use proc_macro_error::proc_macro_error;

///
/// Log a CONFIG message.
///
/// CONFIG is a message level for static configuration messages.
///
/// CONFIG messages are intended to provide a variety of static
/// configuration information, to assist in debugging problems
/// that may be associated with particular configurations.
///
/// For example, a CONFIG message might include the CPU type, the
/// graphics depth, the GUI look-and-feel, etc.
///
/// If the logger is currently enabled for the CONFIG message level
/// then the given message is forwarded to all the registered output
/// Handler objects.
///
/// ## Parameters
/// - `msg` - See [Special Note](index.html#special-note)
///
/// ## Examples
///
/// ```no_run
/// use flogging::*;
/// use chrono::Local;
///
/// const_logger!({
///     Logger::console_logger(module_path!())
/// });
///
/// #[logger]
/// pub fn my_func(data: &str) {
///     config!("Some text to store.");
///
///     let time = Local::now();
///
///     config!(time);
///     config!(time, data);
///     config!("The configuration as at: {}", time);
///     config!("The configuration as at: {time}: {}", data);
///     config!("The configuration as at: {time:?}: {data}");
/// }
///
/// fn main(){
///     let data = "Some data";
///     my_func(data);
/// }
/// ```
/// Output:
/// ```text
/// |flogging->my_func| [CONFIG ] Some text to store.
/// |flogging->my_func| [CONFIG ] 2025-07-18 19:52:06.927418853 +08:00
/// |flogging->my_func| [CONFIG ] 2025-07-18 19:52:06.927418853 +08:00, Some data
/// |flogging->my_func| [CONFIG ] The configuration as at: 2025-07-18 19:52:06.927418853 +08:00
/// |flogging->my_func| [CONFIG ] The configuration as at: 2025-07-18 19:52:06.927418853 +08:00: Some data
/// |flogging->my_func| [CONFIG ] The configuration as at: 2025-07-18T19:52:06.927418853+08:00: Some data
/// ```
/// [format]: https://doc.rust-lang.org/std/macro.format.html
///
// #[proc_macro_error]
#[proc_macro]
pub fn config(msg: TokenStream) -> TokenStream {
    format_impl("__log.config({&__fmt});\n", msg)
}

///
/// Log entry into a function/method.
///
/// This is a convenience macro that can be used to log entry into to a function/method. It can be used
/// without an alternative message. A possible use is to provide the function/method's parameters
/// to track what is being passed-in.
///
/// If no alternative message is provided, then the default message, "Entry", is used.
///
/// A `LogEntry` is created with a log level of FINER, that is then logged.
///
/// ## Parameters
/// - `msg` - (Optional) The same as for [`std::format!`](https://doc.rust-lang.org/std/macro.format.html)
///
/// ## Examples
///```no_run
/// #[logger]
/// pub fn add_student(name: String, age: u8) {
///     entering!("name: {name}, age: {age}");
/// }
///
/// fn main(){
///     let name = "Mary Jane Thompson".to_string();
///     let age = 18;
///
///     add_student(name, age);
/// }
/// ```
/// Output:
/// ```text
/// |flogging->add_student| [FINER  ] Entry: (name: Mary Jane Thompson, age: 18)
/// ```
///
#[proc_macro]
pub fn entering(_msg: TokenStream) -> TokenStream {
    if _msg.to_string().is_empty() {
        "__log.entering();\n".parse().unwrap_or_default()
    } else {
        format_impl("__log.entering_with({&__fmt});\n", _msg)
    }
}

///
/// Log return from a function/method.
///
/// This is a convenience macro that can be used to log exiting from a function/method. It can be used
/// without an alternative message. A possible use is to provide the function/method's return value
/// to track what is being passed-out.
///
/// If no alternative message is provided, then the default message, "Return", is used.
///
/// A `LogEntry` is created with a log level of FINER, that is then logged.
///
/// ## Parameters
/// - `msg` - (Optional) The same as for [`std::format!`](https://doc.rust-lang.org/std/macro.format.html)
///
/// ## Examples
///```no_run
/// #[logger]
/// pub fn add_student(name: String, age: u8) -> bool {
///     let mut rtn = false;
///
///     entering!("name: {name}, age: {age}");
///
///     /* Some processing that provides a result (rtn) */
///     rtn = true;
///
///     exiting!("rtn: {rtn}");
///     rtn
/// }
///
/// fn main(){
///     let name = "Mary Jane Thompson".to_string();
///     let age = 18;
///
///     if add_student(name, age) {
///         println!("Success");
///     } else {
///         println!("Failure!");
///     }
/// }
/// ```
/// Output:
/// ```text
/// |flogging->add_student| [FINER  ] Entry: (name: Mary Jane Thompson, age: 18)
/// |flogging->add_student| [FINER  ] Return: (rtn: true)
/// Success
/// ```
///
#[proc_macro]
pub fn exiting(_msg: TokenStream) -> TokenStream {
    if _msg.to_string().is_empty() {
        "__log.exiting();\n".parse().unwrap_or_default()
    } else {
        format_impl("__log.exiting_with({&__fmt});\n", _msg)
    }
}

///
/// Log a FINE message.
///
/// FINE is a message level providing tracing information.
///
/// All of FINE, FINER, and FINEST are intended for relatively
/// detailed tracing. The exact meaning of the three levels will
/// vary between subsystems, but in general, FINEST should be
/// used for the most voluminous detailed output, FINER for somewhat
/// less detailed output, and FINE for the lowest volume (and most
/// important) messages.
///
/// In general the FINE level should be used for information that
/// will be broadly interesting to developers who do not have a
/// specialized interest in the specific subsystem.
///
/// FINE messages might include things like minor (recoverable)
/// failures. Issues indicating potential performance problems are
/// also worth logging as FINE.
///
/// If the logger is currently enabled for the FINE message level
/// then the given message is forwarded to all the registered output
/// Handler objects.
///
/// ## Parameters
/// - `msg` - The same as for [`std::format!`](https://doc.rust-lang.org/std/macro.format.html)
///
/// ## Examples
///
/// See [config](macro.config.html#examples). The syntax/usage is the same.
/// Just substitute `fine!` for `config!`.
///
#[proc_macro]
pub fn fine(msg: TokenStream) -> TokenStream {
    format_impl("__log.fine({&__fmt});\n", msg)
}

///
/// Log a FINER message.
///
/// FINER indicates a fairly detailed tracing message.
/// Suggest logging calls for entering, returning,
/// or `Error`s, such as returned via `Result`, are traced at
/// this level.
///
/// If the logger is currently enabled for the FINER message level
/// then the given message is forwarded to all the registered output
/// Handler objects.
///
/// ## Parameters
/// - `msg` - The same as for [`std::format!`](https://doc.rust-lang.org/std/macro.format.html)
///
/// ## Examples
///
/// See [config](macro.config.html#examples). The syntax/usage is the same.
/// Just substitute `finer!` for `config!`.
///
#[proc_macro]
pub fn finer(msg: TokenStream) -> TokenStream {
    format_impl("__log.finer({&__fmt});\n", msg)
}

///
/// Log a FINEST message.
///
/// FINEST indicates a highly detailed tracing message.
///
/// If the logger is currently enabled for the FINEST message level
/// then the given message is forwarded to all the registered output
/// Handler objects.
///
/// ## Parameters
/// - `msg` - The same as for [`std::format!`](https://doc.rust-lang.org/std/macro.format.html)
///
/// ## Examples
///
/// See [config](macro.config.html#examples). The syntax/usage is the same.
/// Just substitute `finest!` for `config!`.
///
#[proc_macro]
pub fn finest(msg: TokenStream) -> TokenStream {
    format_impl("__log.finest({&__fmt});\n", msg)
}

///
/// Get the required `Handler`.
///
/// ## Examples
/// ```no_run
/// extern crate flogging;
/// use flogging::*;
///
/// // Setting up the module level logger.
/// const_logger!({
///     Logger::builder(module_path!())
///         .add_string_handler()
///         .set_level(Level::ALL)
///         .build()
/// });
///
/// #[logger]
/// fn my_func(){
///     info!("Some text to store.");
///     warning!("Rain is wet!");
///     severe!("Hurricanes are windy!");
///
///     if let Some(h) = get_handler!(Handler::String) {
///         println!(
///             "\n(h.get_log())\n======v======\n{}\n======^======",
///             h.get_log()
///         );
///     } else {
///         println!("Sorry. Not there!");
///     }
/// }
/// ```
/// Output:
/// ```text
/// (h.get_log())
/// ======v======
/// |flogging->my_func| [INFO   ] Some text to store.
/// |flogging->my_func| [WARNING] Rain is wet!
/// |flogging->my_func| [SEVERE ] Hurricanes are windy!
///
/// ======^======
/// ```
///
#[proc_macro]
pub fn get_handler(handler: TokenStream) -> TokenStream {
    format!("__log.get_handler({handler})")
        .parse()
        .unwrap_or_default()
}

///
/// Log an INFO message.
///
/// INFO is a message level for informational messages.
///
/// Typically INFO messages will be written to the console or its
/// equivalent. So the INFO level should only be used for reasonably
/// significant messages that will make sense to end users and system
/// administrators.
///
/// **\[default level]** : This is the default level used when a logger is created.
/// See [set_level!] for changing this.
///
/// If the logger is currently enabled for the INFO message level
/// then the given message is forwarded to all the registered output
/// Handler objects.
///
/// ## Parameters
/// - `msg` - The same as for [`std::format!`](https://doc.rust-lang.org/std/macro.format.html)
///
/// ## Examples
///
/// See [config](macro.config.html#examples). The syntax/usage is the same.
/// Just substitute `info!` for `config!`.
///
#[proc_macro]
pub fn info(msg: TokenStream) -> TokenStream {
    format_impl("__log.info({&__fmt});\n", msg)
}

///
/// Provides for logging within the attributed function/method.
///
/// This is required to be able to use the [macros](index.html#macros)
///
/// ```no_run
/// #[logger]
/// pub fn my_func(msg: &str){
///     entering!();
///     fine!("msg: {msg}");
///
///     ...
/// }
/// ```
///
#[proc_macro_attribute]
pub fn logger(attr: TokenStream, item: TokenStream) -> TokenStream {
    logger_impl(attr, item)
}

///
/// Set the logging level for this `Logger` instance.
///
/// The default level is INFO.
///
/// ## Parameters
/// - `level` - The required logging level.
///
/// ## Examples
/// ```
/// #[logger]
/// pub fn my_func(msg: &str){
///     entering!("msg: {msg}");
///     fine!("Everything is just fine!");
///
///     // ...
/// }
///
/// fn main(){
///     set_level!(Level::FINER);
///
///     let msg = "Just some text to work with.";
///
///     my_func(msg);
/// }
///
/// ```
/// Output:
/// ```text
/// |flogging->my_func| [FINER  ] Entry: (msg: Just some text to work with.)
/// |flogging->my_func| [FINE   ] Everything is just fine!
/// ```
///
#[proc_macro]
pub fn set_level(level: TokenStream) -> TokenStream {
    format!("__log.set_level({level});\n")
        .parse()
        .unwrap_or_default()
}

///
/// Log a SEVERE message.
///
/// SEVERE is a message level indicating a serious failure.
///
/// In general SEVERE messages should describe events that are of
/// considerable importance and which will prevent normal program
/// execution. They should be reasonably intelligible to end users
/// and to system administrators.
///
/// If the logger is currently enabled for the SEVERE message level
/// then the given message is forwarded to all the registered output
/// Handler objects.
///
/// ## Parameters
/// - `msg` - The same as for [`std::format!`](https://doc.rust-lang.org/std/macro.format.html)
///
/// ## Examples
///
/// See [config](macro.config.html#examples). The syntax/usage is the same.
/// Just substitute `severe!` for `config!`.
///
#[proc_macro]
pub fn severe(msg: TokenStream) -> TokenStream {
    format_impl("__log.severe({&__fmt});\n", msg)
}

///
/// Log a WARNING message.
///
/// WARNING is a message level indicating a potential problem.
///
/// In general WARNING messages should describe events that will be
/// of interest to end users or system managers, or which indicate
/// potential problems.
///
/// If the logger is currently enabled for the WARNING message level
/// then the given message is forwarded to all the registered output
/// Handler objects.
///
/// ## Parameters
/// - `msg` - The same as for [`std::format!`](https://doc.rust-lang.org/std/macro.format.html)
///
/// ## Examples
///
/// See [config](macro.config.html#examples). The syntax/usage is the same.
/// Just substitute `warning!` for `config!`.
///
#[proc_macro]
pub fn warning(msg: TokenStream) -> TokenStream {
    format_impl("__log.warning({&__fmt});\n", msg)
}
