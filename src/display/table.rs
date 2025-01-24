use std::borrow::{Borrow, BorrowMut};

use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::Span,
    widgets::{Block, Borders, Row, Table},
    Frame,
};

use crate::app::BTDevice;

use super::{icons::get_icon_for_bt_type, UIState};

pub fn draw_table(f: &mut Frame, area: Rect, ui_state: &mut UIState) {
    // Create a block for the table
    let block = Block::default().title(" Devices ");

    let d = ui_state.devices.as_ref().borrow(); // Thank u borrow checker :pray:

    // Define table rows
    let mut rows = vec![Row::new(vec![
        "",
        "Name",
        "Paired",
        "Connected",
        "Type",
        "MAC Address",
    ])];

    let paired_text = Span::styled("Yes", Style::new().green());
    let empty = Span::raw("");

    rows.extend(d.iter().map(|d: &BTDevice| {
        Row::new(vec![
            Span::raw(get_icon_for_bt_type(&d.icon_name).to_owned()),
            Span::raw(d.name.to_owned()),
            if d.paired {
                Span::styled("Yes", Style::new().green())
            } else {
                Span::default()
            },
            if d.connected {
                Span::styled("Yes", Style::new().green())
            } else {
                Span::default()
            },
            Span::styled(d.icon_name.to_owned(), Style::new().dark_gray()),
            Span::styled(d.address.to_owned(), Style::new().dark_gray()),
        ])
    }));

    // Define the table
    let table = Table::new(
        rows,
        vec![
            Constraint::Percentage(2),
            Constraint::Percentage(99),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(18),
            Constraint::Length(20),
        ],
    )
    .block(block)
    .column_spacing(1)
    .row_highlight_style(Style::new().add_modifier(Modifier::REVERSED));

    f.render_stateful_widget(table, area, &mut ui_state.table_state);
}
