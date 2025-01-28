use futures::StreamExt;
use std::sync::Arc;

use crossterm::event::{Event, EventStream, KeyCode};
use tokio::{sync::mpsc::Sender, task::JoinHandle};

use super::btui::AppEvent;

pub fn launch_key_listener(event_send_chan: Arc<Sender<AppEvent>>) -> JoinHandle<()> {
    let mut event_stream = EventStream::new();

    tokio::spawn(async move {
        loop {
            match event_stream.next().await {
                Some(Ok(Event::Key(evnt))) => match evnt.code {
                    // Quit key
                    KeyCode::Char('q') => {
                        event_send_chan.send(AppEvent::Exit).await.unwrap();
                    }
                    KeyCode::Char('j') | KeyCode::Down => {
                        event_send_chan.send(AppEvent::ScrollDown).await.unwrap();
                    }
                    KeyCode::Char('k') | KeyCode::Up => {
                        event_send_chan.send(AppEvent::ScrollUp).await.unwrap();
                    }
                    KeyCode::Char('c') => {
                        event_send_chan
                            .send(AppEvent::ConnectRequested)
                            .await
                            .unwrap();
                    }
                    KeyCode::Char('p') => {
                        event_send_chan
                            .send(AppEvent::ConnectRequested)
                            .await
                            .unwrap();
                    }
                    KeyCode::Char('d') => {
                        event_send_chan
                            .send(AppEvent::DisconnectRequested)
                            .await
                            .unwrap();
                    }
                    KeyCode::Char('r') => {
                        event_send_chan
                            .send(AppEvent::RemoveRequested)
                            .await
                            .unwrap();
                    }
                    KeyCode::Char('e') => {
                        event_send_chan
                            .send(AppEvent::DebugFailBanner)
                            .await
                            .unwrap();
                    }
                    KeyCode::Char('b') => {
                        event_send_chan
                            .send(AppEvent::DebugSuccessBanner)
                            .await
                            .unwrap();
                    }
                    KeyCode::Char('h') => {
                        event_send_chan
                            .send(AppEvent::ShowHideUnnamed)
                            .await
                            .unwrap();
                    }
                    _ => {
                        event_send_chan.send(AppEvent::Pass).await.unwrap();
                    }
                },
                _ => {}
            };
        }
    })
}
