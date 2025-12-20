mod conn;
mod role;
mod event;

use chrono::Utc;
use conn::WsConn;
use event::WsEvent;
use role::WsRole;
use serde_json::json;
use tokio::sync::mpsc;
use tokio_tungstenite::connect_async;
use serde::Serialize;
use crate::binance::spot::{WsBuilder,StreamMode};
use uuid::Uuid;
use sqlx::Value;


pub struct WsClient {
    conn: WsConn,
    events: WsEvent,
    pub role: WsRole,
    pub authed: bool,
    pub last_logon: Option<i64>,
}

impl WsClient {
    pub async fn connect(builder: WsBuilder) -> anyhow::Result<Self> {
        let url = builder.base_url.clone();
        let (ws, _) = connect_async(&url).await?;

        let role = match builder.mode {
            StreamMode::Public => WsRole::Public,
            StreamMode::UserData => WsRole::UserData,
            StreamMode::WsApi => WsRole::WsApi {
                api_key: builder.api_key,
                secret: builder.secret,
            },
        };
        let (event_tx, event_rx) = mpsc::channel(1024);
        let authed = false;
        let last_logon  = None;
        Ok(Self {
            conn: WsConn::new(ws),
            events: WsEvent::new(event_tx),
            role,
            authed,
            last_logon
        })
    }

    // -------- transport ----------
    pub async fn read_once(&mut self) -> anyhow::Result<Option<String>> {
        self.conn.read_once().await
    }

    pub async fn read_loop(&mut self) -> anyhow::Result<()> {
        while let Some(txt) = self.read_once().await? {
            let msg = serde_json::from_str(&txt)?;
            self.events.dispatch(msg).await;
        }
        Ok(())
    }

    pub async fn close(&mut self) -> anyhow::Result<()> {
        self.conn.close().await
    }

    // -------- generic send ----------
    pub async fn send<T: Serialize>(&mut self, data: &T) -> anyhow::Result<()> {
        let txt = serde_json::to_string(data)?;
        self.conn.send_text(txt).await
    }
    
    // -------- ws-api ----------
    pub async fn call_wsapi(
        &mut self,
        method: &str,
        params: serde_json::Value,
    ) -> anyhow::Result<serde_json::Value> {
        let id = Uuid::new_v4().to_string();
        let params = self.role.sign_wsapi(params)?;
    
        let req = serde_json::json!({
            "id": id,
            "method": method,
            "params": params
        });
    
        let rx = self.events.register(id);
        self.conn.send_text(req.to_string()).await?;
    
        Ok(rx.await?)
    }

    pub async fn logon(&mut self) -> anyhow::Result<serde_json::Value> {
        match &self.role {
            WsRole::WsApi { api_key, secret: _ } => {
                let resp = self
                    .call_wsapi("session.logon", json!({ "api_key": api_key }))
                    .await?;
    
                if resp["status"] == 200 {
                    self.authed = true;
                    self.last_logon = Some(Utc::now().timestamp());
                    let api_keys =  &resp["result"]["apiKey"];
                    let authorized_since =  &resp["result"]["authorizedSince"];
                    Ok(json!({ 
                        "api_keys":api_keys,
                        "authorized_since":authorized_since
                    }))
                } else {
                    anyhow::bail!("logon failed: {:?}", resp);
                }
            }
            _ => {
                anyhow::bail!("logon called but WsRole is not WsApi");
            }
        }
    }

    pub async fn logout(&mut self) -> anyhow::Result<serde_json::Value> {
        match &self.role {
            WsRole::WsApi { api_key, secret: _ } => {
                let resp = self
                    .call_wsapi("session.logout", json!({ }))
                    .await?;
                let api_keys =  &resp["result"]["apiKey"];
                let authorized_since =  &resp["result"]["authorizedSince"];
                if resp["status"] == 200  && api_keys.is_null()  {
                    self.authed = false;
                    Ok(json!({ 
                        "api_keys":api_keys,
                        "authorized_since":authorized_since
                    }))
                } else {
                    anyhow::bail!("logout failed: {:?}", resp);
                }
            }
            _ => {
                anyhow::bail!("logout called but WsRole is not WsApi");
            }
        }
    }
    
    pub async fn status(&mut self) -> anyhow::Result<serde_json::Value> {
        match &self.role {
            WsRole::WsApi { api_key, secret: _ } => {
                let resp = self
                    .call_wsapi("session.status", json!({ }))
                    .await?;
                if resp["status"] == 200  &&  resp["result"]["apiKey"].is_null() {
                    let api_keys =  &resp["result"]["apiKey"];
                    let authorized_since =  &resp["result"]["authorizedSince"];
                    Ok(json!({ 
                        "api_keys":api_keys,
                        "authorized_since":authorized_since
                    }))
                } else {
                    anyhow::bail!("status failed: {:?}", resp);
                }
            }
            _ => {
                anyhow::bail!("status called but WsRole is not WsApi");
            }
        }
    }
}

// next
// timeout ของ pending
// retry
// typed event enum
// map error code → Result