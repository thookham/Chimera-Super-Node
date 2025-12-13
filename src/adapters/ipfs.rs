use super::ProtocolAdapter;
use crate::config::IpfsSettings;
use anyhow::Result;
use async_trait::async_trait;
use log::{info, warn};
use std::sync::Arc;
use tokio::sync::Mutex;

/// IPFS adapter using Kubo RPC API via HTTP
///
/// Connects to a local Kubo node's API (default: http://127.0.0.1:5001)
/// and provides gateway access for `ipfs://` and `ipns://` URLs.
pub struct IpfsAdapter {
    settings: IpfsSettings,
    connected: Arc<Mutex<bool>>,
}

impl IpfsAdapter {
    pub fn new(settings: IpfsSettings) -> Self {
        Self {
            settings,
            connected: Arc::new(Mutex::new(false)),
        }
    }
}

#[async_trait]
impl ProtocolAdapter for IpfsAdapter {
    async fn start(&self) -> Result<()> {
        if !self.settings.enabled {
            return Ok(());
        }

        info!("Initializing IPFS adapter...");

        // Test connection to Kubo API
        let client = reqwest::Client::new();
        let id_url = format!("{}/api/v0/id", self.settings.api_url);

        match client.post(&id_url).send().await {
            Ok(resp) if resp.status().is_success() => {
                if let Ok(json) = resp.json::<serde_json::Value>().await {
                    let node_id = json.get("ID").and_then(|v| v.as_str()).unwrap_or("unknown");
                    let agent = json
                        .get("AgentVersion")
                        .and_then(|v| v.as_str())
                        .unwrap_or("unknown");
                    info!("Connected to IPFS node: {} ({})", node_id, agent);
                }
            }
            Ok(resp) => {
                warn!("IPFS API responded with status {}", resp.status());
                return Ok(());
            }
            Err(e) => {
                warn!(
                    "IPFS node not responding at {}: {}",
                    self.settings.api_url, e
                );
                warn!("Start Kubo with: ipfs daemon");
                return Ok(());
            }
        }

        let mut connected = self.connected.lock().await;
        *connected = true;

        info!(
            "IPFS adapter started (Gateway: http://127.0.0.1:{})",
            self.settings.gateway_port
        );
        Ok(())
    }

    async fn stop(&self) -> Result<()> {
        let mut connected = self.connected.lock().await;
        if *connected {
            *connected = false;
            info!("IPFS adapter stopped.");
        }
        Ok(())
    }

    fn get_proxy_addr(&self) -> String {
        // IPFS uses HTTP gateway, not SOCKS5
        format!("127.0.0.1:{}", self.settings.gateway_port)
    }

    async fn is_healthy(&self) -> bool {
        let connected = self.connected.lock().await;
        *connected
    }
}
