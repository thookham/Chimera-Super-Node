use super::ProtocolAdapter;
use crate::config::TriblerSettings;
use anyhow::Result;
use async_trait::async_trait;
use log::{info, warn};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Tribler Adapter
///
/// Interacts with Tribler via REST API.
/// Default port: 8085
pub struct TriblerAdapter {
    settings: TriblerSettings,
    connected: Arc<Mutex<bool>>,
}

impl TriblerAdapter {
    pub fn new(settings: TriblerSettings) -> Self {
        Self {
            settings,
            connected: Arc::new(Mutex::new(false)),
        }
    }
}

#[async_trait]
impl ProtocolAdapter for TriblerAdapter {
    async fn start(&self) -> Result<()> {
        if !self.settings.enabled {
            return Ok(());
        }

        info!("Initializing Tribler adapter...");

        let client = reqwest::Client::new();
        let url = format!("{}/variables", self.settings.api_url);

        // Needs API key usually, but simple check first
        let req = client.get(&url);
        let req = if let Some(key) = &self.settings.api_key {
            req.header("X-Api-Key", key)
        } else {
            req
        };

        match req.send().await {
            Ok(_resp) => {
                // If we get any response, the server is there
                info!("Tribler API detected at {}", self.settings.api_url);
                let mut connected = self.connected.lock().await;
                *connected = true;
            }
            Err(e) => {
                warn!(
                    "Tribler API not responding at {}: {}",
                    self.settings.api_url, e
                );
                return Ok(());
            }
        }

        info!("Tribler adapter started.");
        Ok(())
    }

    async fn stop(&self) -> Result<()> {
        let mut connected = self.connected.lock().await;
        if *connected {
            *connected = false;
            info!("Tribler adapter stopped.");
        }
        Ok(())
    }

    fn get_proxy_addr(&self) -> String {
        // Tribler typically doesn't offer a SOCKS proxy for external apps
        // It manages its own anonymous downloads.
        // We'll return the API URL.
        self.settings.api_url.clone()
    }

    async fn is_healthy(&self) -> bool {
        let connected = self.connected.lock().await;
        *connected
    }
}
