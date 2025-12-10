use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsRequest<T> {
    pub id: u64,
    pub method: String,
    pub params: T,
}

// ----------------------------
//  Request Payloads
// ----------------------------

// ไม่มี params → ต้องเป็น object ว่าง {}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Empty {}

pub type AccountStatusRequest = WsRequest<Empty>;
pub type AccountBalanceRequest = WsRequest<Empty>;
pub type OpenOrdersRequest = WsRequest<Empty>;

pub fn build_account_status_request(id: u64) -> AccountStatusRequest {
    WsRequest {
        id,
        method: "account.status".into(),
        params: Empty {},
    }
}
