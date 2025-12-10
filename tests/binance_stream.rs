



#[cfg(test)]
mod tests {
    use std::env;
    use std::sync::Once;
    use dotenvy::dotenv;
    use utx::binance::spot::rest::order;
    use utx::binance::spot::rest::depth;
    use utx::utils::{get_env};
    
    static INIT: Once = Once::new();

    
    fn init() {
        INIT.call_once(|| {
            dotenv().ok();
        });
    }
    
    #[tokio::test]
    async  fn test_binance_spot_stream(){
        init();
        let binance_api = get_env("BINANCE_API_TEST");
        let binance_secret = get_env("BINANCE_SECRET_TEST");
        let binance_api_endpoint = get_env("BINANCE_WS_SPOT_API_ENDPOINT_TEST");
        let binance_ws_public_endpoint_test = get_env("BINANCE_WS_SPOT_PUBLIC_ENDPOINT_TEST");
        unsafe { 
            env::set_var("BINANCE_API", binance_api);
            env::set_var("BINANCE_SECRET", binance_secret);
            env::set_var("BINANCE_WS_SPOT_API_ENDPOINT", binance_api_endpoint);
            env::set_var("BINANCE_WS_SPOT_PUBLIC_ENDPOINT", binance_ws_public_endpoint_test);
        };

        assert_eq!("", "");
    }
}


