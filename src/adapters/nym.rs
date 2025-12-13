use super::ProtocolAdapter;
use crate::config::NymSettings;
use anyhow::Result;
use async_trait::async_trait;
use log::{error, info, warn};
use nym_sdk::mixnet;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct NymAdapter {
    settings: NymSettings,
    // We store the client wrapped in an Option.
    // If the type is not exactly SmsMixnetClient, we might need to adjust.
    // Based on SDK patterns, it is likely exported or public.
    client: Arc<Mutex<Option<mixnet::Socks5MixnetClient>>>,
}

impl NymAdapter {
    pub fn new(settings: NymSettings) -> Self {
        Self {
            settings,
            client: Arc::new(Mutex::new(None)),
        }
    }
}

#[async_trait]
impl ProtocolAdapter for NymAdapter {
    async fn start(&self) -> Result<()> {
        if !self.settings.enabled {
            return Ok(());
        }

        let provider = match &self.settings.upstream_provider {
            Some(p) => p,
            None => {
                error!("Nym is enabled but no 'upstream_provider' is configured. SOCKS5 client cannot start.");
                warn!("Please set 'nym.upstream_provider' in your config to a valid Nym Network Requester address.");
                // We return Ok to not crash the Main server, but Nym won't work.
                return Ok(());
            }
        };

        info!("Initializing Native Nym Client with provider: {}", provider);

        let data_dir = Path::new("data/nym");
        if !data_dir.exists() {
            fs::create_dir_all(data_dir)?;
        }

        // Configure SOCKS5
        let socks5_config = mixnet::Socks5::new(provider.clone());

        // Use new_ephemeral for now to avoid storage complexity issues in this phase
        let client_builder =
            mixnet::MixnetClientBuilder::new_ephemeral().socks5_config(socks5_config);

        let client_instance = client_builder.build()?;

        info!("Connecting to Mixnet...");
        let connected_client = client_instance.connect_to_mixnet_via_socks5().await?;

        let url = connected_client.socks5_url();
        info!("Nym SOCKS5 Client listening at: {}", url);

        let mut client_lock = self.client.lock().await;
        *client_lock = Some(connected_client);

        info!("Nym Client started successfully.");
        Ok(())
    }

    async fn stop(&self) -> Result<()> {
        let mut client_lock = self.client.lock().await;
        if let Some(client) = client_lock.take() {
            info!("Disconnecting Nym Client...");
            client.disconnect().await;
        }
        Ok(())
    }

    fn get_proxy_addr(&self) -> String {
        format!("127.0.0.1:{}", self.settings.socks_port)
    }

    async fn is_healthy(&self) -> bool {
        let client_lock = self.client.lock().await;
        client_lock.is_some()
    }
}
