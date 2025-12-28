use super::model;
use tokio::sync::{mpsc};
use std::error::Error;
use serde::ser::StdError;
use log;
type Event = model::Kline;
type Response = model::Response;
use crate::binance::spot::{WsClient};
#[allow(dead_code)]
#[derive(Clone)]
pub struct KlineService {
    tx: mpsc::Sender<Vec<Event>>,
}

impl From<model::RawKline> for model::Kline {
    fn from(r: model::RawKline) -> Self {
        Self {
            open_time: r.0,
            open: r.1,
            high: r.2,
            low: r.3,
            close: r.4,
            volume: r.5,
            close_time: r.6,
            quote_volume: r.7,
            trades: r.8,
            taker_buy_base_volume: r.9,
            taker_buy_quote_volume: r.10,
        }
    }
}

#[allow(dead_code)]
    impl KlineService {
/// Call websocket API to fetch kline (candlestick) data.
///
/// # Arguments
/// - `ws` - Authenticated websocket client
/// - `param` - JSON parameters for kline request:
///   - `symbol` (`String`) : trading pair symbol (e.g. "BTCUSDT")
///   - `interval` (`String`) : kline interval (e.g. "1h")
///   - `startTime` (`u64`) : unix timestamp (seconds)
///   - `limit` (`u32`) : number of klines to fetch
///
/// # Example
/// ```rust
/// let param = serde_json::json!({
///     "symbol": "BTCUSDT",
///     "interval": "1h",
///     "startTime": 1710000000,
///     "limit": 1
/// });
/// let (svc, mut rx) = KlineService::new();
/// svc::call(&ws, param).await?;
/// let ev: Vec<model::Kline> = rx.recv().await?;
/// ```
    pub fn new() -> (Self, mpsc::Receiver<Vec<Event>>) {
        let (tx, rx) = mpsc::channel(100);
        (Self { tx }, rx)
    }

    pub async fn call(self,ws:&mut WsClient,param:serde_json::Value) -> Result<(), Box<dyn StdError>> {
        let method = "klines";
        log::debug!("{} param : {:#}", method, param);
        let res = ws.call_wsapi(method, param).await?;
        log::debug!("{} ok : {:#}", method, res);
        self.handle(res).await?;
        Ok(())
    }

    pub async fn handle(&self, json: serde_json::Value) -> Result<(), Box<dyn Error>> {
        let resp:Response = serde_json::from_value(json)?;
        let klines: Vec<Event> = resp
            .result
            .into_iter()
            .map(|r| model::Kline::from(r))
            .collect();
        if resp.status == 200 {
            self.tx.send(klines).await?;
        }
        Ok(())

    }
}

#[tokio::test]
async fn test_binance_spot_ws_kline_service() {
    let (svc, mut rx) = KlineService::new();

    let sample = r#"{
        "id": "1dbbeb56-8eea-466a-8f6e-86bdcfa2fc0b",
        "status": 200,
        "result": [
            [
            1655971200000,
            "0.01086000",
            "0.01086600",
            "0.01083600",
            "0.01083800",
            "2290.53800000",
            1655974799999,
            "24.85074442",
            2283,
            "1171.64000000",
            "12.71225884",
            "0"
            ]
        ],
        "rateLimits": [
            {
            "rateLimitType": "REQUEST_WEIGHT",
            "interval": "MINUTE",
            "intervalNum": 1,
            "limit": 6000,
            "count": 2
            }
        ]
        }"#;

    svc.handle( serde_json::from_str(sample).expect("`Err` convert json value") ).await.expect("KlineService handle event");
    let ev: Vec<model::Kline> = rx.recv().await.expect("channel closed");
    let open_time = &ev[0].open_time;
    let open_price = &ev[0].open;
    println!("{:?}",&ev[0]);
    assert_eq!(open_time, &1655971200000_i64);
    assert_eq!(open_price, &0.0108600_f64);
}
