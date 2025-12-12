use serde_json::Value;
use super::model;
use tokio::sync::{mpsc};
use std::error::Error;
type Event = model::Trade;
#[allow(dead_code)]
#[derive(Clone)]
pub struct TradeService {
    tx: mpsc::Sender<Event>,
}

#[allow(dead_code)]
impl TradeService {
    pub fn new() -> (Self, mpsc::Receiver<Event>) {
        let (tx, rx) = mpsc::channel(100);
        (Self { tx }, rx)
    }

    pub async fn handle(&self,txt:&str) ->Result< (), Box<dyn Error>> {
        let parsed: Value = serde_json::from_str(txt)?;
    
        let event_type = parsed.get("e")
            .or_else(|| parsed.get("data")
            .and_then(|d| d.get("e")))
            .and_then(|v| v.as_str());
    
        if event_type == Some("trade") {
            let data = parsed.get("data").unwrap_or(&parsed);
            let ev = serde_json::from_value::<Event>(data.clone())?;
            self.tx.send(ev).await?;
            return Ok(());
        }else{
            Ok(())
        }
    
    
    }
}


#[tokio::test]
async fn test_binance_spot_pub_stream_trade_service() {
    let (svc, mut rx) = TradeService::new();

    let sample = r#"{
        "e": "trade",       
        "E": 1672515782136, 
        "s": "BNBBTC",      
        "t": 12345,         
        "p": "0.001",       
        "q": "100",         
        "T": 1672515782136,
        "m": true,         
        "M": true           
        }"#;

    svc.handle(sample).await.expect("trade handle event");

    let ev = rx.recv().await.expect("channel closed");
    assert_eq!(ev.price, 0.001);
}
