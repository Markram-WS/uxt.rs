use crate::utils::convert::{str_to_f64};
use serde::{Deserialize, Serialize};

pub struct BalanceWrapper {
    pub subscriptionId: u64,
    pub event: Balance,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Balance {
    #[serde(rename = "e")]
    pub event_type : String,
    #[serde(rename = "E")]
    pub event_time : i64,
    #[serde(rename = "a")]
    pub asset : String,
    #[serde(rename = "d",deserialize_with = "str_to_f64")]
    pub balance_delta : f64,
    #[serde(rename = "T")]
    pub clear_time : i64,
}
