// Copyright 2023 Jonathan Bowman
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::process;

mod cmd;

fn main() {
    if let Err(err) = cmd::run() {
        eprintln!("{}", err);
        process::exit(1);
    }
}
