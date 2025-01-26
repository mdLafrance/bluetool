use ratatui::style::Color;

pub struct BMColors;

impl BMColors {
    pub const BLUE: Color = Color::Rgb(0, 127, 245);
    pub const RED: Color = Color::Rgb(245, 20, 30);
    pub const ORANGE: Color = Color::Rgb(245, 145, 30);
    pub const YELLOW: Color = Color::Rgb(100, 245, 30);
    pub const GREEN: Color = Color::Rgb(0, 245, 10);
}
