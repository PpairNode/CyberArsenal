use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

use super::{app::ArsenalApp, event::LevelCode};


pub fn render<B: Backend>(f: &mut Frame<B>, app: &mut ArsenalApp) {
    // COMPLETE WINDOW
    let window = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref()) 
        .split(f.size());

    // RIGHT PANE
    let right_pane = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(window[1]);

    // LEFT PANE
    // Iterate through all elements in the `items` app and append some debug text to it.
    let commands: Vec<ListItem> = app
        .items
        .items
        .iter()
        .map(|command| {
            ListItem::new(format!("{}", command)).style(Style::default())
        })
        .collect();
    // Create a List from all list items and highlight the currently selected one
    let commands_list_pane = List::new(commands)
        .block(Block::default().borders(Borders::ALL).title("Commands"))
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    // RIGHT UPPER PANE
    let info_pane = Block::default()
        .title("Info Pane")
        .borders(Borders::ALL);

    let info_paragraph_pane = match app.items.state.selected() {
        Some(s) => {
            match app.items.items.get(s) {
                Some(s) => Paragraph::new(s.info()),
                None => Paragraph::new("")
            }
        },
        None => Paragraph::new("")
    }
        .block(info_pane)
        .wrap(Wrap { trim: true });

    // RIGHT LOWER PANE
    // Let's do the same for the events.
    // The event list doesn't have any state and only displays the current state of the list.
    // let events: Vec<ListItem> = app
    //     .events
    //     .iter()
    //     .rev()
    //     .map(|app_event| {
    //         // Colorcode the level depending on its type
    //         let s = match app_event.level {
    //             LevelCode::CRITICAL => Style::default().fg(Color::Red),
    //             LevelCode::ERROR => Style::default().fg(Color::Magenta),
    //             LevelCode::WARNING => Style::default().fg(Color::Yellow),
    //             LevelCode::INFO => Style::default().fg(Color::Blue),
    //             _ => Style::default(),
    //         };
    //         // Add a example datetime and apply proper spacing between them
    //         let header = Spans::from(vec![
    //             Span::styled(format!("[{:<9}]", app_event.level), s),
    //             Span::styled(
    //                 format!("[{}]", app_event.datetime.format("%Y-%m-%d %T")),
    //                 Style::default().add_modifier(Modifier::DIM),
    //             ),
    //         ]);
    //         // The event gets its own line
    //         let log = Spans::from(vec![Span::raw(app_event.text.clone())]);

    //         // Here several things happen:
    //         // 1. Add a `---` spacing line
    //         // 2. Add the Level + datetime
    //         // 3. Add the log
    //         ListItem::new(vec![
    //             Spans::from("-".repeat(right_pane[1].width as usize)),
    //             header,
    //             log
    //         ])
    //     })
    //     .collect();

    let events_spans: Vec<Spans> = 
        app.events.iter()
            .rev()
            .map(|app_event| {
                // Colorcode the level depending on its type
                let s = match app_event.level {
                    LevelCode::CRITICAL => Style::default().fg(Color::Red),
                    LevelCode::ERROR => Style::default().fg(Color::Magenta),
                    LevelCode::WARNING => Style::default().fg(Color::Yellow),
                    LevelCode::INFO => Style::default().fg(Color::Blue),
                    _ => Style::default(),
                };
                // Add a example datetime and apply proper spacing between them
                Spans::from(vec![
                    Span::from("-".repeat(right_pane[1].width as usize - 2)),  // -2 => for event_paragraph_pane border
                    Span::styled(format!("[{:<9}]", app_event.level), s),
                    Span::styled(
                        format!("[{}]", app_event.datetime.format("%Y-%m-%d %T")),
                        Style::default().add_modifier(Modifier::DIM),
                    ),
                    Span::raw(app_event.text.clone())
                ])
            }).collect();

    // let events_list_pane = List::new(events)
    //     .block(Block::default().borders(Borders::ALL).title("Events"));

    let events_paragraph_pane = Paragraph::new(events_spans)
        .block(Block::default().title("Paragraph").borders(Borders::ALL))
        .style(Style::default())
        .wrap(Wrap { trim: true });

    // RENDER LEFT PANE
    f.render_stateful_widget(commands_list_pane, window[0], &mut app.items.state);
    // RENDER RIGHT UPPER PANE (info)
    f.render_widget(info_paragraph_pane, right_pane[0]);
    // RENDER RIGHT LOWER PANE (events)
    f.render_widget(events_paragraph_pane, right_pane[1]);
}