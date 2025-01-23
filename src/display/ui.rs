use std::{
    collections::HashSet,
    error::Error,
    io::{stdout, Stdout},
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
        Layout, Rect,
    },
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};

use super::table::draw_table;

pub struct UIState<'a> {
    pub devices: &'a HashSet<Device>,
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

pub fn draw_ui(f: &mut Frame, ui_state: &mut UIState) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(1), Constraint::Percentage(99)])
        .split(f.area());

    let header_area = layout[0];
    let table_area = layout[1];

    draw_header(f, header_area);
    draw_table(f, table_area, ui_state);
}

//pub fn draw(
//    state: &mut UIState,
//    data: &SystemData,
//    poll_data: &RingBuffer<SystemPollResult>,
//    f: &mut Frame,
//) {
//    let l = Layout::default()
//        .constraints(vec![Constraint::Length(2), Constraint::Percentage(99)])
//        .split(f.size());
//
//    let (header_area, area) = (l[0], l[1]);
//
//    draw_header(state, f, header_area);
//
//    let content_layout = Layout::default()
//        .direction(Direction::Horizontal)
//        .constraints(vec![Constraint::Length(45), Constraint::Percentage(99)])
//        .split(area);
//
//    let (layout_l, layout_r) = (content_layout[0], content_layout[1]);
//
//    let sys_information_layout = Layout::default()
//        .direction(Direction::Vertical)
//        .constraints(vec![Constraint::Length(10), Constraint::Percentage(99)])
//        .split(layout_l);
//
//    let (sysinfo_layout, left_area) = (sys_information_layout[0], sys_information_layout[1]);
//
//    // Split left layout
//    let left_layout = Layout::default()
//        .direction(Direction::Vertical)
//        .constraints(vec![Constraint::Length(5), Constraint::Percentage(99)])
//        .split(left_area);
//
//    let (memory_area, gpu_area) = (left_layout[0], left_layout[1]);
//
//    draw_sys_info(&data.info, f, sysinfo_layout);
//
//    // Draw right side
//    let p = poll_data.last().expect("No poll data could be read.");
//
//    // Split right side
//    let right_layout = Layout::default()
//        .direction(Direction::Vertical)
//        .constraints(vec![
//            Constraint::Length(3),
//            Constraint::Length(3),
//            Constraint::Percentage(99),
//        ])
//        .split(layout_r);
//
//    let (cpu_temp_area, cpu_average_area, cpu_usage_area) =
//        (right_layout[0], right_layout[1], right_layout[2]);
//
//    draw_cpu_temp_block(&p.cpu_temperature, f, cpu_temp_area);
//    draw_cpu_average_block(&p.cpu_usage, f, cpu_average_area);
//    draw_cpu_usage_block(&p.cpu_usage, f, cpu_usage_area);
//    draw_memory_usage_block(
//        data.info.total_memory as f32,
//        p.memory_usage.value,
//        f,
//        memory_area,
//    );
//    draw_gpu_info_block(&p.gpu_info, f, gpu_area);
//}

///// Draws the header which sits at the top of the ui.

/// The header contains a title, version information, and tab information.
/// The header also contains current keybinds.
fn draw_header(f: &mut Frame, area: Rect) {
    // Draw header bg and outer styling elements
    let header_block = Block::default().borders(Borders::BOTTOM);

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
        Span::styled("Blueman 󰂯", Style::new().bold().white().on_blue()),
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
