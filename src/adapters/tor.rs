use super::ProtocolAdapter;
use crate::config::TorSettings;
use anyhow::Result;
use async_trait::async_trait;
use log::{info, warn};
use std::path::Path;
use std::process::Stdio;
use std::sync::Arc;
use tokio::process::{Child, Command};
use tokio::sync::Mutex;

pub struct TorAdapter {
    settings: TorSettings,
    process: Arc<Mutex<Option<Child>>>,
}

impl TorAdapter {
    pub fn new(settings: TorSettings) -> Self {
        Self {
            settings,
            process: Arc::new(Mutex::new(None)),
        }
    }
}

#[async_trait]
impl ProtocolAdapter for TorAdapter {
    async fn start(&self) -> Result<()> {
        if !self.settings.enabled {
            return Ok(());
        }

        if !Path::new(&self.settings.binary_path).exists() {
            warn!(
                "Tor binary not found at {}. Skipping Tor start.",
                self.settings.binary_path
            );
            return Ok(());
        }

        info!("Starting Tor...");
        let child = Command::new(&self.settings.binary_path)
            .arg("--SocksPort")
            .arg(self.settings.socks_port.to_string())
            .arg("--ControlPort")
            .arg(self.settings.control_port.to_string())
            .arg("--DataDirectory")
            .arg("data/tor")
            .stdout(Stdio::null()) // TODO: Capture logs
            .stderr(Stdio::null())
            .spawn()?;

        let mut proc_lock = self.process.lock().await;
        *proc_lock = Some(child);

        info!("Tor started successfully.");
        Ok(())
    }

    async fn stop(&self) -> Result<()> {
        let mut proc_lock = self.process.lock().await;
        if let Some(mut child) = proc_lock.take() {
            info!("Stopping Tor...");
            child.kill().await?;
        }
        Ok(())
    }

    fn get_proxy_addr(&self) -> String {
        format!("127.0.0.1:{}", self.settings.socks_port)
    }

    async fn is_healthy(&self) -> bool {
        let proc_lock = self.process.lock().await;
        proc_lock.is_some()
    }
}
