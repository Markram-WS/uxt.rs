
use super::public::kline::model::Kline;
use super::public::trade::model::Trade;
use super::public::ticker::model::Ticker;
use tokio::sync::{mpsc,Mutex};

pub struct StreamManager {
    pub kline_tx: Option<mpsc::Sender<Kline>>,
    pub trade_tx: Option<mpsc::Sender<Trade>>,
    pub ticker_tx: Option<mpsc::Sender<Ticker>>,
}