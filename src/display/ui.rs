use std::{
    cell::RefCell,
    error::Error,
    io::{stdout, Stdout},
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::Result;
use bluer::Device;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use ratatui::{
    backend::CrosstermBackend,
    layout::{
        Constraint,
        Direction::{self, Horizontal},
        Layout, Margin, Rect,
    },
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, TableState},
    Frame, Terminal,
};

use crate::app::{BMMode, BTDevice, Banner, BannerType};

use super::{banner::draw_banner, table::draw_table};

pub struct UIState {
    pub devices: Rc<RefCell<Vec<BTDevice>>>,
    pub banner: Option<Banner>,
    pub table_state: TableState,
    pub show_unnamed: bool,
}

/// Setup the necessary components to make terminal ui calls.
pub fn init_ui() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout()))?)
}

/// Teardown ui components, and release the terminal back to the user.
pub fn shutdown_ui() -> Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

pub fn draw_ui(f: &mut Frame, ui_state: &mut UIState, mode: BMMode) {
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

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::new().blue());

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(99), Constraint::Length(1)])
        .vertical_margin(1)
        .horizontal_margin(1)
        .split(block.inner(f.area()));

    f.render_widget(block, f.area());
    f.render_widget(title, rect);

    let table_area = layout[0];
    let banner_area = layout[1];

    // draw_header(f, header_area);
    draw_table(f, table_area, ui_state);
    draw_banner(f, banner_area, ui_state);

    match mode {
        BMMode::TryConnect(d) => draw_try_connect_panel(f, d),
        BMMode::TryDisconnect(d) => draw_try_disconnect_panel(f, d),
        _ => {}
    }
}

/// The header contains a title, version information, and tab information.
/// The header also contains current keybinds.
fn draw_header(f: &mut Frame, area: Rect) {
    // Draw header bg and outer styling elements
    let header_block = Block::default()
        .borders(Borders::BOTTOM)
        .border_type(BorderType::Thick)
        .style(Style::new().dark_gray());

    let header_area = header_block.inner(area);

    f.render_widget(header_block, area);

    // Split layout
    let l = Layout::default()
        .direction(Horizontal)
        .constraints(vec![Constraint::Percentage(50), Constraint::Length(5)])
        .split(header_area);

    let (title_area, hints_area) = (l[0], l[1]);

    let title = Paragraph::new(vec![Line::from(vec![
        Span::styled("", Style::new().blue()),
        Span::styled(" Blueman 󰂯 ", Style::new().bold().white().on_blue()),
        Span::styled("", Style::new().blue()),
        Span::raw(" "),
        Span::styled(
            format!("v{}", env!("CARGO_PKG_VERSION")),
            Style::new().dim(),
        ),
    ])])
    .alignment(ratatui::layout::Alignment::Left);

    f.render_widget(title, title_area);
}

fn draw_try_connect_panel(f: &mut Frame, d: BTDevice) {
    let area = f.area();

    let block_width = 60;
    let block_height = 5;

    let x = (area.width.saturating_sub(block_width)) / 2;
    let y = (area.height.saturating_sub(block_height)) / 2;

    // Define the centered block's area.
    let centered_area = Rect::new(x, y, block_width, block_height);

    // Create a block with borders and a title.
    let block = Block::default()
        .title("Centered Block")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White).bg(Color::Blue));

    let dots = ".".repeat(get_anim_frame() % 3).to_string();
    let span = Span::styled(format!("Connecting to {}{}", d.name, dots), Style::new());

    let p = Paragraph::new(span).block(block);

    // Render the block in the centered area.
    f.render_widget(p, centered_area);
}

fn get_anim_frame() -> usize {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
}

fn draw_try_disconnect_panel(f: &mut Frame, d: BTDevice) {
    let area = f.area();

    let block_width = 60;
    let block_height = 5;

    let x = (area.width.saturating_sub(block_width)) / 2;
    let y = (area.height.saturating_sub(block_height)) / 2;

    // Define the centered block's area.
    let centered_area = Rect::new(x, y, block_width, block_height);

    // Create a block with borders and a title.
    let block = Block::default()
        .title("Centered Block")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White).bg(Color::Blue));

    let dots = ".".repeat(get_anim_frame() % 3).to_string();
    let span = Span::styled(
        format!("Disconnecting from {}{}", d.name, dots),
        Style::new(),
    );

    let p = Paragraph::new(span).block(block);

    // Render the block in the centered area.
    f.render_widget(p, centered_area);
}
