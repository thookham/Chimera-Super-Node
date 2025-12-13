pub mod freenet;
pub mod gnunet;
pub mod i2p;
pub mod ipfs;
pub mod lokinet;
pub mod nym;
pub mod retroshare;
pub mod tor;
pub mod tribler;
pub mod zeronet;

use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait ProtocolAdapter {
    /// Start the protocol daemon/client
    async fn start(&self) -> Result<()>;

    /// Stop the protocol daemon/client
    async fn stop(&self) -> Result<()>;

    /// Get the SOCKS5 proxy address for this protocol
    fn get_proxy_addr(&self) -> String;

    /// Check if the protocol is healthy/connected
    async fn is_healthy(&self) -> bool;
}
