use serde_json::Value;
use super::model;
use tokio::sync::{mpsc};
use std::error::Error;

type Event = model::Time;
type Response = model::Response;
#[allow(dead_code)]
#[derive(Clone)]
pub struct TimeService {
    tx: mpsc::Sender<Event>,
}

#[allow(dead_code)]
impl TimeService {
    pub fn new() -> (Self, mpsc::Receiver<Event>) {
        let (tx, rx) = mpsc::channel(100);
        (Self { tx }, rx)
    }

    pub async fn handle(&self, txt: &str) -> Result<(), Box<dyn Error>> {
        let resp: Response = serde_json::from_str(txt)?;
        if resp.status == 200 {
            self.tx.send(resp.result).await?;
        }
        Ok(())

    }
}

#[tokio::test]
async fn test_binance_spot_ws_time_service() {
    let (svc, mut rx) = TimeService::new();

    let sample = r#"
        {
        "id": "922bcc6e-9de8-440d-9e84-7c80933a8d0d",
        "status": 200,
        "result": {
            "serverTime": 1656400526260
            },
        "rateLimits": [
            {
            "rateLimitType": "REQUEST_WEIGHT",
            "interval": "MINUTE",
            "intervalNum": 1,
            "limit": 6000,
            "count": 1
            }
        ]
        }
    "#;

    svc.handle(sample).await.expect("TimeService handle event");
    let ev: Event = rx.recv().await.expect("channel closed");
    let time = &ev.serverTime;
    println!("{:?}",&ev);
    assert_eq!(time, &1656400526260_i64);
}
