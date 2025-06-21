use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

use super::app::ArsenalApp;


pub fn render<B: Backend>(f: &mut Frame<B>, app: &mut ArsenalApp) {
    // COMPLETE WINDOW
    let window = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Max(89), Constraint::Max(7)])
        .split(f.size());

    // SEARCH PARAGRAPH
    let search_spans: Vec<Spans> = vec![
        Spans::from(vec![
            Span::styled(">> ", Style::default()),
            Span::styled(format!("{}", app.search_commands.search.to_string()), Style::default().fg(Color::LightRed))
        ])
    ];
    let search_pane = Block::default()
        .title("Search CMD")
        .borders(Borders::ALL);
    let search_paragraph_pane = Paragraph::new(search_spans)
        .block(search_pane)
        .wrap(Wrap { trim: true });

    // BODY
    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)]) 
        .split(window[1]);

    // BODY RIGHT PANE
    let footer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)])
        .split(window[2]);

    // BODY LEFT PANE
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

    // INFO PANE
    let info_pane = Block::default()
        .title("Info")
        .borders(Borders::ALL);
    let info_paragraph_pane = match app.search_commands.listful_cmds.state.selected() {
        Some(s) => {
            match app.search_commands.listful_cmds.items.get(s) {
                Some(c) => {
                    let mut info_spans: Vec<Spans> = vec![
                        Spans::from(vec![
                            Span::styled("Command:", Style::default().fg(Color::LightBlue)),
                            Span::styled(format!("{}\n", c.name_cmd), Style::default().fg(Color::Green))
                        ]),
                        Spans::from(vec![
                            Span::styled("TYPE:", Style::default().fg(Color::LightBlue)),
                            Span::styled(format!("{:<11}", 
                                c.cmd_types.iter()
                                    .map(|cmd_type| format!("{:?}", cmd_type))
                                    .collect::<Vec<String>>().join(" ")), Style::default().fg(Color::Green)),
                        ]),
                        Spans::from(vec![
                            Span::raw("")
                        ])
                    ];

                    // Add Explanation if any
                    if !c.short_desc.is_empty() {
                        info_spans.extend(vec![
                            Spans::from(vec![
                                Span::styled("Explanation:", Style::default().fg(Color::LightBlue))
                            ])
                        ]);

                        let explanation_line_spans: Vec<Vec<Spans>> = c.short_desc.split("\n")
                            .map(|s| {
                                vec![
                                    Spans::from(vec![
                                        Span::styled(format!("{}", s), Style::default().fg(Color::LightCyan))
                                    ])
                                ]
                            })
                            .collect();

                            for e in explanation_line_spans {
                                info_spans.extend(e);
                            }
                            info_spans.extend(vec![
                                Spans::from(vec![
                                    Span::raw("")
                                ])
                            ]);
                    }

                    // info_spans.extend(vec![
                    //     Spans::from(vec![
                    //         Span::styled("Full Command:", Style::default().fg(Color::LightBlue))
                    //     ]),
                    //     Spans::from(vec![
                    //         Span::styled(c.copy_raw(), Style::default().fg(Color::LightCyan))
                    //     ]),
                    //     Spans::from(vec![
                    //         Span::raw("")
                    //     ])
                    // ]);

                    // // Add Examples if any
                    // if !c.examples.is_empty() {
                    //     info_spans.extend(vec![
                    //         Spans::from(vec![
                    //             Span::styled("Examples:", Style::default().fg(Color::LightBlue))
                    //         ]),
                    //         Spans::from(vec![
                    //             Span::from("\n-\n".repeat(footer[0].width as usize - 2)),  // -2 => for event_paragraph_pane border
                    //         ])
                    //     ]);

                    //     let example_spans: Vec<Vec<Spans>> = c.examples.iter()
                    //         .map(|s| {
                    //             vec![
                    //                 Spans::from(vec![
                    //                     Span::styled(format!("{}", s), Style::default().fg(Color::LightCyan))
                    //                 ]),
                    //                 Spans::from(vec![
                    //                     Span::from("\n-\n".repeat(footer[0].width as usize - 2)),  // -2 => for event_paragraph_pane border
                    //                 ])
                    //             ]
                    //         })
                    //         .collect();

                    //     for e in example_spans {
                    //         info_spans.extend(e);
                    //     }
                    // }

                    Paragraph::new(info_spans)
                },
                None => Paragraph::new("")
            }
        },
        None => Paragraph::new("")
    }
        .block(info_pane)
        .wrap(Wrap { trim: true });


    // ========== RENDERER ==========
    // RENDER SEARCH PANE
    f.render_widget(search_paragraph_pane, window[0]);
    // RENDER BODY LEFT PANE
    f.render_stateful_widget(commands_list_pane, body[0], &mut app.search_commands.listful_cmds.state);
    // RENDER INFO PANE
    f.render_widget(info_paragraph_pane, footer[0]);


    // If App command popup is opened, this should show the popup
    match &mut app.chosen_command {
        Some(chosen) => {
            // POPUP Centered
            let area = centered_rect(60, 40, f.size());
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

            let popup_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(95)])
                .split(area2);

            // COMMAND Block
            let command_spans: Vec<Spans> = vec![
                Spans::from(vec![
                    Span::styled(">> ", Style::default()),
                    Span::styled(chosen.command.copy_basic(), Style::default().fg(Color::LightRed))
                ])
            ];
            let command_paragraph_block = Block::default()
                .borders(Borders::BOTTOM);
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
                .block(Block::default().borders(Borders::NONE))
                .highlight_style(
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::LightGreen)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol("> ");

            // RENDERER
            f.render_widget(Clear, area);  // this clears out the background
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