# Explain (Part 2)

Now the next part of the code:

```rust, no_run
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
```

Firstly, the attribute macro: `logger`.

The API says:

```text
Provides for logging within the attributed function/method.

This is required to be able to use the macros. It sets up the local variable used by
the other macros, and it also registers the function/method name used by the log
entries (if included in the formatter’s 'fmt_string').
```

This is what handles all of the repetitive coding needed for each of the logged functions/methods.

As you can see, using the macros is simple, coding efficient and tidy. Part of this, is the ability to have a _variable number of parameters_. As a **Java** programmer, this is something that I miss! In **Rust**, functions/methods don't have this option. As you will see with [`Methods`](../methods.md), this needs to be handled differently.

In this example, you can see macros with:

- no parameters (`entering!()` and `exiting()`),
- a single `&str` (`fine!("Bit more detail.")`),
- a single `&str` with an interpolated variable (`info!("Did some work here.\n  {result}")`),
- multiple parameters (`warning!("Error: {}", e)`)\
  I understand that this could have been interpolated, but it is shown here as an example of the ability to emulate macros like `format!()`.
