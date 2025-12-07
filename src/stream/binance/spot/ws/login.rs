use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct LoginParams {
    pub(crate) api_key:String,
    pub(crate) signature: String,
    pub(crate) timestamp: i64,
}

#[derive(Debug, Serialize)]
pub struct LoginRequest {
    pub(crate) id: String,
    pub(crate) method: String,
    pub(crate) params: LoginParams,
}
