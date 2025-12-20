



#[cfg(test)]
mod tests {
    use std::env;
    use std::sync::Once;
    use dotenvy::dotenv;
    use utx::binance::spot::{WsBuilder,WsClient,RestClient};
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


        // let builder = WsBuilder::spot(&binance_api,&binance_secret).ws().build();
        // ws_client = WsClient.connect(builder);
        // ws_client.send_wsapi()


    }
}
