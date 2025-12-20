use serde_json::Value;
use super::model;
use tokio::sync::{mpsc};
use std::error::Error;

#[derive(Debug, PartialEq)]
pub enum Event {
    Pong,
}
type Response = model::Response;


#[allow(dead_code)]
#[derive(Clone)]
pub struct PingService {
    tx: mpsc::Sender<Event>,
}

#[allow(dead_code)]
impl PingService {
    pub fn new() -> (Self, mpsc::Receiver<Event>) {
        let (tx, rx) = mpsc::channel(100);
        (Self { tx }, rx)
    }

    pub async fn handle(&self, txt: &str) -> Result<(), Box<dyn Error>> {
        let resp: Response = serde_json::from_str(txt)?;
        if resp.status == 200 {
            self.tx.send(Event::Pong).await?;
        }
        Ok(())

    }
}

#[tokio::test]
async fn test_binance_spot_ws_ping_service() {
    let (svc, mut rx) = PingService::new();

    let sample = r#"
        {
        "id": "93fb61ef-89f8-4d6e-b022-4f035a3fadad",
        "status": 200,
        "result": {},
        "rateLimits": [
            {
            "rateLimitType": "REQUEST_WEIGHT",
            "interval": "MINUTE",
            "intervalNum": 1,
            "limit": 6000,
            "count": 2
            }
        ]
        }
    "#;

    svc.handle(sample).await.expect("TickerService handle event");
    let ev: Event = rx.recv().await.expect("channel closed");
    println!("{:?}",&ev);
    assert_eq!(ev, Event::Pong);
}
