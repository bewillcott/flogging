# Step 5

Oops! Forgot the testing!!!!

Ok, what are we doing with that?

Since we are using a custom handler, we need to use the appropriate function.

```rust, no_run
#[cfg(test)]
mod tests {
    use super::*;

-    use crate::{Logger, logger};

    #[test]
    fn handler_trait() {
-        let mut log = Logger::file_logger(module_path!(), "test.log");
+        let mut log = Logger::custom_logger(
+            module_path!(),
+            "ConfileHandler",
+            Box::new(ConfileHandler::create("example.log").unwrap()),
+        );

+        log.set_fn_name("handler_trait");
        log.info("trait methods");

-        let handler = log.get_handler(crate::Handler::File).unwrap();
+        let handler = log
+            .get_handler(Handler::Custom("ConfileHandler".to_string()))
+            .unwrap();
        assert!(handler.is_open());
        assert_eq!(handler.get_formatter().to_string(), "dt_fmt: \"%+\" - fmt_string: \"{dt:35} |{mod_path}->{fn_name}| [{level:7}] {message}\"".to_string());
        assert_eq!(handler.get_log(), "".to_string());
        handler.flush();
        handler.close();
    }

    #[test]
    #[should_panic(expected = "'filename' must not be empty")]
    fn filename_empty() {
-        let mut log = Logger::file_logger(module_path!(), "");
+        let mut log = Logger::custom_logger(
+            module_path!(),
+            "ConfileHandler",
+            Box::new(ConfileHandler::create("").unwrap()),
+        );
    }
}
```

Possible output, minus the tester's extraneous stuff:

- console

```text
my_project::confile_handler::tests->handler_trait [INFO   ] trait methods
```

- `example.log`

```text
2025-08-14T18:14:37.468423953+08:00 my_project::confile_handler::tests->handler_trait [INFO   ] trait methods
```
