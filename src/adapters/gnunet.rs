use super::ProtocolAdapter;
use crate::config::GnunetSettings;
use anyhow::Result;
use async_trait::async_trait;
use log::{info, warn};
use std::process::Command;
use std::sync::Arc;
use tokio::sync::Mutex;

/// GNUnet Adapter
///
/// Interacts with GNUnet.
/// GNUnet typically runs as a service `gnunet-arm`.
/// Routing via GNS (GNU Name System) usually done via SOCKS proxy on port 1080 (if configured)
/// or via DNS interception.
pub struct GnunetAdapter {
    settings: GnunetSettings,
    connected: Arc<Mutex<bool>>,
}

impl GnunetAdapter {
    pub fn new(settings: GnunetSettings) -> Self {
        Self {
            settings,
            connected: Arc::new(Mutex::new(false)),
        }
    }
}

#[async_trait]
impl ProtocolAdapter for GnunetAdapter {
    async fn start(&self) -> Result<()> {
        if !self.settings.enabled {
            return Ok(());
        }

        info!("Initializing GNUnet adapter...");

        // Check if gnunet-arm is running or accessible
        // Using strict command execution to check version
        let output = Command::new("gnunet-arm").arg("-I").output();

        match output {
            Ok(o) => {
                if o.status.success() {
                    info!("GNUnet service detected.");
                    let mut connected = self.connected.lock().await;
                    *connected = true;
                } else {
                    warn!("GNUnet service check returned error status.");
                }
            }
            Err(e) => {
                warn!("Failed to execute gnunet-arm: {}", e);
                warn!("Is GNUnet installed and in PATH?");
                return Ok(());
            }
        }

        info!(
            "GNUnet adapter started (SOCKS: 127.0.0.1:{})",
            self.settings.socks_port
        );
        Ok(())
    }

    async fn stop(&self) -> Result<()> {
        let mut connected = self.connected.lock().await;
        if *connected {
            *connected = false;
            info!("GNUnet adapter stopped.");
        }
        Ok(())
    }

    fn get_proxy_addr(&self) -> String {
        format!("127.0.0.1:{}", self.settings.socks_port)
    }

    async fn is_healthy(&self) -> bool {
        let connected = self.connected.lock().await;
        *connected
    }
}
