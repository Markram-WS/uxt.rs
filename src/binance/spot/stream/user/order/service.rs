use serde_json::Value;
use super::model;
use tokio::sync::{mpsc};
use std::error::Error;
type Event = model::Order;
#[allow(dead_code)]
pub struct OrderService {
    tx: mpsc::Sender<Event>,
}

#[allow(dead_code)]
impl OrderService {
    pub fn new() -> (Self, mpsc::Receiver<Event>) {
        let (tx, rx) = mpsc::channel(100);
        (Self { tx }, rx)
    }

    pub async fn handle(&self,txt: &str) ->Result< (), Box<dyn Error>> {
        let parsed: Value = serde_json::from_str(txt)?;
    
        let event_type = parsed
            .get("event")
            .and_then(|ev| ev.get("e"))
            .and_then(|v| v.as_str());
        
        if event_type == Some("executionReport") {
            let data = parsed.get("event").unwrap_or(&parsed);
            let ev = serde_json::from_value::<model::Order>(data.clone())?;
            self.tx.send(ev).await?;
            return Ok(());
        }else{
            Ok(())
        }
    
    
    }
}

#[tokio::test]
async fn test_binance_spot_user_stream_order_service() {
    let (svc, mut rx) = OrderService::new();

    let sample = r#"
        {
        "subscriptionId": 0,
        "event": {
            "e": "executionReport",
            "E": 1499405658658,
            "s": "ETHBTC",
            "c": "mUvoqJxFIILMdfAW5iGSOW",
            "S": "BUY",
            "o": "LIMIT",
            "f": "GTC",
            "q": "1.00000000",
            "p": "0.10264410",
            "P": "0.00000000",
            "F": "0.00000000",
            "g": -1,
            "C": "",
            "x": "NEW",
            "X": "NEW",
            "r": "NONE",
            "i": 4293153,
            "l": "0.00000000",
            "z": "0.00000000",
            "L": "0.00000000",
            "n": "0",
            "N": null,
            "T": 1499405658657,
            "t": -1,
            "v": 3,
            "I": 8641984,
            "w": true,
            "m": false,
            "M": false,
            "O": 1499405658657,
            "Z": "0.00000000",
            "Y": "0.00000000",
            "Q": "0.00000000",
            "W": 1499405658657,
            "V": "NONE"
        }
        }
        "#;

    svc.handle(sample).await.expect("executionReport handle event");

    let ev = rx.recv().await.expect("channel closed");
    assert_eq!(ev.order_id, 4293153);
}




