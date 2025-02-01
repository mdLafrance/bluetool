use std::{cell::RefCell, rc::Rc, sync::Arc};

use anyhow::Result;
use ratatui::widgets::TableState;
use scopeguard::defer;
use tokio::{
    sync::mpsc::{channel, Receiver, Sender},
    time::{sleep, Duration},
};

use crate::{
    app::{bluetooth::launch_bluetooth_listener, input::launch_key_listener},
    display::{draw_ui, format_inspect_text, init_ui, shutdown_ui, UIState},
};

use super::bluetooth::BTDevice;

pub enum AppEvent {
    Pass,
    Exit,
    Esc,
    ScrollDown,
    ScrollUp,
    DeviceAdded(BTDevice),
    DeviceRemoved(BTDevice),
    DeviceModified(BTDevice),
    BannerExpired(String),
    ConnectRequested,
    PairRequested,
    DisconnectRequested,
    RemoveRequested,
    ShowHideUnnamed,
    InspectCurrent,
}

#[derive(Clone)]
pub enum BannerType {
    Success,
    Failure,
    Status,
}

#[derive(Clone)]
pub struct Banner(pub String, pub BannerType);

#[derive(Debug, Clone)]
pub enum AppMode {
    Browse,
    Inspect(BTDevice),
    TryConnect(BTDevice),
    TryPair(BTDevice),
    TryDisconnect(BTDevice),
    TryRemove(BTDevice),
}

pub struct BTUIApp {
    devices: Rc<RefCell<Vec<BTDevice>>>,
    event_recv_chan: Receiver<AppEvent>,
    event_send_chan: Arc<Sender<AppEvent>>,
    mode: AppMode,
    banner: Option<Banner>,
}

impl BTUIApp {
    /// Instantiate an instance of the app object
    pub fn new() -> Self {
        let (send, recv) = channel(128);
        BTUIApp {
            devices: Rc::new(RefCell::new(Vec::with_capacity(64))),
            event_recv_chan: recv,
            event_send_chan: Arc::new(send),
            mode: AppMode::Browse,
            banner: None,
        }
    }

    /// Get a new handle to the event send bus
    pub fn get_event_chan_handle(&self) -> Arc<Sender<AppEvent>> {
        self.event_send_chan.clone()
    }

