//
// File Name:    fmt_log.rs
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
//! # format Macro Impl
//!
//! Format the Log function call (Rust code)
//!

use dyn_fmt::AsStrFormatExt;
use proc_macro::TokenStream;
use regex::RegexBuilder;

pub(crate) fn format_impl(fmt_str: &str, msg: TokenStream) -> TokenStream {
    // println!("msg: {}", &msg);

    let fmt = "let __fmt = format!({});\n".format(&[&process_msg(msg).unwrap()]);

    let mut buf = String::new();
    buf.push_str(&fmt);
    buf.push_str(fmt_str);

    // let rtn = buf.parse().unwrap_or_default();
    // println!("{rtn}");
    // rtn
    buf.parse().unwrap_or_default()
}

fn process_msg(msg: TokenStream) -> Option<String> {
    let text = msg.to_string();

    if text.is_empty() {
        return None;
    }

    if text.starts_with('\"') && text.ends_with('\"') {
        return Some(text);
    }

    let regex_str = "(?<fmt>\".*\\{.*\\}.*\")(,[\\s]*(?<attrs>.*)*)";

    let re = RegexBuilder::new(regex_str)
        .dot_matches_new_line(true)
        .build()
        .unwrap();

    if re.is_match(&text) {
        Some(text)
    } else {
        let count = text.split(',').count();

        let mut buf = String::new();

        if count == 1 {
            buf.push_str("\"{}\", ");
        } else {
            buf.push_str("\"{}");

            for _i in 1..count {
                buf.push_str(", {}");
            }

            buf.push_str("\", ");
        }

        buf.push_str(&text);

        Some(buf)
    }
}
