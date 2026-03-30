use serde_json::json;
use crate::binance::spot::WsClient;

pub struct UserDataAuthService;

impl UserDataAuthService {
    pub async fn subscribe(client: &mut WsClient) -> anyhow::Result<()> {
        match client.call_wsapi("userDataStream.subscribe",json!({}) ).await  {
            Ok(_) => {
                log::info!("> subscribe userDataStream");
                Ok(())
            }
            Err(e) => {
                log::error!("Err userDataStream.subscribe : {}",e);
                Err(e)
            }
        }
    }
}





// Example UserDataEventHandler
// use serde_json::Value;
// use log::info;

// pub struct UserDataEventHandler;

// impl UserDataEventHandler {
//     pub fn handle(msg: &str) {
//         let Ok(json) = serde_json::from_str::<Value>(msg) else {
//             info!("Invalid JSON from WS: {}", msg);
//             return;
//         };

//         if let Some(event_type) = json.get("e").and_then(|v| v.as_str()) {
//             match event_type {
//                 "balanceUpdate" => Self::balance_update(&json),
//                 "executionReport" => Self::execution_report(&json),
//                 "outboundAccountPosition" => Self::outbound_position(&json),
//                 _ => info!("Unknown event type: {}", event_type),
//             }
//         } else {
//             info!("No event type in WS payload: {}", msg);
//         }
//     }

//     fn balance_update(v: &Value) {
//         let asset = v["a"].as_str().unwrap_or("-");
//         let balance = v["d"].as_str().unwrap_or("-");
//         info!("Balance Update → {} = {}", asset, balance);
//     }

//     fn execution_report(v: &Value) {
//         let symbol = v["s"].as_str().unwrap_or("-");
//         let side   = v["S"].as_str().unwrap_or("-");
//         let price  = v["p"].as_str().unwrap_or("-");

//         info!("Order → {}, side {}, price {}", symbol, side, price);
//     }

//     fn outbound_position(v: &Value) {
//         info!("Outbound account position: {:?}", v);
//     }
// }
