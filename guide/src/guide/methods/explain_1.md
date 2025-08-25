# Explain (Part 1)

If you compare the code under `Methods` with that under `Macros`, you will notice
up front that the macros has a file level 'logger', where as the methods has just a
global (crate) level constant (`DEBUG_LEVEL`).

Now there is _nothing_ stopping you from directly copying all of the expanded macro code
into your files. I would consider that a waste of effort though, as that is what macros are for.

Now then, let's look at the `main()` function first:

```rust, no_run
fn main() {
    let mut log = Logger::builder(module_path!())
        .set_fn_name("main")
        .add_pconsole_handler()
        //   ^^^^^^^^
        .remove_file("test_logs/usage.log")
        //^^^^^^^^^^
        .add_file_handler("test_logs/usage.log")
        .set_level(DEBUG_LEVEL)
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

First, notice the use of both `add_pconsole_handler()` and `remove_file()`, in comparison to
the other functions.

The use of _pconcole_ instead of _econsole_, will have `.info()` log
entries sent to the 'stdout' console, unformatted. This would facilitate providing normal
text messages to the user. Those messages that are part of the normal interaction with the program.
Yet still allowing other log entries to be highlighted as formatted output. And, since there is
also a file handler, of course all messages will go there, formatted.

Having the `remove_file()` function here, allows you to always have just the entries from
the last run available in the log file. This is _not_ necessary if you need to track entries
over multiple runs. It is here to keep it clean.

The code (`Logger::builder...`) is basically a copy of that used in the macros version to setup
the file level 'logger'. The main difference is that it also has the `.set_fn_name("main")`
function, which, among other things, is set by the `#[logger]` macro in the macros version.

There other difference between the macros and methods version , is the use of the `log` variable
to access the logging functionality. Again we have more coding to keep tract of.
