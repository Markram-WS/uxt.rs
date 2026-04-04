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

        // ตรวจสอบก่อนว่าข้อมูลที่ส่งมาถูกห่ออยู่ในฟิลด์ "data" หรือไม่
        let data = parsed.get("data").unwrap_or(parsed);

        
        if data.get("lastUpdateId").is_some() && data.get("bids").is_some() && data.get("asks").is_some() {
            //แปลงค่าจาก 'data' (ที่เป็นก้อนข้อมูล Depth จริงๆ)
            let ev = serde_json::from_value::<Event>(data.clone())?;
            self.tx.send(ev).await?;
        return Ok(());
        } else {
        Ok(())
        }

    }
}

#[tokio::test]
async fn test_binance_spot_user_stream_depth_service() {
    let (svc, mut rx) = DepthService::new();

    let sample = r#"{

        "lastUpdateId": 160, 
        "bids": [              
            [
                "0.0024", 
                "10"       
            ]
        ],
        "asks": [              
            [
                "0.0026",     
                "100"
            ]
        ]

        }"#;
    let sample_json = serde_json::from_str(sample).unwrap();
    svc.handle(&sample_json).await.expect("DepthService handle event");
    let ev = rx.recv().await.expect("channel closed");
    let vec = &ev.bids[0].price;
    assert_eq!(vec, &0.0024);
}
