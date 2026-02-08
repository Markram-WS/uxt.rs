use super::model;

use tokio::sync::{mpsc};
use anyhow;
type Event = model::Depth;
#[allow(dead_code)]
#[derive(Clone)]
pub struct DepthService {
    tx: mpsc::Sender<Event>,
}

#[allow(dead_code)]
impl DepthService {
    pub fn new() -> (Self, mpsc::Receiver<Event>) {
        let (tx, rx) = mpsc::channel(100);
        (Self { tx }, rx)
    }

    pub async fn handle(&self,parsed:&serde_json::Value) ->anyhow::Result<()> {
        //let parsed: Value = serde_json::from_str(txt ?;
        if parsed.get("lastUpdateId").is_some() && parsed.get("bids").is_some()  && parsed.get("asks").is_some() {
            let ev = serde_json::from_value::<Event>(parsed.clone())?;
            self.tx.send(ev).await?;
            return Ok(());
        }else{
            Ok(())
        }
    }
}

#[tokio::test]
async fn test_binance_spot_user_stream_depth_service() {
    let (svc, mut rx) = DepthService::new();

    let sample = r#"{

        "lastUpdateId": 160,     // Last update ID
        "bids": [                // Bids to be updated
            [
                "0.0024",        // Price level to be updated
                "10"             // Quantity
            ]
        ],
        "asks": [                // Asks to be updated
            [
                "0.0026",        // Price level to be updated
                "100"            // Quantity
            ]
        ]

        }"#;
    let sample_json = serde_json::from_str(sample).unwrap();
    svc.handle(&sample_json).await.expect("DepthService handle event");
    let ev = rx.recv().await.expect("channel closed");
    let vec = &ev.bids[0].price;
    assert_eq!(vec, &0.0024);
}
