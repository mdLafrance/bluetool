use std::borrow::{Borrow, BorrowMut};

use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Row, Table},
    Frame,
};

use crate::app::BTDevice;

use super::{icons::get_icon_for_bt_type, UIState};

pub fn draw_table(f: &mut Frame, area: Rect, ui_state: &mut UIState) {
    // Create a block for the table
    let block = Block::default().title("Sample Table").borders(Borders::ALL);

    let d = ui_state.devices.as_ref().borrow(); // Thank u borrow checker :pray:

    let mut devices = d.into_iter().collect::<Vec<BTDevice>>();
    devices.sort();

    // Define table rows
    let mut rows = vec![Row::new(vec![
        "",
        "Name",
        "Paired",
        "Connected",
        "Type",
        "MAC Address",
    ])];

    rows.extend(d.iter().map(|d: &BTDevice| {
        Row::new(vec![
            get_icon_for_bt_type(&d.icon_name).to_owned(),
            d.name.to_owned(),
            if d.paired {
                format!("Yes")
            } else {
                format!("")
            },
            if d.connected {
                format!("Yes")
            } else {
                format!("")
            },
            d.icon_name.to_owned(),
            d.address.to_owned(),
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
            Constraint::Length(15),
            Constraint::Length(20),
        ],
    )
    .block(block)
    .column_spacing(1)
    .row_highlight_style(Style::new().add_modifier(Modifier::REVERSED));

    f.render_stateful_widget(table, area, &mut ui_state.table_state);
}
