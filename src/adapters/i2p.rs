use async_trait::async_trait;
use anyhow::Result;
use log::{info, warn};
use tokio::process::{Command, Child};
use std::process::Stdio;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::config::I2pSettings;
use super::ProtocolAdapter;

pub struct I2pAdapter {
    settings: I2pSettings,
    process: Arc<Mutex<Option<Child>>>,
}

impl I2pAdapter {
    pub fn new(settings: I2pSettings) -> Self {
        Self {
            settings,
            process: Arc::new(Mutex::new(None)),
        }
    }
}

#[async_trait]
impl ProtocolAdapter for I2pAdapter {
    async fn start(&self) -> Result<()> {
        if !self.settings.enabled {
            return Ok(());
        }

        if !Path::new(&self.settings.binary_path).exists() {
            warn!("I2PD binary not found at {}. Skipping I2P start.", self.settings.binary_path);
            return Ok(());
        }

        info!("Starting I2PD...");
        let child = Command::new(&self.settings.binary_path)
            .arg(format!("--socksproxy.port={}", self.settings.socks_port))
            .arg(format!("--httpproxy.port={}", self.settings.http_proxy_port))
            .arg("--datadir=data/i2p")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;

        let mut proc_lock = self.process.lock().await;
        *proc_lock = Some(child);
        
        info!("I2PD started successfully.");
        Ok(())
    }

    async fn stop(&self) -> Result<()> {
        let mut proc_lock = self.process.lock().await;
        if let Some(mut child) = proc_lock.take() {
            info!("Stopping I2PD...");
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
