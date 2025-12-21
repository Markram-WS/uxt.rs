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
use std::{time::Duration};
use anyhow::anyhow;
use tokio::time::timeout;




pub struct WsClient {
    conn: WsConn,
    events: WsEvent,
    pub role: WsRole,
    pub authed: bool,
    pub authorized_since: Option<i64>,
    pub timeout_sec: u64



}

impl WsClient {
    pub async fn connect(builder: WsBuilder) -> anyhow::Result<(Self)> {
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
        let (event_tx, _event_rx) = mpsc::channel(1024);
        let mut client = Self {
            conn: WsConn::new(ws),
            events: WsEvent::new(event_tx),
            role,
            authed: false,
            authorized_since: None,
            timeout_sec: 10,
        };
        Ok(client)
    }

    pub async fn set_timeout(mut self, timeout_sec:u64) {
        self.timeout_sec = timeout_sec;
    }

    // -------- transport ----------
    pub async fn read_once(&mut self) -> anyhow::Result<Option<String>> {
        self.conn.read_once().await
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

        let resp: serde_json::Value = timeout(Duration::from_secs(self.timeout_sec), rx)
            .await
            .map_err(|_| anyhow!("WS API request {} timeout\n",method))??;
        if resp["status"] == 200 {
            Ok(resp)
        }
        else{
            anyhow::bail!("{} failed: {:?}",method, resp);
        }
        
    }

    pub async fn logon(&mut self) -> anyhow::Result<serde_json::Value> {
        match &self.role {
            WsRole::WsApi { api_key, secret: _ } => {
                let resp: serde_json::Value = self
                    .call_wsapi("session.logon", json!({ "api_key": api_key }))
                    .await?;
    
               
                self.authed = true;
                let api_keys =  &resp["result"]["apiKey"];
                self.authorized_since = resp["result"]["authorizedSince"].clone().as_i64();
                Ok(json!({ 
                    "api_keys":api_keys,
                    "authorized_since":self.authorized_since
                }))
              
            }
            _ => {
                anyhow::bail!("\nlogon called but WsRole is not WsApi");
            }
        }
    }

    pub async fn logout(&mut self) -> anyhow::Result<()> {
        match &self.role {
            WsRole::WsApi { api_key, secret: _ } => {
                let resp = self
                    .call_wsapi("session.logout", json!({ }))
                    .await?;
                self.authed = false;
                Ok(())
              
            }
            _ => {
                anyhow::bail!("\nlogout called but WsRole is not WsApi");
            }
        }
    }
    
    pub async fn status(&mut self) -> anyhow::Result<serde_json::Value> {
        match &self.role {
            WsRole::WsApi { api_key, secret: _ } => {
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
        match &self.role {
            WsRole::WsApi { api_key, secret: _ } => {
                match &self.authed {
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
            WsRole::WsApi { api_key, secret: _ } => {
                match &self.authed {
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
