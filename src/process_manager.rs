use tokio::process::Command;
use std::process::Stdio;
use log::{info, warn, error};
use std::path::Path;
use crate::config::{TorSettings, I2pSettings};

pub struct ProcessManager {
    tor: TorSettings,
    i2p: I2pSettings,
}

impl ProcessManager {
    pub fn new(tor: TorSettings, i2p: I2pSettings) -> Self {
        Self { tor, i2p }
    }

    pub async fn start_processes(&self) -> anyhow::Result<()> {
        if self.tor.enabled {
            self.start_tor().await?;
        }
        if self.i2p.enabled {
            self.start_i2p().await?;
        }
        Ok(())
    }

    async fn start_tor(&self) -> anyhow::Result<()> {
        if !Path::new(&self.tor.binary_path).exists() {
            warn!("Tor binary not found at {}. Skipping Tor start.", self.tor.binary_path);
            return Ok(());
        }

        info!("Starting Tor...");
        let child = Command::new(&self.tor.binary_path)
            .arg("--SocksPort")
            .arg(self.tor.socks_port.to_string())
            .arg("--ControlPort")
            .arg(self.tor.control_port.to_string())
            .arg("--DataDirectory")
            .arg("data/tor")
            .stdout(Stdio::null()) // TODO: Capture logs
            .stderr(Stdio::null())
            .spawn();

        match child {
            Ok(_) => info!("Tor started successfully."),
            Err(e) => error!("Failed to start Tor: {}", e),
        }
        Ok(())
    }

    async fn start_i2p(&self) -> anyhow::Result<()> {
        if !Path::new(&self.i2p.binary_path).exists() {
            warn!("I2PD binary not found at {}. Skipping I2P start.", self.i2p.binary_path);
            return Ok(());
        }

        info!("Starting I2PD...");
        let child = Command::new(&self.i2p.binary_path)
            .arg(format!("--socksproxy.port={}", self.i2p.socks_port))
            .arg(format!("--httpproxy.port={}", self.i2p.http_proxy_port))
            .arg("--datadir=data/i2p")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn();

        match child {
            Ok(_) => info!("I2PD started successfully."),
            Err(e) => error!("Failed to start I2PD: {}", e),
        }
        Ok(())
    }
}

