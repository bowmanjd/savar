// Copyright 2023 Jonathan Bowman
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use anyhow::Result;
use savar::{read, write};
use std::env::args;
use std::io::{self, Read};

pub fn run() -> Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let key = args().nth(1);
    let value = args().nth(2);
    if value.is_some() {
        write(&buffer, key.as_deref())?;
    } else {
        read(&buffer, key.as_deref())?;
    }

    Ok(())
}
