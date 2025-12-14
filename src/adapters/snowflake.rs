use super::ProtocolAdapter;
use async_trait::async_trait;
use log::info;
use std::process::{Child, Command, Stdio};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Snowflake Pluggable Transport Adapter
/// WebRTC-based pluggable transport for Tor
pub struct SnowflakeAdapter {
    enabled: bool,
    binary_path: String,
    broker_url: String,
    process: Arc<Mutex<Option<Child>>>,
}

impl SnowflakeAdapter {
    pub fn new(enabled: bool, binary_path: String, broker_url: String) -> Self {
        Self {
            enabled,
            binary_path,
            broker_url,
            process: Arc::new(Mutex::new(None)),
        }
    }
}

#[async_trait]
impl ProtocolAdapter for SnowflakeAdapter {
    async fn start(&self) -> anyhow::Result<()> {
        if !self.enabled {
            info!("Snowflake is disabled, skipping start.");
            return Ok(());
        }

        info!("Starting Snowflake proxy...");
        let child = Command::new(&self.binary_path)
            .arg("-url")
            .arg(&self.broker_url)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;

        let mut proc_lock = self.process.lock().await;
        *proc_lock = Some(child);

        info!("Snowflake proxy started successfully.");
        Ok(())
    }

    async fn stop(&self) -> anyhow::Result<()> {
        let mut proc_lock = self.process.lock().await;
        if let Some(ref mut child) = *proc_lock {
            child.kill()?;
            *proc_lock = None;
            info!("Snowflake stopped.");
        }
        Ok(())
    }

    async fn is_healthy(&self) -> bool {
        let proc_lock = self.process.lock().await;
        proc_lock.is_some()
    }

    fn get_proxy_addr(&self) -> String {
        // Snowflake is a transport, not a direct proxy
        String::new()
    }
}
