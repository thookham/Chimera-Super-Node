use async_trait::async_trait;
use anyhow::Result;
use log::{info, warn, error};
use tokio::process::{Command, Child};
use std::process::Stdio;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::config::LokinetSettings;
use super::ProtocolAdapter;

pub struct LokinetAdapter {
    settings: LokinetSettings,
    process: Arc<Mutex<Option<Child>>>,
}

impl LokinetAdapter {
    pub fn new(settings: LokinetSettings) -> Self {
        Self {
            settings,
            process: Arc::new(Mutex::new(None)),
        }
    }
}

#[async_trait]
impl ProtocolAdapter for LokinetAdapter {
    async fn start(&self) -> Result<()> {
        if !self.settings.enabled {
            return Ok(());
        }

        if !Path::new(&self.settings.binary_path).exists() {
            warn!("Lokinet binary not found at {}. Skipping Lokinet start.", self.settings.binary_path);
            return Ok(());
        }

        info!("Starting Lokinet...");
        // Lokinet usually runs as a system service or needs admin privileges
        // For this adapter, we assume we are running the binary directly
        let child = Command::new(&self.settings.binary_path)
            .arg(format!("--dns-bind=127.0.0.1:{}", self.settings.dns_port))
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;

        let mut proc_lock = self.process.lock().await;
        *proc_lock = Some(child);
        
        info!("Lokinet started successfully.");
        Ok(())
    }

    async fn stop(&self) -> Result<()> {
        let mut proc_lock = self.process.lock().await;
        if let Some(mut child) = proc_lock.take() {
            info!("Stopping Lokinet...");
            child.kill().await?;
        }
        Ok(())
    }

    fn get_proxy_addr(&self) -> String {
        // Lokinet is usually a VPN/Tun interface, not a SOCKS proxy.
        // But for consistency, we might return a DNS port or a placeholder.
        // Or maybe we need to implement a SOCKS-to-Lokinet bridge.
        // For now, returning empty as it's not a standard SOCKS proxy.
        String::new() 
    }

    async fn is_healthy(&self) -> bool {
        let proc_lock = self.process.lock().await;
        proc_lock.is_some()
    }
}
