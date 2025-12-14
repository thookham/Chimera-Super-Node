use super::ProtocolAdapter;
use async_trait::async_trait;
use log::info;
use std::process::{Child, Command, Stdio};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Mysterium Network Decentralized VPN Adapter
/// Pay-per-use dVPN on blockchain
pub struct MysteriumAdapter {
    enabled: bool,
    binary_path: String,
    socks_port: u16,
    process: Arc<Mutex<Option<Child>>>,
}

impl MysteriumAdapter {
    pub fn new(enabled: bool, binary_path: String, socks_port: u16) -> Self {
        Self {
            enabled,
            binary_path,
            socks_port,
            process: Arc::new(Mutex::new(None)),
        }
    }
}

#[async_trait]
impl ProtocolAdapter for MysteriumAdapter {
    async fn start(&self) -> anyhow::Result<()> {
        if !self.enabled {
            info!("Mysterium is disabled, skipping start.");
            return Ok(());
        }

        info!("Starting Mysterium Network...");
        let child = Command::new(&self.binary_path)
            .arg("service")
            .arg("--agreed-terms-and-conditions")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;

        let mut proc_lock = self.process.lock().await;
        *proc_lock = Some(child);

        info!(
            "Mysterium Network started on SOCKS port {}.",
            self.socks_port
        );
        Ok(())
    }

    async fn stop(&self) -> anyhow::Result<()> {
        let mut proc_lock = self.process.lock().await;
        if let Some(ref mut child) = *proc_lock {
            child.kill()?;
            *proc_lock = None;
            info!("Mysterium stopped.");
        }
        Ok(())
    }

    async fn is_healthy(&self) -> bool {
        let proc_lock = self.process.lock().await;
        proc_lock.is_some()
    }

    fn get_proxy_addr(&self) -> String {
        format!("127.0.0.1:{}", self.socks_port)
    }
}
