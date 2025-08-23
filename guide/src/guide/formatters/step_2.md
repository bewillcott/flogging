# Step 2

Creating a custom formatter will require some investigation of other crates. Crates that are used by `FLogging`:

- [crono] - Used for the date/time functionality, with its [specifiers], in the `dt_fmt` string of each formatter.
- [std::fmt] - The format options used in the `fmt_string` string of each formatter.
- [strfmt] - Used to do the actual formatting.

Now you should check out the built-in formatters to find one that is the closest to what you are after.

For our example, none of the built-in formatters are really closer than the other, so we will choose the `UnixTimestampFormatter`, simply because I am running Linux as my O/S.

To find them, use either the [API documentation], or the Github repository: [flogging].

As with the [Custom Handlers][ch], to obtain the code from the API documentation, at the top navigation bar, click “flogging-0.6.0”, then under “LINKS” click “Source”. This will bring up the “Source” tab. Now we need to navigate to the required file.

---

- src
  - handlers
    - formatter
      - unixtimestamp_formatter.rs

---

Now select ALL of the code, from the top down, then ‘copy [ctrl/c]’. You need to include the file header (Copyright).

In your project **src** directory somewhere, create your new formatter file, and paste this code into it.

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

Our file will be called: `csv_formatter.rs`, with the module: `CsvFormatter`.

First things first. We now need to do some changes:

- `unixtimestamp_formatter.rs` to `csv_formatter.rs`
- `UnixTimestampFormatter` to `CsvFormatter`
- `use crate::FormatTrait;` to `use flogging::*;`
- `&crate::LogEntry` to `&LogEntry`

I have used a form of ‘diff’ to represent the changes:

- ’- ’ old line code
- ’+ ’ new line code

```rust, no_run
//
- // File Name:    unixtimestamp_formatter.rs
+ // File Name:    csv_formatter.rs
// Directory:    src/handlers/formatters
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
- //! # UnixTimeStamp Formatter
+ //! # CSV Formatter
//!

use std::fmt;
- use crate::FormatTrait;
+ use flogging::*;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]

///
- /// Unix Timestamp format.
+ /// CSV format.
///
/// The first part (before the decimal point) is
/// the number of seconds since 1970-01-01 00:00 UTC.
///
/// The second part is the number of nanoseconds since
/// the last whole second.
///
/// Example:
/// ```text
/// 1752817859.157970496
/// ```
/// Template:
/// - `dt` in the template would be the datetime string, similar to the above.
/// - `mod_path`, `fn_name`, `level`, and `message` all come out of the `LogEntry`
- ///   provided to the [`format()`][UnixTimestampFormatter::format] method.
+ ///   provided to the [`format()`][CsvFormatter::format] method.
///
/// ```ignore
/// format!("{dt} {mod_path}->{fn_name} [{level:7}] {message}");
/// ```
/// Sample output:
/// ```text
/// 1752818461.051538870 flogging->main [SEVERE ] Hurricanes are windy!
/// ```
///
- pub struct UnixTimestampFormatter {
+ pub struct CsvFormatter {
    dt_fmt: String,
    fmt_string: String,
}

- impl UnixTimestampFormatter {
+ impl CsvFormatter {
    ///
-    /// Creates a new instance of `UnixTimestampFormatter`.
+    /// Creates a new instance of `CsvFormatter`.
    ///
    pub fn new() -> Self {
        Self {
            dt_fmt: "%s.%f".to_string(),
            fmt_string: "{dt} {mod_path}->{fn_name} [{level:7}] {message}".to_string(),
        }
    }

    ///
    /// Returns the date/time format string.
    ///
    pub fn dt_fmt(&self) -> String {
        self.dt_fmt.clone()
    }

    ///
    /// Returns the primary format string.
    ///
    pub fn fmt_string(&self) -> String {
        self.fmt_string.clone()
    }
}

- impl Default for UnixTimestampFormatter {
+ impl Default for CsvFormatter {
    fn default() -> Self {
        Self::new()
    }
}

- impl fmt::Display for UnixTimestampFormatter {
+ impl fmt::Display for CsvFormatter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "dt_fmt: \"{}\" - fmt_string: \"{}\"",
            self.dt_fmt, self.fmt_string
        )
    }
}

- impl FormatTrait for UnixTimestampFormatter {
+ impl FormatTrait for CsvFormatter {
-     fn format(&self, log_entry: &crate::LogEntry) -> String {
+     fn format(&self, log_entry: &LogEntry) -> String {
        self.ft_fmt(self.dt_fmt(), self.fmt_string(), log_entry)
    }
}
```

[API documentation]: https://docs.rs/flogging/latest/flogging/index.html
[ch]: ../handlers/step_2.md
[crono]: https://crates.io/crates/chrono
[flogging]: https://github.com/bewillcott/flogging
[specifiers]: https://docs.rs/chrono/latest/chrono/format/strftime
[std::fmt]: https://doc.rust-lang.org/stable/std/fmt/index.html
[strfmt]: https://crates.io/crates/strfmt
