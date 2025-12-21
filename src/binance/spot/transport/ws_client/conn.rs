

use tokio_tungstenite::{
    WebSocketStream, MaybeTlsStream,
    tungstenite::Message,
    tungstenite::protocol::{CloseFrame, frame::coding::CloseCode},
};
use tokio::net::TcpStream;
use futures_util::{SinkExt, StreamExt};

type WsSocket = WebSocketStream<MaybeTlsStream<TcpStream>>;

pub struct WsConn {
    ws: WsSocket,
}

impl WsConn {
    pub fn new(ws: WsSocket) -> Self {
        Self { ws }
    }

    pub async fn send_text(&mut self, txt: String) -> anyhow::Result<()> {
        self.ws.send(Message::Text(txt.into())).await?;
        Ok(())
    }

    pub async fn read_once(&mut self) -> anyhow::Result<Option<String>> {
        match self.ws.next().await {
            Some(Ok(Message::Text(txt))) => Ok(Some(txt.to_string())),
            Some(Ok(_)) => Ok(None),
            Some(Err(e)) => Err(e.into()),
            None => Ok(None),
        }
    }

    pub async fn close(&mut self) -> anyhow::Result<()> {
        self.ws
            .close(Some(CloseFrame {
                code: CloseCode::Normal,
                reason: "client shutdown".into(),
            }))
            .await?;
        Ok(())
    }
}
