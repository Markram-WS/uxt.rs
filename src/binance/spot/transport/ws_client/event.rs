use std::collections::HashMap;
use tokio::sync::{mpsc, oneshot};
use dashmap::DashMap;
pub struct WsEvent {
    // ws-api response (มี id)
    pending: DashMap<String, oneshot::Sender<serde_json::Value>>  ,

    // stream event (ไม่มี id)
    event_tx: mpsc::Sender<serde_json::Value>,
}

impl WsEvent {
    pub fn new(event_tx: mpsc::Sender<serde_json::Value>) -> Self {
        Self {
            pending: DashMap::new(),
            event_tx,
        }
    }
    pub fn clone_for_reader(&self) -> WsEvent {
        WsEvent {
            pending: self.pending.clone(),
            event_tx:self.event_tx
        }
    }
    pub fn register(&self, id: String) -> oneshot::Receiver<serde_json::Value> {
        let (tx, rx) = oneshot::channel();
        self.pending.insert(id, tx);
        rx
    }

    pub async fn dispatch(&self, msg: serde_json::Value) {
        if let Some(id) = msg.get("id").and_then(|v| v.as_str()) {
            if let Some((_, tx)) = self.pending.remove(id) {
                let _ = tx.send(msg);
            }
        } else {
            let _ = self.event_tx.send(msg).await;
        }
    }
}
