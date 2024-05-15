use crossterm::event::{self, Event, KeyCode};
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

use crate::arsenal_objects::command::{load_values_into_commands, Command};
use super::{event::AppEvent, renderer, stateful_list::StatefulList};
use super::event::LevelCode;


pub struct ArsenalApp {
    pub max_events: usize,
    pub items: StatefulList<Command>,
    pub events: Vec<AppEvent>,
    pub quit_app: bool
}

impl ArsenalApp {
    pub fn new(max_events: usize) -> ArsenalApp {
        ArsenalApp {
            items: StatefulList::with_items(vec![]),
            max_events,
            events: vec![],
            quit_app: false
        }
    }

    fn on_tick(&mut self) {
        // Do something on tick
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
            vec![
                ("help".to_string(), "[-h]".to_string()),
                ("verbose".to_string(), "[-v]".to_string()),
                ("destination".to_string(), "<destination>".to_string())
            ],
            vec!["ping 127.0.0.1".to_string(), "ping -v 127.0.0.1".to_string()]));

        self.push_event(AppEvent::new(&format!("Number of commands loaded: {}", self.items.items.len()), LevelCode::INFO));
    }

    fn push_event(&mut self, event: AppEvent) {
        if self.events.len() > self.max_events {
            _ = self.events.remove(0);
        }
        self.events.push(event);
    }

    fn handle_event_key(&mut self, key_code: KeyCode) {
        // self.push_event(AppEvent::new(&format!("KeyCode triggered: {:?}", key_code), ErrorCode::TRACE));
        match key_code {
            // If key is 'q' => app should quit
            KeyCode::Char('q') => {
                self.push_event(AppEvent::new(&format!("Quitting program!"), LevelCode::INFO));
                self.quit_app = true;
            }
            // KeyCode::Left => app.items.unselect(),
            KeyCode::Down => self.items.next(),
            KeyCode::Up => self.items.previous(),
            _ => {}
        }
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
                app.handle_event_key(key.code);
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