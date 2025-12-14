use super::ProtocolAdapter;
use async_trait::async_trait;
use log::info;
use std::process::{Child, Command, Stdio};
use std::sync::Arc;
use tokio::sync::Mutex;

/// WireGuard VPN Adapter
/// Fast, modern VPN for secure exit traffic
pub struct WireGuardAdapter {
    enabled: bool,
    binary_path: String,
    interface_name: String,
    config_path: Option<String>,
    process: Arc<Mutex<Option<Child>>>,
}

impl WireGuardAdapter {
    pub fn new(
        enabled: bool,
        binary_path: String,
        interface_name: String,
        config_path: Option<String>,
    ) -> Self {
        Self {
            enabled,
            binary_path,
            interface_name,
            config_path,
            process: Arc::new(Mutex::new(None)),
        }
    }
}

#[async_trait]
impl ProtocolAdapter for WireGuardAdapter {
    async fn start(&self) -> anyhow::Result<()> {
        if !self.enabled {
            info!("WireGuard is disabled, skipping start.");
            return Ok(());
        }

        info!("Starting WireGuard...");
        let config = self.config_path.as_deref().unwrap_or("wg0.conf");

        let child = Command::new(&self.binary_path)
            .arg("up")
            .arg(config)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;

        let mut proc_lock = self.process.lock().await;
        *proc_lock = Some(child);

        info!("WireGuard interface {} started.", self.interface_name);
        Ok(())
    }

    async fn stop(&self) -> anyhow::Result<()> {
        let config = self.config_path.as_deref().unwrap_or("wg0.conf");

        // WireGuard uses wg-quick down to stop
        let _ = Command::new(&self.binary_path)
            .arg("down")
            .arg(config)
            .output();

        let mut proc_lock = self.process.lock().await;
        *proc_lock = None;
        info!("WireGuard stopped.");
        Ok(())
    }

    async fn is_healthy(&self) -> bool {
        let proc_lock = self.process.lock().await;
        proc_lock.is_some()
    }

    fn get_proxy_addr(&self) -> String {
        // WireGuard creates a VPN tunnel, not a SOCKS proxy
        String::new()
    }
}
