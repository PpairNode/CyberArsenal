use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};


use crate::arsenal_objects::command::CommandArg;

use super::app::ArsenalApp;
use super::panes::info;
use super::panes::search;

pub fn render<B: Backend>(f: &mut Frame<B>, app: &mut ArsenalApp) {
    // ========== LAYOUTS ==========
    // WINDOW
    // BODY (commands)
    // FOOTER (info)
    // =============================
    // Complete window
    let window = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(10), Constraint::Length(5)])
        .split(f.size());
    // Body
    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)]) 
        .split(window[1]);
    // Footer
    let footer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)])
        .split(window[2]);


    // BLOCKS AND PARAGRAPHS
    // Search bar
    let search_pane = Block::default()
        .title("Search CMD")
        .borders(Borders::ALL);
    let search_paragraph_pane = search::create_info_paragraph_pane(&app.search_commands.search, search_pane);

    // Command list
    // Iterate through all elements in the `items` app and append some debug text to it.
    let commands: Vec<ListItem> = app.search_commands.listful_cmds.items.iter()
        .map(|command| {
            ListItem::new(command.copy_raw_shifted()).style(Style::default().fg(Color::LightBlue))
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

    // Info
    let info_pane = Block::default()
        .title("Info")
        .borders(Borders::ALL);
    
    let info_paragraph_pane = match app.search_commands.listful_cmds.state.selected() {
        Some(s) => {
            info::create_info_paragraph_pane_light(app.search_commands.listful_cmds.items.get(s), info_pane)
        },
        None => Paragraph::new("")
    };


    // ========== RENDERER ==========
    // Search
    f.render_widget(search_paragraph_pane, window[0]);
    // Commands list
    f.render_stateful_widget(commands_list_pane, body[0], &mut app.search_commands.listful_cmds.state);
    // Info
    f.render_widget(info_paragraph_pane, footer[0]);


    // If App command popup is opened, this should show the popup
    match &mut app.chosen_command {
        Some(chosen) => {
            // POPUP Centered
            let area = centered_rect(70, 70, f.size());
            // Create a rectangle inside the rectangle
            let mut area2 = area.clone();
            area2.x += 1;
            area2.y += 1;
            area2.height -= 2;
            area2.width -= 2;

            // POPUP Window
            let popup_window = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(100)])
                .split(area);
            let popup_block = Block::default()
                .title("Popup CMD")
                .borders(Borders::ALL);

            let input_args: Vec<&CommandArg> = chosen.command.cmd_args.iter().filter(|c| c.is_input).collect();
            let popup_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(3), Constraint::Length(input_args.len() as u16 + 1), Constraint::Max(15)])
                .split(area2);

            // COMMAND Block
            let text = chosen.command.copy_basic();
            let command_spans: Vec<Spans> = vec![
                Spans::from(vec![
                    Span::styled(">> ", Style::default()),
                    Span::styled(text, Style::default().fg(Color::LightRed))
                ])
            ];
            let command_paragraph_block = Block::default();
            let command_paragraph_pane = Paragraph::new(command_spans)
                .style(Style::default())
                .wrap(Wrap { trim: true })
                .block(command_paragraph_block);

            // COMMAND VALUES Block
            let command_args: Vec<ListItem> = chosen.listful_args.items.iter()
                .map(|cmd_arg| {
                    ListItem::new(format!("{}", cmd_arg)).style(Style::default().fg(Color::LightBlue))
                })
                .collect();
            // Create a List from all list items and highlight the currently selected one
            let command_arg_list_pane = List::new(command_args)
                .block(Block::default().borders(Borders::TOP).title("Arguments"))
                .highlight_style(
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::LightGreen)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol("> ");

            let examples_pane = Block::default()
                .title("Examples")
                .borders(Borders::TOP);
            
            let examples_paragraph_pane = match app.search_commands.listful_cmds.state.selected() {
                Some(s) => {
                    info::create_examples_paragraph_pane(app.search_commands.listful_cmds.items.get(s), examples_pane)
                },
                None => Paragraph::new("")
            };

            // RENDERER
            f.render_widget(Clear, area);  // this clears out the background
            f.render_widget(command_paragraph_pane, popup_layout[0]);
            f.render_stateful_widget(command_arg_list_pane, popup_layout[1], &mut chosen.listful_args.state);
            f.render_widget(examples_paragraph_pane, popup_layout[2]);
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