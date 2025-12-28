use super::ProtocolAdapter;
use crate::config::I2pSettings;
use anyhow::Result;
use async_trait::async_trait;
use log::{info, warn};
use std::path::Path;
use std::process::Stdio;
use std::sync::Arc;
use tokio::process::{Child, Command};
use tokio::sync::Mutex;

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
            warn!(
                "I2PD binary not found at {}. Skipping I2P start.",
                self.settings.binary_path
            );
            return Ok(());
        }

        info!("Starting I2PD...");
        let mut cmd = Command::new(&self.settings.binary_path);
        cmd.arg(format!("--socksproxy.port={}", self.settings.socks_port))
            .arg(format!(
                "--httpproxy.port={}",
                self.settings.http_proxy_port
            ))
            .arg("--datadir=data/i2p");

        let data_dir = Path::new("data/i2p");
        if !data_dir.exists() {
            info!("Creating I2P data directory: {:?}", data_dir);
            std::fs::create_dir_all(data_dir)?;
        }

        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

        let mut child = cmd.spawn()?;

        if let Some(stdout) = child.stdout.take() {
             tokio::spawn(async move {
                 use tokio::io::{AsyncBufReadExt, BufReader};
                 let mut reader = BufReader::new(stdout).lines();
                 while let Ok(Some(line)) = reader.next_line().await {
                     info!("[I2PD] {}", line);
                 }
             });
        }
        if let Some(stderr) = child.stderr.take() {
             tokio::spawn(async move {
                 use tokio::io::{AsyncBufReadExt, BufReader};
                 let mut reader = BufReader::new(stderr).lines();
                 while let Ok(Some(line)) = reader.next_line().await {
                     log::warn!("[I2PD] {}", line);
                 }
             });
        }

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
