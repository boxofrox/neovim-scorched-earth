// Copyright 2017 Justin Charette
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod args;
mod event;
mod handler;
mod position;

use crate::event::Event;
use crate::handler::NeovimHandler;
use crate::position::Position;

use log::{error, info, log};

use neovim_lib::neovim::Neovim;
use neovim_lib::neovim_api::NeovimApi;
use neovim_lib::session::Session;

use simplelog::{Config, LogLevel, LogLevelFilter, WriteLogger};

use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::sync::mpsc;

fn main() {
    use std::process;

    init_logging().expect("scorched earth: unable to initialize logger.");

    match start_program() {
        Ok(_) => process::exit(0),

        Err(msg) => {
            error!("{}", msg);
            process::exit(1);
        }
    };
}

fn init_logging() -> Result<(), Box<Error>> {
    use std::env;
    use std::env::VarError;
    use std::fs::File;

    let log_level_filter = match env::var("LOG_LEVEL")
        .unwrap_or(String::from("trace"))
        .to_lowercase()
        .as_ref()
    {
        "debug" => LogLevelFilter::Debug,
        "error" => LogLevelFilter::Error,
        "info" => LogLevelFilter::Info,
        "off" => LogLevelFilter::Off,
        "trace" => LogLevelFilter::Trace,
        "warn" => LogLevelFilter::Warn,
        _ => LogLevelFilter::Off,
    };

    let config = Config {
        time: Some(LogLevel::Error),
        level: Some(LogLevel::Error),
        target: Some(LogLevel::Error),
        location: Some(LogLevel::Error),
    };

    let filepath = match env::var("LOG_FILE") {
        Err(err) => match err {
            VarError::NotPresent => return Ok(()),
            e @ VarError::NotUnicode(_) => {
                return Err(Box::new(e));
            }
        },
        Ok(path) => path.to_owned(),
    };

    let log_file = File::create(filepath)?;

    WriteLogger::init(log_level_filter, config, log_file).unwrap();

    Ok(())
}

fn start_program() -> Result<(), Box<Error>> {
    info!("connecting to neovim via stdin/stdout");

    let (sender, receiver) = mpsc::channel();
    let mut session = Session::new_parent()?;
    session.start_event_loop_handler(NeovimHandler(sender));

    let mut nvim = Neovim::new(session);

    info!("let's notify neovim the plugin is connected!");
    nvim.command("echom \"rust client connected to neovim\"")
        .unwrap();
    info!("notification complete!");

    nvim.subscribe("cursor-moved-i")
        .expect("error: cannot subscribe to event: change-cursor-i");
    nvim.subscribe("insert-enter")
        .expect("error: cannot subscribe to event: insert-enter");
    nvim.subscribe("insert-leave")
        .expect("error: cannot subscribe to event: insert-leave");
    nvim.subscribe("quit")
        .expect("error: cannot subscribe to event: quit");

    start_event_loop(receiver, nvim);

    Ok(())
}

enum Mode {
    Insert,
    Replace,
    Other,
}

/*
fn print_args(args: &Vec<Value>) {
    for (i, val) in args.iter().enumerate() {
        info!("arg {}: {:?}", i, val);
    }
}
*/

