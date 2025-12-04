use tokio::process::Command;
use std::process::Stdio;
use log::{info, warn, error};
use std::path::Path;

pub struct ProcessManager {
    tor_path: String,
    i2p_path: String,
}

impl ProcessManager {
    pub fn new() -> Self {
        Self {
            tor_path: "bin/tor.exe".to_string(),
            i2p_path: "bin/i2pd.exe".to_string(),
        }
    }

    pub async fn start_processes(&self) -> anyhow::Result<()> {
        self.start_tor().await?;
        self.start_i2p().await?;
        Ok(())
    }

    async fn start_tor(&self) -> anyhow::Result<()> {
        if !Path::new(&self.tor_path).exists() {
            warn!("Tor binary not found at {}. Skipping Tor start.", self.tor_path);
            return Ok(());
        }

        info!("Starting Tor...");
        let child = Command::new(&self.tor_path)
            .arg("--SocksPort")
            .arg("9052") // Internal Tor port (Chimera listens on 9050)
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
        if !Path::new(&self.i2p_path).exists() {
            warn!("I2PD binary not found at {}. Skipping I2P start.", self.i2p_path);
            return Ok(());
        }

        info!("Starting I2PD...");
        let child = Command::new(&self.i2p_path)
            .arg("--socksproxy.enabled=true")
            .arg("--socksproxy.port=4447")
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
