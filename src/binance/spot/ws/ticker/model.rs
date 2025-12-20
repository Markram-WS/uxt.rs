

use serde::{Deserialize};
use crate::utils::convert::{str_to_f64};

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Response {
    pub id: String,
    pub status: u16,
    pub result: Ticker,
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
#[derive(Debug, Deserialize)]
pub struct Ticker {
    pub symbol: String,

    #[serde(rename = "priceChange", deserialize_with = "str_to_f64")]
    pub price_change: f64,

    #[serde(rename = "priceChangePercent", deserialize_with = "str_to_f64")]
    pub price_change_percent: f64,

    #[serde(rename = "weightedAvgPrice", deserialize_with = "str_to_f64")]
    pub weighted_avg_price: f64,

    #[serde(rename = "prevClosePrice", deserialize_with = "str_to_f64")]
    pub prev_close_price: f64,

    #[serde(rename = "lastPrice", deserialize_with = "str_to_f64")]
    pub last_price: f64,

    #[serde(rename = "lastQty", deserialize_with = "str_to_f64")]
    pub last_qty: f64,

    #[serde(rename = "bidPrice", deserialize_with = "str_to_f64")]
    pub bid_price: f64,

    #[serde(rename = "bidQty", deserialize_with = "str_to_f64")]
    pub bid_qty: f64,

    #[serde(rename = "askPrice", deserialize_with = "str_to_f64")]
    pub ask_price: f64,

    #[serde(rename = "askQty", deserialize_with = "str_to_f64")]
    pub ask_qty: f64,

    #[serde(rename = "openPrice", deserialize_with = "str_to_f64")]
    pub open_price: f64,

    #[serde(rename = "highPrice", deserialize_with = "str_to_f64")]
    pub high_price: f64,

    #[serde(rename = "lowPrice", deserialize_with = "str_to_f64")]
    pub low_price: f64,

    #[serde(rename = "volume", deserialize_with = "str_to_f64")]
    pub volume: f64,

    #[serde(rename = "quoteVolume", deserialize_with = "str_to_f64")]
    pub quote_volume: f64,

    #[serde(rename = "openTime")]
    pub open_time: i64,

    #[serde(rename = "closeTime")]
    pub close_time: i64,

    #[serde(rename = "firstId")]
    pub first_trade_id: u64,

    #[serde(rename = "lastId")]
    pub last_trade_id: u64,

    pub count: u64,
}
