

use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::MaybeTlsStream;
// connection.rs
use tokio_tungstenite::{connect_async};
use futures::{ StreamExt}; 
use tokio::net::TcpStream;
use tokio::sync::{mpsc,Mutex};
use std::sync::Arc;
type WsSocket = WebSocketStream<MaybeTlsStream<TcpStream>>;
use std::error::Error;
use sha2::{Digest, Sha256};
use chrono::Utc;



pub struct WsTransport {
    ws: Arc<Mutex<Option<WsSocket>>>,
    url: String,
    raw_tx: mpsc::Sender<String>,
}

impl WsTransport {
    pub fn new(url: String) -> (Self, mpsc::Receiver<String>) {
        let ws = Arc::new(Mutex::new(None));
        let (tx, rx) = mpsc::channel::<String>(100);

        (Self { ws, url, raw_tx: tx }, rx)
    }

    pub async fn connect(&self) -> Result<(),Box<dyn Error>> {
        let (ws_stream, _) = connect_async(&self.url).await?;
        let mut ws = self.ws.lock().await;
        *ws = Some(ws_stream);
        Ok(())
  
    }

    pub async fn listen(&self) {
        let ws = self.ws.clone();
        let tx = self.raw_tx.clone();

        tokio::spawn(async move {
            let mut guard = ws.lock().await;
            let socket = guard.as_mut().unwrap();

            while let Some(msg) = socket.next().await {
                if let Ok(m) = msg {
                    if let Ok(text) = m.to_text() {
                        let _ = tx.send(text.to_string()).await;
                    }
                }
            }
        });
    }
}


fn gen_hash_id ( event : &str)  -> String {
    let current_timestamp = Utc::now().timestamp_millis();
    let timestamp_string = current_timestamp.to_string();
    let data_to_hash = timestamp_string.as_bytes();
    let mut hasher = Sha256::new();
    hasher.update(data_to_hash);
    hasher.update(event.as_bytes());
    let result_bytes = hasher.finalize();
    let hex_hash = hex::encode(result_bytes);
    hex_hash
}