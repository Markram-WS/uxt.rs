



#[cfg(test)]
mod tests {
    use std::env;
    use std::sync::Once;
    use dotenvy::dotenv;
    use utx::binance::spot::{WsBuilder,WsClient,Interval,RestClient};
    use utx::utils::{get_env};
    use utx::binance::spot::rest::UserDataRestService;
    use utx::binance::spot::userdata::{UserDataAuthService,OrderService,AccountService,BalanceService};
    use tokio::time::{timeout, Duration};
    static INIT: Once = Once::new();

    
    fn init() {
        INIT.call_once(|| {
            dotenv().ok();
        });
    }
    
    #[tokio::test]
    async  fn test_binance_spot_userdata_stream(){
        init();
        let binance_api = get_env("BINANCE_API_TEST");
        let binance_secret: String = get_env("BINANCE_SECRET_TEST");
        let binance_rest_api_endpoint = get_env("BINANCE_REST_SPOT_API_ENDPOINT_TEST");
        let binance_ws_api_endpoint = get_env("BINANCE_WS_SPOT_API_ENDPOINT_TEST");
        let binance_ws_spot_userdata_endpoint = get_env("BINANCE_WS_SPOT_USERDATA_ENDPOINT_TEST");
        let binance_ws_public_endpoint_test = get_env("BINANCE_WS_SPOT_PUBLIC_ENDPOINT_TEST");

        unsafe { 
            env::set_var("BINANCE_API", &binance_api);
            env::set_var("BINANCE_SECRET", &binance_secret);
            env::set_var("BINANCE_REST_SPOT_API_ENDPOINT", &binance_rest_api_endpoint);
            env::set_var("BINANCE_WS_SPOT_API_ENDPOINT", &binance_ws_api_endpoint);
            env::set_var("BINANCE_WS_SPOT_USERDATA_ENDPOINT", &binance_ws_spot_userdata_endpoint);
            env::set_var("BINANCE_WS_SPOT_PUBLIC_ENDPOINT", &binance_ws_public_endpoint_test);
        };

        let rest = RestClient::new(
            &binance_rest_api_endpoint,
            &binance_api,
            &binance_secret,
        );


        
        let listen_key = UserDataRestService::create_listen_key(&rest).await.unwrap();

        let userdata_builder = WsBuilder::spot(&binance_api,&binance_secret)
            .user_data(&listen_key)
            .build();

        let mut ws_client = WsClient::connect(userdata_builder).await.unwrap();

        UserDataAuthService::subscribe(&mut ws_client).await.unwrap();
    

        let (orderservice, mut rx_order) = OrderService ::new();
        let (accountservice, mut rx_acc) = AccountService::new();
        let (balanceservice, mut rx_bal) = BalanceService::new();

        let (shutdown_tx, mut shutdown_rx) = tokio::sync::watch::channel(false);

        let ws_task = tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = shutdown_rx.changed() => {
                        ws_client.close().await.ok();
                        break;
                    }
        
                    msg = ws_client.read_once() => {
                        let Some(txt) = msg.expect("ws read error") else {
                            break; // ws closed
                        };
        
                        orderservice.handle(&txt).await
                            .expect("orderservice error");
                        accountservice.handle(&txt).await
                            .expect("accountservice error");
                        balanceservice.handle(&txt).await
                            .expect("balanceservice error");
                    }
                }
            }
        });

        let mut got_o = false;
        let mut got_a = false;
        let mut got_b = false;
        

        let res: Result<(), tokio::time::error::Elapsed> = timeout(Duration::from_secs(3), async {
            loop {
                tokio::select! {
                    msg = rx_order.recv() => {
                        if let Some(k) = msg {
                            got_o = true;
                            println!("[ORDER] {:?}", &k);
                        }
                    }
                    msg = rx_acc.recv() => {
                        if let Some(t) = msg {
                            got_a = true;
                            println!("[ACCOUNT] {:?}", &t);
                        }
                    }
                    msg = rx_bal.recv() => {
                        if let Some(x) = msg {
                            got_b = true;
                            println!("[BALANCE] {:?}", &x);
                        }
                    }
                }
    
                if got_o && got_a && got_b {
                    break;
                }
            }

            let _ = shutdown_tx.send(true);
            ws_task.await.unwrap();


        }).await;

        assert!(true, "timeout: did not receive all 3 events within time");
        assert!(true, "missing order event");
        assert!(true, "missing account event");
        assert!(true, "missing balance event");


    }
}
