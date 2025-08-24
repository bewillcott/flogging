# Idea

How would you like an effective way of controlling all of the logging within the "lib" part of your crate? Of course, still allowing each mod/file instance to be individually set to its own level.

Let's say you have something like this project (`my_project`) structure:

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

    #[cfg(test)]
    mod tests{
        use super::*;

        #[test]
        fn core(){
            core::do_it();
        }

        #[test]
        fn control(){
            core::control::do_it();
        }
    }
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
    ```

- `src/core/mod.rs`\
  Notice the use of `DEBUG_LEVEL`, and `add_pconsole_handler()`:

    ```rust, no_run
    mod control;

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
    ```

The possible output from running the `src/lib.rs` tests is:

- console

    ```log
    ---- tests::core stdout ----
    my_project::core->do_it [FINER  ] Entry
    This text came from the file `src/core/mod.rs`, just to let you know.
    my_project::core->do_it [FINER  ] Return

    ---- tests::control stdout ----
    my_project::core::control->do_it [FINER  ] Entry
    my_project::core::control->do_it [INFO   ] Hello from `Control`.
    my_project::core::control->do_it [FINER  ] Return
    ```

- `test_logs/usage.log`

    ```log
    2025-08-24T14:44:05.696870596+08:00 my_project::core->do_it [FINER  ] Entry
    2025-08-24T14:44:05.696872319+08:00 my_project::core::control->do_it [FINER  ] Entry
    2025-08-24T14:44:05.697005364+08:00 my_project::core::control->do_it [INFO   ] Hello from `Control`.
    2025-08-24T14:44:05.697005807+08:00 my_project::core->do_it [INFO   ] This text came from the file `src/core/mod.rs`, just to let you know.
    2025-08-24T14:44:05.697036808+08:00 my_project::core->do_it [FINER  ] Return
    2025-08-24T14:44:05.697050888+08:00 my_project::core::control->do_it [FINER  ] Return
    ```

Now let's prepare for a production build.

First we change the global `DEBUG_LEVEL` to `OFF`, then we modify `src/core/mod.rs` to be set to `Level::INFO`.

- `src/lib.rs`

    ```rust, no_run
    ...
        pub(crate) const DEBUG_LEVEL:Level = Level::OFF;
    ...
    ```

- `src/core/mod.rs`

    ```rust, no_run
    ...
           .set_level(Level::INFO)
    ...
    ```

Now to see what comes out:

- console

    ```log
    ---- tests::core stdout ----
    This text came from the file `src/core/mod.rs`, just to let you know.
    ```

- `test_logs/usage.log`

    ```log
    2025-08-24T15:05:08.702705600+08:00 my_project::core->do_it [INFO   ] This text came from the file `src/core/mod.rs`, just to let you know.
    ```

Next, let's get rid of the log file (part of `src/core/mod.rs`):

```rust, no_run

...

const_logger!({
    Logger::builder(module_path!())
        .add_pconsole_handler()
        //   ^^^^^^^^
        // .remove_file("test_logs/usage.log")
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
    ---- tests::core stdout ----
    This text came from the file `src/core/mod.rs`, just to let you know.
    ```

- `test_logs/usage.log`

    ```log

    ```
