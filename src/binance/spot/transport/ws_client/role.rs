use chrono::Utc;
use serde_json::Value;
use crate::utils::sign_ed25519;
use std::fmt::Debug;
use std::collections::BTreeMap;
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
                let ts = Utc::now().timestamp_millis();
                let api_key = self.api_key()?;
                
                params["apiKey"] = api_key.into();
                params["timestamp"] = ts.into();
                let params_map: BTreeMap<String, Value> = serde_json::from_value(params.clone())?;
                let query = serde_urlencoded::to_string(&params_map)
                    .map_err(|e| anyhow::anyhow!("Query encoding failed: {}", e))?;
                let sig = sign_ed25519(secret, &query);
                params["signature"] = sig.into();

                Ok(params)
            }
            _ => anyhow::bail!("not ws-api role"),
        }
    }
}
