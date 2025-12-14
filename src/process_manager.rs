use crate::adapters::{
    freenet::FreenetAdapter, gnunet::GnunetAdapter, i2p::I2pAdapter, ipfs::IpfsAdapter,
    lokinet::LokinetAdapter, nym::NymAdapter, retroshare::RetroShareAdapter, tor::TorAdapter,
    tribler::TriblerAdapter, zeronet::ZeroNetAdapter, ProtocolAdapter,
};
use crate::config::{
    ChainMode, FreenetSettings, GnunetSettings, I2pSettings, IpfsSettings, LokinetSettings,
    NymSettings, RetroShareSettings, TorSettings, TriblerSettings, ZeroNetSettings,
};
use log::{error, info};

pub struct ProcessManager {
    chain_mode: ChainMode,
    tor_adapter: TorAdapter,
    i2p_adapter: I2pAdapter,
    nym_adapter: NymAdapter,
    lokinet_adapter: LokinetAdapter,
    ipfs_adapter: IpfsAdapter,
    zeronet_adapter: ZeroNetAdapter,
    freenet_adapter: FreenetAdapter,
    retroshare_adapter: RetroShareAdapter,
    gnunet_adapter: GnunetAdapter,
    tribler_adapter: TriblerAdapter,
}

impl ProcessManager {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        chain_mode: ChainMode,
        tor: TorSettings,
        i2p: I2pSettings,
        nym: NymSettings,
        lokinet: LokinetSettings,
        ipfs: IpfsSettings,
        zeronet: ZeroNetSettings,
        freenet: FreenetSettings,
        retroshare: RetroShareSettings,
        gnunet: GnunetSettings,
        tribler: TriblerSettings,
    ) -> Self {
        Self {
            chain_mode,
            tor_adapter: TorAdapter::new(tor),
            i2p_adapter: I2pAdapter::new(i2p),
            nym_adapter: NymAdapter::new(nym),
            lokinet_adapter: LokinetAdapter::new(lokinet),
            ipfs_adapter: IpfsAdapter::new(ipfs),
            zeronet_adapter: ZeroNetAdapter::new(zeronet),
            freenet_adapter: FreenetAdapter::new(freenet),
            retroshare_adapter: RetroShareAdapter::new(retroshare),
            gnunet_adapter: GnunetAdapter::new(gnunet),
            tribler_adapter: TriblerAdapter::new(tribler),
        }
    }

    pub async fn start_processes(&self) -> anyhow::Result<()> {
        // Phase 4: Protocol Chaining - Start dependencies first
        match self.chain_mode {
            ChainMode::TorOverNym => {
                info!("Chain Mode: Tor over Nym. Starting Nym first.");
                if let Err(e) = self.nym_adapter.start().await {
                    error!("Failed to start Nym (required for chaining): {}", e);
                }
                // Give Nym time to initialize before starting Tor
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                if let Err(e) = self.tor_adapter.start().await {
                    error!("Failed to start Tor: {}", e);
                }
            }
            ChainMode::NymOverTor => {
                info!("Chain Mode: Nym over Tor. Starting Tor first.");
                if let Err(e) = self.tor_adapter.start().await {
                    error!("Failed to start Tor (required for chaining): {}", e);
                }
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                if let Err(e) = self.nym_adapter.start().await {
                    error!("Failed to start Nym: {}", e);
                }
            }
            ChainMode::None => {
                // Normal startup order
                if let Err(e) = self.tor_adapter.start().await {
                    error!("Failed to start Tor: {}", e);
                }
                if let Err(e) = self.nym_adapter.start().await {
                    error!("Failed to start Nym: {}", e);
                }
            }
        }

        // Start remaining adapters
        if let Err(e) = self.i2p_adapter.start().await {
            error!("Failed to start I2PD: {}", e);
        }
        if let Err(e) = self.lokinet_adapter.start().await {
            error!("Failed to start Lokinet: {}", e);
        }
        // Phase 2.5: Decentralized Web
        if let Err(e) = self.ipfs_adapter.start().await {
            error!("Failed to start IPFS: {}", e);
        }
        if let Err(e) = self.zeronet_adapter.start().await {
            error!("Failed to start ZeroNet: {}", e);
        }
        if let Err(e) = self.freenet_adapter.start().await {
            error!("Failed to start Freenet: {}", e);
        }
        // Extended Phase 2.5
        if let Err(e) = self.retroshare_adapter.start().await {
            error!("Failed to start RetroShare: {}", e);
        }
        if let Err(e) = self.gnunet_adapter.start().await {
            error!("Failed to start GNUnet: {}", e);
        }
        if let Err(e) = self.tribler_adapter.start().await {
            error!("Failed to start Tribler: {}", e);
        }

        Ok(())
    }
}
