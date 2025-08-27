# Explain (Part 3)

And finally, we get to the last function.

```rust, no_run, noplayground
fn error_prone() -> Result<(), Box<dyn Error>> {
    let mut log = Logger::builder(module_path!())
        .set_fn_name("error_prone")
        .add_econsole_handler()
        .add_file_handler("test_logs/usage.log")
        .set_level(DEBUG_LEVEL)
        .build();

    log.entering();
    let rtn = Err(Box::from("Bad day!"));
    log.exiting_with(&format!("{rtn:?}"));
    rtn
}
```

I expect you are noticing a pattern here. Boilerplate code everywhere.
Apart from the function name being different in each case, the rest of
the 'logger' code is the same as with the `do_something()` function. And, I
would expect this to be the case for most situations.

We also have a case of using `format!()` to preformat a message:

```rust, no_run, noplayground
log.exiting_with(&format!("{rtn:?}"));
```
