use crate::adapters::{
    freenet::FreenetAdapter, gnunet::GnunetAdapter, i2p::I2pAdapter, ipfs::IpfsAdapter,
    lokinet::LokinetAdapter, nym::NymAdapter, retroshare::RetroShareAdapter, tor::TorAdapter,
    tribler::TriblerAdapter, zeronet::ZeroNetAdapter, ProtocolAdapter,
};
use crate::config::{
    ChainMode, FreenetSettings, GnunetSettings, I2pSettings, IpfsSettings, LokinetSettings,
    NymSettings, RetroShareSettings, TorSettings, TriblerSettings, ZeroNetSettings,
};
use crate::health_monitor::{new_health_state, run_health_monitor, HealthState, Protocol};
use log::{error, info};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;

pub struct ProcessManager {
    chain_mode: ChainMode,
    tor_adapter: Arc<TorAdapter>,
    i2p_adapter: Arc<I2pAdapter>,
    nym_adapter: Arc<NymAdapter>,
    lokinet_adapter: Arc<LokinetAdapter>,
    ipfs_adapter: Arc<IpfsAdapter>,
    zeronet_adapter: Arc<ZeroNetAdapter>,
    freenet_adapter: Arc<FreenetAdapter>,
    retroshare_adapter: Arc<RetroShareAdapter>,
    gnunet_adapter: Arc<GnunetAdapter>,
    tribler_adapter: Arc<TriblerAdapter>,
    pub health_state: HealthState,
    enabled_protocols: HashSet<Protocol>,
}

