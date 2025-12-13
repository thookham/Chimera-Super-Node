use super::ProtocolAdapter;
use crate::config::ZeroNetSettings;
use anyhow::Result;
use async_trait::async_trait;
use log::{info, warn};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct ZeroNetAdapter {
    settings: ZeroNetSettings,
    connected: Arc<Mutex<bool>>,
}

impl ZeroNetAdapter {
    pub fn new(settings: ZeroNetSettings) -> Self {
        Self {
            settings,
            connected: Arc::new(Mutex::new(false)),
        }
    }
}

#[async_trait]
impl ProtocolAdapter for ZeroNetAdapter {
    async fn start(&self) -> Result<()> {
        if !self.settings.enabled {
            return Ok(());
        }

        info!("Initializing ZeroNet adapter...");

        // Test connection to ZeroNet daemon
        let client = reqwest::Client::new();
        let status_url = format!("{}/ZeroNet-Internal/Stats", self.settings.proxy_url);

        match client.get(&status_url).send().await {
            Ok(resp) if resp.status().is_success() => {
                info!("Connected to ZeroNet daemon at {}", self.settings.proxy_url);
            }
            Ok(resp) => {
                warn!("ZeroNet responded with status {}", resp.status());
            }
            Err(e) => {
                warn!(
                    "ZeroNet daemon not responding at {}: {}",
                    self.settings.proxy_url, e
                );
                warn!("Start ZeroNet with: python zeronet.py");
                return Ok(());
            }
        }

        let mut connected = self.connected.lock().await;
        *connected = true;

        info!(
            "ZeroNet adapter started (Proxy: {})",
            self.settings.proxy_url
        );
        Ok(())
    }

    async fn stop(&self) -> Result<()> {
        let mut connected = self.connected.lock().await;
        if *connected {
            *connected = false;
            info!("ZeroNet adapter stopped.");
        }
        Ok(())
    }

    fn get_proxy_addr(&self) -> String {
        // ZeroNet uses HTTP proxy
        self.settings.proxy_url.clone()
    }

    async fn is_healthy(&self) -> bool {
        let connected = self.connected.lock().await;
        *connected
    }
}
