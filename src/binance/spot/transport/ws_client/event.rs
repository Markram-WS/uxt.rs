use std::collections::HashMap;
use tokio::sync::{mpsc, oneshot};

pub struct WsEvent {
    // ws-api response (มี id)
    pending: HashMap<String, oneshot::Sender<serde_json::Value>>,

    // stream event (ไม่มี id)
    event_tx: mpsc::Sender<serde_json::Value>,
}

impl WsEvent {
    pub fn new(event_tx: mpsc::Sender<serde_json::Value>) -> Self {
        Self {
            pending: HashMap::new(),
            event_tx,
        }
    }

    pub fn register(&mut self, id: String) -> oneshot::Receiver<serde_json::Value> {
        let (tx, rx) = oneshot::channel();
        self.pending.insert(id, tx);
        rx
    }

    pub async fn dispatch(&mut self, msg: serde_json::Value) {
        if let Some(id) = msg.get("id").and_then(|v| v.as_str()) {
            if let Some(tx) = self.pending.remove(id) {
                let _ = tx.send(msg);
            }
        } else {
            let _ = self.event_tx.send(msg).await;
        }
    }
}
