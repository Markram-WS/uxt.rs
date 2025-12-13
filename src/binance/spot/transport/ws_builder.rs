
use crate::utils::env::get_env;
use crate::binance::spot::public::Interval; 

pub enum MarketType {
    Spot,
    Futures, // future support
}

pub enum StreamMode {
    Public,        // no sign
    UserData,      // signed/needs send_signed()
    ApiSigned,          
}

pub struct WsBuilder {
    market: MarketType,
    mode: StreamMode,
    streams: Vec<String>,
    listen_key: Option<String>,
    pub api_key: String,
    pub secret: String,

    // final URL ที่จะใช้เชื่อมต่อ
    pub base_url: String,
}

impl WsBuilder {
    pub fn new(market: MarketType, api_key: &str, secret: &str) -> Self {
        Self {
            market,
            mode: StreamMode::Public,
            streams: Vec::new(),
            listen_key: None,
            api_key: api_key.to_string(),
            secret: secret.to_string(),
            base_url: "".into(),
        }
    }

    pub fn spot(api_key: &str, secret: &str) -> Self {
        Self::new(MarketType::Spot, api_key, secret)
    }

    // -------------------
    // AUTHENTICATOR
    // -------------------

    pub fn api_signed(mut self) -> Self {
        self.mode = StreamMode::ApiSigned;
        self
    }

    // -------------------
    // PUBLIC STREAMS
    // -------------------

    pub fn kline(mut self, symbol: &str, interval: Interval) -> Self {
        self.mode = StreamMode::Public;
        self.streams.push(format!("{}@kline_{}", symbol.to_lowercase(), &interval.as_str() ));
        self
    }

    pub fn trade(mut self, symbol: &str) -> Self {
        self.mode = StreamMode::Public;
        self.streams.push(format!("{}@trade", symbol.to_lowercase()));
        self
    }

    pub fn ticker(mut self, symbol: &str) -> Self {
        self.mode = StreamMode::Public;
        self.streams.push(format!("{}@bookTicker", symbol.to_lowercase()));
        self
    }

    // -------------------
    // USER DATA STREAM
    // -------------------

    pub fn user_data(mut self, listen_key: &str) -> Self {
        self.mode = StreamMode::UserData;
        self.listen_key = Some(listen_key.to_string());
        self
    }

    // -------------------
    // BUILD URL
    // -------------------

    pub fn build(mut self) -> Self {
        match self.mode {
            StreamMode::Public => {
                self.base_url = self.build_public_url();
            }
            StreamMode::UserData => {
                self.base_url = self.build_userdata_url();
            }
            StreamMode::ApiSigned => {
                self.base_url = self.build_apisigned_url();
            }
        }
        self
    }
  
    fn build_public_url(&self) -> String {
        let base = get_env("BINANCE_WS_SPOT_PUBLIC_ENDPOINT");

        if self.streams.is_empty() {
            panic!("Public stream requires at least one stream");
        }

        let q = self.streams.join("/");
        format!("{}/stream?streams={}", base, q)
    }

    fn build_userdata_url(&self) -> String {
        let key: &String = self
            .listen_key
            .as_ref()
            .expect("UserData stream requires listenKey");

        let base = get_env("BINANCE_WS_SPOT_USERDATA_ENDPOINT_TEST");
        format!("{}/ws/{}", base, key)
    }

    fn build_apisigned_url(&self) -> String {
        let key: &String = self
        .listen_key
        .as_ref()
        .expect("UserData stream requires listenKey");

    let base = get_env("BINANCE_WS_SPOT_API_ENDPOINT");
    format!("{}/ws-api/v3", base)    }
}