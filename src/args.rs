// Copyright 2017 Justin Charette
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use neovim_lib::Value;

pub fn parse_string(value: &Value) -> Result<String, String> {
    value
        .as_str()
        .ok_or("cannot parse error".to_owned())
        .map(|s| String::from(s))
}

pub fn parse_usize(value: &Value) -> Result<usize, String> {
    value
        .as_u64()
        .ok_or("cannot parse usize".to_owned())
        .map(|n| n as usize)
}
