use config::{Config, ConfigError, File};
use serde::Deserialize;

/// Protocol chaining mode for multi-hop routing
#[derive(Debug, Deserialize, Clone, Default, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ChainMode {
    #[default]
    None,
    TorOverNym,
    NymOverTor,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: ServerSettings,
    pub chain_mode: ChainMode,
    pub tor: TorSettings,
    pub i2p: I2pSettings,
    pub lokinet: LokinetSettings,
    pub nym: NymSettings,
    pub ipfs: IpfsSettings,
    pub zeronet: ZeroNetSettings,
    pub freenet: FreenetSettings,
    pub retroshare: RetroShareSettings,
    pub gnunet: GnunetSettings,
    pub tribler: TriblerSettings,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
    pub log_level: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TorSettings {
    pub enabled: bool,
    pub binary_path: String,
    pub socks_port: u16,
    pub control_port: u16,
    /// Optional upstream SOCKS5 proxy (for chaining, e.g., Nym)
    pub upstream_proxy: Option<String>,
    /// Fallback protocol if Tor is unhealthy
    pub fallback_protocol: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct I2pSettings {
    pub enabled: bool,
    pub binary_path: String,
    pub socks_port: u16,
    pub http_proxy_port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LokinetSettings {
    pub enabled: bool,
    pub binary_path: String,
    pub dns_port: u16,
    pub socks_port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NymSettings {
    pub enabled: bool,
    pub binary_path: String,
    pub socks_port: u16,
    pub upstream_provider: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct IpfsSettings {
    pub enabled: bool,
    pub api_url: String,
    pub gateway_port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ZeroNetSettings {
    pub enabled: bool,
    pub proxy_url: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct FreenetSettings {
    pub enabled: bool,
    pub host: String,
    pub fcp_port: u16,
    pub fproxy_port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RetroShareSettings {
    pub enabled: bool,
    pub api_url: String,
    pub user: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GnunetSettings {
    pub enabled: bool,
    pub socks_port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TriblerSettings {
    pub enabled: bool,
    pub api_url: String,
    pub api_key: Option<String>,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            // Start with default values
            .set_default("server.host", "127.0.0.1")?
            .set_default("server.port", 9050)?
            .set_default("server.log_level", "info")?
            // Phase 4: Protocol Chaining
            .set_default("chain_mode", "none")?
            .set_default("tor.enabled", true)?
            .set_default("tor.binary_path", "bin/tor.exe")?
            .set_default("tor.socks_port", 9052)?
            .set_default("tor.control_port", 9051)?
            .set_default("tor.upstream_proxy", None::<String>)?
            .set_default("tor.fallback_protocol", None::<String>)?
            .set_default("i2p.enabled", true)?
            .set_default("i2p.binary_path", "bin/i2pd.exe")?
            .set_default("i2p.socks_port", 4447)?
            .set_default("i2p.http_proxy_port", 4444)?
            .set_default("lokinet.enabled", false)?
            .set_default("lokinet.binary_path", "bin/lokinet.exe")?
            .set_default("lokinet.dns_port", 1053)?
            .set_default("lokinet.socks_port", 1090)?
            .set_default("nym.enabled", false)?
            .set_default("nym.binary_path", "bin/nym-socks5-client.exe")?
            .set_default("nym.socks_port", 1080)?
            .set_default("nym.upstream_provider", None::<String>)?
            // Phase 2.5: Decentralized Web
            .set_default("ipfs.enabled", false)?
            .set_default("ipfs.api_url", "http://127.0.0.1:5001")?
            .set_default("ipfs.gateway_port", 8080)?
            .set_default("zeronet.enabled", false)?
            .set_default("zeronet.proxy_url", "http://127.0.0.1:43110")?
            .set_default("zeronet.port", 43110)?
            .set_default("freenet.enabled", false)?
            .set_default("freenet.host", "127.0.0.1")?
            .set_default("freenet.fcp_port", 9481)?
            .set_default("freenet.fproxy_port", 8888)?
            // Extended Deferred Protocols
            .set_default("retroshare.enabled", false)?
            .set_default("retroshare.api_url", "http://127.0.0.1:9090")?
            .set_default("retroshare.user", None::<String>)?
            .set_default("retroshare.password", None::<String>)?
            .set_default("gnunet.enabled", false)?
            .set_default("gnunet.socks_port", 2080)? // Placeholder default
            .set_default("tribler.enabled", false)?
            .set_default("tribler.api_url", "http://127.0.0.1:8085")?
            .set_default("tribler.api_key", None::<String>)?
            // Merge with config file
            .add_source(File::with_name("chimera").required(false))
            // Merge with environment variables (e.g. CHIMERA_SERVER_PORT=9090)
            .add_source(config::Environment::with_prefix("CHIMERA").separator("__"))
            .build()?;

        s.try_deserialize()
    }
}