fn start_event_loop(receiver: mpsc::Receiver<Event>, mut nvim: Neovim) {
    let mut cursor_start: Option<Position> = None;
    let mut cursor_end: Option<Position> = None;
    let mut mode = Mode::Other;

    let highlight_groups = HighlightGroup::load(&mut nvim);

    loop {
        match receiver.recv() {
            Ok(Event::CursorMovedI { line, column }) => {
                if let Mode::Other = mode {
                    continue;
                }

                let pos = Position::new(line, column);

                cursor_start = keep_min_position(&cursor_start, &pos);
                cursor_end = keep_max_position(&cursor_end, &pos);

                info!("start: sending echo message to neovim");
                define_syntax_region(
                    &mut nvim,
                    cursor_start.as_ref().unwrap(),
                    cursor_end.as_ref().unwrap(),
                );
                info!("finish: sending echo message to neovim");
            }
            Ok(Event::InsertEnter {
                mode: neovim_mode,
                line,
                column,
            }) => {
                info!("insert enter: mode is {}", neovim_mode);

                match neovim_mode.as_ref() {
                    "r" => mode = Mode::Replace,
                    "i" => mode = Mode::Insert,
                    _ => continue,
                }

                cursor_start = Some(Position::new(line, column));
                cursor_end = Some(Position::new(line, column));

                if let Some(parent_group) =
                    get_valid_parent_highlight_group(&mut nvim, &highlight_groups)
                {
                    link_highlight_group(&mut nvim, &parent_group);
                }
            }
            Ok(Event::InsertLeave) => {
                mode = Mode::Other;
                cursor_start = None;
                cursor_end = None;
                remove_syntax_group(&mut nvim);
            }
            Ok(Event::Quit) => break,
            _ => {}
        }
    }
    info!("quitting");
    nvim.command("echom \"rust client disconnected from neovim\"")
        .unwrap();
}

fn keep_min_position(target: &Option<Position>, pos: &Position) -> Option<Position> {
    match target {
        &None => Some(pos.clone()),
        &Some(ref start) => {
            if pos < start {
                Some(pos.clone())
            } else {
                Some(start.clone())
            }
        }
    }
}

fn keep_max_position(target: &Option<Position>, pos: &Position) -> Option<Position> {
    match target {
        &None => Some(pos.clone()),
        &Some(ref end) => {
            if pos > end {
                Some(pos.clone())
            } else {
                Some(end.clone())
            }
        }
    }
}

fn link_highlight_group(nvim: &mut Neovim, parent_highlight_group: &HighlightGroup) {
    nvim.command(&format!(
        "highlight link ScorchedEarth {}",
        parent_highlight_group
    ))
    .unwrap()
}

fn define_syntax_region(nvim: &mut Neovim, cursor_start: &Position, cursor_end: &Position) {
    nvim.command(&format!(
        "syntax region ScorchedEarth start=/\\%{}l\\%{}c/ end=/\\%{}l\\%{}c/",
        cursor_start.line, cursor_start.column, cursor_end.line, cursor_end.column
    ))
    .unwrap();
}

fn remove_syntax_group(nvim: &mut Neovim) {
    nvim.command("syntax clear ScorchedEarth").unwrap();
}

fn get_valid_parent_highlight_group(
    nvim: &mut Neovim,
    group_set: &HashSet<HighlightGroup>,
) -> Option<HighlightGroup> {
    match get_parent_highlight_group(nvim) {
        None => None,
        Some(group) => {
            if group_set.contains(&group) {
                Some(group)
            } else {
                None
            }
        }
    }
}

fn get_parent_highlight_group(nvim: &mut Neovim) -> Option<HighlightGroup> {
    match nvim.get_var("scorched_earth_parent_highlight_group") {
        Err(_) => None,
        Ok(ref v) => v.as_str().map(HighlightGroup::from),
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct HighlightGroup(String);

impl HighlightGroup {
    pub fn new(name: String) -> HighlightGroup {
        HighlightGroup(name)
    }

    pub fn load(nvim: &mut Neovim) -> HashSet<HighlightGroup> {
        let highlight = nvim
            .command_output("silent highlight")
            .expect("unable to list highlights");

        let highlight = highlight
            .lines()
            .filter(|line| line.starts_with(char::is_alphabetic))
            .flat_map(|line| line.split_whitespace().take(1))
            .map(str::to_owned)
            .map(HighlightGroup::new)
            .collect::<HashSet<HighlightGroup>>();

        info!("highlight groups:\n{:?}", highlight);

        highlight
    }
}

impl fmt::Display for HighlightGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.0)
    }
}

impl From<String> for HighlightGroup {
    fn from(name: String) -> HighlightGroup {
        HighlightGroup(name)
    }
}

impl<'a> From<&'a str> for HighlightGroup {
    fn from(name: &'a str) -> HighlightGroup {
        HighlightGroup(name.to_owned())
    }
}
