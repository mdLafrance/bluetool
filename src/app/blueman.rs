use std::{collections::HashSet, sync::Arc};

use anyhow::Result;
use bluer::Device;
use crossterm::event::{Event, EventStream, KeyCode};
use futures::StreamExt;
use scopeguard::defer;
use tokio::{
    sync::mpsc::{channel, Receiver, Sender},
    task::JoinHandle,
};

use crate::display::{draw_ui, init_ui, shutdown_ui, UIState};

#[derive(Debug)]
pub enum BMEvent {
    Exit,
    DeviceAdded(Device),
    DeviceRemoved(Device),
    DeviceModified(Device),
}

pub struct BluemanApp {
    devices: HashSet<Device>,
    event_recv_chan: Receiver<BMEvent>,
    event_send_chan: Arc<Sender<BMEvent>>,
}

impl BluemanApp {
    /// Instantiate an instance of the app object
    pub fn new() -> Self {
        let (send, recv) = channel(128);
        BluemanApp {
            devices: HashSet::new(),
            event_recv_chan: recv,
            event_send_chan: Arc::new(send),
        }
    }

    /// Get a new handle to the event send bus
    pub fn get_event_chan_handle(&self) -> Arc<Sender<BMEvent>> {
        self.event_send_chan.clone()
    }

    pub async fn run(&mut self) -> Result<()> {
        let mut terminal = init_ui()?;
        let mut ui_state = UIState {
            devices: &self.devices,
        };

        defer! {
            shutdown_ui().unwrap();
        }

        // Launch listener tasks that will forward key and bluetooth events
        // to the event chan.
        // NOTE: These threads will exit themselves when the event chan is
        // closed.
        self.launch_key_listener();
        self.launch_bt_event_listener();

        // Main loop, listen for events and draw ui
        loop {
            match self.event_recv_chan.try_recv() {
                Ok(BMEvent::Exit) => break,
                _ => {}
            };

            terminal.draw(|f| draw_ui(f, &mut ui_state))?;
        }

        Ok(())
    }

    /// Launches a subprocess which asynchronously listens for input events,
    /// mashalls them, and forwards them to the send channel.
    fn launch_key_listener(&self) -> JoinHandle<()> {
        let mut event_stream = EventStream::new();
        let event_chan = self.get_event_chan_handle();

        tokio::spawn(async move {
            'event_loop: loop {
                match event_stream.next().await {
                    Some(Ok(Event::Key(evnt))) => match evnt.code {
                        // Quit key
                        KeyCode::Char('q') => {
                            event_chan.send(BMEvent::Exit).await.unwrap();
                        }
                        _ => (),
                    },
                    _ => break 'event_loop,
                };
            }
        })
    }

    /// Launches a subprocess which asynchronously listens for bluetooth
    /// events. These events can include device connections, disconnections,
    /// and modifications.
    ///
    /// These events are marshalled, and then forwarded to the send channel.
    fn launch_bt_event_listener(&self) -> JoinHandle<()> {
        tokio::spawn(async {})
    }
}
