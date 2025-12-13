use log::{debug, error, info};
use reqwest;
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

pub struct Socks5Server {
    port: u16,
    tor_proxy: SocketAddr,
    i2p_proxy: SocketAddr,
    lokinet_proxy: SocketAddr,
    nym_proxy: SocketAddr,
    ipfs_proxy: SocketAddr,
    zeronet_proxy: SocketAddr,
    freenet_proxy: SocketAddr,
    gnunet_proxy: SocketAddr,
    retroshare_proxy: SocketAddr, // Typically HTTP/API
    tribler_proxy: SocketAddr,    // Typically REST API
}

impl Socks5Server {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        port: u16,
        tor_port: u16,
        i2p_port: u16,
        lokinet_port: u16,
        nym_port: u16,
        ipfs_port: u16,
        zeronet_port: u16,
        freenet_port: u16,
        gnunet_port: u16,
        retroshare_url: String, // parse port from URL
        tribler_url: String,    // parse port from URL
    ) -> Self {
        // Simple helper to parse port from URL or default
        let get_port = |url: &str, def: u16| -> u16 {
            url.parse::<reqwest::Url>()
                .ok()
                .and_then(|u| u.port())
                .unwrap_or(def)
        };

        let rs_port = get_port(&retroshare_url, 9090);
        let tr_port = get_port(&tribler_url, 8085);

        Self {
            port,
            tor_proxy: SocketAddr::from(([127, 0, 0, 1], tor_port)),
            i2p_proxy: SocketAddr::from(([127, 0, 0, 1], i2p_port)),
            lokinet_proxy: SocketAddr::from(([127, 0, 0, 1], lokinet_port)),
            nym_proxy: SocketAddr::from(([127, 0, 0, 1], nym_port)),
            ipfs_proxy: SocketAddr::from(([127, 0, 0, 1], ipfs_port)),
            zeronet_proxy: SocketAddr::from(([127, 0, 0, 1], zeronet_port)),
            freenet_proxy: SocketAddr::from(([127, 0, 0, 1], freenet_port)),
            gnunet_proxy: SocketAddr::from(([127, 0, 0, 1], gnunet_port)),
            retroshare_proxy: SocketAddr::from(([127, 0, 0, 1], rs_port)),
            tribler_proxy: SocketAddr::from(([127, 0, 0, 1], tr_port)),
        }
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        let addr = SocketAddr::from(([127, 0, 0, 1], self.port));
        let listener = TcpListener::bind(addr).await?;
        info!("🧦 SOCKS5 Proxy listening on {}", addr);

        loop {
            let (socket, _) = listener.accept().await?;
            let tor = self.tor_proxy;
            let i2p = self.i2p_proxy;
            let lokinet = self.lokinet_proxy;
            let nym = self.nym_proxy;
            let ipfs = self.ipfs_proxy;
            let zeronet = self.zeronet_proxy;
            let freenet = self.freenet_proxy;
            let gnunet = self.gnunet_proxy;
            let retroshare = self.retroshare_proxy;
            let tribler = self.tribler_proxy;

            tokio::spawn(async move {
                if let Err(e) = handle_connection(
                    socket, tor, i2p, lokinet, nym, ipfs, zeronet, freenet, gnunet, retroshare,
                    tribler,
                )
                .await
                {
                    error!("Connection error: {}", e);
                }
            });
        }
    }
}

