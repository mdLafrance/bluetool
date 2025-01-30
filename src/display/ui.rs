use std::{
    cell::RefCell,
    io::{stdout, Stdout},
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::Result;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use ratatui::{
    backend::CrosstermBackend,
    layout::{
        Constraint,
        Direction::{self},
        Layout, Rect,
    },
    style::{Color, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Paragraph, TableState},
    Frame, Terminal,
};

use crate::app::{AppMode, BTDevice, Banner};

use super::{
    banner::draw_banner,
    colors::BMColors,
    controls::{draw_controls, draw_quit_hint},
    header::draw_header,
    inspect::draw_inspect_panel,
    table::draw_table,
};

pub struct UIState<'a> {
    pub devices: Rc<RefCell<Vec<BTDevice>>>,
    pub banner: Option<Banner>,
    pub table_state: TableState,
    pub show_unnamed: bool,
    pub inspect_scroll: u16,
    pub inspect_text: Option<Paragraph<'a>>,
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

pub fn draw_ui(f: &mut Frame<'_>, ui_state: &mut UIState<'_>, mode: AppMode) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(2),
            Constraint::Percentage(99),
            Constraint::Length(1),
        ])
        .split(f.area());

    let header_area = layout[0];
    let table_area = layout[1];
    let banner_area = layout[2];

    draw_header(f, header_area, ui_state);
    draw_quit_hint(f, f.area(), ui_state);

    draw_banner(f, banner_area, ui_state);

    match &mode {
        AppMode::Inspect(d) => {
            draw_inspect_panel(f, table_area, ui_state, d);
        }
        _ => {
            draw_table(f, table_area, ui_state);
            draw_controls(f, f.area(), ui_state);
        }
    }

    match mode {
        AppMode::TryConnect(d) => draw_try_connect_panel(f, d),
        AppMode::TryDisconnect(d) => draw_try_disconnect_panel(f, d),
        _ => {}
    }
}

fn draw_try_connect_panel(f: &mut Frame, d: BTDevice) {
    let area = f.area();

    let block_width = 60;
    let block_height = 3;

    let x = (area.width.saturating_sub(block_width)) / 2;
    let y = (area.height.saturating_sub(block_height)) / 2;

    // Define the centered block's area.
    let centered_area = Rect::new(x, y, block_width, block_height);

    // Create a block with borders and a title.
    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White).bg(Color::Blue));

    let span = Span::styled(format!("Connecting to {}", d.name), Style::new());

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
