# Step 3

With "Step 2" completed, you should be able to build your project without errors.

At this point, our new handler can output to any log file we need. However, we now need to add the console output component. For this exercise, we are going to use a different formatter for the console output, to that for the file output. To keep things simple, we will only provide a "default" formatter option. That is, there will be no ability to pass-in a different one later. The same limitation for the file handler component.

Firstly, we will need to modify: `struct ConfileHandler`.

```rust, no_run
///
/// Publishes log entries to the file whose name was provided during
/// initialization.
///
#[derive(Debug, Default)]
pub struct ConfileHandler {
    filename: String,
-    formatter: Formatter,
+    file_fmt: Formatter,
+    con_fmt: Formatter,
    file: Option<File>,
}
```

Now: `impl ConfileHandler`.

```rust, no_run
impl ConfileHandler {
    fn create(filename: &str) -> Result<Self, Error> {
        if filename.is_empty() {
            return Err(Error::new(InvalidInput, "'filename' must not be empty"));
        }

        let fh = ConfileHandler {
            filename: filename.to_string(),
-            formatter: FormatType::Iso8601.create(None),
+            file_fmt: FormatType::Iso8601.create(None),
+            con_fmt: FormatType::Simple.create(None),
            file: {
                let f = File::options().append(true).create(true).open(filename)?;

                Some(f)
            },
        };

        Ok(fh)
    }
}
```

Now: `impl fmt::Display for ConfileHandler`.

```rust, no_run
impl fmt::Display for ConfileHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
-        write!(f, "{} : {}", self.filename, self.formatter)
+        write!(f, "{} : {}\nConsole: {}", self.filename, self.file_fmt, self.con_fmt)
    }
}

```
