use ratatui::{
    layout::Rect,
    style::{Style, Stylize},
    widgets::Paragraph,
    Frame,
};

use crate::app::BannerType;

use super::{colors::BMColors, UIState};

pub fn draw_banner(f: &mut Frame, area: Rect, ui_state: &mut UIState) {
    if let Some(banner) = &mut ui_state.banner {
        let banner_icon = match banner.1 {
            BannerType::Success => " 󰂱 ",
            BannerType::Failure => "  ",
            BannerType::Status => "  ",
        };

        let banner_style = match banner.1 {
            BannerType::Success => Style::new().fg(BMColors::GREEN),
            BannerType::Failure => Style::new().white().bg(BMColors::RED),
            BannerType::Status => Style::new().white().bg(BMColors::DARK_GRAY),
        };

        let p = Paragraph::new(format!("{}{}", banner_icon, banner.0.clone())).style(banner_style);

        f.render_widget(p, area);
    }
}
