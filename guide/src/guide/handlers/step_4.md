# Step 4

Ok, so far so good. Hopefully, your project still builds without errors.

Now we get to the actual working parts.

Firstly, some cleanup. Since we are not allowing outside changes to the formatters, `get_formatter()` and `set_formatter()` will be changed.

```rust, no_run
    fn get_formatter(&self) -> Formatter {
-        self.formatter.clone()
+        self.file_fmt.clone()
    }

```

```rust, no_run
-    fn set_formatter(&mut self, formatter: Formatter) {
-        self.formatter = formatter;
-    }
+    fn set_formatter(&mut self, _formatter: Formatter) {}
```

Now the final changes:

```rust, no_run
    fn publish(&mut self, log_entry: &LogEntry) {
        if self.is_open() {
-            let mut buf = self.formatter.format(log_entry);
+            let mut buf = self.file_fmt.format(log_entry);
            buf.push('\n');

            self.file.as_mut().unwrap().write_all(buf.as_bytes());
+           println!("{}", self.con_fmt.format(log_entry));
        }
    }

```

It is done.
