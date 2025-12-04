use clap::Parser;
use log::{info, debug};

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
    env_logger::init();

    // Parse arguments
    let args = Args::parse();

    if args.verbose {
        std::env::set_var("RUST_LOG", "debug");
        env_logger::init(); // Re-init might not work as expected, usually set env var before init. 
                            // For MVP, we'll rely on user setting RUST_LOG env var or improve this later.
    }

    info!("🦁 Chimera Super Node starting...");
    info!("Listening on 127.0.0.1:{}", args.port);

    // TODO: Initialize Protocol Adapters (Tor, I2PD, Lokinet, Nym)
    
    // TODO: Start SOCKS5 Listener
    
    // Keep the main thread running
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}
