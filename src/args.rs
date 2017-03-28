// Copyright 2017 Justin Charette
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use neovim_lib::{Integer, Value};

pub fn parse_string(value: &Value) -> Result<String, String> {
    if let &Value::String(ref s) = value {
        Ok(s.to_owned())
    } else {
        Err("cannot parse string".to_owned())
    }
}

pub fn parse_usize(value: &Value) -> Result<usize, String> {
    if let &Value::Integer(ref x) = value {
        match x {
            &Integer::U64(x) => Ok(x as usize),
            &Integer::I64(x) => {
                if x >= 0 {
                    Ok(x as usize)
                } else {
                    Err("cannot parse usize".to_owned())
                }
            }
        }
    } else {
        Err("cannot parse usize".to_owned())
    }
}
