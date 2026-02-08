use super::model;
use std::error::Error;
type Event = model::OrderCancel;
type Response = model::Response;
use crate::binance::spot::{WsClient};
use serde::ser::StdError;
use anyhow;
#[allow(dead_code)]
#[derive(Clone)]
pub struct OrderCancelService {
}
/// Calls the authenticated WebSocket API to cancel an order.
///
/// This function sends a `cancelOrder` request through an already
/// authenticated [`WsClient`] and processes the response internally.
/// On success, the result will be forwarded to the service handler
/// (e.g. through a channel).
///
/// # Arguments
///
/// * `ws` - An authenticated WebSocket client.  
///   The client **must be logged in** before calling this function.
/// * `param` - JSON parameters required for the cancel order request:
///   * `symbol` (`String`) - Trading pair symbol (e.g. `"BTCUSDT"`)
///   * `origClientOrderId` (`String`) - Client order ID to be canceled
///   * `apiKey` (`String`) - API key for authentication
///   * `signature` (`String`) - Request signature
///   * `timestamp` (`u64`) - Request timestamp in milliseconds
///
/// # Errors
///
/// Returns an error if:
/// - The WebSocket request fails
/// - The server responds with an error
/// - The response cannot be handled or parsed correctly
///
/// # Example
///
/// ```rust,no_run
/// use serde_json::json;
///
/// let param = json!({
///     "symbol": "BTCUSDT",
///     "origClientOrderId": "4d96324ff9d44481926157",
///     "apiKey": "xxxxxxxx",
///     "signature": "xxxxxxxx",
///     "timestamp": 1660801715830
/// });
///
/// let ev:model::OrderCancel = OrderCancelService.call(&ws, param).await?;
/// ```
///
/// # Notes
///
/// - This function performs side effects (network I/O).
/// - The caller is responsible for managing retries and error handling
///   at a higher level.
#[allow(dead_code)]
impl OrderCancelService {

    pub async fn call(ws:&mut WsClient,param:serde_json::Value) -> anyhow::Result<Event>  {
        let method = "order.cancel";
        log::debug!("{} param : {:#}", method, param);
        let param_siged = ws.role.sign_wsapi(param)?;
        let res = ws.call_wsapi(method, param_siged).await?;
        log::debug!("{} ok : {:#}", method, &res);
        Ok(OrderCancelService::handle(res).await?)
    }

    pub async fn handle( json: serde_json::Value) -> anyhow::Result<Event> {
        let resp:Response = serde_json::from_value(json)?;
        if resp.status == 200 {
            Ok(resp.result)
        } else {
            anyhow::bail!("order.cancel, unexpected status: {}", resp.status)
        }
    }
}

#[tokio::test]
async fn test_binance_spot_ws_order_cancle_service() {
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

    let ev: Event = OrderCancelService::handle(serde_json::from_str(sample).expect("`Err` convert json value") ).await.expect("OrderCancelService handle event");
    let order_id = &ev.order_id;
    let price = &ev.price;
    println!("{:?}",&ev);
    assert_eq!(order_id, &12569099453_i64);
    assert_eq!(price, &23416.10000000_f64);
}
