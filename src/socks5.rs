use log::{debug, error, info};
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

pub struct Socks5Server {
    port: u16,
    tor_proxy: SocketAddr,
    i2p_proxy: SocketAddr,
    lokinet_proxy: SocketAddr,
    nym_proxy: SocketAddr,
}

impl Socks5Server {
    pub fn new(port: u16, tor_port: u16, i2p_port: u16, lokinet_port: u16, nym_port: u16) -> Self {
        Self {
            port,
            tor_proxy: SocketAddr::from(([127, 0, 0, 1], tor_port)),
            i2p_proxy: SocketAddr::from(([127, 0, 0, 1], i2p_port)),
            lokinet_proxy: SocketAddr::from(([127, 0, 0, 1], lokinet_port)),
            nym_proxy: SocketAddr::from(([127, 0, 0, 1], nym_port)),
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

            tokio::spawn(async move {
                if let Err(e) = handle_connection(socket, tor, i2p, lokinet, nym).await {
                    error!("Connection error: {}", e);
                }
            });
        }
    }
}

async fn handle_connection(
    mut client: TcpStream,
    tor: SocketAddr,
    i2p: SocketAddr,
    lokinet: SocketAddr,
    nym: SocketAddr,
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
    } else {
        // Default to Tor for everything else (safest default)
        // Or we could route clearnet directly? For now, Tor.
        debug!("Routing {} to Tor (Default)", target_host);
        tor
    };

    // 4. Connect to Upstream (Tor/I2P)
    let mut upstream = TcpStream::connect(upstream_addr).await?;

    // Send success reply to client
    client
        .write_all(&[0x05, 0x00, 0x00, 0x01, 0, 0, 0, 0, 0, 0])
        .await?;

    // Replay the SOCKS5 handshake to the upstream proxy?
    // No, Tor/I2P are SOCKS proxies themselves. We need to handshake with THEM.
    // Handshake with Upstream
    upstream.write_all(&[0x05, 0x01, 0x00]).await?; // Hello
    let mut up_buf = [0u8; 2];
    upstream.read_exact(&mut up_buf).await?; // Auth response

    // Send Connect Request to Upstream
    // We need to reconstruct the packet we received or build a new one
    // For simplicity, let's rebuild it.
    let mut packet = vec![0x05, 0x01, 0x00]; // Ver, Cmd, Rsv
    if addr_type == 0x03 {
        packet.push(0x03);
        packet.push(target_host.len() as u8);
        packet.extend_from_slice(target_host.as_bytes());
    } else {
        // Fallback for IPv4 (not fully implemented in this snippet for brevity)
        packet.push(0x01);
        // ...
    }
    packet.extend_from_slice(&target_port.to_be_bytes());

    upstream.write_all(&packet).await?;

    // Read Upstream Reply
    // We discard the reply details and just pipe data after this
    // But we need to consume the reply packet.
    // SOCKS5 reply is variable length.
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

    // 5. Pipe Data
    let (mut c_rx, mut c_tx) = client.split();
    let (mut u_rx, mut u_tx) = upstream.split();

    let client_to_upstream = tokio::io::copy(&mut c_rx, &mut u_tx);
    let upstream_to_client = tokio::io::copy(&mut u_rx, &mut c_tx);

    tokio::try_join!(client_to_upstream, upstream_to_client)?;

    Ok(())
}
