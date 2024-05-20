use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use toml::Value;
use std::{
    fs::File, io, time::{Duration, Instant}
};
use std::io::prelude::*;
use tui::{
    backend::Backend,
    Terminal,
};
use anyhow::Result;

use crate::arsenal_objects::command::{load_values_into_commands, CommandArg, Command};
use crate::misc::inputs::write_co_clipboard;
use super::{event::AppEvent, renderer, stateful_list::StatefulList};
use super::event::LevelCode;


pub struct ArsenalApp {
    pub max_events: usize,
    pub items: StatefulList<Command>,
    pub events: Vec<AppEvent>,
    pub search: String,
    pub show_command: bool,
    pub chosen_command: Option<Command>,
    pub list_cmd_args: StatefulList<CommandArg>,
    pub quit_app: bool
}

impl ArsenalApp {
    pub fn new(max_events: usize) -> ArsenalApp {
        ArsenalApp {
            max_events,
            items: StatefulList::with_items(vec![]),
            events: vec![],
            search: "".to_string(),
            show_command: false,
            chosen_command: None,
            list_cmd_args: StatefulList::with_items(vec![]),
            quit_app: false
        }
    }

    pub fn load_settings(&mut self, settings: String) -> Result<()> {
        let mut file = File::open(&settings).map_err(|e| anyhow::format_err!("File::open() err={}", e))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let value = contents.parse::<Value>()?;
        self.push_event(AppEvent::new(&format!("Settings: {}", &value), LevelCode::INFO));

        // Check and Load values
        let mut commands = load_values_into_commands(value)?;
        self.items.items.append(&mut commands);

        self.push_event(AppEvent::new(&format!("Settings loaded from: {}", &settings), LevelCode::INFO));
        Ok(())
    }

    pub fn load_example_commands(&mut self) {
        self.items.items.push(Command::new(
            "ping".to_string(),
            "network".to_string(),
            "Simple ping with verbose on".to_string(),
            "-v <destination>".to_string(),
            vec!["ping 127.0.0.1".to_string(), "ping -v 127.0.0.1".to_string()]));

        self.push_event(AppEvent::new(&format!("Number of commands loaded: {}", self.items.items.len()), LevelCode::INFO));
    }

    pub fn push_event(&mut self, event: AppEvent) {
        if self.events.len() > self.max_events {
            _ = self.events.remove(0);
        }
        self.events.push(event);
    }

    pub fn get_selected_command(&self) -> Result<Command> {
        let Some(selected) = self.items.state.selected() else {
            anyhow::bail!("Cannot get selected value from list!")
        };
        // Get item from list
        let Some(command) = self.items.items.get(selected).clone() else {
            anyhow::bail!("Cannot retrieve value from item list!")
        };

        Ok(command.clone())
    }

    pub fn get_mut_selected_arg(&mut self) -> Option<&mut CommandArg> {
        let idx = match self.list_cmd_args.state.selected() {
            Some(i) => i,
            None => return None
        };
        let cmd_arg = match self.list_cmd_args.items.get_mut(idx) {
            Some(c) => c,
            None => return None
        };
        Some(cmd_arg)
    }

    fn handle_event_key(&mut self, key: KeyEvent) {
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            // ^C => Should quit app
            self.push_event(AppEvent::new(&format!("Quitting program!"), LevelCode::INFO));
            self.quit_app = true;
            return
        }

