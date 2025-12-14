use super::ProtocolAdapter;
use async_trait::async_trait;
use log::info;
use std::process::{Child, Command, Stdio};
use std::sync::Arc;
use tokio::sync::Mutex;

/// CJDNS Encrypted Mesh Network Adapter
/// Provides encrypted IPv6 mesh networking
pub struct CjdnsAdapter {
    enabled: bool,
    binary_path: String,
    config_path: Option<String>,
    process: Arc<Mutex<Option<Child>>>,
}

impl CjdnsAdapter {
    pub fn new(enabled: bool, binary_path: String, config_path: Option<String>) -> Self {
        Self {
            enabled,
            binary_path,
            config_path,
            process: Arc::new(Mutex::new(None)),
        }
    }
}

#[async_trait]
impl ProtocolAdapter for CjdnsAdapter {
    async fn start(&self) -> anyhow::Result<()> {
        if !self.enabled {
            info!("CJDNS is disabled, skipping start.");
            return Ok(());
        }

        info!("Starting CJDNS...");
        let config = self.config_path.as_deref().unwrap_or("cjdroute.conf");

        let child = Command::new(&self.binary_path)
            .stdin(std::fs::File::open(config)?)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;

        let mut proc_lock = self.process.lock().await;
        *proc_lock = Some(child);

        info!("CJDNS started successfully.");
        Ok(())
    }

    async fn stop(&self) -> anyhow::Result<()> {
        let mut proc_lock = self.process.lock().await;
        if let Some(ref mut child) = *proc_lock {
            child.kill()?;
            *proc_lock = None;
            info!("CJDNS stopped.");
        }
        Ok(())
    }

    async fn is_healthy(&self) -> bool {
        let proc_lock = self.process.lock().await;
        proc_lock.is_some()
    }

    fn get_proxy_addr(&self) -> String {
        // CJDNS creates a TUN interface, no SOCKS proxy
        String::new()
    }
}
