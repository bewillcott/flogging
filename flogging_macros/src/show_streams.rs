//
// File Name:    show_streams.rs
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
//! # Show Streams Macro
//!

use proc_macro::TokenStream;
use regex::RegexBuilder;

pub(crate) fn show_streams_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{attr}\"");
    println!("item: \"{item}\"");
    println!("fn_name: \"{}\"", get_fn_name(&item).unwrap_or_default());
    item
}

pub(crate) fn get_fn_name(item: &TokenStream) -> Option<String> {
    let re = RegexBuilder::new(r"^.*fn\s+(?<fn_name>[_]*[a-z][_\w]*)(.*)$")
        .dot_matches_new_line(true)
        .build()
        .unwrap();

    let binding = item.clone().to_string();
    let caps = match re.captures(&binding) {
        Some(caps) => caps,
        None => {
            eprintln!("Nothing captured!");
            return None;
        }
    };

    Some(caps["fn_name"].to_string().clone())
}
