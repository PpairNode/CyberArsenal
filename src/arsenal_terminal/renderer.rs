use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use super::{app::ArsenalApp, event::ErrorCode};


pub fn render<B: Backend>(f: &mut Frame<B>, app: &mut ArsenalApp) {
    // Create two chunks with equal horizontal screen space
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());

    // Iterate through all elements in the `items` app and append some debug text to it.
    let items: Vec<ListItem> = app
        .items
        .items
        .iter()
        .map(|line| {
            ListItem::new(line.clone()).style(Style::default())
        })
        .collect();

    // Create a List from all list items and highlight the currently selected one
    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Commands"))
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    // We can now render the item list
    f.render_stateful_widget(items, chunks[0], &mut app.items.state);

    // Let's do the same for the events.
    // The event list doesn't have any state and only displays the current state of the list.
    let events: Vec<ListItem> = app
        .events
        .iter()
        .rev()
        .map(|app_event| {
            // Colorcode the level depending on its type
            let s = match app_event.level {
                ErrorCode::CRITICAL => Style::default().fg(Color::Red),
                ErrorCode::ERROR => Style::default().fg(Color::Magenta),
                ErrorCode::WARNING => Style::default().fg(Color::Yellow),
                ErrorCode::INFO => Style::default().fg(Color::Blue),
                _ => Style::default(),
            };
            // Add a example datetime and apply proper spacing between them
            let header = Spans::from(vec![
                Span::styled(format!("[{:<9}]", app_event.level), s),
                Span::styled(
                    format!("[{}]", app_event.datetime.format("%Y-%m-%d %T")),
                    Style::default().add_modifier(Modifier::DIM),
                ),
            ]);
            // The event gets its own line
            let log = Spans::from(vec![Span::raw(app_event.text.clone())]);

            // Here several things happen:
            // 1. Add a `---` spacing line
            // 2. Add the Level + datetime
            // 3. Add the log
            ListItem::new(vec![
                Spans::from("-".repeat(chunks[1].width as usize)),
                header,
                log
            ])
        })
        .collect();
    let events_list = List::new(events)
        .block(Block::default().borders(Borders::ALL).title("Events"));
    f.render_widget(events_list, chunks[1]);
}