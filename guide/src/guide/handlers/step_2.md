# Step 2

Now we need to check out the existing handlers and their code, to see which one is the closest to what we are after.

For our example, we could use either the `ConsoleHandler` or the `FileHandler`.
As file handling is the more complex task, we will use the `FileHandler` as our template.

There are two ways to obtain the code. If you are viewing through the on-line [API documentation], then at the top navigation bar, click "flogging-0.6.0", then under "LINKS" click "Source". This will bring up the "Source" tab. Now we need to navigate to the required file.

- src
  - handlers
    - file_handler.rs

Now select ALL of the code, from the top down, then 'copy [ctrl/c]'. You need to include the file header (Copyright).

The other way, is to access the github repository: [flogging]. The directory structure is the same as above.

In your project **src** directory somewhere, create your new handler file, and paste this code into it.

Our file will be called: `confile_handler.rs`, with the module: `ConfileHandler`.

First things first. We now need to do some changes:

- `file_handler.rs` to `confile_handler.rs`
- `FileHandler` to `ConfileHandler`
- `use crate::...;` to `use flogging::*;`

I have used a form of 'diff' to represent the changes:

- '- ' old line code
- '+ ' new line code

```rust, no_run

//
- // File Name:    file_handler.rs
+ // File Name:    confile_handler.rs
// Project Name: flogging
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

#![allow(unused)]

use std::{
    fmt,
    fs::{File, exists},
    io::{Error, ErrorKind::InvalidInput, Write},
};

- use crate::{
-     handlers::{
-         formatter::{FormatType, Formatter},
-         handler::HandlerTrait,
-     },
-     logger::{Level, LogEntry},
- };
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
}

- impl FileHandler {
+ impl ConfileHandler {
    fn create(filename: &str) -> Result<Self, Error> {
        if filename.is_empty() {
            return Err(Error::new(InvalidInput, "'filename' must not be empty"));
        }

-        let fh = FileHandler {
+        let fh = ConfileHandler {
            filename: filename.to_string(),
            formatter: FormatType::Iso8601.create(None),
            file: {
                let f = File::options().append(true).create(true).open(filename)?;

                Some(f)
            },
        };

        Ok(fh)
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
-        FileHandler::create(name)
+        ConfileHandler::create(name)
    }

    fn close(&mut self) {
        self.flush();
        self.file = None;
    }

    fn flush(&mut self) {
        if let Some(f) = &self.file {
            f.sync_all();
        }
    }

    fn get_formatter(&self) -> Formatter {
        self.formatter.clone()
    }

    fn get_log(&self) -> String {
        String::new()
    }

    fn is_open(&self) -> bool {
        self.file.is_some()
    }

    #[allow(private_interfaces)]
    fn publish(&mut self, log_entry: &LogEntry) {
        if self.is_open() {
            let mut buf = self.formatter.format(log_entry);
            buf.push('\n');

            self.file.as_mut().unwrap().write_all(buf.as_bytes());
        }
    }

    fn set_formatter(&mut self, formatter: Formatter) {
        self.formatter = formatter;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{Logger, logger};

    #[test]
    fn handler_trait() {
        let mut log = Logger::file_logger(module_path!(), "test.log");

        log.info("trait methods");

        let handler = log.get_handler(crate::Handler::File).unwrap();
        assert!(handler.is_open());
        assert_eq!(handler.get_formatter().to_string(), "dt_fmt: \"%+\" - fmt_string: \"{dt:35} |{mod_path}->{fn_name}| [{level:7}] {message}\"".to_string());
        assert_eq!(handler.get_log(), "".to_string());
        handler.flush();
        handler.close();
    }

    #[test]
    #[should_panic(expected = "'filename' must not be empty")]
    fn filename_empty() {
        let mut log = Logger::file_logger(module_path!(), "");
    }
}
```

[API documentation]: https://docs.rs/flogging/latest/flogging/index.html
[flogging]: https://github.com/bewillcott/flogging
