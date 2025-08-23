# Step 2

Now we need to check out the existing handlers and their code, to see which one is the closest to what we are after.

For our example, we could use either the `ConsoleHandler` or the `FileHandler`.
As file handling is the more complex task, we will use the `FileHandler` as our template.

There are two ways to obtain the code. If you are viewing through the on-line [API documentation], then at the top navigation bar, click "flogging-X.X.X", where "X.X.X" is the version number, then under "LINKS" click "Source". This will bring up the "Source" tab. Now we need to navigate to the required file.

---

- src
  - handlers
    - file_handler.rs

---

Now select ALL of the code, from the top down, then 'copy [ctrl/c]'. You need to include the file header (Copyright).

The other way, is to access the github repository: [flogging]. The directory structure is the same as above.

In your project **src** directory somewhere, create your new handler file, and paste this code into it.

For this example, (we'll name it `my_project`) we'll have the following basic layout:

---

- src/\
  lib.rs\
  main.rs
  - handlers/\
    confile_handler.rs\
    mod.rs
    - formatters/\
      csv_formatter.rs
- test_logs/

---

Our file will be called: `confile_handler.rs`, with the module: `ConfileHandler`.

First things first. We now need to do some changes:

- `file_handler.rs` to `confile_handler.rs`
- `FileHandler` to `ConfileHandler`
- `use crate::...;` to `use flogging::*;`

I have used a form of 'diff' to represent the changes:

```text
 unchanged code
-  old line code
+  new line code
 unchanged code
```

---

```rust, no_run

//
- // File Name:    file_handler.rs
+ // File Name:    confile_handler.rs
// Directory:    src/handlers
- // Project Name: flogging
+ // Project Name: my_project
//
// Copyright (C) 2025 Bradley Willcott
//
// SPDX-License-Identifier: GPL-3.0-or-later
//
// This library (crate) is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This library (crate) is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this library (crate).  If not, see <https://www.gnu.org/licenses/>.
//

//!
- //! # FileHandler
+ //! # ConfileHandler
//!

use std::{
    fmt,
    fs::{File, exists},
    io::{Error, ErrorKind::InvalidInput, Write},
};

- use crate::*;
+ use flogging::*;

///
/// Publishes log entries to the file whose name was provided during
/// initialization.
///
#[derive(Debug, Default)]
- pub struct FileHandler {
+ pub struct ConfileHandler {
    filename: String,
    formatter: Formatter,
    file: Option<File>,
    writer: Option<Vec<u8>>,
}

- impl FileHandler {
+ impl ConfileHandler {
    fn create(filename: &str) -> Result<Self, Error> {
        if filename.is_empty() {
            return Err(Error::new(InvalidInput, "'filename' must not be empty"));
        }

-         let fh = FileHandler {
+         let fh = ConfileHandler {
            filename: filename.to_string(),
            formatter: FormatType::Iso8601.create(None),
            file: {
                let f = File::options().append(true).create(true).open(filename)?;
                Some(f)
            },
            writer: None,
        };

        Ok(fh)
    }

    fn log(&self) -> String {
        if let Some(w) = self.writer.to_owned() {
            String::from_utf8(w).unwrap()
        } else {
            String::new()
        }
    }
}

- impl fmt::Display for FileHandler {
+ impl fmt::Display for ConfileHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} : {}", self.filename, self.formatter)
    }
}

- impl HandlerTrait for FileHandler {
+ impl HandlerTrait for ConfileHandler {
    ///
    /// Create a new handler instance.
    ///
    /// ## Parameters
    /// - `name` - This the `filename` of the log file.
    ///
    fn create(name: &str) -> Result<Self, Error> {
-         FileHandler::create(name)
+         ConfileHandler::create(name)
    }

    ///
    /// Flushes and closes the file.\
    /// Also, removes the internal buffer, if in `test_mode`.\
    /// Will therefore, no longer be *in* `test_mode`.
    ///
    fn close(&mut self) {
        self.flush();
        self.file = None;
    }

    fn flush(&mut self) {
        if let Some(f) = &self.file {
            f.sync_all().expect("sync_all failed");
        }
    }

    fn get_formatter(&self) -> Formatter {
        self.formatter.clone()
    }

    fn get_log(&self) -> String {
        self.log()
    }

    fn is_open(&self) -> bool {
        self.file.is_some()
    }

    fn publish(&mut self, log_entry: &LogEntry) {
        if self.is_open() {
            let mut buf = self.formatter.format(log_entry);
            buf.push('\n');

            if let Some(w) = self.writer.as_mut() {
                writeln!(w, "{}", self.formatter.format(log_entry)).expect("writeln!() failed");
            } else {
                self.file
                    .as_mut()
                    .unwrap()
                    .write_all(buf.as_bytes())
                    .expect("write_all() failed");
            }
        }
    }

    fn set_formatter(&mut self, formatter: Formatter) {
        self.formatter = formatter;
    }

    ///
    /// Sets the test mode to `state`.
    ///
    /// If set to `true`, use `get_log()` to obtain the
    /// log.
    ///
    fn set_test_mode(&mut self, state: bool) {
        if state {
            // true
            self.writer = Some(Vec::new());
        } else {
            self.writer = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::{
        fs::File,
        io::{Error, Read, Result},
    };

    #[test]
    fn file_handler() {
        let mut log = Logger::file_logger(module_path!(), "test_logs/file_handler.log");
        log.set_fn_name("file_handler");

        let h = log.get_handler(crate::Handler::File).unwrap();
        h.set_test_mode(false);

        assert!(h.is_open());
        assert_eq!(
            h.get_formatter().to_string(),
            "dt_fmt: \"%+\" - fmt_string: \"{dt:35} {mod_path}->{fn_name} [{level:7}] {message}\""
                .to_string()
        );

        log.info("trait methods");
        log.warning("The sky is falling!");

        let h = log.get_handler(crate::Handler::File).unwrap();

        assert_eq!(h.get_log(), "".to_string());

        h.flush();
        h.close();
        log.exiting_with("This should get thrown away.");
    }

    #[test]
    fn file_handler_file_test() {
        let expected = "flogging::handlers::file_handler::tests->file_handler_file_test [INFO   ] trait methods
flogging::handlers::file_handler::tests->file_handler_file_test [WARNING] The sky is falling!\n"
            .to_string();

        let mut log = Logger::builder(module_path!())
            .set_fn_name("file_handler_file_test")
            .remove_file("test_logs/file_handler_file_test.log")
            .add_file_handler_with(
                "test_logs/file_handler_file_test.log",
                FormatType::Simple,
                None,
            )
            .build();

        let h = log.get_handler(crate::Handler::File).unwrap();
        h.set_test_mode(false);

        assert!(h.is_open());
        assert_eq!(
            h.get_formatter().to_string(),
            "dt_fmt: \"\" - fmt_string: \"{mod_path}->{fn_name} [{level:7}] {message}\""
                .to_string()
        );

        log.info("trait methods");
        log.warning("The sky is falling!");

        let h = log.get_handler(crate::Handler::File).unwrap();

        assert_eq!(h.get_log(), "".to_string());

        h.flush();
        h.close();
        assert!(!h.is_open());

        log.severe("This should get thrown away.");

        if let Ok(mut file) = File::open("test_logs/file_handler_file_test.log") {
            let mut buf = String::new();
            if let Ok(_count) = file.read_to_string(&mut buf) {
                assert_eq!(expected, buf);
            }
        }
    }

    #[test]
    fn file_handler_test_mode() {
        let expected = "flogging::handlers::file_handler::tests->file_handler_test_mode [INFO   ] trait methods
flogging::handlers::file_handler::tests->file_handler_test_mode [WARNING] The sky is falling!\n"
            .to_string();

        let mut log = Logger::builder(module_path!())
            .set_fn_name("file_handler_test_mode")
            .remove_file("test_logs/file_handler_test_mode.log")
            .add_file_handler_with(
                "test_logs/file_handler_test_mode.log",
                FormatType::Simple,
                None,
            )
            .build();

        let h = log.get_handler(crate::Handler::File).unwrap();
        h.set_test_mode(true);

        assert!(h.is_open());
        assert_eq!(
            h.get_formatter().to_string(),
            "dt_fmt: \"\" - fmt_string: \"{mod_path}->{fn_name} [{level:7}] {message}\""
                .to_string()
        );

        log.info("trait methods");
        log.warning("The sky is falling!");

        let h = log.get_handler(crate::Handler::File).unwrap();
        let buf = h.get_log();

        assert_eq!(expected, buf);

        h.flush();
        h.close();
    }

    #[test]
    #[should_panic(expected = "'filename' must not be empty")]
    fn filename_empty() {
        let _ = Logger::file_logger(module_path!(), "");
    }
}
```

[API documentation]: https://docs.rs/flogging/latest/flogging/index.html
[flogging]: https://github.com/bewillcott/flogging
