
use crate::utils::{get_env};

#[allow(dead_code)]
pub fn ws_public(streams: &[String]) -> String {
    let base = get_env("BINANCE_WS_PUBLIC_ENDPOINT");  
    format!("{}/stream?streams={}", base, streams.join("/"))
}

#[allow(dead_code)]
pub fn ws_userdata(listen_key: &str) -> String {
    let base = get_env("BINANCE_WS_USERDATA_ENDPOINT");
    format!("{}/ws/{}", base, listen_key)
}

#[allow(dead_code)]
pub fn rest(listen_key: &str) -> String {
    let base = get_env("BINANCE_REST_ENDPOINT");  
    format!("{}/api/v3/{}", base, listen_key)
}

#[allow(dead_code)]
pub enum EventType{
    UserData,
    Public,
}
impl  EventType  {
    pub fn as_str(&self) -> &'static str {
        match self {
            EventType::UserData => "user",
            EventType::Public => "public",
        }

    }
}