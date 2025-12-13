use uuid::Uuid;
use chrono::Utc;

use crate::binance::spot::transport::signer::sign;
use super::model::{UserDataSubscribeRequest, UserDataSubscribeParams};
use crate::binance::spot::transport::ws::WsClient;

pub struct UserDataAuthService;

impl UserDataAuthService {
    pub async fn subscribe(client: &mut WsClient) -> anyhow::Result<()> {
        // timestamp
        let ts = Utc::now().timestamp_millis();

        // query = "timestamp=<ts>"
        let query = format!("timestamp={}", ts);
        let signature = sign(&client.secret, &query);

        let req = UserDataSubscribeRequest {
            id: Uuid::new_v4().to_string(),
            method: "userDataStream.subscribe.signature".into(),
            params: UserDataSubscribeParams {
                apiKey: client.api_key.clone(),
                timestamp: ts,
                signature,
            },
        };

        client.send_json(&req).await?;
        Ok(())
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
