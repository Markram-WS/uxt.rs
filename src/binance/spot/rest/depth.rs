use crate::utils::{self, get_env};
use crate::utils::convert::{vec_str_pair_to_f64};
#[derive(Debug)]
pub struct Params<'a> {
    symbol: &'a str,
    limit: &'a str,
}

impl<'a> Params<'a> {
    #[allow(dead_code)]
    pub fn new(symbol: &'a str) -> Self {
        Self { symbol, limit: "100"  } 
    }
    #[allow(dead_code)]
    pub fn limit(mut self, limit: &'a str) -> Self {
        self.limit = limit;
        self
    }
    #[allow(dead_code)]
   fn to_pairs(&self) -> Vec<(&str, String)> {
        vec![
            ("symbol", self.symbol.to_string()),
            ("limit", self.limit.to_string()),
        ]
    }
}


use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct OrderBook {
    #[serde(rename = "lastUpdateId")]
    pub last_update_id: u64,
    #[serde(deserialize_with = "vec_str_pair_to_f64")]
    pub bids: Vec<(f64, f64)>,
    #[serde(deserialize_with = "vec_str_pair_to_f64")]
    pub asks: Vec<(f64, f64)>,
}

#[allow(dead_code)]
pub async fn get_depth<'a>(payload: Params<'a>) -> Result<OrderBook, Box<dyn std::error::Error>> {
    let api_host = get_env("API_HOST");
    let api_key = get_env("API_KEY");
    let query_string = serde_urlencoded::to_string(&payload.to_pairs())?;
    let url: String = format!("{}/api/v3/depth?{}", api_host, query_string);
    let client = reqwest::Client::new();
    let res = client
        .get(&url)
        .header("X-MBX-APIKEY", &api_key)
        .header("Accept", "application/json")
        .send()
        .await?;

    let status = res.status();
    let text = res.text().await?;
    //println!("{} : {}", status.as_u16(), status.as_str());

    if status.is_success() {
        let ob: OrderBook = serde_json::from_str(&text)?;
        Ok(ob)
    } else {
        let err = format!("status {} : {}", status.as_u16(), text);
        Err(err.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::sync::Once;
    use dotenvy::dotenv;
    static INIT: Once = Once::new();

    fn init() {
        INIT.call_once(|| {
            dotenv().ok();
        });
    }

    //get_env
    #[tokio::test]
    async  fn test_api_binance_spot_depth(){
        init();
        let api_key = get_env("API_KEY_TEST");
        let api_secret_test: String = get_env("API_SECRET_TEST");
        unsafe { 
            env::set_var("API_HOST", "https://testnet.binance.vision");
            env::set_var("API_SECRET", api_secret_test);
            env::set_var("API_KEY", api_key);

        };
        let payload = Params::new("BTCUSDT");
        println!("payload : {:?}", &payload);
        match get_depth(payload).await {
            Ok(res) => {
                println!("response : {:?}",res);
                assert_eq!(200, 200);
            },
            Err(e) => panic!("API error: {}", e),
        }
        

    }
}
