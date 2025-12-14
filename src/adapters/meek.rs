use super::ProtocolAdapter;
use async_trait::async_trait;
use log::info;
use std::process::{Child, Command, Stdio};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Meek Pluggable Transport Adapter
/// Domain fronting for Tor to bypass censorship
pub struct MeekAdapter {
    enabled: bool,
    binary_path: String,
    front_domain: String,
    process: Arc<Mutex<Option<Child>>>,
}

impl MeekAdapter {
    pub fn new(enabled: bool, binary_path: String, front_domain: String) -> Self {
        Self {
            enabled,
            binary_path,
            front_domain,
            process: Arc::new(Mutex::new(None)),
        }
    }
}

#[async_trait]
impl ProtocolAdapter for MeekAdapter {
    async fn start(&self) -> anyhow::Result<()> {
        if !self.enabled {
            info!("Meek is disabled, skipping start.");
            return Ok(());
        }

        info!("Starting Meek transport...");
        let child = Command::new(&self.binary_path)
            .env("TOR_PT_MANAGED_TRANSPORT_VER", "1")
            .env("TOR_PT_STATE_LOCATION", "data/meek")
            .env("TOR_PT_CLIENT_TRANSPORTS", "meek_lite")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;

        let mut proc_lock = self.process.lock().await;
        *proc_lock = Some(child);

        info!(
            "Meek transport started with front domain: {}",
            self.front_domain
        );
        Ok(())
    }

    async fn stop(&self) -> anyhow::Result<()> {
        let mut proc_lock = self.process.lock().await;
        if let Some(ref mut child) = *proc_lock {
            child.kill()?;
            *proc_lock = None;
            info!("Meek stopped.");
        }
        Ok(())
    }

    async fn is_healthy(&self) -> bool {
        let proc_lock = self.process.lock().await;
        proc_lock.is_some()
    }

    fn get_proxy_addr(&self) -> String {
        // Meek is a transport, not direct proxy
        String::new()
    }
}
