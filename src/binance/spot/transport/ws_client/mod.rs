mod conn;
mod role;

use conn::WsConn;
use role::WsRole;
use tokio_tungstenite::connect_async;
use serde::Serialize;
use crate::binance::spot::{WsBuilder,StreamMode};

pub struct WsClient {
    conn: WsConn,
    pub role: WsRole,
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

        Ok(Self {
            conn: WsConn::new(ws),
            role,
        })
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
    pub async fn send_wsapi(
        &mut self,
        id: &str,
        method: &str,
        params: serde_json::Value,
    ) -> anyhow::Result<()> {
        let params = self.role.sign_wsapi(params)?;

        let req = serde_json::json!({
            "id": id,
            "method": method,
            "params": params
        });

        self.conn.send_text(req.to_string()).await
    }

}
