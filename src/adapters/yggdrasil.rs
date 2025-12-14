use super::ProtocolAdapter;
use async_trait::async_trait;
use log::info;
use std::process::{Child, Command, Stdio};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Yggdrasil Mesh Network Adapter
/// Provides end-to-end encrypted IPv6 overlay network
pub struct YggdrasilAdapter {
    enabled: bool,
    binary_path: String,
    config_path: Option<String>,
    process: Arc<Mutex<Option<Child>>>,
}

impl YggdrasilAdapter {
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
impl ProtocolAdapter for YggdrasilAdapter {
    async fn start(&self) -> anyhow::Result<()> {
        if !self.enabled {
            info!("Yggdrasil is disabled, skipping start.");
            return Ok(());
        }

        info!("Starting Yggdrasil...");
        let mut cmd = Command::new(&self.binary_path);
        cmd.arg("-useconffile");

        if let Some(ref config) = self.config_path {
            cmd.arg(config);
        } else {
            cmd.arg("yggdrasil.conf");
        }

        let child = cmd.stdout(Stdio::null()).stderr(Stdio::null()).spawn()?;

        let mut proc_lock = self.process.lock().await;
        *proc_lock = Some(child);

        info!("Yggdrasil started successfully.");
        Ok(())
    }

    async fn stop(&self) -> anyhow::Result<()> {
        let mut proc_lock = self.process.lock().await;
        if let Some(ref mut child) = *proc_lock {
            child.kill()?;
            *proc_lock = None;
            info!("Yggdrasil stopped.");
        }
        Ok(())
    }

    async fn is_healthy(&self) -> bool {
        let proc_lock = self.process.lock().await;
        proc_lock.is_some()
    }

    fn get_proxy_addr(&self) -> String {
        // Yggdrasil doesn't use a SOCKS proxy; it creates a TUN interface
        String::new()
    }
}
