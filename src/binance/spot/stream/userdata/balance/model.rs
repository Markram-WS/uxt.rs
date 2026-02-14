use crate::utils::convert::{str_to_f64};
use serde::{Deserialize, Serialize};
#[allow(dead_code)]
#[derive(Serialize)]
pub struct BalanceWrapper {
    #[serde(rename = "subscriptionId")]
    pub subscription_id: u64,
    pub event: Balance,
}


#[derive(Debug, Serialize, Deserialize,Clone)]
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
