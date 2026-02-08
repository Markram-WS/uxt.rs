use serde::{Deserialize, Serialize};
#[allow(dead_code)]
#[derive(Serialize)]
#[derive(Debug, Deserialize)]

pub struct PriceStep {
    pub price: f64,
    pub quantity: f64,
}

impl From<(String, String)> for PriceStep {
    fn from(data: (String, String)) -> Self {
        Self {
            price: data.0.parse().unwrap_or(0.0), 
            quantity: data.1.parse().unwrap_or(0.0),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Depth {
    #[serde(rename = "lastUpdateId")]
    pub last_update_id: i64,
    pub bids: Vec<PriceStep>,
    pub asks: Vec<PriceStep>,
}