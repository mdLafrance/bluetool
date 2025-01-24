use std::{borrow::BorrowMut, cell::RefCell, error::Error, rc::Rc, sync::Arc, thread::current};

use anyhow::Result;
use bluer::Device;
use color_eyre::owo_colors::OwoColorize;
use crossterm::event::{Event, EventStream, KeyCode};
use futures::StreamExt;
use ratatui::widgets::TableState;
use scopeguard::defer;
use tokio::{
    sync::mpsc::{channel, Receiver, Sender},
    task::JoinHandle,
    time::{self, sleep, Duration, Sleep},
};

use crate::{
    app::{bluetooth::launch_bluetooth_listener, input::launch_key_listener},
    display::{draw_ui, init_ui, shutdown_ui, UIState},
};

use super::bluetooth::BTDevice;

#[derive(Debug)]
pub enum BMEvent {
    Pass,
    Exit,
    ScrollDown,
    ScrollUp,
    DeviceAdded(BTDevice),
    DeviceRemoved(BTDevice),
    DeviceModified(BTDevice),
    BannerExpired(String),
    SwitchToMode(BMMode),
}

#[derive(Clone)]
pub enum BannerType {
    Success,
    Failure,
}

#[derive(Clone)]
pub struct Banner(pub String, pub BannerType);

#[derive(Debug)]
pub enum BMMode {
    Browse,
    TryConnect(BTDevice),
}

pub struct BluemanApp {
    devices: Rc<RefCell<Vec<BTDevice>>>,
    event_recv_chan: Receiver<BMEvent>,
    event_send_chan: Arc<Sender<BMEvent>>,
    mode: BMMode,
    banner: Option<Banner>,
}

impl BluemanApp {
    /// Instantiate an instance of the app object
    pub fn new() -> Self {
        let (send, recv) = channel(128);
        BluemanApp {
            devices: Rc::new(RefCell::new(Vec::with_capacity(64))),
            event_recv_chan: recv,
            event_send_chan: Arc::new(send),
            mode: BMMode::Browse,
            banner: None,
        }
    }

    /// Get a new handle to the event send bus
    pub fn get_event_chan_handle(&self) -> Arc<Sender<BMEvent>> {
        self.event_send_chan.clone()
    }

    pub async fn run(&mut self) -> Result<()> {
        let mut terminal = init_ui()?;
        let mut ui_state = UIState {
            devices: self.devices.clone(),
            table_state: TableState::new(),
            banner: None,
        };

        defer! {
            shutdown_ui().unwrap();
        }

        let key_listener = launch_key_listener(self.get_event_chan_handle());
        let bluetooth_listener = launch_bluetooth_listener(self.get_event_chan_handle()).await;

        // NOTE: Send one dummy event so we trigger a draw
        self.event_send_chan.send(BMEvent::Pass).await?;

        // Main loop, listen for events and draw ui
        loop {
            if let Some(e) = self.event_recv_chan.recv().await {
                // Process mode-independent events
                match &e {
                    BMEvent::Exit => break,
                    BMEvent::BannerExpired(msg) => {
                        if let Some(current_banner) = &mut self.banner {
                            if &current_banner.0 == msg {
                                self.banner = None;
                            }
                        }
                    }
                    _ => {}
                };

                match &self.mode {
                    BMMode::Browse => match e {
                        BMEvent::Exit => break,
                        BMEvent::ScrollUp => match ui_state.table_state.selected() {
                            Some(1) => *ui_state.table_state.selected_mut() = None,
                            Some(idx) => *ui_state.table_state.selected_mut() = Some(idx - 1),
                            _ => {}
                        },
                        BMEvent::ScrollDown => match ui_state.table_state.selected() {
                            None => *ui_state.table_state.selected_mut() = Some(1),
                            Some(idx) => *ui_state.table_state.selected_mut() = Some(idx + 1),
                        },
                        BMEvent::DeviceAdded(device) => {
                            let mut devices = self.devices.as_ref().borrow_mut();

                            if !devices.contains(&device) {
                                devices.push(device);
                            }

                            devices.sort_by(|a, b| b.cmp(a));
                        }
                        BMEvent::DeviceRemoved(device) => {
                            let mut devices = self.devices.as_ref().borrow_mut();
                            devices.retain(|d| d != &device);
                        }
                        BMEvent::DeviceModified(device) => {
                            let mut devices = self.devices.as_ref().borrow_mut();
                            let device_mac = device.address.clone();

                            for d in devices.iter_mut() {
                                if d.address == device_mac {
                                    *d = device.clone();
                                }
                            }
                        }
                        _ => continue,
                    },

                    BMMode::TryConnect(device) => match device.connect().await {
                        Ok(_) => {
                            self.event_send_chan
                                .send(BMEvent::SwitchToMode(BMMode::Browse))
                                .await?;

                            let b = Banner(
                                format!("Successfully connected to {}", device.name),
                                BannerType::Success,
                            );
                            self.set_new_banner(b).await;
                        }
                        Err(e) => {
                            let b = Banner(
                                format!("Failed to connect to {}: {}", device.name, e.to_string()),
                                BannerType::Failure,
                            );
                            self.set_new_banner(b).await
                        }
                    },
                }

                ui_state.banner = self.banner.clone();
                terminal.draw(|f| draw_ui(f, &mut ui_state))?;
            } else {
                break;
            }
        }

        key_listener.abort();
        bluetooth_listener.abort();

        Ok(())
    }

    async fn set_new_banner(&mut self, b: Banner) {
        let chan = self.get_event_chan_handle();

        self.banner = Some(b.clone());

        tokio::spawn(async move {
            sleep(Duration::from_millis(3000)).await;
            chan.send(BMEvent::BannerExpired(b.0)).await.unwrap();
        });
    }
}
