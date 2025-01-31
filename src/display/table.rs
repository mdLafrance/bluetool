use ratatui::{
    layout::{Constraint, Rect},
    style::{Modifier, Style, Stylize},
    text::Span,
    widgets::{Block, BorderType, Borders, Padding, Row, Table},
    Frame,
};

use crate::app::BTDevice;

use super::{colors::BMColors, icons::get_icon_for_bt_type, UIState};

pub fn draw_table(f: &mut Frame, area: Rect, ui_state: &mut UIState) {
    let d = ui_state.devices.as_ref().borrow(); // Thank u borrow checker :pray:

    let table_header_style = Style::new().bold().dark_gray();

    // Define table rows
    let mut rows = vec![Row::new(
        vec![
            " ",
            " ",
            "Name",
            "Paired",
            "Connected",
            "Signal",
            "Type",
            "Address",
        ]
        .iter()
        .map(|t| Span::styled(*t, table_header_style)),
    )];

    rows.extend(
        d.iter()
            .filter(|d| (d.name != "???") || ui_state.show_unnamed)
            .map(|d: &BTDevice| {
                let s = match d.connected {
                    true => Style::new().fg(BMColors::BLUE), // Style::new().on_blue(),
                    false => Style::new(),
                };

                Row::new(vec![
                    if d.connected {
                        Span::styled("󰂱", s)
                    } else if d.paired {
                        Span::styled("󰂯", s)
                    } else {
                        Span::raw(" ")
                    },
                    Span::styled(get_icon_for_bt_type(&d.icon_name).to_owned() + " ", s),
                    Span::styled(d.name.to_owned(), s),
                    if d.paired {
                        Span::styled("Yes", s.green())
                    } else {
                        Span::default()
                    },
                    if d.connected {
                        Span::styled("Yes", s.green())
                    } else {
                        Span::default()
                    },
                    format_signal_span(d.rssi),
                    Span::styled(d.icon_name.to_owned(), s.dark_gray()),
                    Span::styled(d.address.to_owned(), s.dark_gray()),
                ])
            }),
    );

    // Define the table
    let table = Table::new(
        rows,
        vec![
            Constraint::Length(1),
            Constraint::Length(2),
            Constraint::Percentage(99),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(18),
            Constraint::Length(20),
        ],
    )
    .column_spacing(1)
    .block(
        Block::new()
            .title(" Devices ")
            .title_style(Style::new().fg(BMColors::GRAY).bold())
            .padding(Padding {
                left: 1,
                right: 1,
                top: 0,
                bottom: 0,
            })
            .borders(Borders::ALL)
            .border_style(Style::new().fg(BMColors::DARK_GRAY))
            .border_type(BorderType::Rounded),
    )
    .row_highlight_style(Style::new().add_modifier(Modifier::REVERSED));

    f.render_stateful_widget(table, area, &mut ui_state.table_state);
}

fn format_battery_span(battery: Option<u8>) -> Span<'static> {
    if let Some(b) = battery {
        match b {
            0..10 => Span::styled(" ", Style::new().fg(BMColors::RED)),
            10..40 => Span::styled(" ", Style::new().fg(BMColors::ORANGE)),
            40..60 => Span::styled(" ", Style::new().fg(BMColors::YELLOW)),
            70..90 => Span::styled(" ", Style::new().fg(BMColors::GREEN)),
            90.. => Span::styled(" ", Style::new().fg(BMColors::GREEN)),
            _ => unreachable!(),
        }
    } else {
        Span::raw("")
    }
}

fn format_signal_span(battery: Option<i16>) -> Span<'static> {
    if let Some(b) = battery {
        match b {
            -50..0 => Span::styled("󰢾 ", Style::new().fg(BMColors::GREEN)),
            -90..-51 => Span::styled("󰢽 ", Style::new().fg(BMColors::YELLOW)),
            _ => Span::styled("󰢼 ", Style::new().fg(BMColors::RED)),
        }
    } else {
        Span::raw("")
    }
}
