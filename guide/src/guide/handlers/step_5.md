# Step 5

Oops! Forgot the testing!!!!

Ok, what are we doing with that?

Since we are using a custom handler, we need to use the appropriate methods.

```rust, no_run, noplayground

#[cfg(test)]
mod tests {
-     use crate::*;
-     use std::{
-         fs::File,
-         io::{Error, Read, Result},
-     };

+     use super::*;
+     use std::io::Read;
+     use regex::Regex;
```

---

```rust, no_run, noplayground
    #[test]
-     fn file_handler() {
+     fn confile_handler() {
-         let mut log = Logger::file_logger(module_path!(), "test_logs/file_handler.log");
-         log.set_fn_name("file_handler");
+         let mut log = Logger::custom_logger(
+             module_path!(),
+             "ConfileHandler",
+             Box::new(ConfileHandler::create("test_logs/confile_handler.log").unwrap()),
+         );
+
+         log.set_fn_name("confile_handler");

-         let h = log.get_handler(crate::Handler::File).unwrap();
+         let h = log
+             .get_handler(Handler::Custom("ConfileHandler".to_string()))
+             .unwrap();
+
        h.set_test_mode(false);

        assert!(h.is_open());
        assert_eq!(
            h.get_formatter().to_string(),
            "dt_fmt: \"%+\" - fmt_string: \"{dt:35} {mod_path}->{fn_name} [{level:7}] {message}\""
                .to_string()
        );

        log.info("trait methods");
        log.warning("The sky is falling!");

-         let h = log.get_handler(crate::Handler::File).unwrap();
+         let h = log
+             .get_handler(Handler::Custom("ConfileHandler".to_string()))
+             .unwrap();

        assert_eq!(h.get_log(), "".to_string());

        h.flush();
        h.close();
        log.exiting_with("This should get thrown away.");
    }
```

Possible output:

- console

    ```text
    ---- handlers::confile_handler::tests::confile_handler stdout ----
    my_project::handlers::confile_handler::tests->confile_handler [INFO   ] trait methods
    my_project::handlers::confile_handler::tests->confile_handler [WARNING] The sky is falling!
    ```

- "test_logs/confile_handler.log"\
    Note: This file will continue to grow. So there maybe previous entries.

    ```text
    2025-08-27T11:52:13.273205045+08:00 my_project::handlers::confile_handler::tests->confile_handler [INFO   ] trait methods
    2025-08-27T11:52:13.273294181+08:00 my_project::handlers::confile_handler::tests->confile_handler [WARNING] The sky is falling!
    ```

---

```rust, no_run, noplayground
    #[test]
-     fn file_handler_file_test() {
+     fn confile_handler_file_test() {
-         let expected = "flogging::handlers::file_handler::tests->file_handler_file_test [INFO   ] trait methods
- flogging::handlers::file_handler::tests->file_handler_file_test [WARNING] The sky is falling!\n"
-         .to_string();
+         let re_str =
+ "^(?:\\d{4}-\\d{2}-\\d{2}T\\d{2}:\\d{2}:\\d{2}\\.\\d{9}\\+\\d{2}:\\d{2}) my_project::handlers::confile_handler::tests->confile_handler_file_test \\[INFO   ] trait methods
+ (?:\\d{4}-\\d{2}-\\d{2}T\\d{2}:\\d{2}:\\d{2}\\.\\d{9}\\+\\d{2}:\\d{2}) my_project::handlers::confile_handler::tests->confile_handler_file_test \\[WARNING] The sky is falling!
+ $";
+
+         let re = Regex::new(re_str).unwrap();

        let mut log = Logger::builder(module_path!())
-             .set_fn_name("file_handler_file_test")
-             .remove_file("test_logs/file_handler_file_test.log")
-             .add_file_handler_with(
-                 "test_logs/file_handler_file_test.log",
-                 FormatType::Simple,
-                 None,
-             )
+             .set_fn_name("confile_handler_file_test")
+             .remove_file("test_logs/confile_handler_file_test.log")
+             .add_custom_handler(
+                 "ConfileHandler",
+                 Box::new(
+                     ConfileHandler::create("test_logs/confile_handler_file_test.log").unwrap(),
+                 ),
+             )
            .build();

-         let h = log.get_handler(crate::Handler::File).unwrap();
+         let h = log
+             .get_handler(crate::Handler::Custom("ConfileHandler".to_string()))
+             .unwrap();
+
        h.set_test_mode(false);

        assert!(h.is_open());
        assert_eq!(
            h.get_formatter().to_string(),
-             "dt_fmt: \"\" - fmt_string: \"{mod_path}->{fn_name} [{level:7}] {message}\""
+             "dt_fmt: \"%+\" - fmt_string: \"{dt:35} {mod_path}->{fn_name} [{level:7}] {message}\""
                .to_string()
        );

        log.info("trait methods");
        log.warning("The sky is falling!");

-         let h = log.get_handler(crate::Handler::File).unwrap();
+         let h = log
+             .get_handler(crate::Handler::Custom("ConfileHandler".to_string()))
+             .unwrap();

        assert_eq!(h.get_log(), "".to_string());

        h.flush();
        h.close();
        assert!(!h.is_open());

        log.severe("This should get thrown away.");

-         if let Ok(mut file) = File::open("test_logs/file_handler_file_test.log") {
+         if let Ok(mut file) = File::open("test_logs/confile_handler_file_test.log") {
            let mut buf = String::new();
            if let Ok(count) = file.read_to_string(&mut buf) {
-                 assert_eq!(expected, buf);
+                 assert!(re.is_match(&buf));
            }
        }
    }
```

