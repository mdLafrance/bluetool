use ratatui::{
    layout::Rect,
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
    Frame,
};

use crate::app::BTDevice;

use super::{colors::BMColors, UIState};

pub async fn format_inspect_text(device: BTDevice) -> Paragraph<'static> {
    let mut lines: Vec<Line> = vec![];

    lines.push(Line::raw(format!(
        "Address type: {}",
        device
            .inner
            .address_type()
            .await
            .map_or_else(|_| "???".to_string(), |a| a.to_string()),
    )));
    lines.push(Line::raw(format!(
        "Name: {}",
        device
            .inner
            .name()
            .await
            .unwrap_or(None)
            .unwrap_or("???".to_string())
    )));
    // println!("Name: {:?}", device.inner.name().await?);
    // println!("Icon: {:?}", device.inner.icon().await?);
    // println!("Class: {:?}", device.inner.class().await?);
    // println!(
    //     "UUIDs: {:?}",
    //     device.inner.uuids().await?.unwrap_or_default()
    // );
    // println!("Paired: {:?}", device.inner.is_paired().await?);
    // println!("Connected: {:?}", device.inner.is_connected().await?);
    // println!("Trusted: {:?}", device.inner.is_trusted().await?);
    // println!("Modalias: {:?}", device.inner.modalias().await?);
    // println!("RSSI: {:?}", device.inner.rssi().await?);
    // println!("TX power: {:?}", device.inner.tx_power().await?);
    // println!(
    //     "Manufacturer data: {:?}",
    //     device.inner.manufacturer_data().await?
    // );
    // println!("Service data: {:?}", device.inner.service_data().await?);

    Paragraph::new(lines)
}

pub fn draw_inspect_panel(
    f: &mut Frame<'_>,
    area: Rect,
    ui_state: &mut UIState,
    device: &BTDevice,
) {
    let b = Block::new()
        .title(format!(" Device: {} ", device.name))
        .title_style(Style::new().bold().white())
        .borders(Borders::ALL)
        .border_style(Style::new().fg(BMColors::BLUE))
        .border_type(BorderType::Rounded);

    if let Some(p) = &ui_state.inspect_text {
        f.render_widget(p, area);
    }
}
