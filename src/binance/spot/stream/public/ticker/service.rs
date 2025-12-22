use serde_json::Value;
use super::model;
use tokio::sync::{mpsc};
use std::error::Error;
type Event = model::Ticker;
#[allow(dead_code)]
#[derive(Clone)]
pub struct TickerService {
    tx: mpsc::Sender<Event>,
}

#[allow(dead_code)]
impl TickerService {
    pub fn new() -> (Self, mpsc::Receiver<Event>) {
        let (tx, rx) = mpsc::channel(100);
        (Self { tx }, rx)
    }

    pub async fn handle(&self,parsed:&serde_json::Value) ->Result< (), Box<dyn Error>> {
        //let parsed: Value = serde_json::from_str(txt)?;
    
        let event_type = parsed.get("e")
            .or_else(|| parsed.get("data").and_then(|d| d.get("e")))
            .and_then(|v| v.as_str());
    
        if event_type == Some("24hrTicker") {
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
async fn test_binance_spot_pub_stream_ticker_service() {
    let (svc, mut rx) = TickerService::new();

    let sample = r#"{
        "e": "24hrTicker",  
        "E": 1672515782136, 
        "s": "BNBBTC",      
        "p": "0.0015",      
        "P": "250.00",      
        "w": "0.0018",      
        "x": "0.0009",      
        "c": "0.0025",      
        "Q": "10",          
        "b": "0.0024",      
        "B": "10",          
        "a": "0.0026",      
        "A": "100",      
        "o": "0.0010",     
        "h": "0.0025",   
        "l": "0.0010",     
        "v": "10000",
        "q": "18", 
        "O": 0,
        "C": 86400000,
        "F": 0,
        "L": 18150,
        "n": 18151
        }"#;
    let sample_json = serde_json::from_str(sample).unwrap();
    svc.handle(&sample_json).await.expect("ticker handle event");

    let ev = rx.recv().await.expect("channel closed");
    assert_eq!(ev.last_price, 0.0025);
}


