use ratatui::{
    layout::Rect,
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders},
    Frame,
};

use super::{colors::BMColors, UIState};

static CONTROLS: [(&'static str, &'static str); 7] = [
    ("jk", "Select"),
    ("c", "Connect"),
    ("p", "Pair"),
    ("d", "Disconnect"),
    ("r", "Remove"),
    ("h", "Show/Hide Unammed"),
    ("i", "Inspect device"),
];

static INSPECT_CONTROLS: [(&'static str, &'static str); 1] = [("ESC", "Return")];

pub fn draw_browse_controls(f: &mut Frame, area: Rect, ui_state: &mut UIState) {
    draw_controls(f, area, ui_state, &CONTROLS);
}

pub fn draw_inspect_controls(f: &mut Frame, area: Rect, ui_state: &mut UIState) {
    draw_controls(f, area, ui_state, &INSPECT_CONTROLS);
}

pub fn draw_controls(
    f: &mut Frame,
    area: Rect,
    ui_state: &mut UIState,
    control_strings: &[(&'static str, &'static str)],
) {
    let block = Block::new()
        .borders(Borders::ALL)
        .border_style(Style::new().fg(BMColors::DARK_GRAY))
        .border_type(BorderType::Rounded);

    let block_inner = block.inner(area);

    let mut width_remaining = block_inner.width as i32 - 4;

    let mut control_spans: Vec<Span> = vec![];

    for &(a, b) in control_strings.iter() {
        let t0 = format!(" [{}] ", a);
        let t1 = format!("{} ", b);
        let l = t0.len() + t1.len();

        width_remaining -= l as i32;

        if width_remaining > 0 {
            control_spans.push(Span::styled(t0, Style::new().bold().fg(BMColors::BLUE2)));
            control_spans.push(Span::styled(t1, Style::new().fg(BMColors::GRAY)));
        } else {
            break;
        }
    }

    if width_remaining < 0 {
        control_spans.push(Span::styled(
            "...".to_string(),
            Style::new().fg(BMColors::GRAY),
        ));
    }

    let controls = Line::from(control_spans);

    let offset = block_inner.width as i32 / 2 - controls.width() as i32 / 2;

    let controls_rect = Rect {
        x: offset as u16,
        y: block_inner.y,
        width: controls.width() as u16,
        height: 1,
    };

    f.render_widget(block, area);
    f.render_widget(controls, controls_rect);
}

pub fn draw_quit_hint(f: &mut Frame, area: Rect, _: &mut UIState) {
    let quit_hint = Line::from(vec![
        Span::styled("", Style::new().fg(BMColors::DARK_GRAY)),
        Span::styled(" [q] ", Style::new().bold().white().bg(BMColors::DARK_GRAY)),
        Span::styled("quit ", Style::new().white().bg(BMColors::DARK_GRAY)),
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
