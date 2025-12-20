

use serde::{Deserialize};
use crate::utils::convert::{str_to_f64};

#[derive(Debug, Deserialize)]
pub struct KlineResponse {
    pub id: String,
    pub status: u16,
    pub result: Vec<RawKline>,
    #[serde(rename = "rateLimits")]
    pub rate_limits: Vec<RateLimit>,
}

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

#[derive(Debug, Deserialize)]
pub struct RawKline(
    pub i64,
    #[serde(deserialize_with = "str_to_f64")] pub f64,
    #[serde(deserialize_with = "str_to_f64")] pub f64,
    #[serde(deserialize_with = "str_to_f64")] pub f64,
    #[serde(deserialize_with = "str_to_f64")] pub f64,
    #[serde(deserialize_with = "str_to_f64")] pub f64,
    pub i64,
    #[serde(deserialize_with = "str_to_f64")] pub f64,
    pub u64,
    #[serde(deserialize_with = "str_to_f64")] pub f64,
    #[serde(deserialize_with = "str_to_f64")] pub f64,
    pub serde_json::Value,
);

#[derive(Debug, Deserialize)]
pub struct Kline {
    pub open_time: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub close_time: i64,
    pub quote_volume: f64,
    pub trades: u64,
    pub taker_buy_base_volume: f64,
    pub taker_buy_quote_volume: f64,
}