        match key.code {
            KeyCode::Char(x) => {
                // There is 2 possibility when `down` is triggered:
                // - 1. Popup is opened and so it's writing to command values to fill
                // - 2. Popup is not opened and it's writing to search bar
                if self.show_command {
                    // CommandArg modifier
                    let cmd_arg = match self.get_mut_selected_arg() {
                        Some(c) => c,
                        None => return
                    };
                    match &mut cmd_arg.modified {
                        Some(s) => s.push(x),
                        None => cmd_arg.modified = Some(x.to_string())
                    };
                    let id = cmd_arg.id;  // Avoid the 2nd mut borrow

                    // Command modifier
                    match &mut self.chosen_command {
                        Some(c) => {
                            match c.cmd_args.get_mut(id) {
                                Some(c) => {
                                    match &mut c.modified {
                                        Some(s) => s.push(x),
                                        None => c.modified = Some(x.to_string())
                                    }
                                },
                                None => {}
                            }
                        },
                        None => {}
                    }
                } else {
                    self.push_event(AppEvent::new(&format!("KeyCode: {}", x), LevelCode::TRACE));
                    self.search.push(x);
                }
            }
            // KeyCode::Left => app.items.unselect(),
            KeyCode::Backspace => _ = {
                if self.show_command {
                    let cmd_arg = match self.get_mut_selected_arg() {
                        Some(c) => c,
                        None => return
                    };
                    match &mut cmd_arg.modified {
                        Some(s) => {
                            _ = s.pop();
                            if s.is_empty() { cmd_arg.modified = None }
                        },
                        None => cmd_arg.modified = None
                    };
                    let id = cmd_arg.id;  // Avoid the 2nd mut borrow

                    // Command modifier
                    match &mut self.chosen_command {
                        Some(c) => {
                            match c.cmd_args.get_mut(id) {
                                Some(c) => {
                                    match &mut c.modified {
                                        Some(s) => {
                                            _ = s.pop();
                                            if s.is_empty() { c.modified = None }
                                        },
                                        None => c.modified = None
                                    }
                                },
                                None => {}
                            }
                        },
                        None => {}
                    }
                } else {
                    self.search.pop();
                }
            },
            KeyCode::Down => {
                // There is 2 possibility when `down` is triggered:
                // - 1. Popup is opened and so it's switching between command values to fill
                // - 2. Popup is not opened and it's switching between commands
                match self.show_command {
                    true => self.list_cmd_args.next(),
                    false => self.items.next()
                }
            },
            KeyCode::Up => {
                // There is 2 possibility when `up` is triggered:
                // - 1. Popup is opened and so it's switching between command values to fill
                // - 2. Popup is not opened and it's switching between commands
                match self.show_command {
                    true => self.list_cmd_args.previous(),
                    false => self.items.previous()
                }
            },
            KeyCode::Esc => {
                // There is 2 possibility when `escape` is triggered:
                // - 1. Popup is opened and so it's closing the popup
                // - 2. Popup is not opened and it's quitting the program
                match self.show_command {
                    true => {
                        self.show_command = false;
                        self.chosen_command = None;
                        self.list_cmd_args = StatefulList::with_items(vec![]);
                    },
                    false => {
                        self.push_event(AppEvent::new(&format!("Quitting program!"), LevelCode::INFO));
                        self.quit_app = true;
                    }
                }
            }
            KeyCode::Enter => {
                // There is 2 possibility when `enter` is triggered:
                // - 1. Popup is opened and so it's going to copy command to clipboard
                // - 2. Popup is not opened and it is opening the command popup
                if self.show_command {
                    // Get chosen command
                    let command = match &self.chosen_command {
                        Some(c) => c,
                        None => {
                            self.push_event(AppEvent::new(&format!("Cannot retrieve chosen command!"), LevelCode::ERROR));
                            return
                        }
                    };
                    let final_cmd = command.copy();
                    // let command_str = format!("{command}");
                    if let Err(e) = write_co_clipboard(&final_cmd) {
                        self.push_event(AppEvent::new(&format!("Error when writing to clipboard, error={}", e), LevelCode::ERROR));
                        return
                    };
                    self.push_event(AppEvent::new(&format!("Value copied to clipboard: \"{}\"", final_cmd), LevelCode::DEBUG));
                } else {
                    // Get command
                    self.show_command = match self.get_selected_command() {
                        Ok(c) => {
                            let mut args_filled_list = StatefulList::with_items(c.get_input_args());
                            args_filled_list.state.select(Some(0));
                            self.list_cmd_args = args_filled_list;
                            self.chosen_command = Some(c.clone());
                            true
                        },
                        Err(e) => {
                            self.push_event(AppEvent::new(&format!("Cannot get selected command: {}", e), LevelCode::ERROR));
                            false
                        }
                    };
                }
            }
            _ => {}
        }
    }

    fn on_tick(&mut self) {
        // Do something on tick
    }

    // Before quitting, clean some things
    fn quit(&mut self) {
        // Do some cleaning if any
        // sleep(Duration::from_millis(300));
    }
}

pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: ArsenalApp,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| renderer::render(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                app.handle_event_key(key);
                if app.quit_app {
                    terminal.draw(|f| renderer::render(f, &mut app))?;  // Render quitting program event
                    app.quit();
                    return Ok(())
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}