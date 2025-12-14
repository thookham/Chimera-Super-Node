use super::ProtocolAdapter;
use async_trait::async_trait;
use log::info;
use std::process::{Child, Command, Stdio};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Session Messenger Adapter (Oxen Network)
/// Decentralized privacy-focused messaging
pub struct SessionAdapter {
    enabled: bool,
    binary_path: String,
    process: Arc<Mutex<Option<Child>>>,
}

impl SessionAdapter {
    pub fn new(enabled: bool, binary_path: String) -> Self {
        Self {
            enabled,
            binary_path,
            process: Arc::new(Mutex::new(None)),
        }
    }
}

#[async_trait]
impl ProtocolAdapter for SessionAdapter {
    async fn start(&self) -> anyhow::Result<()> {
        if !self.enabled {
            info!("Session is disabled, skipping start.");
            return Ok(());
        }

        info!("Starting Session (Oxen)...");
        let child = Command::new(&self.binary_path)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;

        let mut proc_lock = self.process.lock().await;
        *proc_lock = Some(child);

        info!("Session started successfully.");
        Ok(())
    }

    async fn stop(&self) -> anyhow::Result<()> {
        let mut proc_lock = self.process.lock().await;
        if let Some(ref mut child) = *proc_lock {
            child.kill()?;
            *proc_lock = None;
            info!("Session stopped.");
        }
        Ok(())
    }

    async fn is_healthy(&self) -> bool {
        let proc_lock = self.process.lock().await;
        proc_lock.is_some()
    }

    fn get_proxy_addr(&self) -> String {
        // Session uses onion routing, not a SOCKS proxy
        String::new()
    }
}
