use chrono::Utc;
use serde_json::Value;
use crate::utils::sign;
use std::fmt::Debug;

#[derive(Debug)]
pub enum WsRole {
    Public,
    UserData{
        api_key: String,
        secret: String,
    },
    WsApi {
        api_key: String,
        secret: String,
    },
}

impl WsRole {
    pub fn api_key(&self) -> Result<&str, anyhow::Error>{
        match self {
            WsRole::WsApi {  api_key,.. } => Ok(&api_key) ,
            WsRole::UserData {  api_key,.. } => Ok(&api_key),
            WsRole::Public =>anyhow::bail!("public api_key not available")
        }
    }

    pub fn secret(&self) -> Result<&str, anyhow::Error> {
        match self {
            WsRole::WsApi {  secret,.. } => Ok(&secret) ,
            WsRole::UserData {  secret,.. } => Ok(&secret),
            WsRole::Public =>anyhow::bail!("public secret not available")
        }
    }

    pub fn sign_wsapi(&self, mut params: Value) -> anyhow::Result<Value> {
        match self {
            WsRole::WsApi { secret, .. } => {
                params["apiKey"] = self.api_key().unwrap().into();

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
