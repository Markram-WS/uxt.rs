use serde_json::Value;
use super::model;
use tokio::sync::{mpsc};
use std::error::Error;

type Event = model::OrderCreat;
type Response = model::Response;
#[allow(dead_code)]
#[derive(Clone)]
pub struct OrderCreatService {
    tx: mpsc::Sender<Event>,
}

#[allow(dead_code)]
impl OrderCreatService {
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
async fn test_binance_spot_ws_order_creat_service() {
    let (svc, mut rx) = OrderCreatService::new();

    let sample = r#"
        {
        "id": "56374a46-3061-486b-a311-99ee972eb648",
        "status": 200,
        "result": {
            "symbol": "BTCUSDT",
            "orderId": 12569099453,
            "orderListId": -1,
            "clientOrderId": "4d96324ff9d44481926157ec08158a40",
            "transactTime": 1660801715639,
            "price": "23416.10000000",
            "origQty": "0.00847000",
            "executedQty": "0.00000000",
            "origQuoteOrderQty": "0.000000",
            "cummulativeQuoteQty": "0.00000000",
            "status": "NEW",
            "timeInForce": "GTC",
            "type": "LIMIT",
            "side": "SELL",
            "workingTime": 1660801715639,
            "selfTradePreventionMode": "NONE"
        },
        "rateLimits": [
            {
            "rateLimitType": "ORDERS",
            "interval": "SECOND",
            "intervalNum": 10,
            "limit": 50,
            "count": 1
            },
            {
            "rateLimitType": "ORDERS",
            "interval": "DAY",
            "intervalNum": 1,
            "limit": 160000,
            "count": 1
            },
            {
            "rateLimitType": "REQUEST_WEIGHT",
            "interval": "MINUTE",
            "intervalNum": 1,
            "limit": 6000
        ,
            "count": 1
            }
        ]
        }
    "#;

    svc.handle(sample).await.expect("OrderCreatService handle event");
    let ev: Event = rx.recv().await.expect("channel closed");
    let order_id = &ev.order_id;
    let price = &ev.price;
    println!("{:?}",&ev);
    assert_eq!(order_id, &12569099453_i64);
    assert_eq!(price, &23416.10000000_f64);
}
