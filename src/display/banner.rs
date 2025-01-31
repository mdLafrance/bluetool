use ratatui::{
    layout::Rect,
    style::{Style, Stylize},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::app::BannerType;

use super::{colors::BMColors, UIState};

pub fn draw_banner(f: &mut Frame, ui_state: &mut UIState) {
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

        let p = Paragraph::new(format!("{}{}", banner_icon, banner.0.clone()))
            .style(banner_style)
            .block(
                Block::new()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::new().white()),
            );

        let area = f.area();

        let horizontal_padding = 5;

        let r = Rect {
            x: horizontal_padding,
            y: area.height / 2 - 1,
            width: area.width - 2 * horizontal_padding,
            height: 3,
        };

        f.render_widget(p, r);
    }
}
