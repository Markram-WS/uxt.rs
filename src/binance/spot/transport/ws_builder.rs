use crate::utils::env::get_env;

#[derive(Debug, Clone)]
pub enum MarketType {
    Spot,
    Futures,
}

#[derive(Debug, Clone)]
pub enum Broker {
    Binance,
    // Bybit, Okx ... future
}

pub struct WsBuilder {
    broker: Broker,
    market: MarketType,
    streams: Vec<String>,
}

impl WsBuilder {
    pub fn new(broker: Broker, market: MarketType) -> Self {
        Self {
            broker,
            market,
            streams: Vec::new(),
        }
    }

    pub fn spot() -> Self {
        Self::new(Broker::Binance, MarketType::Spot)
    }

    pub fn futures() -> Self {
        Self::new(Broker::Binance, MarketType::Futures)
    }

    // ------- PUBLIC STREAMS -------

    pub fn kline(mut self, symbol: &str, interval: &str) -> Self {
        self.streams.push(format!("{}@kline_{}", symbol.to_lowercase(), interval));
        self
    }

    pub fn trade(mut self, symbol: &str) -> Self {
        self.streams.push(format!("{}@trade", symbol.to_lowercase()));
        self
    }

    pub fn book_ticker(mut self, symbol: &str) -> Self {
        self.streams.push(format!("{}@bookTicker", symbol.to_lowercase()));
        self
    }

    pub fn depth(mut self, symbol: &str, level: u16) -> Self {
        self.streams.push(format!("{}@depth{}", symbol.to_lowercase(), level));
        self
    }

    // ------- USER DATA -------

    pub fn listen_key(self, listen_key: &str) -> String {
        match (self.broker, self.market) {
            (Broker::Binance, MarketType::Spot) => {
                let base = get_env("BINANCE_WS_SPOT_USERDATA");
                format!("{}/ws/{}", base, listen_key)
            }

            (Broker::Binance, MarketType::Futures) => {
                let base = get_env("BINANCE_WS_FUTURES_USERDATA");
                format!("{}/ws/{}", base, listen_key)
            }

            //_ => todo!("support other brokers")
        }
    }

    // ------- BUILD URL -------

    pub fn build(self) -> String {
        if self.streams.is_empty() {
            panic!("No streams added to WsBuilder");
        }

        let stream_query = self.streams.join("/");

        match (self.broker, self.market) {
            (Broker::Binance, MarketType::Spot) => {
                let base = get_env("BINANCE_WS_SPOT_PUBLIC");
                format!("{}/stream?streams={}", base, stream_query)
            }

            (Broker::Binance, MarketType::Futures) => {
                let base = get_env("BINANCE_WS_FUTURES_PUBLIC");
                format!("{}/stream?streams={}", base, stream_query)
            }

            //_ => todo!("support other brokers")
        }
    }
}
