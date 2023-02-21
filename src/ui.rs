use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Table},
    Frame,
};

use crate::AppState;

pub fn ui<B: Backend>(f: &mut Frame<B>, state: &mut AppState) {
    let parent_chunk = Layout::default()
        .direction(tui::layout::Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(2),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(f.size());

    let header_block = Block::default()
        .title("npkill-rs v0.1 - Kill them with fire!")
        .title_alignment(tui::layout::Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let header_text = if state.loading {
        "loading..."
    } else {
        "To delete a folder press 'd' on your selection."
    };

    let header_paragraph = Paragraph::new(header_text).block(header_block);

    let footer_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let list_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let list_chunks = Layout::default()
        .margin(2)
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3)].as_ref())
        .split(parent_chunk[1]);

    let data: Vec<Row> = state
        .dirs
        .iter()
        .map(|dir| {
            Row::new(vec![
                Cell::from(String::from(dir.dir_entry.path().to_str().unwrap())),
                Cell::from(dir.size.clone()),
            ])
        })
        .collect();

    let table = Table::new(data)
        .block(Block::default())
        .widths(&[Constraint::Percentage(90), Constraint::Percentage(10)])
        .highlight_symbol("->")
        .highlight_style(
            Style::default()
                .bg(Color::LightYellow)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        );

    f.render_stateful_widget(table, list_chunks[0], &mut state.list_state);

    f.render_widget(header_paragraph, parent_chunk[0]);
    f.render_widget(list_block, parent_chunk[1]);
    f.render_widget(footer_block, parent_chunk[2]);
}
