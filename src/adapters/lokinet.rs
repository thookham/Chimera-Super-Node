use super::ProtocolAdapter;
use crate::config::LokinetSettings;
use anyhow::Result;
use async_trait::async_trait;
use log::{info, warn};
use std::fs;
use std::path::Path;
use std::process::Stdio;
use std::sync::Arc;
use tokio::process::{Child, Command};
use tokio::sync::Mutex;

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

        warn!("⚠️  Lokinet integration is EXPERIMENTAL. VPN/Tun mode may not fully support SOCKS5 proxy routing.");

        if !Path::new(&self.settings.binary_path).exists() {
            warn!(
                "Lokinet binary not found at {}. Skipping Lokinet start.",
                self.settings.binary_path
            );
            return Ok(());
        }

        info!("Starting Lokinet...");

        // Prepare data directory
        let data_dir = Path::new("data/lokinet");
        if !data_dir.exists() {
            fs::create_dir_all(data_dir)?;
        }

        // Copy template config to data dir
        let config_path = data_dir.join("lokinet.ini");
        if Path::new("chimera.lokinet.ini").exists() {
            fs::copy("chimera.lokinet.ini", &config_path)?;
        } else {
            warn!("chimera.lokinet.ini template not found!");
        }

        // Lokinet usually runs as a system service or needs admin privileges
        // For this adapter, we assume we are running the binary directly
        let child = Command::new(&self.settings.binary_path)
            .arg(config_path.to_str().unwrap_or("lokinet.ini"))
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;

        let mut proc_lock = self.process.lock().await;
        *proc_lock = Some(child);

        info!(
            "Lokinet started successfully (SOCKS5 on port {}).",
            self.settings.socks_port
        );
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
        // Return the configured SOCKS5 port for Lokinet
        // Note: Requires exit node configuration in lokinet.ini for full SOCKS5 support
        format!("127.0.0.1:{}", self.settings.socks_port)
    }

    async fn is_healthy(&self) -> bool {
        let proc_lock = self.process.lock().await;
        proc_lock.is_some()
    }
}
