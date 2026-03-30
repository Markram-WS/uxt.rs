use std::{collections::HashMap, sync::Arc};
use tokio::sync::{oneshot};
use std::sync::Mutex;
#[derive(Clone)]
pub struct WsEvent {
    // ใช้เก็บ oneshot สำหรับ call_wsapi
    pending: Arc<Mutex<HashMap<String, oneshot::Sender<serde_json::Value>>>>,
}

impl WsEvent {
    pub fn new() -> Self {
        Self { pending: Arc::new(Mutex::new(HashMap::new())) }
    }

    pub fn register(&self, id: String) -> oneshot::Receiver<serde_json::Value> {
        let (tx, rx) = oneshot::channel();
        self.pending.lock().unwrap().insert(id, tx);
        rx
    }
    pub fn dispatch(&self, msg: serde_json::Value) -> bool {
        // Extract ID as String whether it is a String or Number in JSON
        let id = msg.get("id").and_then(|v| {
            if v.is_string() {
                v.as_str().map(|s| s.to_string())
            } else if v.is_number() {
                Some(v.to_string())
            } else {
                None
            }
        });

        if let Some(id_str) = id {
            // Check if it looks like an API response (has status or result field)
            if msg.get("status").is_some() || msg.get("result").is_some() || msg.get("error").is_some() {
                let mut pending = self.pending.lock().unwrap();
                if let Some(tx) = pending.remove(&id_str) {
                    let _ = tx.send(msg);
                    return true; // Handled as API Response
                }
            }
        }
        false // Not an API response, handle as Stream Data
    }
}