use super::model;
use tokio::sync::{mpsc};
use std::error::Error;
type Event = model::Account;
#[allow(dead_code)]
#[derive(Clone)]
pub struct AccountService {
    tx: mpsc::Sender<Event>,
}

#[allow(dead_code)]
impl AccountService {
    pub fn new() -> (Self, mpsc::Receiver<Event>) {
        let (tx, rx) = mpsc::channel(100);
        (Self { tx }, rx)
    }

    pub async fn handle(&self,parsed:&serde_json::Value) ->Result< (), Box<dyn Error>> {
        //let parsed: Value = serde_json::from_str(txt)?;

        let event_type = parsed
            .get("event")
            .and_then(|ev| ev.get("e"))
            .and_then(|v| v.as_str());

        if event_type == Some("outboundAccountPosition") {
            let data = parsed.get("event").unwrap_or(&parsed);
            let ev = serde_json::from_value::<model::Account>(data.clone())?;
            self.tx.send(ev).await?;
            return Ok(());
        }else{
            Ok(())
        }
    
    
    }
}

#[tokio::test]
async fn test_binance_spot_user_stream_account_service() {
    let (svc, mut rx) = AccountService::new();

    let sample = r#"{
        "subscriptionId": 0,
        "event": {
            "e": "outboundAccountPosition", 
            "E": 1564034571105,             
            "u": 1564034571073,             
            "B":                            
            [
                {
                    "a": "ETH",                
                    "f": "10000.000000",        
                    "l": "0.000000"            
                }
            ]
        }
        }"#;
    let sample_json = serde_json::from_str(sample).unwrap();
    svc.handle(&sample_json).await.expect("outboundAccountPosition handle event");
    let ev = rx.recv().await.expect("channel closed");
    let vec = &ev.balances[0];
    assert_eq!(vec.free, 10000.000000);
}
