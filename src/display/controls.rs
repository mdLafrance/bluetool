use ratatui::{
    layout::Rect,
    style::{Style, Stylize},
    text::{Line, Span},
    Frame,
};

use super::{colors::BMColors, UIState};

pub fn draw_controls(f: &mut Frame, area: Rect, ui_state: &mut UIState) {
    let control_texts: Vec<(&'static str, &'static str)> = vec![
        ("jk", "Select"),
        ("c", "Connect"),
        ("p", "Pair"),
        ("d", "Disconnect"),
        ("r", "Remove"),
        ("h", "Show/Hide Unammed"),
        ("i", "Inspect device"),
    ];

    let mut control_spans: Vec<Span> = vec![];

    control_texts.iter().for_each(|(a, b)| {
        control_spans.push(Span::raw(" "));
        control_spans.push(Span::styled(
            format!("[{}]", a),
            Style::new().bold().fg(BMColors::BLUE2),
        ));
        control_spans.push(Span::raw(" "));
        control_spans.push(Span::styled(b.to_string(), Style::new().fg(BMColors::GRAY)));
        control_spans.push(Span::raw(" "));
    });

    let controls = Line::from(control_spans);

    let offset = area.width as i32 / 2 - controls.width() as i32 / 2;

    let rect = Rect {
        x: offset as u16,
        y: area.height - 1,
        width: controls.width() as u16,
        height: 1,
    };

    f.render_widget(controls, rect);
}

pub fn draw_quit_hint(f: &mut Frame, area: Rect, _: &mut UIState) {
    let quit_hint = Line::from(vec![
        Span::styled("", Style::new().fg(BMColors::DARK_GRAY)),
        Span::styled("[q] ", Style::new().bold().white().bg(BMColors::DARK_GRAY)),
        Span::styled("quit", Style::new().white().bg(BMColors::DARK_GRAY)),
        Span::styled("", Style::new().fg(BMColors::DARK_GRAY)),
    ]);

    let x = area.width as u16 - 1 - quit_hint.width() as u16;

    let rect = Rect {
        x,
        y: 0,
        width: quit_hint.width() as u16,
        height: 1,
    };

    f.render_widget(quit_hint, rect);
}