    pub async fn run(&mut self) -> Result<()> {
        let mut terminal = init_ui()?;
        let mut ui_state = UIState {
            devices: self.devices.clone(),
            table_state: TableState::new(),
            banner: None,
            show_unnamed: false,
            inspect_text: None,
        };

        defer! {
            shutdown_ui().unwrap();
        }

        let key_listener = launch_key_listener(self.get_event_chan_handle());
        let bluetooth_listener = launch_bluetooth_listener(self.get_event_chan_handle()).await;

        // NOTE: Send one dummy event so we trigger a draw
        self.event_send_chan.send(AppEvent::Pass).await?;

        // Main loop, listen for events and draw ui
        loop {
            if let Some(e) = self.event_recv_chan.recv().await {
                // Process mode-independent events
                match &e {
                    AppEvent::Exit => break,
                    AppEvent::Esc => {
                        self.mode = AppMode::Browse;
                        ui_state.inspect_text = None;
                    }
                    AppEvent::BannerExpired(msg) => {
                        if let Some(current_banner) = &mut self.banner {
                            if &current_banner.0 == msg {
                                self.banner = None;
                            }
                        }
                    }
                    _ => {}
                };

                match self.mode.clone() {
                    AppMode::Browse => match e {
                        AppEvent::Exit => break,
                        AppEvent::ScrollUp => match ui_state.table_state.selected() {
                            Some(1) => *ui_state.table_state.selected_mut() = None,
                            Some(idx) => *ui_state.table_state.selected_mut() = Some(idx - 1),
                            _ => {}
                        },
                        AppEvent::ScrollDown => match ui_state.table_state.selected() {
                            None => {
                                if self.devices.as_ref().borrow().len() != 0 {
                                    *ui_state.table_state.selected_mut() = Some(1);
                                }
                            }
                            Some(idx) => *ui_state.table_state.selected_mut() = Some(idx + 1),
                        },
                        AppEvent::DeviceAdded(device) => {
                            let mut devices = self.devices.as_ref().borrow_mut();

                            if !devices.contains(&device) {
                                devices.push(device);
                            }

                            devices.sort_by(|a, b| b.cmp(a));
                        }
                        AppEvent::DeviceRemoved(device) => {
                            let mut devices = self.devices.as_ref().borrow_mut();
                            devices.retain(|d| d != &device);
                        }
                        AppEvent::DeviceModified(device) => {
                            let mut devices = self.devices.as_ref().borrow_mut();
                            let device_mac = device.address.clone();

                            for d in devices.iter_mut() {
                                if d.address == device_mac {
                                    *d = device.clone();
                                }
                            }

                            devices.sort_by(|a, b| b.cmp(a));
                        }
                        AppEvent::ConnectRequested => {
                            // Find which device we're highlighting
                            if let Some(idx) = ui_state.table_state.selected() {
                                let device = self.devices.as_ref().borrow()[idx - 1].clone();

                                self.mode = AppMode::TryConnect(device);
                            }
                        }
                        AppEvent::PairRequested => {
                            // Find which device we're highlighting
                            if let Some(idx) = ui_state.table_state.selected() {
                                let device = self.devices.as_ref().borrow()[idx - 1].clone();

                                self.mode = AppMode::TryPair(device);
                            }
                        }
                        AppEvent::DisconnectRequested => {
                            // Find which device we're highlighting
                            if let Some(idx) = ui_state.table_state.selected() {
                                let device = self.devices.as_ref().borrow()[idx - 1].clone();

                                self.mode = AppMode::TryDisconnect(device);
                            }
                        }
                        AppEvent::RemoveRequested => {
                            // Find which device we're highlighting
                            if let Some(idx) = ui_state.table_state.selected() {
                                let device = self.devices.as_ref().borrow()[idx - 1].clone();

                                self.mode = AppMode::TryRemove(device);
                            }
                        }
                        AppEvent::ShowHideUnnamed => {
                            ui_state.show_unnamed = !ui_state.show_unnamed;
                        }
                        AppEvent::InspectCurrent => {
                            if let Some(idx) = ui_state.table_state.selected() {
                                let device = self.devices.as_ref().borrow()[idx - 1].clone();
                                self.mode = AppMode::Inspect(device);
                                self.event_send_chan.send(AppEvent::Pass).await?;
                            }
                        }
                        _ => {}
                    },
                    AppMode::Inspect(device) => {
                        ui_state.inspect_text = Some(format_inspect_text(device.clone()).await);
                    }
                    AppMode::TryConnect(device) => {
                        if device.connected {
                            let b = Banner(
                                format!("{} already connected", device.name),
                                BannerType::Status,
                            );
                            self.set_new_banner(b).await;
                            self.mode = AppMode::Browse;
                            continue;
                        } else {
                            let res = device.connect().await;

                            match res {
                                Ok(_) => {
                                    let b = Banner(
                                        format!("Successfully connected to {}", device.name),
                                        BannerType::Success,
                                    );
                                    self.set_new_banner(b).await;

                                    self.mode = AppMode::Browse;
                                }
                                Err(e) => {
                                    let b = Banner(
                                        format!(
                                            "Failed to connect to {}: {}",
                                            device.name,
                                            e.to_string()
                                        ),
                                        BannerType::Failure,
                                    );
                                    self.set_new_banner(b).await;

                                    self.mode = AppMode::Browse;
                                }
                            }
                        }
                    }

                    AppMode::TryPair(device) => {
                        if device.paired {
                            let b = Banner(
                                format!("{} already paired", device.name),
                                BannerType::Status,
                            );
                            self.set_new_banner(b).await;
                            self.mode = AppMode::Browse;
                            continue;
                        } else {
                            let res = device.pair().await;

                            match res {
                                Ok(_) => {
                                    let b = Banner(
                                        format!("Successfully paired with {}", device.name),
                                        BannerType::Success,
                                    );
                                    self.set_new_banner(b).await;

                                    self.mode = AppMode::Browse;
                                }
                                Err(e) => {
                                    let b = Banner(
                                        format!(
                                            "Failed to pair with {}: {}",
                                            device.name,
                                            e.to_string()
                                        ),
                                        BannerType::Failure,
                                    );
                                    self.set_new_banner(b).await;

                                    self.mode = AppMode::Browse;
                                }
                            }
                        }
                    }

                    AppMode::TryDisconnect(device) => {
                        if !device.connected {
                            let b = Banner(
                                format!("{} is not connected", device.name),
                                BannerType::Status,
                            );
                            self.set_new_banner(b).await;
                            self.mode = AppMode::Browse;
                            continue;
                        } else {
                            let res = device.disconnect().await;

                            match res {
                                Ok(_) => {
                                    let b = Banner(
                                        format!("Successfully disconnected from {}", device.name),
                                        BannerType::Success,
                                    );
                                    self.set_new_banner(b).await;

                                    self.mode = AppMode::Browse;
                                }
                                Err(e) => {
                                    let b = Banner(
                                        format!(
                                            "Failed to disconnect from {}: {}",
                                            device.name,
                                            e.to_string()
                                        ),
                                        BannerType::Failure,
                                    );
                                    self.set_new_banner(b).await;

                                    self.mode = AppMode::Browse;
                                }
                            }
                        }
                    }

                    AppMode::TryRemove(device) => {
                        if !device.paired {
                            let b = Banner(
                                format!("{} is not paired", device.name),
                                BannerType::Status,
                            );
                            self.set_new_banner(b).await;
                            self.mode = AppMode::Browse;
                            continue;
                        } else {
                            let res = device.remove().await;

                            match res {
                                Ok(_) => {
                                    let b = Banner(
                                        format!("Successfully removed device {}", device.name),
                                        BannerType::Success,
                                    );
                                    self.set_new_banner(b).await;

                                    self.mode = AppMode::Browse;
                                }
                                Err(e) => {
                                    let b = Banner(
                                        format!(
                                            "Failed to remove device {}: {}",
                                            device.name,
                                            e.to_string()
                                        ),
                                        BannerType::Failure,
                                    );
                                    self.set_new_banner(b).await;

                                    self.mode = AppMode::Browse;
                                }
                            }
                        }
                    }
                }

                ui_state.banner = self.banner.clone();
                terminal.draw(|f| draw_ui(f, &mut ui_state, self.mode.clone()))?;
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

        let duration = match b.1 {
            BannerType::Failure => Duration::from_secs(4),
            _ => Duration::from_secs(3),
        };

        self.banner = Some(b.clone());

        tokio::spawn(async move {
            sleep(duration).await;
            chan.send(AppEvent::BannerExpired(b.0)).await.unwrap();
        });
    }
}
