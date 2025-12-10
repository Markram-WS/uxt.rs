use anyhow::Result;
use reqwest::Client;
use std::time::Duration;

pub struct RestClient {
    client: Client,
    api_key: String,
    base: String, // e.g. "https://api.binance.com"
}

impl RestClient {
    pub fn new(api_key: impl Into<String>, base: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.into(),
            base: base.into(),
        }
    }

    /// POST /api/v3/userDataStream  -> returns listenKey as String
    pub async fn create_listen_key(&self) -> Result<String> {
        let url = format!("{}/api/v3/userDataStream", self.base);
        let resp = self.client
            .post(&url)
            .header("X-MBX-APIKEY", &self.api_key)
            .send()
            .await?
            .error_for_status()?;
        let v: serde_json::Value = resp.json().await?;
        let key = v.get("listenKey")
            .and_then(|k| k.as_str())
            .ok_or_else(|| anyhow::anyhow!("no listenKey in response"))?;
        Ok(key.to_string())
    }

    /// PUT /api/v3/userDataStream -> keepalive
    pub async fn keepalive_listen_key(&self, listen_key: &str) -> Result<()> {
        let url = format!("{}/api/v3/userDataStream?listenKey={}", self.base, listen_key);
        self.client
            .put(&url)
            .header("X-MBX-APIKEY", &self.api_key)
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    /// convenience: auto-refresh task (spawn this)
    pub fn spawn_keepalive_task(self: std::sync::Arc<Self>, listen_key: String) {
        // keepalive every 30 minutes (we choose 25m to be safe)
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(25 * 60));
            loop {
                interval.tick().await;
                if let Err(e) = self.keepalive_listen_key(&listen_key).await {
                    tracing::warn!("keepalive failed: {:?}", e);
                    // continue and try again next tick; you may also choose to notify/stop
                } else {
                    tracing::info!("listenKey refreshed");
                }
            }
        });
    }
}
