

use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::MaybeTlsStream;
use tokio::net::TcpStream;
use chrono::Utc;
use futures_util::{SinkExt};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use super::signer::sign;
use super::ws_builder::WsBuilder;
type WsSocket = WebSocketStream<MaybeTlsStream<TcpStream>>;





pub struct WsClient {
    ws: WsSocket,
    api_key:String,
    secret:String,
    base_url:String
    
}

impl WsClient {
    pub async fn connect(builder:WsBuilder) -> anyhow::Result<Self> {
        
        let (ws, _ ) = connect_async( builder.base_url.clone() ).await?;
        Ok(Self {
            ws,
            api_key:builder.api_key,
            secret:builder.secret,
            base_url:builder.base_url
        })
    }

    pub async  fn send<T:serde::Serialize> (&mut self ,reqwest: &T) -> anyhow::Result<()> {
        let json = serde_json::to_value(reqwest)?;
        let text = serde_json::to_string(&json)?;
        self.ws.send(Message::Text(text.into())).await?;
        Ok(())
    }


    pub async  fn send_signed<T:serde::Serialize> (&mut self ,reqwest: &T) -> anyhow::Result<()> {
        let mut json = serde_json::to_value(reqwest)?;
        let timestamp = Utc::now().timestamp_millis();
        json["timestamp"] = timestamp.into();
        let query = format!("timestamp={}", timestamp);
        let signature = sign(&self.secret, &query);
        json["signature"] = signature.into();
        let text = serde_json::to_string(&json)?;
        self.ws.send(Message::Text(text.into())).await?;
        Ok(())
    }

    pub async fn send_json<T: serde::Serialize>(&mut self, data: &T) -> anyhow::Result<()> {
        let txt = serde_json::to_string(data)?;
        self.ws.send(Message::Text(txt.into())).await?;
        Ok(())
    }
}
