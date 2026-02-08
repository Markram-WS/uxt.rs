mod conn;
mod role;
mod event;

use chrono::Utc;
use conn::WsConn;
use event::WsEvent;
use futures::StreamExt;
use role::WsRole;
use serde_json::json;
use tokio::sync::mpsc::{self};
use tokio_tungstenite::connect_async;
use serde::Serialize;
use crate::binance::spot::{WsBuilder,StreamMode};
use uuid::Uuid;
use std::{time::Duration};

use tokio::time::timeout;
use anyhow;


pub struct WsClient {
    conn: WsConn,
    events: WsEvent,
    stream_rx: mpsc::UnboundedReceiver<serde_json::Value>, // ถังพักสำหรับ read_once
    pub role: WsRole,
    pub authed:bool,
    pub authorized_since:Option<i64>,
    pub timeout_sec: i64,

}

impl WsClient {
    pub async fn connect(builder: WsBuilder) -> anyhow::Result<Self> {
        let (ws, _) = connect_async(&builder.base_url).await?;
        let (writer, mut reader) = ws.split(); // แยก ร่าง!

        let (stream_tx, stream_rx) = mpsc::unbounded_channel();
        let events = WsEvent::new();
        let events_clone = events.clone();

        //---authed--
        let mut authed = false;
        let mut authorized_since = None;
        //---role--
        let role: WsRole = match builder.mode {
            StreamMode::Public => {
                authed = true;
                authorized_since = Some(Utc::now().timestamp_millis());
                WsRole::Public
            },
            StreamMode::UserData => {
                authed = true;
                authorized_since = Some(Utc::now().timestamp_millis());
                WsRole::UserData {
                    api_key: builder.api_key,
                    secret: builder.secret
                }
            },
            StreamMode::WsApi => WsRole::WsApi {
                api_key: builder.api_key,
                secret: builder.secret
            }
        };
        

        // --- Background Loop: ตัวแยกประเภทข้อมูล ---
        tokio::spawn(async move {
            while let Some(Ok(msg)) = reader.next().await {
                if let Ok(text) = msg.to_text() {
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(text) {
                        // 1. ลองส่งให้ oneshot (API) ก่อน
                        if !events_clone.dispatch(json.clone()) {
                            // 2. ถ้าไม่ใช่ API Response ให้โยนลงถังพักสตรีม
                            let _ = stream_tx.send(json);
                        }
                    }
                }
            }
        });

        Ok(Self {
            conn: WsConn::new(writer),
            events,
            stream_rx,
            role,
            authed,
            authorized_since,
            timeout_sec: 10,
        })
    }

    pub async fn set_timeout(mut self, timeout_sec:i64) {
        self.timeout_sec = timeout_sec;
    }

    pub async fn close(&mut self) -> Result<(), anyhow::Error> {
        self.conn.close().await?;
        Ok(())
    }
    // -------- transport ----------
    pub async fn read_once(&mut self) -> Option<serde_json::Value> {
        self.stream_rx.recv().await
    }

    // -------- generic send ----------
    pub async fn send<T: Serialize>(&mut self, data: &T) -> anyhow::Result<()> {
        let txt = serde_json::to_string(data)?;
        self.conn.send_text(txt).await
    }
    
    // -------- WS-API ----------
    // API: ส่งแล้วรอ Response ด้วย ID
    pub async fn call_wsapi(&mut self, method: &str, params: serde_json::Value) -> anyhow::Result<serde_json::Value> {
        let id = Uuid::new_v4().to_string();
        let rx = self.events.register(id.clone());
        
        let req = json!({ "id": id, "method": method, "params": params });
        self.conn.send_text(req.to_string()).await?;

        timeout(Duration::from_secs(self.timeout_sec.try_into()?), rx).await??
            .as_object()
            .ok_or_else(|| anyhow::anyhow!("Invalid response"))
            .map(|obj| serde_json::Value::Object(obj.clone()))
    }

    pub async fn logon(&mut self) -> anyhow::Result<serde_json::Value> {

        match &self.role {
            WsRole::WsApi {  api_key ,.. } => {
                let _resp = self.call_wsapi("session.logon", json!({ "api_key": &api_key })).await?;
                self.authed = true;
                self.authorized_since = _resp["result"]["authorizedSince"].as_i64();

                let mut json_res = _resp["rateLimits"].clone();
                json_res["authorizedSince"] =  _resp["result"]["authorizedSince"].clone();
                json_res["apiKey"]= _resp["result"]["apiKey"].clone();
                Ok(json!(json_res))
            }      
            _ => {
                anyhow::bail!("\nlogout called but WsRole is not WsApi");
            }
        }
    }

    pub async fn logout(&mut self) -> anyhow::Result<()> {

        match &mut self.role {
            WsRole::WsApi {  .. } => {
                let _resp = self.call_wsapi("session.logout", json!({})).await?;
                self.authed = false;
                Ok(())
            }      
            _ => {
                anyhow::bail!("\nlogout called but WsRole is not WsApi");
            }
        }
    }

    pub async fn status(&mut self) -> anyhow::Result<serde_json::Value> {
        match &mut self.role {
            WsRole::WsApi {  .. } => {
                let resp = self
                    .call_wsapi("session.status", json!({ }))
                    .await?;
                if resp["result"]["apiKey"].is_null() {
                    let mut json_res = resp["rateLimits"].clone();
                    json_res["authorizedSince"] =  resp["result"]["authorizedSince"].clone();
                    json_res["apiKey"]= resp["result"]["apiKey"].clone();
                    Ok(json!(json_res))
                } else {
                    anyhow::bail!("\nstatus failed apiKey null : {:?} ", resp);
                }
            }
            _ => {
                anyhow::bail!("\nstatus called but WsRole is not WsApi");
            }
        }
    }

    pub async fn ping(&mut self) -> anyhow::Result<serde_json::Value> {
        match &mut self.role {
            WsRole::WsApi {..} => {
                match self.authed {
                    true => {
                        let resp = self
                        .call_wsapi("ping", json!({  }))
                        .await?;
        
                        let rate_limit = &resp["rateLimits"] ;
                        Ok(json!(rate_limit))
                    }
                    _ => {
                        anyhow::bail!("\nping called but authed must be logon");
                    }
                }

            }
            _ => {
                anyhow::bail!("\nping called but WsRole is not WsApi");
            }
        }
    }

    pub async fn time(&mut self) -> anyhow::Result<Option<i64>> {
        match &self.role {
            WsRole::WsApi {  .. } => {
                match self.authed {
                    true => {
                        let resp = self
                        .call_wsapi("time", json!({}))
                        .await?;
        
                      
                        let server_time = resp["result"]["serverTime"].as_i64();
                        Ok(server_time)
                    
                    }
                    _ => {
                        anyhow::bail!("\nping called but authed must be logon");
                    }
                }

            }
            _ => {
                anyhow::bail!("\nping called but WsRole is not WsApi");
            }
        }
    }

}
