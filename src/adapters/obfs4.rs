use super::ProtocolAdapter;
use async_trait::async_trait;
use log::info;
use std::process::{Child, Command, Stdio};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Obfs4 Pluggable Transport Adapter
/// Obfuscation layer for Tor to bypass censorship
pub struct Obfs4Adapter {
    enabled: bool,
    binary_path: String,
    listen_port: u16,
    process: Arc<Mutex<Option<Child>>>,
}

impl Obfs4Adapter {
    pub fn new(enabled: bool, binary_path: String, listen_port: u16) -> Self {
        Self {
            enabled,
            binary_path,
            listen_port,
            process: Arc::new(Mutex::new(None)),
        }
    }
}

#[async_trait]
impl ProtocolAdapter for Obfs4Adapter {
    async fn start(&self) -> anyhow::Result<()> {
        if !self.enabled {
            info!("Obfs4 is disabled, skipping start.");
            return Ok(());
        }

        info!("Starting Obfs4 transport...");
        let child = Command::new(&self.binary_path)
            .env("TOR_PT_MANAGED_TRANSPORT_VER", "1")
            .env("TOR_PT_STATE_LOCATION", "data/obfs4")
            .env("TOR_PT_SERVER_TRANSPORTS", "obfs4")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;

        let mut proc_lock = self.process.lock().await;
        *proc_lock = Some(child);

        info!("Obfs4 transport started.");
        Ok(())
    }

    async fn stop(&self) -> anyhow::Result<()> {
        let mut proc_lock = self.process.lock().await;
        if let Some(ref mut child) = *proc_lock {
            child.kill()?;
            *proc_lock = None;
            info!("Obfs4 stopped.");
        }
        Ok(())
    }

    async fn is_healthy(&self) -> bool {
        let proc_lock = self.process.lock().await;
        proc_lock.is_some()
    }

    fn get_proxy_addr(&self) -> String {
        // Obfs4 is a transport, not direct proxy
        String::new()
    }
}
