use super::model::ListenKeyResponse;
use crate::binance::spot::transport::rest::RestClient;
use log;
use tokio::time::interval;
use tokio::sync::watch;
use tokio::time::Duration;

pub struct UserDataRestService;

impl UserDataRestService {
    pub async fn create_listen_key(
        client: &RestClient,
    ) -> anyhow::Result<String> {
        let res = client
            .post::<()>(
                "/api/v3/userDataStream",
                None
            )
            .await?;
        let bytes = res.bytes().await?;
        let body: ListenKeyResponse = serde_json::from_slice(&bytes)?;
        Ok(body.listen_key)
    }

    /// PUT /api/v3/userDataStream
    pub async fn keepalive(
        client: &RestClient,
        listen_key: &str,
    ) -> anyhow::Result<()> {
        client
            .put(
                &format!("/api/v3/userDataStream?listenKey={}", listen_key),
                None::<&()>,
            )
            .await?;
        Ok(())
    }

    /// DELETE /api/v3/userDataStream
    pub async fn close(
        client: &RestClient,
        listen_key: &str,
    ) -> anyhow::Result<()> {
        client
            .delete(
                &format!("/api/v3/userDataStream?listenKey={}", listen_key),
                None::<&()>,
            )
            .await?;
        Ok(())
    }

    pub async fn spawn_keepalive(
        rest: RestClient,
        listen_key: String,
        mut shutdown: watch::Receiver<bool>,
    ) {
        let mut ticker = interval(Duration::from_secs(30 * 60));
    
        loop {
            tokio::select! {
                _ = ticker.tick() => {
                    if let Err(e) =
                        UserDataRestService::keepalive(&rest, &listen_key).await
                    {
                        log::error!("listenKey keepalive failed: {:?}", e);
                    }
                }
                _ = shutdown.changed() => {
                    log::info!("keepalive loop shutdown");
                    break;
                }
            }
        }
    }
}
