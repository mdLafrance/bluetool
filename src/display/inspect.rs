use bluer::Uuid;
use ratatui::{
    layout::Rect,
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, Padding, Paragraph, Widget, Wrap},
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
    lines.push(Line::raw(format!(
        "Class: {}",
        device
            .inner
            .class()
            .await
            .unwrap_or(None)
            .unwrap_or_default()
    )));

    let mut uuids = device
        .inner
        .uuids()
        .await
        .unwrap_or(None)
        .unwrap_or_default()
        .into_iter()
        .collect::<Vec<Uuid>>();
    uuids.sort();

    lines.push(Line::raw(format!(
        "UUIDS: {}",
        uuids
            .iter()
            .map(|u| u.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    )));

    lines.push(Line::raw(format!("Paired: {}", device.paired)));
    lines.push(Line::raw(format!("Connected: {}", device.connected)));
    lines.push(Line::raw(format!(
        "RSSI: {}",
        device
            .inner
            .rssi()
            .await
            .unwrap_or(None)
            .unwrap_or_default()
    )));
    lines.push(Line::raw(format!(
        "TX Power: {}",
        device
            .inner
            .tx_power()
            .await
            .unwrap_or(None)
            .unwrap_or_default()
    )));
    // lines.push(Line::raw(format!(
    //     "Manufacturer Data",
    //     device
    //         .inner
    //         .manufacturer_data()
    //         .await
    //         .unwrap_or(None)
    //         .unwrap_or("???".to_string())
    // )));
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
    if let Some(p) = &ui_state.inspect_text {
        let b = Block::new()
            .padding(Padding {
                left: 4,
                right: 4,
                top: 1,
                bottom: 1,
            })
            .title(format!(" Device: {} ", device.name))
            .title_style(Style::new().bold().white())
            .borders(Borders::ALL)
            .border_style(Style::new().fg(BMColors::BLUE))
            .border_type(BorderType::Rounded);

        let paragraph = p.clone().block(b).wrap(Wrap { trim: true });

        f.render_widget(paragraph, area);
    }
}
