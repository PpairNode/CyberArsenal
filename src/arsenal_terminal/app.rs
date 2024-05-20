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
    pub events: Vec<AppEvent>,
    pub search_commands: SearchCommands,
    pub chosen_command: Option<ChosenCommand>,
    pub quit_app: bool
}

pub struct ChosenCommand {
    pub command: Command,
    pub listful_args: StatefulList<CommandArg>
}

impl ChosenCommand {
    fn refresh_list(&mut self) {
        // Refresh listful from modified command args
        let id = self.listful_args.state.selected();
        self.listful_args = StatefulList::with_items(self.command.get_input_args());
        _ = self.listful_args.state.select(id);
    }
}

impl From<&Command> for ChosenCommand {
    fn from(command: &Command) -> Self {
        ChosenCommand {
            command: command.clone(),
            listful_args: StatefulList::with_items(command.get_input_args()),
        }
    }
}

pub struct SearchCommands {
    pub search: String,
    pub commands: Vec<Command>,
    pub listful_cmds: StatefulList<Command>
}

impl SearchCommands {
    fn new() -> Self {
        SearchCommands {
            search: "".to_string(),
            commands: vec![],
            listful_cmds: StatefulList::with_items(vec![])
        }
    }

    fn refresh_list(&mut self) {
        let mut commands = vec![];
        if self.search.is_empty() {  // No filter
            commands = self.commands.clone();
        } else {  // Filter
            commands.extend(self.commands.iter()
                .filter_map(|c| {
                    if c.name.contains(&self.search) {  // Filter commands: NAME
                        Some(c.clone())
                    } else if c.args.contains(&self.search) {  // Filter commands: ARGS
                        Some(c.clone())
                    } else if format!("{:?}", c.cmd_type).contains(&self.search) {  // Filter commands: TYPE
                        Some(c.clone())
                    } else {
                        None
                    }
                })
                .collect::<Vec<Command>>()
            );
        }

        // Refresh List
        self.listful_cmds = StatefulList::with_items(commands);
        _ = self.listful_cmds.state.select(Some(0)); // When search is modified, id become 0 as list is refreshed
    }
}

impl ArsenalApp {
    pub fn new(max_events: usize) -> ArsenalApp {
        ArsenalApp {
            max_events,
            events: vec![],
            search_commands: SearchCommands::new(),
            chosen_command: None,
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
        // let mut commands = load_values_into_commands(value)?;
        self.search_commands.commands = load_values_into_commands(value)?;
        self.search_commands.refresh_list();

        self.push_event(AppEvent::new(&format!("Settings loaded from: {}", &settings), LevelCode::INFO));
        Ok(())
    }

    pub fn load_example_commands(&mut self) {
        self.search_commands.listful_cmds.items.push(Command::new(
            "ping".to_string(),
            "network".to_string(),
            "Simple ping with verbose on".to_string(),
            "-v <destination>".to_string(),
            vec!["ping 127.0.0.1".to_string(), "ping -v 127.0.0.1".to_string()]));

        self.push_event(AppEvent::new(&format!("Number of commands loaded: {}", self.search_commands.listful_cmds.items.len()), LevelCode::INFO));
    }

    pub fn push_event(&mut self, event: AppEvent) {
        if self.events.len() > self.max_events {
            _ = self.events.remove(0);
        }
        self.events.push(event);
    }

    pub fn set_chosen_command(&mut self) {
        let Some(selected) = self.search_commands.listful_cmds.state.selected() else {
            self.push_event(AppEvent::new(&format!("Cannot get selected command id"), LevelCode::ERROR));
            self.chosen_command = None;
            return;
        };
        // Get item from list
        let Some(command) = self.search_commands.listful_cmds.items.get(selected).clone() else {
            self.push_event(AppEvent::new(&format!("Cannot get selected command"), LevelCode::ERROR));
            self.chosen_command = None;
            return;
        };

        self.chosen_command = Some(ChosenCommand::from(command));
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
                match &mut self.chosen_command {
                    Some(chosen) => {
                        // Retrieve ID of CommandArg
                        let selected_id = match chosen.listful_args.state.selected() {
                            Some(i) => i,
                            None => return
                        };
                        let cmd_id = match chosen.listful_args.items.get(selected_id) {
                            Some(a) => a.id,
                            None => return
                        };

                        // Command modifier
                        match chosen.command.cmd_args.get_mut(cmd_id) {
                            Some(c) => {
                                match &mut c.modified {
                                    Some(s) => s.push(x),
                                    None => c.modified = Some(x.to_string())
                                }
                            },
                            None => {}
                        }

                        // Refresh list after modifications of command
                        chosen.refresh_list();
                    }
                    None => {
                        self.push_event(AppEvent::new(&format!("KeyCode: {}", x), LevelCode::TRACE));
                        self.search_commands.search.push(x);
                        self.search_commands.refresh_list();
                    }
                }
            }
            // KeyCode::Left => app.items.unselect(),
            KeyCode::Backspace => _ = {
                match &mut self.chosen_command {
                    Some(chosen) => {
                        let selected_id = match chosen.listful_args.state.selected() {
                            Some(i) => i,
                            None => return
                        };
                        let cmd_id = match chosen.listful_args.items.get(selected_id) {
                            Some(a) => a.id,
                            None => return
                        };

                        // Command modifier
                        match chosen.command.cmd_args.get_mut(cmd_id) {
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

                        // Refresh list after modifications of command
                        chosen.refresh_list();
                    },
                    None => {
                        _ = self.search_commands.search.pop();
                        self.search_commands.refresh_list();
                    }
                }
            },
            KeyCode::Down => {
                // There is 2 possibility when `down` is triggered:
                // - 1. Popup is opened and so it's switching between command values to fill
                // - 2. Popup is not opened and it's switching between commands
                match &mut self.chosen_command {
                    Some(c) => c.listful_args.next(),
                    None => self.search_commands.listful_cmds.next()
                }
            },
            KeyCode::Up => {
                // There is 2 possibility when `up` is triggered:
                // - 1. Popup is opened and so it's switching between command values to fill
                // - 2. Popup is not opened and it's switching between commands
                match &mut self.chosen_command {
                    Some(c) => c.listful_args.previous(),
                    None => self.search_commands.listful_cmds.previous()
                }
            },
            KeyCode::Esc => {
                // There is 2 possibility when `escape` is triggered:
                // - 1. Popup is opened and so it's closing the popup
                // - 2. Popup is not opened and it's quitting the program
                match &mut self.chosen_command {
                    Some(_) => self.chosen_command = None,
                    None => {
                        self.push_event(AppEvent::new(&format!("Quitting program!"), LevelCode::INFO));
                        self.quit_app = true;
                    }
                }
            }
            KeyCode::Enter => {
                // There is 2 possibility when `enter` is triggered:
                // - 1. Popup is opened and so it's going to copy command to clipboard
                // - 2. Popup is not opened and it is opening the command popup
                match &self.chosen_command {
                    Some(c) => {
                        let final_cmd = c.command.copy();
                        // let command_str = format!("{command}");
                        if let Err(e) = write_co_clipboard(&final_cmd) {
                            self.push_event(AppEvent::new(&format!("Error when writing to clipboard, error={}", e), LevelCode::ERROR));
                            return
                        };
                        self.push_event(AppEvent::new(&format!("Value copied to clipboard: \"{}\"", final_cmd), LevelCode::DEBUG));
                    },
                    None => {
                        // Set new ChosenCommand
                        self.set_chosen_command();
                    }
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