use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ListenKeyResponse {
    pub listenKey: String,
}
