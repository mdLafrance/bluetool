use ratatui::{
    layout::Rect,
    style::{Style, Stylize},
    text::Span,
    widgets::{
        block::{self, BorderType},
        Block, Borders, Paragraph,
    },
    Frame,
};

use crate::app::BannerType;

use super::UIState;

pub fn draw_banner(f: &mut Frame, area: Rect, ui_state: &mut UIState) {
    if let Some(banner) = &mut ui_state.banner {
        let banner_icon = match banner.1 {
            BannerType::Success => " 󰂱 ",
            BannerType::Failure => "  ",
            BannerType::Status => "  ",
        };

        let banner_style = match banner.1 {
            BannerType::Success => Style::new().green(),
            BannerType::Failure => Style::new().white().on_red(),
            BannerType::Status => Style::new().white().on_dark_gray(),
        };

        let p = Paragraph::new(format!("{}{}", banner_icon, banner.0.clone())).style(banner_style);

        f.render_widget(p, area);
    }
}
