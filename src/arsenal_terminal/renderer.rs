use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

use super::{app::ArsenalApp, event::LevelCode};


pub fn render<B: Backend>(f: &mut Frame<B>, app: &mut ArsenalApp) {
    // COMPLETE WINDOW
    let window = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(95)])
        .split(f.size());

    // SEARCH PARAGRAPH
    let search_pane = Block::default()
        .title("Search CMD")
        .borders(Borders::ALL);
    let search_paragraph_pane = Paragraph::new(format!(">> {}", app.search.to_string()))
        .block(search_pane)
        .wrap(Wrap { trim: true });

    // BODY
    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)]) 
        .split(window[1]);

    // BODY RIGHT PANE
    let right_pane = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(body[1]);

    // BODY LEFT PANE
    // Iterate through all elements in the `items` app and append some debug text to it.
    let commands: Vec<ListItem> = app.items.items.iter()
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

    // BODY RIGHT UPPER PANE
    let info_pane = Block::default()
        .title("Info")
        .borders(Borders::ALL);

    let info_paragraph_pane = match app.items.state.selected() {
        Some(s) => {
            match app.items.items.get(s) {
                Some(c) => Paragraph::new(c.info()),
                None => Paragraph::new("")
            }
        },
        None => Paragraph::new("")
    }
        .block(info_pane)
        .wrap(Wrap { trim: true });

    // BODY RIGHT LOWER PANE
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

    let events_paragraph_pane = Paragraph::new(events_spans)
        .block(Block::default().title("Logs").borders(Borders::ALL))
        .style(Style::default())
        .wrap(Wrap { trim: true });

    f.render_widget(search_paragraph_pane, window[0]);
    // RENDER BODY LEFT PANE
    f.render_stateful_widget(commands_list_pane, body[0], &mut app.items.state);
    // RENDER BODY RIGHT UPPER PANE (info)
    f.render_widget(info_paragraph_pane, right_pane[0]);
    // RENDER BODY RIGHT LOWER PANE (events)
    f.render_widget(events_paragraph_pane, right_pane[1]);

    // If App command popup is opened, this should show the popup
    // let command = match app.get_selected_command() {
    //     Ok(c) => c,
    //     Err(e) => {
    //         app.push_event(AppEvent::new(&format!("Cannot get selected command, error={}", e), LevelCode::ERROR));
    //         return
    //     }
    // };
    match &mut app.chosen_command {
        Some(chosen) => {
            // POPUP Centered
            let area = centered_rect(60, 20, f.size());

            // POPUP Window
            let popup_window = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(100)])
                .split(area);
            let popup_block = Block::default()
                .title("Popup CMD")
                .borders(Borders::ALL);

            let popup_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(95)])
                .split(area);

            // COMMAND Block
            let command = format!("{}", chosen.command);

            let command_paragraph_block = Block::default()
                .borders(Borders::ALL);
            let command_paragraph_pane = Paragraph::new(format!("$ {}", command))
                .wrap(Wrap { trim: true })
                .block(command_paragraph_block);

            // COMMAND VALUES Block
            let command_args: Vec<ListItem> = chosen.listful_args.items.iter()
                .map(|cmd_arg| {
                    ListItem::new(format!("{}", cmd_arg)).style(Style::default())
                })
                .collect();
            // Create a List from all list items and highlight the currently selected one
            let command_arg_list_pane = List::new(command_args)
                .block(Block::default().borders(Borders::LEFT))
                .highlight_style(
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::LightGreen)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol("> ");

            // RENDERER
            f.render_widget(Clear, area); //this clears out the background
            f.render_widget(command_paragraph_pane, popup_layout[0]);
            f.render_stateful_widget(command_arg_list_pane, popup_layout[1], &mut chosen.listful_args.state);
            f.render_widget(popup_block, popup_window[0]);
        },
        None => {}
    };
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ],
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ],
        )
        .split(popup_layout[1])[1]
}