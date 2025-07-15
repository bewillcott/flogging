//
// File Name:    logger.rs
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
//! # Logger Macro
//!

use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

pub(crate) fn logger_impl(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input as `ItemFn` which is a type provided
    // by `syn` to represent a function.
    let input = parse_macro_input!(item as ItemFn);

    let ItemFn {
        // The function signature
        sig,
        // The visibility specifier of this function
        vis,
        // The function block or body
        block,
        // Other attributes applied to this function
        attrs,
    } = input;

    // Extract statements in the body of the functions
    let statements = block.stmts;

    // Store the function identifier for logging
    let function_identifier = sig.ident.clone();

    // Reconstruct the function as output using parsed input
    quote!(
        // Reapply all the other attributes on this function.
        // The compiler doesn't include the macro we are
        // currently working in this list.
        #(#attrs)*
        // Reconstruct the function declaration
        #vis #sig {
            // At the beginning of the function, create an instance of `Instant`
            let mut __binding = LOGGER;
            let mut __log = __binding.borrow_mut();
            __log.set_fn_name(stringify!(#function_identifier));

            #(#statements)*
        }
    )
    .into()
}
