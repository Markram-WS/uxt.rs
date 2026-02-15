

use serde::{Deserialize};
use crate::utils::convert::{str_to_f64};

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Response {
    pub id: String,
    pub status: u16,
    pub result: OrderCreat,
    #[serde(rename = "rateLimits")]
    pub rate_limits: Vec<RateLimit>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct RateLimit {
    #[serde(rename = "rateLimitType")]
    pub rate_limit_type: String,
    pub interval: String,
    #[serde(rename = "intervalNum")]
    pub interval_num: u32,
    pub limit: u32,
    pub count: u32,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize,Clone)]
pub struct OrderCreat {
    pub symbol: String,

    #[serde(rename = "orderId")]
    pub order_id: i64,

    #[serde(rename = "orderListId")]
    pub order_list_id: i64,

    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,

    #[serde(rename = "transactTime")]
    pub transact_time: i64,

    #[serde(deserialize_with = "str_to_f64")]
    pub price: f64,

    #[serde(rename = "origQty", deserialize_with = "str_to_f64")]
    pub orig_qty: f64,

    #[serde(rename = "executedQty", deserialize_with = "str_to_f64")]
    pub executed_qty: f64,

    #[serde(rename = "origQuoteOrderQty", deserialize_with = "str_to_f64")]
    pub orig_quote_order_qty: f64,

    #[serde(rename = "cummulativeQuoteQty", deserialize_with = "str_to_f64")]
    pub cummulative_quote_qty: f64,

    pub status: String,

    #[serde(rename = "timeInForce")]
    pub time_in_force: String,

    #[serde(rename = "type")]
    pub order_type: String,

    pub side: String,

    #[serde(rename = "workingTime")]
    pub working_time: i64,

    #[serde(rename = "selfTradePreventionMode")]
    pub self_trade_prevention_mode: String,
}


