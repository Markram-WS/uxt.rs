use serde::{Deserialize};


#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Response {
    pub id: String,
    pub status: u16,
    pub result: serde_json::Value,
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
