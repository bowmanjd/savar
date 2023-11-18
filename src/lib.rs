// Copyright 2023 Jonathan Bowman
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use anyhow::Result;
use toml::{Table, Value};
use shell_escape::escape as sh_escape;
use std::borrow::Cow;

pub fn read(
    toml: &str,
    key: Option<&str>,
) -> Result<()> {
    let vars =  toml.parse::<Table>()?;
    if let Some(key) = key {
        if let Some(value) = vars[key].as_str() {
            println!("export {}={}", key, sh_escape(Cow::from(value)));
        }
    } else {
        for (key, value) in &vars {
            if let Some(value) = value.as_str() {
                println!("export {}={}", key, sh_escape(Cow::from(value)));
            }
        }
    }
    Ok(())
}

pub fn write(
    toml: &str,
    key: Option<&str>,
) -> Result<()> {
    if let Some(key) = key {
        let mut vars =  toml.parse::<Table>()?;
        let value = rpassword::prompt_password("Enter value: ")?;
        vars.insert(key.to_string(), Value::String(value.to_string()));
        println!("{}", toml::to_string(&vars)?);
    }
    Ok(())
}
