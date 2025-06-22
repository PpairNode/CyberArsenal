use tui::{
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block,Paragraph, Wrap},
};


pub fn create_info_paragraph_pane<'a>(search: &str, block: Block<'a>) -> Paragraph<'a> {
    let search_spans: Vec<Spans> = vec![
        Spans::from(vec![
            Span::styled(">> ", Style::default()),
            Span::styled(format!("{}", search.to_string()), Style::default().fg(Color::LightRed))
        ])
    ];
    let search_paragraph_pane = Paragraph::new(search_spans)
        .block(block)
        .wrap(Wrap { trim: true });

    search_paragraph_pane
}
