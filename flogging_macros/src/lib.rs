//
// File Name:    lib.rs
// Project Name: flogger_macros
//
// Copyright (C) 2025 Bradley Willcott
//
// SPDX-License-Identifier: GPL-2.0-or-later
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
//! # Flogging Macros
//!

extern crate proc_macro;
use proc_macro::TokenStream;
use regex::Regex;

///
/// Shows the stringified `TokenStreams` that the attribute macros see.
///
#[proc_macro_attribute]
pub fn show_streams(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{attr}\"");
    println!("item: \"{item}\"");
    // println!("fn_name: \"{}\"", process_item(&item).unwrap_or_default());
    item
}

fn _process_item<'a>(item: &TokenStream) -> Option<String> {
    let re = Regex::new(r"^.*fn\s+(?<fn_name>[_]*[a-z][_\w]*)(.*)$").unwrap();
    let binding = item.to_string();
    let caps = re.captures(&binding).unwrap();
    Some(caps["fn_name"].to_string().clone())
}
