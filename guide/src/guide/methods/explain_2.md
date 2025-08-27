# Explain (Part 2)

Now let's look at the `do_something()` function.

```rust, no_run, noplayground
fn do_something() {
    let mut log = Logger::builder(module_path!())
        .set_fn_name("do_something")
        //^^^^^^^^^^
        .add_econsole_handler()
        //   ^^^^^^^^
        .add_file_handler("test_logs/usage.log")
        .set_level(DEBUG_LEVEL)
        .build();

    log.entering();

    // do some work worth noting
    let result = "Just something to log.";
    log.info(&format!("Did some work here.\n  {result}"));

    // ...

    log.fine("Bit more detail.");

    if let Err(e) = error_prone() {
        log.warning(&format!("Error: {}", e));
    }

    log.exiting();
}
```

Notice we have the `.set_fn_name("do_something")` and the `.add_econsole_handler()`
functions.

We need to set the function name in each function being logged, and in this case, we are sending
entries to the 'stderr' console. Also note, that there is no `remove_file()` function.

The diversity of using different handlers in each function is one of the main reasons for
using the methods over using the macros.

Now we get to one of my personal bug-bears with Rust (v1.88.0). _No variable list of parameters
for functions_. I have been using Java for many years, and it is a major convenience being
able to have that facility. So, here in this code we see the need to work around that limitation.

Let's compare  the macro version with the method version:

```rust, no_run, noplayground
info!("Did some work here.\n  {result}");
```

```rust, no_run, noplayground
log.info(&format!("Did some work here.\n  {result}"));
```

The method version requires the text to be preformatted before being passed to it.
Where as, the macro version does that for you, internally.

The same goes for:

```rust, no_run, noplayground
warning!("Error: {}", e);
```

```rust, no_run, noplayground
log.warning(&format!("Error: {}", e));
```
