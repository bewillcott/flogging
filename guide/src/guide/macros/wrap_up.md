# Wrap Up

Using the macros is simple, coding efficient and tidy. Part of this, is the ability to have a _variable number of parameters_. As a **Java** programmer, this is something that I miss! In **Rust**, functions/methods don't have this option. As you will see with [`Methods`](../methods.md), this needs to be handled differently.

In this example, you've seen macros with:

- no parameters (`entering!()` and `exiting()`),
- a single `&str` (`fine!("Bit more detail.")`),
- a single `&str` with an interpolated variable (`info!("Did some work here.\n  {result}")`),
- multiple parameters (`warning!("Error: {}", e)`)\
  I understand that this _could_ have been interpolated, but it is shown here as an example of the ability to emulate macros like `format!()`.
