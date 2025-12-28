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
        let mut cmd = Command::new(&self.settings.binary_path);
        cmd.arg("--SocksPort")
            .arg(self.settings.socks_port.to_string())
            .arg("--ControlPort")
            .arg(self.settings.control_port.to_string())
            .arg("--DataDirectory")
            .arg("data/tor");

        let data_dir = Path::new("data/tor");
        if !data_dir.exists() {
            info!("Creating Tor data directory: {:?}", data_dir);
            std::fs::create_dir_all(data_dir)?;
        }

        // Phase 4: Protocol Chaining - Use upstream proxy if configured
        if let Some(ref upstream) = self.settings.upstream_proxy {
            info!(
                "Configuring Tor to chain through upstream proxy: {}",
                upstream
            );
            cmd.arg("--Socks5Proxy").arg(upstream);
        }

        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

        let mut child = cmd.spawn()?;

        // Capture stdout/stderr for logging
        if let Some(stdout) = child.stdout.take() {
             tokio::spawn(async move {
                 use tokio::io::{AsyncBufReadExt, BufReader};
                 let mut reader = BufReader::new(stdout).lines();
                 while let Ok(Some(line)) = reader.next_line().await {
                     info!("[Tor] {}", line);
                 }
             });
        }
        if let Some(stderr) = child.stderr.take() {
             tokio::spawn(async move {
                 use tokio::io::{AsyncBufReadExt, BufReader};
                 let mut reader = BufReader::new(stderr).lines();
                 while let Ok(Some(line)) = reader.next_line().await {
                     log::warn!("[Tor] {}", line);
                 }
             });
        }

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
