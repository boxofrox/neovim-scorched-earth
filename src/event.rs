// Copyright 2017 Justin Charette
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;

pub enum Event {
    CursorMovedI {
        line: usize,
        column: usize,
    },
    InsertEnter {
        mode: String,
        line: usize,
        column: usize,
    },
    InsertLeave,
    Quit,
}

impl fmt::Debug for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Event::*;

        match self {
            &CursorMovedI {
                ref line,
                ref column,
            } => write!(
                f,
                "Event::CursorMovedI{{ line: {}, column: {} }}",
                line, column
            ),
            &InsertEnter {
                ref mode,
                ref line,
                ref column,
            } => write!(
                f,
                "Event::InsertEnter{{ mode: {}, line: {}, column: {}}}",
                mode, line, column
            ),
            &InsertLeave => write!(f, "Event::InsertLeave"),
            &Quit => write!(f, "Event::Quit"),
        }
    }
}
