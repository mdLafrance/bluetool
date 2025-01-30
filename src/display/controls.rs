use ratatui::{
    layout::Rect,
    style::{Style, Stylize},
    text::{Line, Span},
    Frame,
};

use super::{colors::BMColors, UIState};

pub fn draw_controls(f: &mut Frame, area: Rect, ui_state: &mut UIState) {
    let controls = Line::from(vec![
        Span::styled("[jk] ", Style::new().bold().fg(BMColors::GRAY)),
        Span::styled("select", Style::new().fg(BMColors::DARK_GRAY)),
        Span::raw(" "),
        Span::styled("[c] ", Style::new().bold().fg(BMColors::GRAY)),
        Span::styled("connect", Style::new().fg(BMColors::DARK_GRAY)),
        Span::raw(" "),
        Span::styled("[p] ", Style::new().bold().fg(BMColors::GRAY)),
        Span::styled("pair", Style::new().fg(BMColors::DARK_GRAY)),
        Span::raw(" "),
        Span::styled("[d] ", Style::new().bold().fg(BMColors::GRAY)),
        Span::styled("disconnect", Style::new().fg(BMColors::DARK_GRAY)),
        Span::raw(" "),
        Span::styled("[r] ", Style::new().bold().fg(BMColors::GRAY)),
        Span::styled("remove", Style::new().fg(BMColors::DARK_GRAY)),
        Span::raw(" "),
        Span::styled("[h] ", Style::new().bold().fg(BMColors::GRAY)),
        Span::styled(
            format!(
                "{} unnamed",
                if ui_state.show_unnamed {
                    "hide"
                } else {
                    "show"
                }
            ),
            Style::new().fg(BMColors::DARK_GRAY),
        ),
        Span::raw(" "),
        Span::styled("[i] ", Style::new().bold().fg(BMColors::GRAY)),
        Span::styled("inspect device", Style::new().fg(BMColors::DARK_GRAY)),
    ]);

    let rect = Rect {
        x: 1,
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
