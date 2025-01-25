use ratatui::{
    layout::Rect,
    style::{Style, Stylize},
    text::{Line, Span},
    Frame,
};

use super::UIState;

pub fn draw_controls(f: &mut Frame, area: Rect, ui_state: &mut UIState) {
    let controls = Line::from(vec![
        Span::styled("┤ ", Style::new().blue()),
        Span::styled("[jk] ", Style::new().bold().dark_gray()),
        Span::styled("select", Style::new().dark_gray()),
        Span::styled(" | ", Style::new().blue()),
        Span::styled("[c] ", Style::new().bold().dark_gray()),
        Span::styled("connect", Style::new().dark_gray()),
        Span::styled(" | ", Style::new().blue()),
        Span::styled("[d] ", Style::new().bold().dark_gray()),
        Span::styled("disconnect", Style::new().dark_gray()),
        Span::styled(" | ", Style::new().blue()),
        Span::styled("[h] ", Style::new().bold().dark_gray()),
        Span::styled(
            format!(
                "{} unnamed",
                if ui_state.show_unnamed {
                    "hide"
                } else {
                    "show"
                }
            ),
            Style::new().dark_gray(),
        ),
        Span::styled(" ├", Style::new().blue()),
    ]);

    let rect = Rect {
        x: 2,
        y: area.height - 1,
        width: controls.width() as u16,
        height: 1,
    };

    f.render_widget(controls, rect);
}

pub fn draw_quit_hint(f: &mut Frame, area: Rect, ui_state: &mut UIState) {
    let quit_hint = Line::from(vec![
        Span::styled("┤ ", Style::new().blue()),
        Span::styled("[q] ", Style::new().bold().dark_gray()),
        Span::styled("quit", Style::new().dark_gray()),
        Span::styled(" ├", Style::new().blue()),
    ]);

    let x = area.width as u16 - 2 - quit_hint.width() as u16;

    let rect = Rect {
        x,
        y: area.height - 1,
        width: quit_hint.width() as u16,
        height: 1,
    };

    f.render_widget(quit_hint, rect);
}
