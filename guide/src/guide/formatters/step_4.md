# Step 4

Let's add some testing:

```rust,no_run

#[cfg(test)]
mod tests{
    use super::*;

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

        info!("Testing a new custom formatter.");
        warning!("Must add more testing.");

        let log_str = get_handler!(Handler::String).unwrap().get_log();

        print!("{log_str}");
    }
}
```

Possible output:

```text
2025-06-23 13:10:45.123456,my_project::csv_formatter::tests->csv_format,INFO,"Testing a new custom formatter."
2025-06-23 13:10:45.123529,my_project::csv_formatter::tests->csv_format,WARNING,"Must add more testing."
```
