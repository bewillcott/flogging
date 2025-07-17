//
// File Name:    it_conversion.rs
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
//! # Integration Tests - Conversion
//!

#[cfg(test)]
mod tests {
    use chrono::Local;
    use flogging::*;

    static_logger!({
        Logger::builder("flogging")
            .add_console_handler()
            .add_file_handler("test.log")
            .set_level(Level::FINEST)
            .build()
    });

    #[logger]
    #[test]
    fn my_func() {
        config!("Some text to store.");

        let time = Local::now();
        config!("The configuration as at: {}", time);
        config!("The configuration as at: {time}");
        config!("The configuration as at: {time:?}");
    }

    #[logger]
    #[test]
    fn format() {
        let arg = "Serious stuff".to_string();
        let arg1 = "NOW!!".to_string();
        let arg2 = "Signed: Brad".to_string();

        config!("Testing `config` macro: {}, {} ({})", arg, arg1, arg2);
        fine!("Testing `fine` macro: {}, {} ({})", arg, arg1, arg2);
        finer!("Testing `finer` macro: {}, {} ({})", arg, arg1, arg2);
        finest!("Testing `finest` macro: {}, {} ({})", arg, arg1, arg2);
        info!("Testing `info` macro: {}, {} ({})", arg, arg1, arg2);
        warning!("Testing `warning` macro: {}, {} ({})", arg, arg1, arg2);
        severe!("Testing `severe` macro: {}, {} ({})", arg, arg1, arg2);
    }

    #[logger]
    #[test]
    fn format2() {
        let arg = "Serious stuff".to_string();
        let arg1 = "NOW!!".to_string();
        let arg2 = "Signed: Brad".to_string();

        config!("Testing `config` macro: {arg:?}, {} ({arg2})", arg1);
    }

    // #[logger]
    // #[test]
    // fn tester() {
    //     let arg = "Some text".to_string();
    //     let arg1 = "Some more text".to_string();

    //     tester!("This is some text: {} - {}", arg, arg1);
    // }
}
