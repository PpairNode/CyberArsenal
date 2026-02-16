use tui::{
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block,Paragraph, Wrap},
};


pub fn create_info_paragraph_pane<'a>(search: &str, key_num: usize, total_key_num: usize, block: Block<'a>) -> Paragraph<'a> {
    let search_spans: Vec<Spans> = vec![
        Spans::from(vec![
            Span::styled(">> ", Style::default()),
            Span::styled(format!("{}", search.to_string()), Style::default().fg(Color::LightRed))
        ]),
        Spans::from(vec![
            Span::styled(format!("K: {}/{}", key_num, total_key_num), Style::default())
        ])
    ];
    let search_paragraph_pane = Paragraph::new(search_spans)
        .block(block)
        .wrap(Wrap { trim: true });

    search_paragraph_pane
}
