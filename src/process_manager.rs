use log::error;
use crate::config::{TorSettings, I2pSettings, NymSettings, LokinetSettings};
use crate::adapters::{
    ProtocolAdapter, 
    nym::NymAdapter, 
    lokinet::LokinetAdapter,
    tor::TorAdapter,
    i2p::I2pAdapter
};

pub struct ProcessManager {
    tor_adapter: TorAdapter,
    i2p_adapter: I2pAdapter,
    nym_adapter: NymAdapter,
    lokinet_adapter: LokinetAdapter,
}

impl ProcessManager {
    pub fn new(
        tor: TorSettings, 
        i2p: I2pSettings,
        nym: NymSettings,
        lokinet: LokinetSettings
    ) -> Self {
        Self { 
            tor_adapter: TorAdapter::new(tor),
            i2p_adapter: I2pAdapter::new(i2p),
            nym_adapter: NymAdapter::new(nym),
            lokinet_adapter: LokinetAdapter::new(lokinet),
        }
    }

    pub async fn start_processes(&self) -> anyhow::Result<()> {
        // Start all adapters
        if let Err(e) = self.tor_adapter.start().await {
            error!("Failed to start Tor: {}", e);
        }
        if let Err(e) = self.i2p_adapter.start().await {
            error!("Failed to start I2PD: {}", e);
        }
        if let Err(e) = self.nym_adapter.start().await {
            error!("Failed to start Nym: {}", e);
        }
        if let Err(e) = self.lokinet_adapter.start().await {
            error!("Failed to start Lokinet: {}", e);
        }

        Ok(())
    }
}

