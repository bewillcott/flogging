# Step 4

Ok, so far so good. Hopefully, your project still builds without errors.

Now we get to the actual working parts.

Firstly, some cleanup. We will make it possible to change the `file_fmt` once we have developed our custom formatter. So let's change `get_formatter()` and `set_formatter()`.

```rust, no_run, noplayground
    fn get_formatter(&self) -> Formatter {
-         self.formatter.clone()
+         self.file_fmt.clone()
    }

```

```rust, no_run, noplayground
    fn set_formatter(&mut self, formatter: Formatter) {
-         self.formatter = formatter;
+         self.file_fmt = formatter;
    }
```

Now the final changes:

```rust, no_run, noplayground
    fn publish(&mut self, log_entry: &LogEntry) {
        if self.is_open() {
-             let mut buf = self.formatter.format(log_entry);
+             let mut buf = self.file_fmt.format(log_entry);
            buf.push('\n');

            if let Some(w) = self.writer.as_mut() {
-                 writeln!(w, "{}", self.formatter.format(log_entry)).expect("writeln!() failed");
+                 writeln!(w, "{}", self.con_fmt.format(log_entry)).expect("writeln!() failed");
+                 writeln!(w, "{}", self.file_fmt.format(log_entry)).expect("writeln!() failed");
            } else {
+                 println!("{}", self.con_fmt.format(log_entry));
                self.file
                    .as_mut()
                    .unwrap()
                    .write_all(buf.as_bytes())
                    .expect("write_all() failed");
            }
        }
    }
```

It is done.
