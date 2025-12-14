use chimera_node::socks5::Socks5Server;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::test]
async fn test_socks5_integration() {
    // 1. Setup Mock Upstream (Fake Tor)
    let mock_tor = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let tor_addr = mock_tor.local_addr().unwrap();
    let tor_port = tor_addr.port();

    // Spawn Mock Tor Handler
    tokio::spawn(async move {
        let (mut socket, _) = mock_tor.accept().await.unwrap();
        // Handshake (if expected)
        // Chimera sends 0x05 0x01 0x00 to upstream if it thinks it's socks
        // OR just raw data if HTTP.
        // Let's assume we route .onion which triggers SOCKS handshake in `socks5.rs`.

        // Read SOCKS5 Hello
        let mut buf = [0u8; 3];
        socket.read_exact(&mut buf).await.unwrap();
        assert_eq!(buf, [0x05, 0x01, 0x00]);
        // Send Auth Response
        socket.write_all(&[0x05, 0x00]).await.unwrap();

        // Read Connect Request
        let mut head = [0u8; 4];
        socket.read_exact(&mut head).await.unwrap();
        let atyp = head[3];

        match atyp {
            0x01 => {
                // IPv4
                let mut tmp = [0u8; 4 + 2]; // IP + Port
                socket.read_exact(&mut tmp).await.unwrap();
            }
            0x03 => {
                // Domain
                let len = socket.read_u8().await.unwrap();
                let mut tmp = vec![0u8; len as usize + 2]; // Domain + Port
                socket.read_exact(&mut tmp).await.unwrap();
            }
            _ => panic!("Unsupported ATYP in mock"),
        }

        // Just write success rest ... (simplified)

        // Just write success
        socket
            .write_all(&[0x05, 0x00, 0x00, 0x01, 0, 0, 0, 0, 0, 0])
            .await
            .unwrap();

        // Echo loop
        let (mut rd, mut wr) = socket.split();
        tokio::io::copy(&mut rd, &mut wr).await.unwrap();
    });

    // 2. Setup SOCKS5 Server
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let proxy_addr = listener.local_addr().unwrap();

    let server = Socks5Server::new(
        proxy_addr.port(),
        tor_port, // Mock Tor
        0,
        0,
        0,
        0,
        0,
        0,
        0, // Unused ports
        "http://unused".to_string(),
        "http://unused".to_string(),
    );

    tokio::spawn(async move {
        server.serve(listener).await.unwrap();
    });

    // 3. Client Test
    let mut client = TcpStream::connect(proxy_addr).await.unwrap();

    // Handshake
    client.write_all(&[0x05, 0x01, 0x00]).await.unwrap();
    let mut buf = [0u8; 2];
    client.read_exact(&mut buf).await.unwrap();
    assert_eq!(buf, [0x05, 0x00]);

    // Request (Connect to .onion)
    let target = "test.onion";
    let mut pkt = vec![0x05, 0x01, 0x00, 0x03];
    pkt.push(target.len() as u8);
    pkt.extend_from_slice(target.as_bytes());
    pkt.extend_from_slice(&80u16.to_be_bytes());
    client.write_all(&pkt).await.unwrap();

    // Expect Success
    let mut head = [0u8; 4];
    client.read_exact(&mut head).await.unwrap();
    assert_eq!(head[1], 0x00); // Success
                               // Consume remaining 6 bytes (IPv4 + Port)
    let mut tmp = [0u8; 6];
    client.read_exact(&mut tmp).await.unwrap();

    // Data Transfer (Echo)
    client.write_all(b"Hello Chimera").await.unwrap();
    let mut resp = [0u8; 13];
    client.read_exact(&mut resp).await.unwrap();
    assert_eq!(&resp, b"Hello Chimera");
}
