<!-- markdownlint-disable-file MD001 MD033 -->

# Explain (Part 1)

Ok, what's happening in this code?

Let's look at the first part:

```rust, no_run, noplayground
use flogging::*;
use std::{error::Error, result::Result};

// Setting up the module level logger.
const_logger!({ // <= [1]
    Logger::builder(module_path!())
        .add_console_handler()
        .remove_file("test_logs/usage.log")
        .add_file_handler("test_logs/usage.log")
        .set_level(Level::ALL)
        .build() // <= [2]
}); // <= [1] [3]
```

<div class="warning">

Some gotchas. Well atleast they keep getting me:

- `[1]` - You _must_ have the internal braces wrapping the\
  `{ Logger::builder(...) ... }` code.
- `[2]` - Do _not_ terminate the braced code with a "`;`".\
  In this example, after the `.build()`
- `[3]` - Don't forget the final "`;`" after the macro: "`});`"

</div>

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
my_project->do_it [FINER  ] Entry
```

or with individual instances (as currently required):

```text
my_project::core::control->do_it [FINER  ] Entry
```

where both examples are from the file: `src/core/control.rs`.

Another reason for separate instances, is that you may want different logger configurations for each one, or atleast, for one or more of them.

One possibility, might be a 'mod' that uses `add_pconsole_handler()`, to be able to have `INFO` level log entries output to the console without any formatting, like this:

```text
This text came from the file `src/core/mod.rs`, just to let you know.
```

instead of:

```text
my_project::core::mod->do_it [INFO   ] This text came from the file `src/core/mod.rs`, just to let you know.
```

You might set this particular file's instance to log level `Level::INFO`, and have regular types of textual output that would be a normal part of the running program.

Mix and match - Have fun!!!

[API]: /api/flogging/struct.LoggerBuilder.html
