mod socks5;
mod process_manager;

use clap::Parser;
use log::{info, error};
use crate::socks5::Socks5Server;
use crate::process_manager::ProcessManager;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Port to listen on for SOCKS5 proxy
    #[arg(short, long, default_value_t = 9050)]
    port: u16,

    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logger
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    // Parse arguments
    let args = Args::parse();

    info!("🦁 Chimera Super Node starting...");
    
    // 1. Start Sidecar Processes (Tor, I2P)
    let pm = ProcessManager::new();
    if let Err(e) = pm.start_processes().await {
        error!("Failed to start background processes: {}", e);
    }

    // 2. Start SOCKS5 Proxy
    // We assume Tor is on 9052 (internal) and I2P is on 4447
    let server = Socks5Server::new(args.port, 9052, 4447);
    
    if let Err(e) = server.run().await {
        error!("SOCKS5 Server crashed: {}", e);
    }

    Ok(())
}

