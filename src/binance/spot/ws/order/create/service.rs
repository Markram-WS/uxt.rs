use super::model;
use std::error::Error;
type Event = model::OrderCreat;
type Response = model::Response;
use crate::binance::spot::{WsClient};
use serde::ser::StdError;

#[allow(dead_code)]
#[derive(Clone)]
pub struct OrderCreatService {}
/// Calls the authenticated WebSocket API to create a new order.
///
/// This function submits a `createOrder` request through an already
/// authenticated [`WsClient`].  
/// On success, the order result will be processed by the service
/// and emitted through its event channel.
///
/// # Arguments
///
/// * `ws` - An authenticated WebSocket client.  
///   The client **must be logged in** before calling this function.
/// * `param` - JSON parameters required for the create order request:
///   * `symbol` (`String`) - Trading pair symbol (e.g. `"BTCUSDT"`)
///   * `side` (`String`) - Order side (`"BUY"` or `"SELL"`)
///   * `type` (`String`) - Order type (e.g. `"LIMIT"`, `"MARKET"`)
///   * `timeInForce` (`String`) - Order execution policy (e.g. `"GTC"`)
///   * `price` (`String`) - Order price (required for limit orders)
///   * `quantity` (`String`) - Order quantity
///   * `apiKey` (`String`) - API key for authentication
///   * `signature` (`String`) - Request signature
///   * `timestamp` (`u64`) - Request timestamp in milliseconds
///
/// # Errors
///
/// Returns an error if:
/// - The WebSocket request fails
/// - The exchange returns an order rejection or validation error
/// - The response cannot be parsed or handled correctly
///
/// # Example
///
/// ```rust,no_run
/// use serde_json::json;
///
/// let param = json!({
///     "symbol": "BTCUSDT",
///     "side": "SELL",
///     "type": "LIMIT",
///     "timeInForce": "GTC",
///     "price": "23416.10000000",
///     "quantity": "0.00847000",
///     "apiKey": "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
///     "signature": "xxxxxxxxxxxxxxxxxxxxxxx",
///     "timestamp": 1660801715431
/// });
/// 
/// let ev:model::OrderCreat = OrderCreateService::call(&ws, param).await?;
/// ```
///
/// # Notes
///
/// - This function performs network I/O and has side effects.
/// - Order validation (e.g. price/quantity precision) should be
///   handled before calling this function.
/// - Retry or reconciliation logic should be implemented at a higher level.

#[allow(dead_code)]
impl OrderCreatService {

    pub async fn call(mut ws:WsClient,param:serde_json::Value) -> Result<Event, Box<dyn StdError>> {
        let method = "order.place";
        log::debug!("{} param : {:#}", method, param);
        let res = ws.call_wsapi(method, param).await?;
        log::debug!("{} ok : {:#}", method, &res);
        Ok(OrderCreatService::handle(res).await?)
    }

    pub async fn handle( json: serde_json::Value) -> Result<Event, Box<dyn Error>> {
        let resp:Response = serde_json::from_value(json)?;
        if resp.status == 200 {
            Ok(resp.result)
        } else {
            Err(format!("unexpected status: {}", resp.status).into())
        }
    }
}

#[tokio::test]
async fn test_binance_spot_ws_order_creat_service() {
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

    
    let ev: Event = OrderCreatService::handle(serde_json::from_str(sample).expect("`Err` convert json value")).await.expect("OrderCreatService handle event");
    let order_id = &ev.order_id;
    let price = &ev.price;
    println!("{:?}",&ev);
    assert_eq!(order_id, &12569099453_i64);
    assert_eq!(price, &23416.10000000_f64);
}
