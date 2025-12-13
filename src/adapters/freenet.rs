use super::ProtocolAdapter;
use crate::config::FreenetSettings;
use anyhow::Result;
use async_trait::async_trait;
use log::{info, warn};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::Mutex;

/// Freenet/Hyphanet adapter using FCP (Freenet Client Protocol)
///
/// FCP is a text-based protocol over TCP, typically on port 9481.
/// This adapter provides basic connectivity - full FCP implementation
/// would require handling complex message types.
pub struct FreenetAdapter {
    settings: FreenetSettings,
    connected: Arc<Mutex<bool>>,
}

impl FreenetAdapter {
    pub fn new(settings: FreenetSettings) -> Self {
        Self {
            settings,
            connected: Arc::new(Mutex::new(false)),
        }
    }

    /// Send a simple FCP command and read response
    async fn fcp_hello(&self) -> Result<bool> {
        let addr = format!("{}:{}", self.settings.host, self.settings.fcp_port);
        let mut stream = TcpStream::connect(&addr).await?;

        // FCP ClientHello message
        let hello = "ClientHello\n\
                     Name=Chimera\n\
                     ExpectedVersion=2.0\n\
                     EndMessage\n";

        stream.write_all(hello.as_bytes()).await?;

        // Read response (NodeHello or error)
        let mut buf = vec![0u8; 1024];
        let n = stream.read(&mut buf).await?;
        let response = String::from_utf8_lossy(&buf[..n]);

        Ok(response.contains("NodeHello"))
    }
}

#[async_trait]
impl ProtocolAdapter for FreenetAdapter {
    async fn start(&self) -> Result<()> {
        if !self.settings.enabled {
            return Ok(());
        }

        info!("Initializing Freenet/Hyphanet adapter...");
        info!(
            "Connecting to FCP at {}:{}",
            self.settings.host, self.settings.fcp_port
        );

        match self.fcp_hello().await {
            Ok(true) => {
                info!("Connected to Freenet node via FCP");
            }
            Ok(false) => {
                warn!("Freenet node rejected connection");
                return Ok(());
            }
            Err(e) => {
                warn!("Freenet FCP not responding: {}", e);
                warn!(
                    "Ensure Hyphanet is running with FCP enabled on port {}",
                    self.settings.fcp_port
                );
                return Ok(());
            }
        }

        let mut connected = self.connected.lock().await;
        *connected = true;

        info!(
            "Freenet adapter started (FProxy: http://{}:{})",
            self.settings.host, self.settings.fproxy_port
        );
        Ok(())
    }

    async fn stop(&self) -> Result<()> {
        let mut connected = self.connected.lock().await;
        if *connected {
            *connected = false;
            info!("Freenet adapter stopped.");
        }
        Ok(())
    }

    fn get_proxy_addr(&self) -> String {
        // Freenet uses FProxy (HTTP) for content access
        format!("{}:{}", self.settings.host, self.settings.fproxy_port)
    }

    async fn is_healthy(&self) -> bool {
        let connected = self.connected.lock().await;
        *connected
    }
}
