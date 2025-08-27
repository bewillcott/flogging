# Idea

How would you like an effective way of controlling all of the logging within the "lib" part of your crate?
Of course, still allowing each mod/file instance to be individually set to its own level.

Let's say you have something like this project ([`my_project`]) structure:

- `src/`
  - `lib.rs`
  - `core/`
    - `control.rs`
    - `mod.rs`

You might have:

- `src/lib.rs`

    ```rust, no_run, noplayground
    mod my_core;

    pub(crate) use flogging::*;
    pub use my_core::*;

    //
    // Cargo.toml
    //
    // [dependencies]
    // ctor = "0.5.0"
    use ctor::*;

    pub(crate) const DEBUG_LEVEL:Level = Level::ALL;
    // pub(crate) const DEBUG_LEVEL:Level = Level::OFF;

    ///
    /// Reset the log file each time `my_project` is loaded.
    ///
    /// This is an alternative to using `remove_file()` in
    /// the individual mod/file setup commands.\
    /// Only useful if all child mods are using the same log file.
    ///
    #[ctor]
    fn reset_log(){
        Logger::remove_file("test_logs/usage.log");
    }

    #[cfg(test)]
    mod tests{
        use super::*;

        #[test]
        fn control(){
            my_core::control::do_it();
        }

        #[test]
        fn my_core(){
            my_core::do_it();
        }

    }
    ```

- `src/core/control.rs`\
  Notice the use of `DEBUG_LEVEL`:

    ```rust, no_run, noplayground
    use crate::*;

    const_logger!({
        Logger::builder(module_path!())
            .add_console_handler()
            .add_file_handler("test_logs/usage.log")
            .set_level(DEBUG_LEVEL)
            //         ^^^^^^^^^^^
            .build()
    });

    #[logger]
    pub fn do_it() {
        entering!();
        info!("Hello from `Control`.");
        exiting!();
    }
    ```

- `src/core/mod.rs`\
  Notice the use of `DEBUG_LEVEL`, and `add_pconsole_handler()`:

    ```rust, no_run, noplayground
    pub mod control;

    use crate::*;

    const_logger!({
        Logger::builder(module_path!())
            .add_pconsole_handler()
            //   ^^^^^^^^
            .add_file_handler("test_logs/usage.log")
            // .set_level(Level::INFO)
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
    ```

The possible output from running the `src/lib.rs` tests is:

- console

    ```log
    ---- tests::control stdout ----
    my_project::my_core::control->do_it [FINER  ] Entry
    my_project::my_core::control->do_it [INFO   ] Hello from `Control`.
    my_project::my_core::control->do_it [FINER  ] Return

    ---- tests::my_core stdout ----
    my_project::my_core->do_it [FINER  ] Entry
    This text came from the file `src/core/mod.rs`, just to let you know.
    my_project::my_core->do_it [FINER  ] Return
    ```

- `test_logs/usage.log`

    ```log
    2025-08-27T10:05:48.581421788+08:00 my_project::my_core::control->do_it [FINER  ] Entry
    2025-08-27T10:05:48.581421917+08:00 my_project::my_core->do_it [FINER  ] Entry
    2025-08-27T10:05:48.581505388+08:00 my_project::my_core::control->do_it [INFO   ] Hello from `Control`.
    2025-08-27T10:05:48.581510841+08:00 my_project::my_core->do_it [INFO   ] This text came from the file `src/core/mod.rs`, just to let you know.
    2025-08-27T10:05:48.581528496+08:00 my_project::my_core::control->do_it [FINER  ] Return
    2025-08-27T10:05:48.581533425+08:00 my_project::my_core->do_it [FINER  ] Return
    ```

Now let's prepare for a production build.

First we change the global `DEBUG_LEVEL` to `OFF`, then we modify `src/core/mod.rs` to be set to `Level::INFO`.

- `src/lib.rs`

    ```rust, no_run, noplayground
    ...
        pub(crate) const DEBUG_LEVEL:Level = Level::OFF;
    ...
    ```

- `src/core/mod.rs`

    ```rust, no_run, noplayground
    ...
           .set_level(Level::INFO)
    ...
    ```

Now to see what comes out:

- console

    ```log
    ---- tests::my_core stdout ----
    This text came from the file `src/core/mod.rs`, just to let you know.
    ```

- `test_logs/usage.log`

    ```log
    2025-08-27T10:07:36.124653478+08:00 my_project::my_core->do_it [INFO   ] This text came from the file `src/core/mod.rs`, just to let you know.
    ```

Next, let's get rid of the log file (part of `src/core/mod.rs`):

```rust, no_run, noplayground

...

const_logger!({
    Logger::builder(module_path!())
        .add_pconsole_handler()
        //   ^^^^^^^^
        // .add_file_handler("test_logs/usage.log")
        // .set_level(DEBUG_LEVEL)
        .set_level(Level::INFO)
        //         ^^^^^^^^^^^
        .build()
});

...
```

And the final production output is:

- console

    ```log
    ---- tests::my_core stdout ----
    This text came from the file `src/core/mod.rs`, just to let you know.
    ```

- `test_logs/usage.log`

    ```log

    ```

[`my_project`]: https://github.com/bewillcott/my_project/tree/Idea
