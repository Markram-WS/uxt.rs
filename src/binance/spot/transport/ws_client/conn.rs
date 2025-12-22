

use tokio_tungstenite::{
    WebSocketStream, MaybeTlsStream,
    tungstenite::Message,
};
use tokio::net::TcpStream;
use futures_util::{SinkExt};
use futures_util::stream::SplitSink;

type WsWriter = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;

pub struct WsConn {
    ws: WsWriter,
}

impl WsConn {
    pub fn new(ws: WsWriter) -> Self {
        Self { ws }
    }

    pub async fn close(&mut self) -> Result<(), anyhow::Error> {
        let _ = self.ws.send(Message::Close(None)).await;
        let _ = self.ws.close().await;
        Ok(())
    }
    pub async fn send_text(&mut self, txt: String) -> anyhow::Result<()> {
        self.ws.send(Message::Text(txt.into())).await?;
        Ok(())
    }
}
