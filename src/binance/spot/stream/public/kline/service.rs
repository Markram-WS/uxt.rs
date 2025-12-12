use serde_json::Value;
use super::model;
use tokio::sync::{mpsc};
type Event = model::Kline;
use std::error::Error;

#[allow(dead_code)]
#[derive(Clone)]
pub struct KlineService {
    tx: mpsc::Sender<Event>,
}

#[allow(dead_code)]
impl KlineService {
    pub fn new() -> (Self, mpsc::Receiver<Event>) {
        let (tx, rx) = mpsc::channel(100);
        (Self { tx }, rx)
    }

    pub async fn handle(&self,txt:&str) ->Result< (), Box<dyn Error>> {
        let parsed: Value = serde_json::from_str(txt)?;
    
        let event_type = parsed.get("e")
            .or_else(|| parsed.get("data").and_then(|d| d.get("e")))
            .and_then(|v| v.as_str());
    
        if event_type == Some("kline") {
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
async fn test_binance_spot_pub_stream_kline_service() {
    let (svc, mut rx) = KlineService::new();

    let sample = r#"{
        "e": "kline",         
        "E": 1672515782136,   
        "s": "BNBBTC",       
        "k": {
            "t": 1672515780000, 
            "T": 1672515839999, 
            "s": "BNBBTC",     
            "i": "1m",         
            "f": 100,          
            "L": 200,           
            "o": "0.0010",     
            "c": "0.0020",     
            "h": "0.0025",      
            "l": "0.0015",      
            "v": "1000",       
            "n": 100,           
            "x": false,         
            "q": "1.0000",     
            "V": "500",         
            "Q": "0.500",       
            "B": "123456" 
        }
    }"#;

    svc.handle(sample).await.expect("kline handle event");

    let ev = rx.recv().await.expect("channel closed");
    assert_eq!(ev.data.close, 0.0020);
}