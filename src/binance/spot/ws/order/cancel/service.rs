use serde_json::Value;
use super::model;
use tokio::sync::{mpsc};
use std::error::Error;

type Event = model::OrderCancel;
type Response = model::Response;
#[allow(dead_code)]
#[derive(Clone)]
pub struct OrderCancelService {
    tx: mpsc::Sender<Event>,
}

#[allow(dead_code)]
impl OrderCancelService {
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
async fn test_binance_spot_ws_order_cancle_service() {
    let (svc, mut rx) = OrderCancelService::new();

    let sample = r#"
        {
        "id": "5633b6a2-90a9-4192-83e7-925c90b6a2fd",
        "status": 200,
        "result": {
            "symbol": "BTCUSDT",
            "origClientOrderId": "4d96324ff9d44481926157",  
            "orderId": 12569099453,
            "orderListId": -1,                
            "clientOrderId": "91fe37ce9e69c90d6358c0",  
            "transactTime": 1684804350068,
            "price": "23416.10000000",
            "origQty": "0.00847000",
            "executedQty": "0.00001000",
            "origQuoteOrderQty": "0.000000",
            "cummulativeQuoteQty": "0.23416100",
            "status": "CANCELED",
            "timeInForce": "GTC",
            "type": "LIMIT",
            "side": "SELL",
            "stopPrice": "0.00000000",         
            "trailingDelta": 0,               
            "icebergQty": "0.00000000",    
            "strategyId": 37463720,       
            "strategyType": 1000000, 
            "selfTradePreventionMode": "NONE"
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

    svc.handle(sample).await.expect("OrderCancelService handle event");
    let ev: Event = rx.recv().await.expect("channel closed");
    let order_id = &ev.order_id;
    let price = &ev.price;
    println!("{:?}",&ev);
    assert_eq!(order_id, &12569099453_i64);
    assert_eq!(price, &23416.10000000_f64);
}