impl ProcessManager {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        chain_mode: ChainMode,
        enabled_protocols: HashSet<Protocol>,
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
            tor_adapter: Arc::new(TorAdapter::new(tor)),
            i2p_adapter: Arc::new(I2pAdapter::new(i2p)),
            nym_adapter: Arc::new(NymAdapter::new(nym)),
            lokinet_adapter: Arc::new(LokinetAdapter::new(lokinet)),
            ipfs_adapter: Arc::new(IpfsAdapter::new(ipfs)),
            zeronet_adapter: Arc::new(ZeroNetAdapter::new(zeronet)),
            freenet_adapter: Arc::new(FreenetAdapter::new(freenet)),
            retroshare_adapter: Arc::new(RetroShareAdapter::new(retroshare)),
            gnunet_adapter: Arc::new(GnunetAdapter::new(gnunet)),
            tribler_adapter: Arc::new(TriblerAdapter::new(tribler)),
            health_state: new_health_state(),
            enabled_protocols,
        }
    }

    pub async fn start_processes(&self) -> anyhow::Result<()> {
        // Phase 4: Protocol Chaining - Start dependencies first
        match self.chain_mode {
            ChainMode::TorOverNym => {
                if self.enabled_protocols.contains(&Protocol::Nym) {
                    info!("Chain Mode: Tor over Nym. Starting Nym first.");
                    if let Err(e) = self.nym_adapter.start().await {
                        error!("Failed to start Nym (required for chaining): {}", e);
                    }
                    // Give Nym time to initialize before starting Tor
                    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                }
                
                if self.enabled_protocols.contains(&Protocol::Tor) {
                    if let Err(e) = self.tor_adapter.start().await {
                        error!("Failed to start Tor: {}", e);
                    }
                }
            }
            ChainMode::NymOverTor => {
                if self.enabled_protocols.contains(&Protocol::Tor) {
                    info!("Chain Mode: Nym over Tor. Starting Tor first.");
                    if let Err(e) = self.tor_adapter.start().await {
                        error!("Failed to start Tor (required for chaining): {}", e);
                    }
                    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                }

                if self.enabled_protocols.contains(&Protocol::Nym) {
                    if let Err(e) = self.nym_adapter.start().await {
                        error!("Failed to start Nym: {}", e);
                    }
                }
            }
            ChainMode::None => {
                // Normal startup order
                if self.enabled_protocols.contains(&Protocol::Tor) {
                    if let Err(e) = self.tor_adapter.start().await {
                        error!("Failed to start Tor: {}", e);
                    }
                }
                if self.enabled_protocols.contains(&Protocol::Nym) {
                    if let Err(e) = self.nym_adapter.start().await {
                        error!("Failed to start Nym: {}", e);
                    }
                }
            }
        }

        // Start remaining adapters if enabled
        if self.enabled_protocols.contains(&Protocol::I2p) {
            if let Err(e) = self.i2p_adapter.start().await {
                error!("Failed to start I2PD: {}", e);
            }
        }
        if self.enabled_protocols.contains(&Protocol::Lokinet) {
            if let Err(e) = self.lokinet_adapter.start().await {
                error!("Failed to start Lokinet: {}", e);
            }
        }
        
        // Phase 2.5: Decentralized Web
        if self.enabled_protocols.contains(&Protocol::Ipfs) {
            if let Err(e) = self.ipfs_adapter.start().await {
                error!("Failed to start IPFS: {}", e);
            }
        }
        if self.enabled_protocols.contains(&Protocol::ZeroNet) {
            if let Err(e) = self.zeronet_adapter.start().await {
                error!("Failed to start ZeroNet: {}", e);
            }
        }
        if self.enabled_protocols.contains(&Protocol::Freenet) {
            if let Err(e) = self.freenet_adapter.start().await {
                error!("Failed to start Freenet: {}", e);
            }
        }
        
        // Extended Phase 2.5
        if self.enabled_protocols.contains(&Protocol::RetroShare) {
            if let Err(e) = self.retroshare_adapter.start().await {
                error!("Failed to start RetroShare: {}", e);
            }
        }
        if self.enabled_protocols.contains(&Protocol::GnuNet) {
            if let Err(e) = self.gnunet_adapter.start().await {
                error!("Failed to start GNUnet: {}", e);
            }
        }
        if self.enabled_protocols.contains(&Protocol::Tribler) {
            if let Err(e) = self.tribler_adapter.start().await {
                error!("Failed to start Tribler: {}", e);
            }
        }

        // Start Health Monitors (only for enabled protocols)
        let interval = Duration::from_secs(5);
        
        if self.enabled_protocols.contains(&Protocol::Tor) { tokio::spawn(run_health_monitor(Protocol::Tor, self.tor_adapter.clone(), self.health_state.clone(), interval)); }
        if self.enabled_protocols.contains(&Protocol::I2p) { tokio::spawn(run_health_monitor(Protocol::I2p, self.i2p_adapter.clone(), self.health_state.clone(), interval)); }
        if self.enabled_protocols.contains(&Protocol::Nym) { tokio::spawn(run_health_monitor(Protocol::Nym, self.nym_adapter.clone(), self.health_state.clone(), interval)); }
        if self.enabled_protocols.contains(&Protocol::Lokinet) { tokio::spawn(run_health_monitor(Protocol::Lokinet, self.lokinet_adapter.clone(), self.health_state.clone(), interval)); }
        if self.enabled_protocols.contains(&Protocol::Ipfs) { tokio::spawn(run_health_monitor(Protocol::Ipfs, self.ipfs_adapter.clone(), self.health_state.clone(), interval)); }
        if self.enabled_protocols.contains(&Protocol::ZeroNet) { tokio::spawn(run_health_monitor(Protocol::ZeroNet, self.zeronet_adapter.clone(), self.health_state.clone(), interval)); }
        if self.enabled_protocols.contains(&Protocol::Freenet) { tokio::spawn(run_health_monitor(Protocol::Freenet, self.freenet_adapter.clone(), self.health_state.clone(), interval)); }
        if self.enabled_protocols.contains(&Protocol::RetroShare) { tokio::spawn(run_health_monitor(Protocol::RetroShare, self.retroshare_adapter.clone(), self.health_state.clone(), interval)); }
        if self.enabled_protocols.contains(&Protocol::GnuNet) { tokio::spawn(run_health_monitor(Protocol::GnuNet, self.gnunet_adapter.clone(), self.health_state.clone(), interval)); }
        if self.enabled_protocols.contains(&Protocol::Tribler) { tokio::spawn(run_health_monitor(Protocol::Tribler, self.tribler_adapter.clone(), self.health_state.clone(), interval)); }

        Ok(())
    }
}
