use futures::StreamExt;
use std::sync::Arc;

use crossterm::event::{Event, EventStream, KeyCode};
use tokio::{sync::mpsc::Sender, task::JoinHandle};

use super::blueman::BMEvent;

pub fn launch_key_listener(event_send_chan: Arc<Sender<BMEvent>>) -> JoinHandle<()> {
    let mut event_stream = EventStream::new();

    tokio::spawn(async move {
        'event_loop: loop {
            match event_stream.next().await {
                Some(Ok(Event::Key(evnt))) => match evnt.code {
                    // Quit key
                    KeyCode::Char('q') => {
                        event_send_chan.send(BMEvent::Exit).await.unwrap();
                    }
                    KeyCode::Char('j') | KeyCode::Down => {
                        event_send_chan.send(BMEvent::ScrollDown).await.unwrap();
                    }
                    KeyCode::Char('k') | KeyCode::Up => {
                        event_send_chan.send(BMEvent::ScrollUp).await.unwrap();
                    }
                    KeyCode::Char('c') => {
                        event_send_chan
                            .send(BMEvent::ConnectRequested)
                            .await
                            .unwrap();
                    }
                    KeyCode::Char('d') => {
                        event_send_chan
                            .send(BMEvent::DisconnectRequested)
                            .await
                            .unwrap();
                    }
                    KeyCode::Char('e') => {
                        event_send_chan
                            .send(BMEvent::DebugFailBanner)
                            .await
                            .unwrap();
                    }
                    KeyCode::Char('b') => {
                        event_send_chan
                            .send(BMEvent::DebugSuccessBanner)
                            .await
                            .unwrap();
                    }
                    KeyCode::Char('h') => {
                        event_send_chan
                            .send(BMEvent::ShowHideUnnamed)
                            .await
                            .unwrap();
                    }
                    _ => (),
                },
                _ => {}
            };
        }
    })
}
