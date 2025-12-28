use super::model;
use tokio::sync::{mpsc};
use std::error::Error;
type Event = model::Ticker;
type Response = model::Response;
use crate::binance::spot::{WsClient};
use serde::ser::StdError;
#[allow(dead_code)]
#[derive(Clone)]
pub struct TickerService {
    tx: mpsc::Sender<Event>,
}
/// Call websocket API to fetch Ticker data.
///
/// # Arguments
/// - `ws` - Authenticated websocket client
/// - `param` - JSON parameters for kline request:
///   - `symbol` (`String`) : trading pair symbol (e.g. "BTCUSDT")
///
/// # Example
/// ```rust
/// let param = serde_json::json!({
///     "symbol": "BTCUSDT",
/// });
/// let (svc, mut rx) = TickerService::new();
/// svc::call(&ws, param).await?;
/// let ev: model::Ticker = rx.recv().await?;
/// ```
#[allow(dead_code)]
impl TickerService {
    pub fn new() -> (Self, mpsc::Receiver<Event>) {
        let (tx, rx) = mpsc::channel(100);
        (Self { tx }, rx)
    }

    pub async fn call(self,mut ws:WsClient,param:serde_json::Value) -> Result<(), Box<dyn StdError>> {
        let method = "ticker.24hr";
        log::debug!("{} param : {:#}", method, param);
        let res = ws.call_wsapi(method, param).await?;
        log::debug!("{} ok : {:#}", method, res);
        self.handle(res).await?;
        Ok(())
    }

    pub async fn handle(&self, json: serde_json::Value) -> Result<(), Box<dyn Error>> {
        let resp:Response = serde_json::from_value(json)?;
        if resp.status == 200 {
            self.tx.send(resp.result).await?;
        }
        Ok(())

    }
}

#[tokio::test]
async fn test_binance_spot_ws_ticker_service() {
    let (svc, mut rx) = TickerService::new();

    let sample = r#"
        {
        "id": "93fb61ef-89f8-4d6e-b022-4f035a3fadad",
        "status": 200,
        "result": {
            "symbol": "BNBBTC",
            "priceChange": "0.00013900",
            "priceChangePercent": "1.020",
            "weightedAvgPrice": "0.01382453",
            "prevClosePrice": "0.01362800",
            "lastPrice": "0.01376700",
            "lastQty": "1.78800000",
            "bidPrice": "0.01376700",
            "bidQty": "4.64600000",
            "askPrice": "0.01376800",
            "askQty": "14.31400000",
            "openPrice": "0.01362800",
            "highPrice": "0.01414900",
            "lowPrice": "0.01346600",
            "volume": "69412.40500000",
            "quoteVolume": "959.59411487",
            "openTime": 1660014164909,
            "closeTime": 1660100564909,
            "firstId": 194696115,       
            "lastId": 194968287,       
            "count": 272173             
        },
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

    svc.handle(serde_json::from_str(sample).expect("`Err` convert json value")).await.expect("TickerService handle event");
    let ev: Event = rx.recv().await.expect("channel closed");
    let last_price = &ev.last_price;
    let symbol: &String = &ev.symbol;
    println!("{:?}",&ev);
    assert_eq!(last_price, &0.01376700_f64);
    assert_eq!(symbol, &"BNBBTC");
}
