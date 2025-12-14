use chimera_node::config::Settings;
use chimera_node::process_manager::ProcessManager;
use chimera_node::socks5::Socks5Server;
use clap::Parser;
use log::{error, info};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to configuration file
    #[arg(short, long, default_value = "chimera.toml")]
    config: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Load Configuration
    let settings = match Settings::new() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to load configuration: {}", e);
            // Fallback to defaults if config fails? Or exit?
            // For now, let's exit to force proper config.
            return Err(anyhow::anyhow!("Configuration load error: {}", e));
        }
    };

    // 2. Initialize Logger
    if std::env::var("RUST_LOG").is_err() {
        unsafe {
            std::env::set_var("RUST_LOG", &settings.server.log_level);
        }
    }
    env_logger::init();

    info!("🦁 Chimera Super Node starting...");
    info!(
        "Listening on {}:{}",
        settings.server.host, settings.server.port
    );

    // 3. Start Sidecar Processes (Tor, I2P, Nym, Lokinet, IPFS, ZeroNet, Freenet)
    let pm = ProcessManager::new(
        settings.chain_mode.clone(),
        settings.tor.clone(),
        settings.i2p.clone(),
        settings.nym.clone(),
        settings.lokinet.clone(),
        settings.ipfs.clone(),
        settings.zeronet.clone(),
        settings.freenet.clone(),
        settings.retroshare.clone(),
        settings.gnunet.clone(),
        settings.tribler.clone(),
    );
    if let Err(e) = pm.start_processes().await {
        error!("Failed to start background processes: {}", e);
    }

    // 4. Start SOCKS5 Proxy
    let server = Socks5Server::new(
        settings.server.port,
        settings.tor.socks_port,
        settings.i2p.socks_port,
        settings.lokinet.socks_port,
        settings.nym.socks_port,
        settings.ipfs.gateway_port,
        settings.zeronet.port,
        settings.freenet.fproxy_port,
        settings.gnunet.socks_port,
        settings.retroshare.api_url,
        settings.tribler.api_url,
    );

    if let Err(e) = server.run().await {
        error!("SOCKS5 Server crashed: {}", e);
    }

    Ok(())
}
