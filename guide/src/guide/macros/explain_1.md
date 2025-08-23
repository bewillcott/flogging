<!-- markdownlint-disable-file MD001 -->

# Explain (Part 1)

Ok, what's happening in this code?

Let's look at the first part:

```rust, no_run
use flogging::*;
use std::{error::Error, result::Result};

// Setting up the module level logger.
const_logger!({
    Logger::builder(module_path!())
        .add_console_handler()
        .remove_file("test_logs/usage.log")
        .add_file_handler("test_logs/usage.log")
        .set_level(Level::FINEST)
        .build()
});
```

I think the first two lines are self explanatory.

So let's dive into the `const_logger({...});` macro.

The API says:

```md
Setup module level logger access.

The basic macro syntax is:

    const_logger!({/* the block of Rust code to build a Logger goes here */});

Notice there are curly braces "{}" wrapping the inner Rust code. **They are required**.

The code you put in here will depend on what configuration of Logger you want to setup.
```

As you can see, we are using the `Logger::builder(...)` method. With this we can be very specific about the logger that we end-up with. Note that this method returns a `LoggerBuilder` object and _not_ a `Logger` object. The appended methods, are all implemented under `LoggerBuilder`. The final method above, `build()`, returns the fully configured `Logger` object.

The primary purpose of this macro, is to setup a **mod**/**file** level logger environment. This is required to be able to use the rest of the macros.

If you look into the [API], you will find many possible options for your logger configuration.

---

**Please note:**

It states "module level".

Each module or file will require its own instance of this setup. This is intentional, to keep things simple to follow and maintain. Each mod/file has a different `module_path!` which can only be set within its own instance of this setup. Without this distinction, your logs would be an anonymous mess, such that you would not easily know from where a log entry came.

Let's assume you want to setup just a "global" instance in `lib.rs`, to be used by all of its "child" mods/files. This is what you might get from logging a function - `do_it()`:

```text
test_proj->do_it [FINER  ] Entry
```

or with individual instances (as currently required):

```text
test_proj::core::control->do_it [FINER  ] Entry
```

where both examples are from the file: `src/core/control.rs`.

Another reason for separate instances, is that you may want different logger configurations for each one, or atleast, for one or more of them.

One possibility, might be a 'mod' that uses `add_pconsole_handler()`, to be able to have `INFO` level log entries output to the console without any formatting, like this:

```text
This text came from the file `src/core/mod.rs`, just to let you know.
```

instead of:

```text
test_proj::core::mod->do_it [INFO   ] This text came from the file `src/core/mod.rs`, just to let you know.
```

You might set this particular file's instance to log level `Level::INFO`, and have regular types of textual output that would be a normal part of the running program.

Mix and match - Have fun!!!

---

### Idea

Let's say you have something like this project (`test_proj`) structure:

- `src/`
  - `lib.rs`
  - `core/`
    - `control.rs`
    - `mod.rs`

You might have:

- `src/lib.rs`

    ```rust, no_run
    mod core;

    pub(crate) use flogging::*;
    pub use core::*;

    pub(crate) const DEBUG_LEVEL:Level = Level::ALL;
    ```

- `src/core/control.rs`\
  Notice the use of `DEBUG_LEVEL`:

    ```rust, no_run
    use crate::*;

    const_logger!({
        Logger::builder(module_path!())
            .add_console_handler()
            .remove_file("test_logs/usage.log")
            .add_file_handler("test_logs/usage.log")
            .set_level(DEBUG_LEVEL)
            //         ^^^^^^^^^^^
            .build()
    });

    #[logger]
    pub fn do_it() {
        entering!();
    }
    ...
    ```

- `src/core/mod.rs`\
  Notice the use of `DEBUG_LEVEL`, and `add_pconsole_handler()`:

    ```rust, no_run
    use crate::*;

    const_logger!({
        Logger::builder(module_path!())
            .add_pconsole_handler()
            //   ^^^^^^^^
            .remove_file("test_logs/usage.log")
            .add_file_handler("test_logs/usage.log")
            .set_level(DEBUG_LEVEL)
            //         ^^^^^^^^^^^
            .build()
    });

    #[logger]
    pub fn do_it() {
        entering!();
        info!("This text came from the file `src/core/mod.rs`, just to let you know.");
        exiting!();
    }
    ...
    ```

This is an effective way of controlling all of the logging within the "lib" part of your crate. Of course, each mod/file instance can be individually set to its own level.

### Warning

Some gotchas. Well atleast they keep getting me:

- "`{}`" - You _must_ have the internal braces wrapping the `Logger::builder(...)` code.
- ! "`;`" - Do _not_ terminate the braced code with a "`;`". In this example, after the `.build()`
- "`;`" - Don't forget the final "`;`" after the macro: "`});`"

[API]: https://docs.rs/flogging/latest/flogging/struct.LoggerBuilder.html
