use crate::adapters::ProtocolAdapter;
use log::{debug, warn};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

/// Protocol identifiers for health tracking
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Protocol {
    Tor,
    I2p,
    Nym,
    Lokinet,
    Ipfs,
    ZeroNet,
    Freenet,
    GnuNet,
    RetroShare,
    Tribler,
}

/// Shared health state map
pub type HealthState = Arc<RwLock<HashMap<Protocol, bool>>>;

/// Creates a new health state with all protocols marked unhealthy initially
pub fn new_health_state() -> HealthState {
    let mut map = HashMap::new();
    for protocol in [
        Protocol::Tor,
        Protocol::I2p,
        Protocol::Nym,
        Protocol::Lokinet,
        Protocol::Ipfs,
        Protocol::ZeroNet,
        Protocol::Freenet,
        Protocol::GnuNet,
        Protocol::RetroShare,
        Protocol::Tribler,
    ] {
        map.insert(protocol, false);
    }
    Arc::new(RwLock::new(map))
}

/// Background health monitor that periodically checks adapter health
pub async fn run_health_monitor<T: ProtocolAdapter + Send + Sync + 'static>(
    protocol: Protocol,
    adapter: Arc<T>,
    state: HealthState,
    interval: Duration,
) {
    loop {
        let healthy = adapter.is_healthy().await;
        {
            let mut state_guard = state.write().await;
            let prev = state_guard.get(&protocol).copied().unwrap_or(false);
            if prev != healthy {
                if healthy {
                    debug!("{:?} is now healthy", protocol);
                } else {
                    warn!("{:?} is now unhealthy", protocol);
                }
            }
            state_guard.insert(protocol, healthy);
        }
        tokio::time::sleep(interval).await;
    }
}
