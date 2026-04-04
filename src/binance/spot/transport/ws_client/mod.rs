mod conn;
mod role;
mod event;
use crate::utils::{create_payload_signature};
use chrono::Utc;
use conn::WsConn;
use event::WsEvent;
use futures::{StreamExt, SinkExt};
use role::WsRole;
use serde_json::json;
use tokio::sync::mpsc::{self};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use serde::Serialize;
use crate::binance::spot::{WsBuilder,StreamMode};
use uuid::Uuid;
use std::{time::Duration};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use tokio::time::timeout;
use anyhow;


pub struct WsClient {
    conn: WsConn,
    events: WsEvent,
    stream_rx: mpsc::UnboundedReceiver<serde_json::Value>, 
    pub role: WsRole,
    pub authed:bool,
    pub authorized_since:Option<i64>,
    pub timeout_sec: i64,
    pub recv_window:Option<i64>,
    debug_log: Arc<AtomicBool>, // เพิ่มตัวแปรควบคุม Log

}

impl WsClient {
    pub fn set_recv_window(&mut self, window: i64) {
        self.recv_window = Some(window);
    }

    pub async fn connect(builder: WsBuilder) -> anyhow::Result<Self> {
        log::info!("> Connecting to WebSocket: {}",&builder.base_url);
        let (ws, _) = connect_async(&builder.base_url).await?;
        let (mut writer, mut reader) = ws.split();

        let (stream_tx, stream_rx) = mpsc::unbounded_channel();
        let (write_tx, mut write_rx) = mpsc::unbounded_channel::<Message>();
        
        let events = WsEvent::new();
        let events_clone = events.clone();
        let write_tx_clone = write_tx.clone();
        
        // สถานะการเปิด/ปิด Log
        let debug_log = Arc::new(AtomicBool::new(builder.debug_log));
        let debug_log_clone = debug_log.clone();

        // --- Background Reader Loop ---
        tokio::spawn(async move {
            log::debug!("WS Reader Loop started");
            while let Some(msg_res) = reader.next().await {
                match msg_res {
                    Ok(Message::Text(text)) => {
                        // ตรวจสอบ toggle ก่อน Log
                        if debug_log_clone.load(Ordering::Relaxed) {
                            log::debug!("WS RECEIVED: {}", text);
                        }
                        
                        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
                            if !events_clone.dispatch(json.clone()) {
                                if json.is_object() || json.is_array() {
                                    let _ = stream_tx.send(json);
                                }
                            }
                        }
                    }
                    Ok(Message::Ping(payload)) => {
                        if debug_log_clone.load(Ordering::Relaxed) { log::debug!("WS RECEIVED: PING"); }
                        let _ = write_tx_clone.send(Message::Pong(payload));
                    }
                    Ok(Message::Pong(_)) => {
                        if debug_log_clone.load(Ordering::Relaxed) { log::debug!("WS RECEIVED: PONG"); }
                    }
                    Ok(Message::Close(cf)) => {
                        log::warn!("WS RECEIVED: CLOSE {:?}", cf);
                        break;
                    }
                    Err(e) => {
                        log::error!("WS Reader Error: {}", e);
                        break;
                    }
                    _ => {}
                }
            }
            log::warn!("WS Reader Loop exited");
        });

        // --- Background Writer Loop ---
        tokio::spawn(async move {
            log::debug!("WS Writer Loop started");
            while let Some(msg) = write_rx.recv().await {
                if let Err(e) = writer.send(msg).await {
                    log::error!("WS Writer Error: {}", e);
                    break;
                }
            }
            log::warn!("WS Writer Loop exited");
        });

        // --- Role & Auth Logic ---
        let mut authed = false;
        let mut authorized_since = None;
        let role: WsRole = match builder.mode {
            StreamMode::Public => {
                authed = true;
                authorized_since = Some(Utc::now().timestamp_millis());
                WsRole::Public
            },
            StreamMode::UserData => {
                authed = true;
                authorized_since = Some(Utc::now().timestamp_millis());
                WsRole::UserData { api_key: builder.api_key, secret: builder.secret }
            },
            StreamMode::WsApi => {
                WsRole::WsApi { api_key: builder.api_key, secret: builder.secret }
            }
        };