#[allow(clippy::too_many_arguments)]
async fn handle_connection(
    mut client: TcpStream,
    tor: SocketAddr,
    i2p: SocketAddr,
    lokinet: SocketAddr,
    nym: SocketAddr,
    ipfs: SocketAddr,
    zeronet: SocketAddr,
    freenet: SocketAddr,
    gnunet: SocketAddr,
    retroshare: SocketAddr,
    tribler: SocketAddr,
) -> anyhow::Result<()> {
    // 1. Handshake
    let mut buf = [0u8; 2];
    client.read_exact(&mut buf).await?;

    if buf[0] != 0x05 {
        return Err(anyhow::anyhow!("Invalid SOCKS version"));
    }

    let n_methods = buf[1] as usize;
    let mut methods = vec![0u8; n_methods];
    client.read_exact(&mut methods).await?;

    // We only support no-auth (0x00)
    client.write_all(&[0x05, 0x00]).await?;

    // 2. Request
    let mut head = [0u8; 4];
    client.read_exact(&mut head).await?;

    let cmd = head[1];
    if cmd != 0x01 {
        // CONNECT
        return Err(anyhow::anyhow!("Unsupported command"));
    }

    let addr_type = head[3];
    let (target_host, target_port) = match addr_type {
        0x01 => {
            // IPv4
            let mut ip = [0u8; 4];
            client.read_exact(&mut ip).await?;
            let mut port = [0u8; 2];
            client.read_exact(&mut port).await?;
            (
                format!("{}.{}.{}.{}", ip[0], ip[1], ip[2], ip[3]),
                u16::from_be_bytes(port),
            )
        }
        0x03 => {
            // Domain
            let len = client.read_u8().await? as usize;
            let mut domain = vec![0u8; len];
            client.read_exact(&mut domain).await?;
            let mut port = [0u8; 2];
            client.read_exact(&mut port).await?;
            (
                String::from_utf8_lossy(&domain).to_string(),
                u16::from_be_bytes(port),
            )
        }
        _ => return Err(anyhow::anyhow!("Unsupported address type")),
    };

    debug!("Request: {}:{}", target_host, target_port);

    // 3. Routing Logic
    let upstream_addr = if target_host.ends_with(".onion") {
        debug!("Routing {} to Tor", target_host);
        tor
    } else if target_host.ends_with(".i2p") {
        debug!("Routing {} to I2P", target_host);
        i2p
    } else if target_host.ends_with(".loki") {
        debug!("Routing {} to Lokinet", target_host);
        lokinet
    } else if target_host.ends_with(".nym") {
        debug!("Routing {} to Nym", target_host);
        nym
    } else if target_host.ends_with(".bit") {
        debug!("Routing {} to ZeroNet", target_host);
        zeronet
    } else if target_host.ends_with(".eth") || target_host.ends_with(".ipfs") {
        debug!("Routing {} to IPFS", target_host);
        ipfs
    } else if target_host.starts_with("USK@")
        || target_host.starts_with("SSK@")
        || target_host.ends_with(".freenet")
    {
        debug!("Routing {} to Freenet", target_host);
        freenet
    } else if target_host.ends_with(".gnu") || target_host.ends_with(".zkey") {
        debug!("Routing {} to GNUnet", target_host);
        gnunet
    } else if target_host.contains("retroshare") {
        // Simple heuristic
        debug!("Routing {} to RetroShare", target_host);
        retroshare
    } else if target_host.contains("tribler") {
        // Simple heuristic
        debug!("Routing {} to Tribler", target_host);
        tribler
    } else {
        debug!("Routing {} to Tor (Default)", target_host);
        tor
    };

    // 4. Connect to Upstream
    let mut upstream = TcpStream::connect(upstream_addr).await?;

    // Send success reply to client
    client
        .write_all(&[0x05, 0x00, 0x00, 0x01, 0, 0, 0, 0, 0, 0])
        .await?;

    let is_socks_upstream = target_host.ends_with(".onion")
        || target_host.ends_with(".i2p")
        || target_host.ends_with(".loki")
        || target_host.ends_with(".nym")
        || target_host.ends_with(".gnu"); // GNUnet GNS proxy is SOCKS

    if is_socks_upstream {
        // Handshake with SOCKS5 Upstream
        upstream.write_all(&[0x05, 0x01, 0x00]).await?;
        let mut up_buf = [0u8; 2];
        upstream.read_exact(&mut up_buf).await?;

        // Send Connect Request to Upstream
        let mut packet = vec![0x05, 0x01, 0x00];
        if addr_type == 0x03 {
            packet.push(0x03);
            packet.push(target_host.len() as u8);
            packet.extend_from_slice(target_host.as_bytes());
        } else {
            packet.push(0x01);
            // Re-using dummy logic from before (assumes we have IPv4 in target_host if addr_type=1 which requires string parsing if we didn't save bytes)
            // Ideally we pass bytes.
            packet.push(0x03);
            packet.push(target_host.len() as u8);
            packet.extend_from_slice(target_host.as_bytes());
        }
        packet.extend_from_slice(&target_port.to_be_bytes());

        upstream.write_all(&packet).await?;

        // Read Upstream Reply
        let mut rep_head = [0u8; 4];
        upstream.read_exact(&mut rep_head).await?;
        let up_addr_type = rep_head[3];
        match up_addr_type {
            0x01 => {
                let mut buf = [0u8; 6];
                upstream.read_exact(&mut buf).await?;
            }
            0x03 => {
                let len = upstream.read_u8().await?;
                let mut buf = vec![0u8; len as usize + 2];
                upstream.read_exact(&mut buf).await?;
            }
            _ => {}
        }
    } else {
        debug!("Connected to HTTP/API upstream: {}", upstream_addr);
    }

    // 5. Pipe Data
    let (mut c_rx, mut c_tx) = client.split();
    let (mut u_rx, mut u_tx) = upstream.split();

    let client_to_upstream = tokio::io::copy(&mut c_rx, &mut u_tx);
    let upstream_to_client = tokio::io::copy(&mut u_rx, &mut c_tx);

    tokio::try_join!(client_to_upstream, upstream_to_client)?;

    Ok(())
}
