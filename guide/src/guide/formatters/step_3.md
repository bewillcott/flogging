# Step 3

At this point your project should still build without errors.

Next, let's put together the datetime format string: `dt_fmt`.

After looking into the `crono` crates specifiers, we come up with this string:

```text
"%Y-%m-%d %H:%M:%S%.6f"
```

which should produce something like:

```text
2025-06-23 13:10:45.123456
```

Now, let's build the main format string: `fmt_string`.

First, look at the provided method: `FormatTrait::ft_fmt()`. It provides the following variables that can be included in your format string via interpolation:

- `dt` - The datetime formatted with: `dt_fmt`.
- `mod_path` - The module path, possibly supplied via: `module_path!()`.
- `fn_name` - The name of the function/method inside which the log entry was generated. Supplied by the `#[logger]` macro, or manually with the `set_fn_name()` method.
- `level` - The log `Level` for which the entry was created.
- `message` - The text of the log entry.

Using the specifiers available in `std:fmt`, we produce this:

```text
"{dt},{mod_path}->{fn_name},{level},\"{message}\""
```

with a possible output of:

```text
2025-06-23 13:10:45.123456,my_project::csv_formatter::tests->csv_format,INFO,"trait methods"
```

Using these format strings, we can now modify our new custom formatter as follows:

```rust, no_run, noplayground
impl CsvFormatter {
    ///
    /// Creates a new instance of `CsvFormatter`.
    ///
    pub fn new() -> Self {
        Self {
-            dt_fmt: "%s.%f".to_string(),
+            dt_fmt: "%Y-%m-%d %H:%M:%S%.6f".to_string(),
-            fmt_string: "{dt} {mod_path}->{fn_name} [{level:7}] {message}".to_string(),
+            fmt_string: "{dt},{mod_path}->{fn_name},{level},\"{message}\"".to_string(),
        }
    }
```