        Ok(Self {
            conn: WsConn::new(write_tx),
            events,
            stream_rx,
            role,
            authed,
            authorized_since,
            timeout_sec: 10,
            recv_window: None,
            debug_log,

        })
    }

    // --- เมธอดใหม่สำหรับ Toggle Log ---
    pub fn set_debug_log(&self, enable: bool) {
        self.debug_log.store(enable, Ordering::Relaxed);
        log::info!("WS Debug Log set to: {}", enable);
    }

    pub async fn set_timeout(mut self, timeout_sec:i64) {
        self.timeout_sec = timeout_sec;
    }

    pub async fn close(&mut self) -> Result<(), anyhow::Error> {
        self.conn.close().await?;
        Ok(())
    }

    pub async fn read_once(&mut self) -> Option<serde_json::Value> {
        self.stream_rx.recv().await
    }

    pub async fn send<T: Serialize>(&mut self, data: &T) -> anyhow::Result<()> {
        let txt = serde_json::to_string(data)?;
        self.conn.send_text(txt).await
    }
    
    pub async fn call_wsapi(&mut self, method: &str, params: serde_json::Value) -> anyhow::Result<serde_json::Value> {
        let id = Uuid::new_v4().to_string();
        let rx = self.events.register(id.clone());
        let req = json!({ "id": id, "method": method, "params": params });
        
        if self.debug_log.load(Ordering::Relaxed) {
            log::debug!("WS SEND (API): {}", req);
        }

        self.conn.send_text(req.to_string()).await?;

        match timeout(Duration::from_secs(self.timeout_sec.try_into()?), rx).await {
            Ok(Ok(resp)) => {
                if self.debug_log.load(Ordering::Relaxed) {
                    log::debug!("WS RECV (API RESPONSE): id: {} method: {}", id, method);
                }
                Ok(resp)
            }
            Ok(Err(_)) => anyhow::bail!("Channel error for method {}", method),
            Err(_) => anyhow::bail!("Timeout waiting for {} (id: {})", method, id),
        }
    }

    pub async fn logon(&mut self) -> anyhow::Result<serde_json::Value> {
        match &self.role {
            WsRole::WsApi { api_key ,.. } => {
                let mut param = json!({ "apiKey": &api_key });
                if let Some(recv_window) = self.recv_window {
                    param["recvWindow"] = recv_window.into();
                }
                let param_siged = self.role.sign_wsapi(param)?;
                let resp = self.call_wsapi("session.logon", param_siged).await?;
                if let Some(result) = resp.get("result") {
                    self.authed = true;
                    Ok(json!(result))
                } else {
                    anyhow::bail!("Logon failed: {:?}", resp)
                }
            }      
            _ => anyhow::bail!("Logon only supported for WsApi role"),
        }
    }

    pub async fn logout(&mut self) -> anyhow::Result<()> {
        if let WsRole::WsApi { .. } = &self.role {
            self.call_wsapi("session.logout", json!({})).await?;
            self.authed = false;
            Ok(())
        } else {
            anyhow::bail!("Logout only supported for WsApi role")
        }
    }

    pub async fn status(&mut self) -> anyhow::Result<serde_json::Value> {
        if let WsRole::WsApi { .. } = &self.role {
            self.call_wsapi("session.status", json!({})).await
        } else {
            anyhow::bail!("Status only supported for WsApi role")
        }
    }

    pub async fn ping(&mut self) -> anyhow::Result<serde_json::Value> {
        if self.authed {
            self.call_wsapi("ping", json!({})).await
        } else {
            anyhow::bail!("Ping requires logon first")
        }
    }

    pub async fn subscribe_streams(&mut self, streams: Vec<String>) -> anyhow::Result<serde_json::Value> {
        let params = json!({ "streams": streams });
        log::info!("WS Subscribing to: {:?}", streams);
        self.call_wsapi("subscribe", params).await
    }

    pub async fn time(&mut self) -> anyhow::Result<Option<i64>> {
        let resp = self.call_wsapi("time", json!({})).await?;
        Ok(resp["result"]["serverTime"].as_i64())
    }
}
