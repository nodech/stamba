use crossterm::event::Event as CrosstermEvent;
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc;

use crate::page::LoadablePage;

#[derive(Debug, Clone)]
pub enum AppAction {
    Exit,
    GoTo(LoadablePage),
}

#[derive(Debug, Clone)]
pub enum AppEvent {
    Crossterm(CrosstermEvent),
    App(AppAction),
}

#[derive(Debug)]
pub struct AppEventSource {
    sender: mpsc::UnboundedSender<AppEvent>,
    receiver: mpsc::UnboundedReceiver<AppEvent>,
    event_watcher: Option<tokio::task::JoinHandle<()>>,
}

impl AppEventSource {
    pub async fn init() -> Self {
        let (sender, receiver) = mpsc::unbounded_channel::<AppEvent>();
        let evloop = event_loop(sender.clone());

        AppEventSource {
            sender,
            receiver,
            event_watcher: Some(evloop),
        }
    }

    pub async fn shutdown(&mut self) {
        self.receiver.close();

        if let Some(watcher) = self.event_watcher.take() {
            watcher.await.unwrap();
        }
    }

    pub async fn collect_events(&mut self) -> Vec<AppEvent> {
        let len = self.receiver.len().max(1);
        let mut events = Vec::with_capacity(len);
        let collected = self.receiver.recv_many(&mut events, len).await;
        assert!(len == collected, "Should callect all events.");
        events
    }

    pub fn get_dispatcher(&self) -> AppEventDispatcher {
        AppEventDispatcher::new(self.sender.clone())
    }
}

pub struct AppEventDispatcher {
    sender: mpsc::UnboundedSender<AppEvent>,
}

impl AppEventDispatcher {
    pub fn new(sender: mpsc::UnboundedSender<AppEvent>) -> Self {
        AppEventDispatcher { sender }
    }

    pub fn dispatch(&self, event: AppEvent) {
        self.sender.send(event).unwrap();
    }
}

fn event_loop(sender: mpsc::UnboundedSender<AppEvent>) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        // let mut interval = tokio::time::interval(Duration::from_millis(1000));
        let mut cross_stream = crossterm::event::EventStream::new();

        loop {
            let cse = cross_stream.next().fuse();

            tokio::select! {
                _ = sender.closed() => {
                    break;
                }
                // _ = interval.tick() => {
                //     sender.send(AppEvent::Tick);
                // }
                Some(Ok(event)) = cse => {
                    sender.send(AppEvent::Crossterm(event)).unwrap();
                }
            }
        }
    })
}
