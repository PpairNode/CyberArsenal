use tui::{
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block,Paragraph, Wrap},
};

use crate::arsenal_objects::command::Command;


pub fn create_search_paragraph_pane<'a>(search: &str, key_num: usize, total_key_num: usize, cmd_opt: Option<&Command>, block: Block<'a>) -> Paragraph<'a> {
    let cmd = match cmd_opt {
        Some(c) => c.copy_raw(),
        None => "".to_string(),
    };
    let search_spans: Vec<Spans> = vec![
        Spans::from(vec![
            Span::styled(">> ", Style::default()),
            Span::styled(format!("{}", search.to_string()), Style::default().fg(Color::LightRed))
        ]),
        Spans::from(vec![
            Span::styled(format!("{}/{}", key_num, total_key_num), Style::default())
        ]),
        Spans::from(vec![
            Span::styled(format!("{}", cmd), Style::default())
        ])
    ];
    let search_paragraph_pane = Paragraph::new(search_spans)
        .block(block)
        .wrap(Wrap { trim: true });

    search_paragraph_pane
}
