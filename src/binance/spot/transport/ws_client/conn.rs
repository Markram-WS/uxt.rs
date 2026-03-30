
use tokio_tungstenite::tungstenite::Message;
use tokio::sync::mpsc;

pub struct WsConn {
    tx: mpsc::UnboundedSender<Message>,
}

impl WsConn {
    pub fn new(tx: mpsc::UnboundedSender<Message>) -> Self {
        Self { tx }
    }

    pub async fn close(&mut self) -> Result<(), anyhow::Error> {
        let _ = self.tx.send(Message::Close(None));
        Ok(())
    }

    pub async fn send_text(&mut self, txt: String) -> anyhow::Result<()> {
        self.tx.send(Message::Text(txt.into()))
            .map_err(|e| anyhow::anyhow!("WS Send Error: {}", e))?;
        Ok(())
    }

    pub fn send_msg(&self, msg: Message) -> anyhow::Result<()> {
        self.tx.send(msg)
            .map_err(|e| anyhow::anyhow!("WS Send Error: {}", e))?;
        Ok(())
    }
}