Possible output:

- console

    ```text
    ---- handlers::confile_handler::tests::confile_handler_file_test stdout ----
    my_project::handlers::confile_handler::tests->confile_handler_file_test [INFO   ] trait methods
    my_project::handlers::confile_handler::tests->confile_handler_file_test [WARNING] The sky is falling!
    ```

- "test_logs/confile_handler_file_test.log"\

    ```text
    2025-08-27T11:54:07.165918573+08:00 my_project::handlers::confile_handler::tests->confile_handler_file_test [INFO   ] trait methods
    2025-08-27T11:54:07.166004387+08:00 my_project::handlers::confile_handler::tests->confile_handler_file_test [WARNING] The sky is falling!
    ```

---

```rust, no_run, noplayground
    #[test]
-     fn file_handler_test_mode() {
+     fn confile_handler_test_mode() {
-         let expected = "flogging::handlers::file_handler::tests->file_handler_test_mode [INFO   ] trait methods
- flogging::handlers::file_handler::tests->file_handler_test_mode [WARNING] The sky is falling!\n"
-             .to_string();
+         let re_str =
+ "^my_project::handlers::confile_handler::tests->confile_handler_test_mode \\[INFO   ] trait methods
+ (?:\\d{4}-\\d{2}-\\d{2}T\\d{2}:\\d{2}:\\d{2}\\.\\d{9}\\+\\d{2}:\\d{2}) my_project::handlers::confile_handler::tests->confile_handler_test_mode \\[INFO   ] trait methods
+ my_project::handlers::confile_handler::tests->confile_handler_test_mode \\[WARNING] The sky is falling!
+ (?:\\d{4}-\\d{2}-\\d{2}T\\d{2}:\\d{2}:\\d{2}\\.\\d{9}\\+\\d{2}:\\d{2}) my_project::handlers::confile_handler::tests->confile_handler_test_mode \\[WARNING] The sky is falling!
+ $";
+
+         let re = Regex::new(re_str).unwrap();

        let mut log = Logger::builder(module_path!())
-             .set_fn_name("file_handler_test_mode")
-             .remove_file("test_logs/file_handler_test_mode.log")
-             .add_file_handler_with(
-                 "test_logs/file_handler_test_mode.log",
-                 FormatType::Simple,
-                 None,
-             )
+             .set_fn_name("confile_handler_test_mode")
+             .add_custom_handler(
+                 "ConfileHandler",
+                 Box::new(
+                     // This file is never written to:
+                     ConfileHandler::create("test_logs/confile_handler_test_mode.log").unwrap(),
+                 ),
+             )
            .build();

-         let h = log.get_handler(crate::Handler::File).unwrap();
+         let h = log
+             .get_handler(crate::Handler::Custom("ConfileHandler".to_string()))
+             .unwrap();
+
+         // All log entries will be stored in the internal buffer.
        h.set_test_mode(true);

        assert!(h.is_open());
        assert_eq!(
            h.get_formatter().to_string(),
-             "dt_fmt: \"\" - fmt_string: \"{mod_path}->{fn_name} [{level:7}] {message}\""
+             "dt_fmt: \"%+\" - fmt_string: \"{dt:35} {mod_path}->{fn_name} [{level:7}] {message}\""
                .to_string()
        );

        log.info("trait methods");
        log.warning("The sky is falling!");

-         let h = log.get_handler(crate::Handler::File).unwrap();
+         let h = log
+             .get_handler(crate::Handler::Custom("ConfileHandler".to_string()))
+             .unwrap();
+
        let buf = h.get_log();

-         assert_eq!(expected, buf);
+         assert!(re.is_match(&buf));

        h.flush();
        h.close();
    }
```

Test passes, but no other output.

---

```rust, no_run, noplayground
    #[test]
    #[should_panic(expected = "'filename' must not be empty")]
    fn filename_empty() {
-         let _ = Logger::file_logger(module_path!(), "");
+         let _ = Logger::builder(module_path!())
+             .set_fn_name("confile_handler_test_mode")
+             .add_custom_handler(
+                 "ConfileHandler",
+                 Box::new(
+                     ConfileHandler::create("").unwrap(),
+                 ),
+             )
+             .build();
    }
}
```

Test passes with output:

- console

    ```text
    thread 'handlers::confile_handler::tests::filename_empty' panicked at src/handlers/confile_handler.rs:323:53:
    called `Result::unwrap()` on an `Err` value: Custom { kind: InvalidInput, error: "'filename' must not be empty" }
    ```

---
