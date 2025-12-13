use reqwest::{Client, Method};
use serde::Serialize;

pub struct RestClient {
    client: Client,
    pub base_url: String,
    pub api_key: String,
    pub secret: String,
}

impl RestClient {
    pub fn new(base_url: &str, api_key: &str, secret: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.into(),
            api_key: api_key.into(),
            secret: secret.into(),
        }
    }

    async fn request<T: Serialize + ?Sized>(
        &self,
        method: Method,
        path: &str,
        body: Option<&T>,
    ) -> anyhow::Result<reqwest::Response> {
        let url = format!("{}{}", self.base_url, path);

        let mut req = self.client
            .request(method, &url)
            .header("X-MBX-APIKEY", &self.api_key);

        if let Some(b) = body {
            req = req.json(b);
        }

        Ok(req.send().await?)
    }

    pub async fn post<T: Serialize + ?Sized>(
        &self,
        path: &str,
        body: Option<&T>,
    ) -> anyhow::Result<reqwest::Response> {
        self.request(Method::POST, path, body).await
    }

    pub async fn put<T: Serialize + ?Sized>(
        &self,
        path: &str,
        body: Option<&T>,
    ) -> anyhow::Result<reqwest::Response> {
        self.request(Method::PUT, path, body).await
    }

    pub async fn delete<T: Serialize + ?Sized>(
        &self,
        path: &str,
        body: Option<&T>,
    ) -> anyhow::Result<reqwest::Response> {
        self.request(Method::DELETE, path, body).await
    }
}
