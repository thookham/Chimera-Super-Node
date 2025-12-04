use async_trait::async_trait;
use anyhow::Result;
use log::{info, warn, error};
use tokio::process::{Command, Child};
use std::process::Stdio;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::config::NymSettings;
use super::ProtocolAdapter;

pub struct NymAdapter {
    settings: NymSettings,
    process: Arc<Mutex<Option<Child>>>,
}

impl NymAdapter {
    pub fn new(settings: NymSettings) -> Self {
        Self {
            settings,
            process: Arc::new(Mutex::new(None)),
        }
    }
}

#[async_trait]
impl ProtocolAdapter for NymAdapter {
    async fn start(&self) -> Result<()> {
        if !self.settings.enabled {
            return Ok(());
        }

        if !Path::new(&self.settings.binary_path).exists() {
            warn!("Nym binary not found at {}. Skipping Nym start.", self.settings.binary_path);
            return Ok(());
        }

        info!("Starting Nym SOCKS5 Client...");
        // Example command: nym-socks5-client run --id my-client
        let child = Command::new(&self.settings.binary_path)
            .arg("run")
            .arg("--id")
            .arg("chimera-client")
            // .arg("--port") // Assuming we can configure port via args or config file
            // .arg(self.settings.socks_port.to_string())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;

        let mut proc_lock = self.process.lock().await;
        *proc_lock = Some(child);
        
        info!("Nym started successfully.");
        Ok(())
    }

    async fn stop(&self) -> Result<()> {
        let mut proc_lock = self.process.lock().await;
        if let Some(mut child) = proc_lock.take() {
            info!("Stopping Nym...");
            child.kill().await?;
        }
        Ok(())
    }

    fn get_proxy_addr(&self) -> String {
        format!("127.0.0.1:{}", self.settings.socks_port)
    }

    async fn is_healthy(&self) -> bool {
        // TODO: Implement actual health check (e.g. TCP connect)
        let proc_lock = self.process.lock().await;
        proc_lock.is_some()
    }
}
