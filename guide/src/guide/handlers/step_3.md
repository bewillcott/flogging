# Step 3

With "Step 2" completed, you should be able to build your project without errors.

At this point, our new handler can output to any log file we need.
However, we now need to add the console output component.
For this exercise, we are going to use a different formatter for the console output,
to that for the file output. To keep things simple, we will only provide a "default"
formatter option. That is, there will be no ability to pass-in a different one later.

Firstly, we will need to modify: `struct ConfileHandler`.

```rust, no_run, noplayground
///
/// Publishes log entries to the file whose name was provided during
/// initialization.
///
#[derive(Debug, Default)]
pub struct ConfileHandler {
    filename: String,
-     formatter: Formatter,
+     con_fmt: Formatter,
+     file_fmt: Formatter,
    file: Option<File>,
    writer: Option<Vec<u8>>,
}
```

Now: `impl ConfileHandler`.

```rust, no_run, noplayground
impl ConfileHandler {
    fn create(filename: &str) -> Result<Self, Error> {
        if filename.is_empty() {
            return Err(Error::new(InvalidInput, "'filename' must not be empty"));
        }

        let fh = ConfileHandler {
            filename: filename.to_string(),
-             formatter: FormatType::Iso8601.create(None),
+             con_fmt: FormatType::Simple.create(None),
+             file_fmt: FormatType::Iso8601.create(None),
            file: {
                let f = File::options().append(true).create(true).open(filename)?;
                Some(f)
            },
            writer: None,
        };

        Ok(fh)
    }
}
```

Now: `impl fmt::Display for ConfileHandler`.

```rust, no_run, noplayground
impl fmt::Display for ConfileHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
-         write!(f, "{} : {}", self.filename, self.formatter)
+         write!(
+             f,
+             "Console: {}\n{} : {}",
+             self.con_fmt, self.filename, self.file_fmt
+         )

    }
}
```
