



#[cfg(test)]
mod tests {
    use std::env;
    use std::sync::Once;
    use dotenvy::dotenv;
    use utx::binance::spot::{WsBuilder,WsClient};
    use utx::utils::{get_env};
    use std::time::{SystemTime, UNIX_EPOCH};
    static INIT: Once = Once::new();
    
    fn init() {
        INIT.call_once(|| {
            dotenv().ok();
        });
    }
    
    #[tokio::test]
    async  fn test_binance_spot_ws_api(){
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


        let symbol =  "BTCUSDT";
        

        let builder = WsBuilder::spot(&binance_api,&binance_secret).ws().build();

        let mut ws_client = WsClient::connect(builder).await.unwrap();
        // test general event 
        match ws_client.logon().await {
            Ok(res) => { 
                assert!(ws_client.authed,"login success");
                println!("status ok: {}", res);
            }
            Err(e) => {
                eprint!("logon error: {:#}", e)
            }
        };

        match ws_client.status().await {
            Ok(res) => { 
                assert!(true,"status ok: {res}");
            }
            Err(e) => {
                eprint!("status error: {:#}", e)
            }
        };
        
        match ws_client.ping().await {
            Ok(res) => { 
                assert!(true,"ping server ok: {res}");
            }
            Err(e) => {
                eprint!("ping error: {:#}", e)
            }
        };

        match ws_client.time().await {
            Ok(res) => { 
                let res_i = serde_json::json!(res);
                assert!(true,"server time ok: {res_i}" );
            }
            Err(e) => {
                eprint!("time error: {:#}", e)
            }
        };

        // test ws event 
        let method = "order.test";
        let param: serde_json::Value = serde_json::json!({
            "symbol": &symbol,
            "type": "LIMIT_MAKER",
            "price": "23416.10000000",
            "quantity": "0.001"
        });
        match ws_client.call_wsapi(&method,param).await {
            Ok(res) => { 
                println!("{} ok : {:#}",&method,res);
                assert!(true );
            }
            Err(e) => {
                eprint!("{} error: {:#}",&method, e)
            }
        };

        let method = "order.test";
        let param: serde_json::Value = serde_json::json!({
            "symbol": &symbol,
            "type": "LIMIT_MAKER",
            "price": "23416.10000000",
            "quantity": "0.001"
        });
        match ws_client.call_wsapi(&method,param).await {
            Ok(res) => { 
                println!("{} ok : {:#}",&method,res);
                assert!(true );
            }
            Err(e) => {
                eprint!("{} error: {:#}",&method, e)
            }
        };

        let method = "ticker.24hr";
        let param: serde_json::Value = serde_json::json!({
            "symbol": &symbol
        });
        match ws_client.call_wsapi(&method,param).await {
            Ok(res) => { 
                println!("{} ok : {:#}",&method,res);
                assert!(true );
            }
            Err(e) => {
                eprint!("{} error: {:#}",&method, e)
            }
        };


        let method = "klines";
        let now_sec = SystemTime::now()
            .duration_since(UNIX_EPOCH).unwrap()
            .as_secs();

        let minus_24h = now_sec - 24 * 60 * 60;
        let param: serde_json::Value = serde_json::json!({
            "symbol": &symbol,
            "interval": "1h",
            "startTime": minus_24h,
            "limit": 1
        });
        match ws_client.call_wsapi(&method,param).await {
            Ok(res) => { 
                println!("{} ok : {:#}",&method,res);
                assert!(true );
            }
            Err(e) => {
                eprint!("{} error: {:#}",&method, e)
            }
        };

        
        // test logout event 
        match ws_client.logout().await {
            Ok(()) => { 
                assert!(true,"logout ok");
            }
            Err(e) => {
                eprint!("logon error: {:#}", e)
            }
        };

        // test order 

    }
}
