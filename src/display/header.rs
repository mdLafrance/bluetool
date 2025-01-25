use ratatui::{
    layout::Rect,
    style::{Style, Stylize},
    text::{Line, Span},
    Frame,
};

use super::UIState;

pub fn draw_title(f: &mut Frame, area: Rect, ui_state: &mut UIState) {
    let title = Line::from(vec![
        Span::styled("", Style::new().blue()),
        Span::styled(" Blueman 󰂯 ", Style::new().bold().white().on_blue()),
        Span::styled("", Style::new().blue()),
        Span::raw(" "),
        Span::styled(
            format!("v{}", env!("CARGO_PKG_VERSION")),
            Style::new().dim(),
        ),
        Span::raw(" "),
    ]);

    let rect = Rect {
        x: 2,
        y: 0,
        width: title.width() as u16,
        height: 1,
    };

    f.render_widget(title, rect);
}
