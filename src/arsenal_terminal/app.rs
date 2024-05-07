use crossterm::event::{self, Event, KeyCode};
use std::{
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::Backend,
    Terminal,
};

use crate::arsenal_objects::command::Command;
use super::{event::AppEvent, renderer, stateful_list::StatefulList};


pub struct ArsenalApp {
    pub max_events: usize,
    pub items: StatefulList<Command>,
    pub events: Vec<AppEvent>,
}

impl ArsenalApp {
    pub fn new(max_events: usize) -> ArsenalApp {
        ArsenalApp {
            items: StatefulList::with_items(vec![]),
            max_events,
            events: vec![],
        }
    }

    fn on_tick(&mut self) {
        // Do something on tick
    }

    pub fn load_example_commands(&mut self) {
        self.items.items.push(Command::new(
            "ping".to_string(),
            vec!["[-h]".to_string(), "[-v]".to_string(), "<destination>".to_string()],
            vec!["ping 127.0.0.1".to_string(), "ping -v 127.0.0.1".to_string()]))
    }

    fn push_event(&mut self, event: AppEvent) {
        if self.events.len() > 100 {
            _ = self.events.remove(0);
        }
        self.events.push(event);
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
                app.push_event(AppEvent::new(&format!("KeyCode triggered: {:?}", key.code), super::event::ErrorCode::TRACE));
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    // KeyCode::Left => app.items.unselect(),
                    KeyCode::Down => app.items.next(),
                    KeyCode::Up => app.items.previous(),
                    _ => {}
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}