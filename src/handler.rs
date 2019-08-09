// Copyright 2017 Justin Charette
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::args;
use crate::event::Event;

use log::*;

use neovim_lib::{Handler, RequestHandler, Value};

use std::sync::mpsc;

pub struct NeovimHandler(pub mpsc::Sender<Event>);

impl NeovimHandler {
    pub fn parse_cursor_moved_i(&mut self, args: &Vec<Value>) -> Result<Event, String> {
        if 2 != args.len() {
            return Err(format!(
                "Wrong number of arguments for 'CursorMoveI'.  Expected 2, found \
                 {}",
                args.len()
            ));
        }

        let line = args::parse_usize(&args[0])?;
        let column = args::parse_usize(&args[1])?;

        Ok(Event::CursorMovedI {
            line: line,
            column: column,
        })
    }

    pub fn parse_insert_enter(&mut self, args: &Vec<Value>) -> Result<Event, String> {
        if 3 != args.len() {
            return Err(format!(
                "Wrong number of arguments for 'InsertEnter'.  Expected 3, found \
                 {}",
                args.len()
            ));
        }

        let mode = args::parse_string(&args[0])?;
        let line = args::parse_usize(&args[1])?;
        let column = args::parse_usize(&args[2])?;

        Ok(Event::InsertEnter {
            mode: mode,
            line: line,
            column: column,
        })
    }
}

impl Handler for NeovimHandler {
    fn handle_notify(&mut self, name: &str, args: Vec<Value>) {
        info!("event: {}", name);
        //print_args(&args);
        match name {
            "cursor-moved-i" => {
                if let Ok(event) = self.parse_cursor_moved_i(&args) {
                    info!("cursor moved i: {:?}", event);
                    if let Err(reason) = self.0.send(event) {
                        error!("{}", reason);
                    }
                }
            }
            "insert-enter" => {
                if let Ok(event) = self.parse_insert_enter(&args) {
                    info!("insert enter: {:?}", event);
                    if let Err(reason) = self.0.send(event) {
                        error!("{}", reason);
                    }
                }
            }
            "insert-leave" => {
                if let Err(reason) = self.0.send(Event::InsertLeave) {
                    error!("{}", reason);
                }
            }
            "quit" => {
                if let Err(reason) = self.0.send(Event::Quit) {
                    error!("{}", reason);
                }
            }
            _ => {}
        }
    }
}

impl RequestHandler for NeovimHandler {
    fn handle_request(&mut self, _name: &str, _args: Vec<Value>) -> Result<Value, Value> {
        Err(Value::from("not implemented"))
    }
}
