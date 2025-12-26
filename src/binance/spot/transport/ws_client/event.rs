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
        if let ( Some(_id),Some(_status),Some(_result)  )  = 
        (msg.get("id").and_then(|v| v.as_str()) 
        , msg.get("status")
        , msg.get("result")
        ){
        
                if let Some(tx) = self.pending.lock().unwrap().remove(_id) {
                    let _ = tx.send(msg);
                    return true; // จัดการแล้ว (เป็น API Response)
                }
        }
        false // ยังไม่ได้จัดการ (อาจเป็น Stream Data)
    }
}