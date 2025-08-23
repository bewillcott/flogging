# Step 4

Let's add some testing:

```rust,no_run
#[cfg(test)]
mod tests{
    use super::*;
    use regex::Regex;

    const_logger!({
        Logger::builder(module_path!())
            .add_string_handler_with(
                FormatType::Custom,
                Some(Box::new(CsvFormatter::new())),
            )
            .build()
    });

    #[test]
    #[logger]
    fn csv_format() {
        entering!();

        let re_str =
"^(?:\\d{4}-\\d{2}-\\d{2} \\d{2}:\\d{2}:\\d{2}\\.\\d{6}),my_project::handlers::formatters::csv_formatter::tests->csv_format,INFO,\"Testing a new custom formatter.\"
(?:\\d{4}-\\d{2}-\\d{2} \\d{2}:\\d{2}:\\d{2}\\.\\d{6}),my_project::handlers::formatters::csv_formatter::tests->csv_format,WARNING,\"Must add more testing.\"
$";

        let re = Regex::new(re_str).unwrap();

        info!("Testing a new custom formatter.");
        warning!("Must add more testing.");

        let log_str = get_handler!(Handler::String).unwrap().get_log();

        println!("{log_str}");
        assert!(re.is_match(&log_str));
    }
}
```

Possible output:

```text
2025-08-21 19:15:04.089061,my_project::handlers::formatters::csv_formatter::tests->csv_format,INFO,"Testing a new custom formatter."
2025-08-21 19:15:04.089138,my_project::handlers::formatters::csv_formatter::tests->csv_format,WARNING,"Must add more testing."
```

Now let's do a final fixup of the API comments.

```rust, no_run
///
/// CSV format.
///
- /// The first part (before the decimal point) is
- /// the number of seconds since 1970-01-01 00:00 UTC.
- ///
- /// The second part is the number of nanoseconds since
- /// the last whole second.
+ /// The datetime format string is:
+ ///
+ /// ```text
+ /// "%Y-%m-%d %H:%M:%S%.6f"
+ /// ```
///
/// Example:
/// ```text
- /// 1752817859.157970496
+ /// 2025-08-21 19:15:04.089061
/// ```
/// Template:
/// - `dt` in the template would be the datetime string, similar to the above.
/// - `mod_path`, `fn_name`, `level`, and `message` all come out of the `LogEntry`
///   provided to the [`format()`][CsvFormatter::format] method.
///
/// ```ignore
- /// format!("{dt} {mod_path}->{fn_name} [{level:7}] {message}");
+ /// format!("{dt},{mod_path}->{fn_name},{level},\"{message}\"");
/// ```
/// Sample output:
/// ```text
- /// 1752818461.051538870 flogging->main [SEVERE ] Hurricanes are windy!
+ /// 2025-08-21 19:26:47.801287,my_project::handlers::formatters::csv_formatter::tests->csv_format,SEVERE,"Hurricanes are windy!"
/// ```
///
pub struct CsvFormatter {
    dt_fmt: String,
    fmt_string: String,
}
```
