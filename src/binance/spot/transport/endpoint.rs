
use crate::utils::{get_env};

pub fn ws_public(streams: &[String]) -> String {
    let base = get_env("BINANCE_WS_PUBLIC_ENDPOINT");  
    format!("{}/stream?streams={}", base, streams.join("/"))
}

pub fn ws_userdata(listen_key: &str) -> String {
    let base = get_env("BINANCE_WS_USERDATA_ENDPOINT");
    format!("{}/ws/{}", base, listen_key)
}

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