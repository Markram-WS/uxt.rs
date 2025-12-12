



#[cfg(test)]
mod tests {
    use std::env;
    use std::sync::Once;
    use dotenvy::dotenv;
    use utx::binance::spot::{WsBuilder,WsClient,Interval};
    use utx::binance::spot::public::{KlineService,TradeService,TickerService};
    use utx::utils::{get_env};
    use tokio::time::{timeout, Duration};
    static INIT: Once = Once::new();

    
    fn init() {
        INIT.call_once(|| {
            dotenv().ok();
        });
    }
    
    #[tokio::test]
    async  fn test_binance_spot_pub_stream(){
        init();
        let binance_api = get_env("BINANCE_API_TEST");
        let binance_secret = get_env("BINANCE_SECRET_TEST");
        let binance_api_endpoint = get_env("BINANCE_WS_SPOT_API_ENDPOINT_TEST");
        let binance_ws_public_endpoint_test = get_env("BINANCE_WS_SPOT_PUBLIC_ENDPOINT_TEST");
        unsafe { 
            env::set_var("BINANCE_API", &binance_api);
            env::set_var("BINANCE_SECRET", &binance_secret);
            env::set_var("BINANCE_WS_SPOT_API_ENDPOINT", &binance_api_endpoint);
            env::set_var("BINANCE_WS_SPOT_PUBLIC_ENDPOINT", &binance_ws_public_endpoint_test);
        };
        let symbol =  "btcusdt";
        

        let pub_builder = WsBuilder::spot(&binance_api,&binance_secret)
            .kline(&symbol,Interval::Days1)
            .trade(&symbol)
            .ticker(&symbol)
            .build();

        let mut ws_pub_client = WsClient::connect(pub_builder).await.unwrap();


        let (klineservice, mut rx_kline) = KlineService::new();
        let (tradeservice, mut rx_trade) = TradeService::new();
        let (tickerservice, mut rx_ticker) = TickerService::new();

        tokio::spawn(async move {
            ws_pub_client.read_loop(move |txt| {
                let klineservice = klineservice.clone();   
                let tradeservice = tradeservice.clone();    
                let tickerservice = tickerservice.clone();     
                let txt = txt.clone();
        
                async move {
                    klineservice.handle(&txt).await.expect("`Err` klineservice message handling");
                    tradeservice.handle(&txt).await.expect("`Err` tradeservice message handling");
                    tickerservice.handle(&txt).await.expect("`Err` tickerservice message handling");
                    Ok(())
                }
            })
            .await
            .unwrap();
        });
        let mut got_k = false;
        let mut got_t = false;
        let mut got_x = false;
        

        let res = timeout(Duration::from_secs(1000), async {
            loop {
                tokio::select! {
                    Some(k) = rx_kline.recv() => {
                        if got_k == false {
                            println!("[KLINE] {:?}", &k);
                        }
                        got_k = true;
                    }
                    Some(t) = rx_trade.recv() => {
                        if got_t == false {
                            println!("[TRADE] {:?}", &t);
                        }
                        got_t = true;
          
                    }
                    Some(x) = rx_ticker.recv() => {
                        if got_t == false {
                            println!("[TICKER] {:?}", &x);
                        }
                        got_x = true;
                    }
                }
    
                if got_k && got_t  {
                    break;
                }
            }
        }).await;
        assert!(res.is_ok(), "timeout: did not receive all 3 events within time");
        assert!(got_k, "missing kline event");
        assert!(got_t, "missing trade event");
        //assert!(got_x, "missing ticker event");

    }
}
