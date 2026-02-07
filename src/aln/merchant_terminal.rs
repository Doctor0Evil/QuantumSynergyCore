use serde::Deserialize;
use tracing::{info, warn};
use hyper::{Client, body::to_bytes};
use hyper::client::HttpConnector;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug, Deserialize)]
struct MerchantConfig {
    merchant: MerchantSection,
    network: NetworkSection,
}

#[derive(Debug, Deserialize)]
struct MerchantSection {
    provider: String,
    id_source: String,
    hostname: String,
}

#[derive(Debug, Deserialize)]
struct NetworkSection {
    framework_connect: String,
    port: u16,
    max_tries: u32,
    retry_with_new_host: bool,
}

pub struct MerchantTerminal {
    cfg: MerchantConfig,
}

impl MerchantTerminal {
    pub fn from_config(path: &str) -> Result<Self, String> {
        let raw = std::fs::read_to_string(path)
            .map_err(|e| format!("read config {path}: {e}"))?;
        let cfg: MerchantConfig = serde_yaml::from_str(&raw)
            .map_err(|e| format!("parse config {path}: {e}"))?;
        Ok(Self { cfg })
    }

    pub async fn resolve_id(&self) -> Result<String, String> {
        let mut tries = 0;
        let max_tries = self.cfg.network.max_tries.max(1);
        let hostname = &self.cfg.merchant.hostname;

        while tries < max_tries {
            tries += 1;
            info!(
                "terminal_merchant_id attempt {} host={} provider={}",
                tries, hostname, self.cfg.merchant.provider
            );

            match self.fetch_from_host(hostname).await {
                Ok(id) => return Ok(id),
                Err(e) => {
                    warn!("host_rejection: {} (attempt {}/{})", e, tries, max_tries);
                    sleep(Duration::from_millis(200)).await;
                }
            }
        }

        if self.cfg.network.retry_with_new_host {
            let fallback = format!("{}_fallback", hostname);
            info!("find_new_host: retry_with_new_host host={}", fallback);
            return self.fetch_from_host(&fallback).await;
        }

        Err("exhausted attempts to resolve merchant id".into())
    }

    async fn fetch_from_host(&self, host: &str) -> Result<String, String> {
        let url = format!("http://{}:{}/terminal/id", host, self.cfg.network.port);
        let client: Client<HttpConnector> = Client::new();

        let req = match url.parse() {
            Ok(uri) => http::Request::get(uri).body(http::Body::empty()).unwrap(),
            Err(e) => return Err(format!("invalid uri: {e}")),
        };

        let resp = client.request(req).await.map_err(|e| format!("request error: {e}"))?;
        if !resp.status().is_success() {
            return Err(format!("status {}", resp.status()));
        }

        let body = to_bytes(resp.into_body()).await.map_err(|e| format!("body error: {e}"))?;
        let id = String::from_utf8_lossy(&body).trim().to_string();

        if id.is_empty() {
            return Err("empty id from host".into());
        }

        Ok(id)
    }
}
