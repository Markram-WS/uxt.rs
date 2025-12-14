use chrono::Utc;
use serde_json::Value;
use crate::utils::sign;

pub enum WsRole {
    Public,
    UserData,
    WsApi {
        api_key: String,
        secret: String,
    },
}

impl WsRole {
    pub fn api_key(&self) -> Result<&str, anyhow::Error>{
        match self {
            WsRole::WsApi { api_key, .. } => Ok(api_key),
            _ => anyhow::bail!("api_key not available"),
        }
    }
    pub fn secret(&self) -> Result<&str, anyhow::Error> {
        match self {
            WsRole::WsApi { secret, .. } => Ok(secret),
            _ => anyhow::bail!("secret not available"),
        }
    }


    pub fn sign_wsapi(&self, mut params: Value) -> anyhow::Result<Value> {
        match self {
            WsRole::WsApi { secret, .. } => {
                let ts = Utc::now().timestamp_millis();
                params["timestamp"] = ts.into();

                let query = format!("timestamp={}", ts);
                let sig = sign(secret, &query);
                params["signature"] = sig.into();

                Ok(params)
            }
            _ => anyhow::bail!("not ws-api role"),
        }
    }
}
