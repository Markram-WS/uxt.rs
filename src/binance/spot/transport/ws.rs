

// use tokio_tungstenite::WebSocketStream;
// use tokio_tungstenite::MaybeTlsStream;
// use tokio::net::TcpStream;
// use chrono::Utc;
// use futures_util::{SinkExt};
// use tokio_tungstenite::connect_async;
// use tokio_tungstenite::tungstenite::Message;
// use crate::utils::sign;
// use super::ws_builder::WsBuilder;
// type WsSocket = WebSocketStream<MaybeTlsStream<TcpStream>>;
// use futures_util::StreamExt; 
// use tokio_tungstenite::tungstenite::protocol::CloseFrame;
// use tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode;

// pub struct WsClient {
//     pub ws: WsSocket,
//     pub api_key:String,
//     pub secret:String,
//     pub base_url:String
    
// }

// impl WsClient {
//     pub async fn connect(builder:WsBuilder) -> anyhow::Result<Self> {
//         println!("WsClient::connect -> {}",&builder.base_url);
//         let (ws, _ ) = connect_async( builder.base_url.clone() ).await?;
//         Ok(Self {
//             ws,
//             api_key:builder.api_key,
//             secret:builder.secret,
//             base_url:builder.base_url
//         })
//     }

//     pub async  fn send<T:serde::Serialize> (&mut self ,reqwest: &T) -> anyhow::Result<()> {
//         let json = serde_json::to_value(reqwest)?;
//         let text = serde_json::to_string(&json)?;
//         self.ws.send(Message::Text(text.into())).await?;
//         Ok(())
//     }


//     pub async  fn send_signed<T:serde::Serialize> (&mut self ,reqwest: &T) -> anyhow::Result<()> {
//         let mut json: serde_json::Value = serde_json::to_value(reqwest)?;
//         let timestamp = Utc::now().timestamp_millis();
//         json["timestamp"] = timestamp.into();
//         let query = format!("timestamp={}", timestamp);
//         let signature = sign(&self.secret, &query);
//         json["signature"] = signature.into();
//         let text = serde_json::to_string(&json)?;
//         self.ws.send(Message::Text(text.into())).await?;
//         Ok(())
//     }

//     pub async fn send_json<T: serde::Serialize>(&mut self, data: &T) -> anyhow::Result<()> {
//         let txt = serde_json::to_string(data)?;
//         self.ws.send(Message::Text(txt.into())).await?;
//         Ok(())
//     }

//     pub async fn read_loop<F, Fut>(&mut self, mut handler: F) -> anyhow::Result<()>
//     where
//         F: FnMut(String) -> Fut,
//         Fut: std::future::Future<Output = anyhow::Result<()>>,
//     {
//         while let Some(msg) = self.ws.next().await {
//             let msg = msg?;
//             if let Message::Text(txt) = msg {
//                 handler(txt.to_string()).await?;
//             }
//         }
//         Ok(())
//     }

//     pub async fn read_once(&mut self) -> anyhow::Result<Option<String>> {
//         match self.ws.next().await {
//             Some(Ok(Message::Text(txt))) => Ok(Some(txt.to_string())),
//             Some(Ok(_)) => Ok(None),          // ignore non-text
//             Some(Err(e)) => Err(e.into()),
//             None => Ok(None),                 // ws closed
//         }
//     }

//     pub async fn close(&mut self) -> anyhow::Result<()> {
//         self.ws
//             .close(Some(CloseFrame {
//                 code: CloseCode::Normal,
//                 reason: "client shutdown".into(),
//             }))
//             .await?;
//         Ok(())
//     }

// }

