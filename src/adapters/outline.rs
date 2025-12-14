use super::ProtocolAdapter;
use async_trait::async_trait;
use log::info;
use std::process::{Child, Command, Stdio};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Outline VPN/Proxy Adapter
/// Shadowsocks-based easy-deploy proxy
pub struct OutlineAdapter {
    enabled: bool,
    binary_path: String,
    config_path: Option<String>,
    socks_port: u16,
    process: Arc<Mutex<Option<Child>>>,
}

impl OutlineAdapter {
    pub fn new(
        enabled: bool,
        binary_path: String,
        config_path: Option<String>,
        socks_port: u16,
    ) -> Self {
        Self {
            enabled,
            binary_path,
            config_path,
            socks_port,
            process: Arc::new(Mutex::new(None)),
        }
    }
}

#[async_trait]
impl ProtocolAdapter for OutlineAdapter {
    async fn start(&self) -> anyhow::Result<()> {
        if !self.enabled {
            info!("Outline is disabled, skipping start.");
            return Ok(());
        }

        info!("Starting Outline (Shadowsocks)...");
        let mut cmd = Command::new(&self.binary_path);
        cmd.arg("-local_port").arg(self.socks_port.to_string());

        if let Some(ref config) = self.config_path {
            cmd.arg("-config").arg(config);
        }

        let child = cmd.stdout(Stdio::null()).stderr(Stdio::null()).spawn()?;

        let mut proc_lock = self.process.lock().await;
        *proc_lock = Some(child);

        info!("Outline started on SOCKS port {}.", self.socks_port);
        Ok(())
    }

    async fn stop(&self) -> anyhow::Result<()> {
        let mut proc_lock = self.process.lock().await;
        if let Some(ref mut child) = *proc_lock {
            child.kill()?;
            *proc_lock = None;
            info!("Outline stopped.");
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
