use super::model;
use std::error::Error;
type Event = model::Ticker;
type Response = model::Response;
use crate::binance::spot::{WsClient};
use serde::ser::StdError;
#[allow(dead_code)]
#[derive(Clone)]
pub struct TickerService {}
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
/// 
/// let ev: model::Ticker = TickerService::call(&ws, param).await?;
/// ```
#[allow(dead_code)]
impl TickerService {
    pub async fn call(ws:&mut WsClient,param:serde_json::Value) -> Result<Event, Box<dyn StdError>> {
        let method = "ticker.24hr";
        log::debug!("{} param : {:#}", method, param);
        let res = ws.call_wsapi(method, param).await?;
        log::debug!("{} ok : {:#}", method, &res);
        Ok(TickerService::handle(res).await?)
    }

    pub async fn handle( json: serde_json::Value) -> Result<Event, Box<dyn Error>> {
        let resp: Response = serde_json::from_value(json)?;
        if resp.status == 200 {
            Ok(resp.result)
        } else {
            Err(format!("unexpected status: {}", resp.status).into())
        }
    }
}

#[tokio::test]
async fn test_binance_spot_ws_ticker_service() {
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

    
    let ev: Event = TickerService::handle(
        serde_json::from_str(sample).expect("`Err` convert json value")
    ).await.expect("TickerService handle event");
    let last_price = &ev.last_price;
    let symbol: &String = &ev.symbol;
    println!("{:?}",&ev);
    assert_eq!(last_price, &0.01376700_f64);
    assert_eq!(symbol, &"BNBBTC");
}
