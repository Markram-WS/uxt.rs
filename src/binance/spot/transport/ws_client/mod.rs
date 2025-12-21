mod conn;
mod role;
mod event;

use conn::WsConn;
use event::WsEvent;
use futures::StreamExt;
use role::WsRole;
use serde_json::json;
use tokio::sync::mpsc;
use tokio_tungstenite::connect_async;
use serde::Serialize;
use crate::binance::spot::{WsBuilder,StreamMode};
use uuid::Uuid;
use std::{time::Duration};
use anyhow::anyhow;
use tokio::time::timeout;




pub struct WsClient {
    conn: WsConn,
    events: WsEvent,
    stream_rx: mpsc::UnboundedReceiver<serde_json::Value>, // ถังพักสำหรับ read_once
    pub role: WsRole,
    pub timeout_sec: u64,
}

impl WsClient {
    pub async fn connect(builder: WsBuilder) -> anyhow::Result<Self> {
        let (ws, _) = connect_async(&builder.base_url).await?;
        let (writer, mut reader) = ws.split(); // แยก ร่าง!

        let (stream_tx, stream_rx) = mpsc::unbounded_channel();
        let events = WsEvent::new();
        let events_clone = events.clone();
        let role: WsRole = match builder.mode {
            StreamMode::Public => WsRole::Public,
            StreamMode::UserData => WsRole::UserData,
            StreamMode::WsApi => WsRole::WsApi {
                api_key: builder.api_key,
                secret: builder.secret,
                authed:false,
                authorized_since:None,

            },
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
            timeout_sec: 10,
        })
    }

    pub async fn set_timeout(mut self, timeout_sec:u64) {
        self.timeout_sec = timeout_sec;
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

        timeout(Duration::from_secs(self.timeout_sec), rx).await??
            .as_object()
            .ok_or_else(|| anyhow!("Invalid response"))
            .map(|obj| serde_json::Value::Object(obj.clone()))
    }

    pub async fn logon(&mut self) -> anyhow::Result<serde_json::Value> {
        // 1. ดึงเฉพาะ api_key ออกมา (Clone ออกมาเพื่อไม่ให้ติด Borrow)
        let api_key_to_use = if let WsRole::WsApi { ref api_key, .. } = self.role {
            api_key.clone()
        } else {
            anyhow::bail!("logon called but WsRole is not WsApi");
        };
    
        // 2. เรียก call_wsapi ได้อย่างอิสระ เพราะ self ไม่ได้ถูกยืมแล้ว
        let resp: serde_json::Value = self
            .call_wsapi("session.logon", json!({ "api_key": api_key_to_use }))
            .await?;
    
        // 3. กลับมาอัปเดตค่าใน role หลังจากได้คำตอบ
        if let WsRole::WsApi { ref mut authed, ref mut authorized_since, .. } = self.role {
            *authed = true;
            *authorized_since = resp["result"]["authorizedSince"].as_i64();
            
            let api_keys = &resp["result"]["apiKey"];
            
            Ok(json!({ 
                "api_keys": api_keys,
                "authorized_since": *authorized_since
            }))
        } else {
            // เคสนี้ปกติจะไม่เกิดขึ้นเพราะเช็คไปแล้วข้างบน แต่ Rust บังคับให้จัดการ
            anyhow::bail!("WsRole changed unexpectedly");
        }
    }

    pub async fn logout(&mut self) -> anyhow::Result<()> {
        // 1. ตรวจสอบ Role ก่อน (ยืมแค่ชั่วคราวแล้วจบไป)
        let is_ws_api = matches!(self.role, WsRole::WsApi { .. });
        
        if !is_ws_api {
            anyhow::bail!("\nlogout called but WsRole is not WsApi");
        }
    
        // 2. เรียกใช้ call_wsapi (ตอนนี้ self ว่างแล้ว เพราะการตรวจสอบข้างบนจบลงแล้ว)
        let _resp = self.call_wsapi("session.logout", json!({})).await?;
    
        // 3. กลับมาอัปเดตค่าใน role (ยืม mutable อีกรอบหลังจากได้คำตอบ)
        if let WsRole::WsApi { ref mut authed, .. } = self.role {
            *authed = false;
            Ok(())
        }else {
            anyhow::bail!("\nlogout failed apiKey null : {:?} ", _resp);

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
            WsRole::WsApi { authed, ..} => {
                match &authed {
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
            WsRole::WsApi {  authed,.. } => {
                match &authed {
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
