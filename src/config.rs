use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: ServerSettings,
    pub tor: TorSettings,
    pub i2p: I2pSettings,
    pub lokinet: LokinetSettings,
    pub nym: NymSettings,
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

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            // Start with default values
            .set_default("server.host", "127.0.0.1")?
            .set_default("server.port", 9050)?
            .set_default("server.log_level", "info")?
            .set_default("tor.enabled", true)?
            .set_default("tor.binary_path", "bin/tor.exe")?
            .set_default("tor.socks_port", 9052)?
            .set_default("tor.control_port", 9051)?
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
            // Merge with config file
            .add_source(File::with_name("chimera").required(false))
            // Merge with environment variables (e.g. CHIMERA_SERVER_PORT=9090)
            .add_source(config::Environment::with_prefix("CHIMERA").separator("__"))
            .build()?;

        s.try_deserialize()
    }
}
