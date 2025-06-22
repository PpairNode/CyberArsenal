use tui::{
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block,Paragraph, Wrap},
};

use crate::arsenal_objects::command::Command;







pub fn create_info_paragraph_pane<'a>(command: Option<&Command>, block: Block<'a>) -> Paragraph<'a> {
    let Some(command) = command else {
        return Paragraph::new("")
    };
    
    let mut info_spans: Vec<Spans> = vec![
        Spans::from(vec![
            Span::styled("Command:", Style::default().fg(Color::LightBlue)),
            Span::styled(format!("{}\n", command.name_exe), Style::default().fg(Color::Green))
        ]),
        Spans::from(vec![
            Span::styled("TYPE:", Style::default().fg(Color::LightBlue)),
            Span::styled(format!("{:<11}", 
                command.cmd_types.iter()
                    .map(|cmd_type| format!("{:?}", cmd_type))
                    .collect::<Vec<String>>().join(" ")), Style::default().fg(Color::Green)),
        ]),
        Spans::from(vec![
            Span::raw("")
        ])
    ];

    // Add Explanation if any
    if !command.short_desc.is_empty() {
        info_spans.extend(vec![
            Spans::from(vec![
                Span::styled("Explanation:", Style::default().fg(Color::LightBlue))
            ])
        ]);

        let explanation_line_spans: Vec<Vec<Spans>> = command.short_desc.split("\n")
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

    let info_paraghraph_pane = Paragraph::new(info_spans)
        .block(block)
        .wrap(Wrap { trim: true });

    info_paraghraph_pane
}