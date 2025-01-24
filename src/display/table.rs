use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Row, Table},
    Frame,
};

use super::UIState;

pub fn draw_table(f: &mut Frame, area: Rect, ui_state: &mut UIState) {
    // Create a block for the table
    let block = Block::default().title("Sample Table").borders(Borders::ALL);

    // Define table rows
    let rows = vec![
        Row::new(vec!["ID", "Name", "Status"]),
        Row::new(vec!["1", "Alice", "Active"]),
        Row::new(vec!["2", "Bob", "Inactive"]),
        Row::new(vec!["3", "Charlie", "Pending"]),
    ];

    // Define the table
    let table = Table::new(
        rows,
        vec![
            Constraint::Percentage(20),
            Constraint::Percentage(40),
            Constraint::Percentage(40),
        ],
    )
    .block(block)
    .column_spacing(2)
    .row_highlight_style(Style::new().add_modifier(Modifier::REVERSED));

    f.render_stateful_widget(table, area, &mut ui_state.table_state);
}
