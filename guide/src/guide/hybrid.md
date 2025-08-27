# Hybrid

Now that I have shown you both sides of the `Flogging` coin, let's
talk practicality.

Both the macros and the methods have their benefits. The macros are convenient, and
the methods are flexible. So, when needed, why not combine them?

What a great idea! Who would have thought?

So, let's see what we can do with our example code.

The following code is the `main.rs` file from the `my_project` example, under the [`Usage_Hybrid`] branch.

```rust, no_run, noplayground
//
// File Name:    main.rs
// Directory:    src
// Project Name: my_project
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
//! # main file
//!

use flogging::*;
use std::{error::Error, result::Result};

// Setting up the module level logger.
const_logger!({
    Logger::builder(module_path!())
        .add_console_handler()
        .add_file_handler("test_logs/usage.log")
        .set_level(Level::ALL)
        .build()
});

#[logger]
fn do_something() {
    entering!();

    // do some work worth noting
    let result = "Just something to log.";
    info!("Did some work here.\n  {result}");

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

fn main() {
    let mut log = Logger::builder(module_path!())
        .set_fn_name("main")
        .add_pconsole_handler()
        .remove_file("test_logs/usage.log")
        .add_file_handler("test_logs/usage.log")
        .set_level(Level::ALL)
        .build();

    log.entering();
    log.info("All logging macros accept the same parameters as `std::format!(...)`");
    log.warning("Those same macros (info, etc.) MUST have atleast one parameter.");
    log.config("This is running on Fedora Linux 42.");
    do_something();
    log.info("Job's done.");
    log.exiting();
}
```

What have I done here?

For the bulk of the functions, in this case, `do_something()` and `error_prone()`, I am
using the macros. This prevents the boilerplating and other code bloat problems. Also, I
have removed the `remove_file()` from the macro setup code.

However, for the `main()` function, where we want to use the _pconsole_ option, we are
using the methods directly. Notice we are using the same log file for all.

I have used `main()` to be my 'methods' recipient here, but you could have any of your functions
being setup in a similar fashion.

Of course, there is no reason you couldn't simply have the 'macro' setup, using the _pconsole_ option,
and restricting your use of the `INFO` level logging, to those messages that are expected to be
displayed during the normal use of your program.

In other words, `FLogging` leaves it entirely up to you and your imagination, or your project's
requirements, as to how it is used.

_Mix and match to your hearts desire!_

P.S.: See what happens when you set the 'macro' setup to `Level::OFF`, and the `main()` to `Level::INFO`.

[`Usage_Hybrid`]: https://github.com/bewillcott/my_project/tree/Usage_Hybrid
