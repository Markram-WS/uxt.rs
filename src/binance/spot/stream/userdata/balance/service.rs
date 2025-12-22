use serde_json::Value;
use super::model;
use tokio::sync::{mpsc};
use std::error::Error;
type Event = model::Balance;
#[allow(dead_code)]
#[derive(Clone)]
pub struct BalanceService {
    tx: mpsc::Sender<Event>,
}

#[allow(dead_code)]
impl BalanceService {
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
    
        if event_type == Some("balanceUpdate") {
            let data = parsed.get("event").unwrap_or(&parsed);
            let ev = serde_json::from_value::<Event>(data.clone())?;
            self.tx.send(ev).await?;
            return Ok(());
        }else{
            Ok(())
        }
    
    
    }
}

#[tokio::test]
async fn test_binance_spot_user_stream_balance_service() {
    let (svc, mut rx) = BalanceService::new();

    let sample = r#"{
            "subscriptionId": 0,
            "event": {
            "e": "balanceUpdate",
            "E": 1573200697110,
            "a": "BTC",
            "d": "100.00000000",
            "T": 1573200697068
            }
        }"#;

    let sample_json = serde_json::from_str(sample).unwrap();
    svc.handle(&sample_json).await.expect("balanceUpdate handle event");

    let ev = rx.recv().await.expect("channel closed");
    assert_eq!(ev.balance_delta, 100.00000000);
}











