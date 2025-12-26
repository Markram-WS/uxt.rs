use serde::{Deserialize,Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ListenKeyResponse {
    #[serde(rename = "listenKey")]
    pub listen_key: String,
}
