use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserDataSubscribeParams {
    pub api_key: String,
    pub timestamp: i64,
    pub signature: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserDataSubscribeRequest {
    pub id: String,
    pub method: String,
    pub params: UserDataSubscribeParams,
}
