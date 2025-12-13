use super::ProtocolAdapter;
use crate::config::RetroShareSettings;
use anyhow::Result;
use async_trait::async_trait;
use log::{info, warn};
use std::sync::Arc;
use tokio::sync::Mutex;

/// RetroShare Adapter
///
/// Interacts with RetroShare via JSON API.
/// Typically RetroShare exposes a web UI/API on port 9090 or similar.
/// This adapter checks connectivity and could route API requests.
pub struct RetroShareAdapter {
    settings: RetroShareSettings,
    connected: Arc<Mutex<bool>>,
}

impl RetroShareAdapter {
    pub fn new(settings: RetroShareSettings) -> Self {
        Self {
            settings,
            connected: Arc::new(Mutex::new(false)),
        }
    }
}

#[async_trait]
impl ProtocolAdapter for RetroShareAdapter {
    async fn start(&self) -> Result<()> {
        if !self.settings.enabled {
            return Ok(());
        }

        info!("Initializing RetroShare adapter...");

        let client = reqwest::Client::new();
        // Just check the root or a known endpoint
        let url = format!("{}", self.settings.api_url);

        match client.get(&url).send().await {
            Ok(resp) if resp.status().is_success() => {
                info!("Connected to RetroShare node at {}", self.settings.api_url);
                let mut connected = self.connected.lock().await;
                *connected = true;
            }
            Ok(resp) => {
                warn!("RetroShare responded with status {}", resp.status());
            }
            Err(e) => {
                warn!(
                    "RetroShare node not responding at {}: {}",
                    self.settings.api_url, e
                );
                warn!("Ensure RetroShare is running with JSON API enabled.");
                return Ok(());
            }
        }

        info!("RetroShare adapter started.");
        Ok(())
    }

    async fn stop(&self) -> Result<()> {
        let mut connected = self.connected.lock().await;
        if *connected {
            *connected = false;
            info!("RetroShare adapter stopped.");
        }
        Ok(())
    }

    fn get_proxy_addr(&self) -> String {
        // RetroShare doesn't typically provide a SOCKS5 proxy for general traffic.
        // It provides services usually accessible via the node.
        // We return the API URL or a placeholder if no proxy exists.
        // For now, let's return the API host/port.
        self.settings.api_url.clone()
    }

    async fn is_healthy(&self) -> bool {
        let connected = self.connected.lock().await;
        *connected
    }
}
