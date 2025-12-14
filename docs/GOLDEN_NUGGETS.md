# Protocol Golden Nuggets Reference

Research from 24 privacy protocols to inform Chimera GUI design.

---

## Per-Protocol Analysis

### 🧅 Tor (The Onion Router)

**Source:** [torproject.org](https://torproject.org)

- **Privacy-First UI**: Minimal data retention, no logging
- **New Identity Button**: Quickly change circuit and clear data
- **Tor Launcher**: Pre-connection bridge/proxy config for censored regions
- **Fingerprint Resistance**: All users appear identical
- **Pre-installed Extensions**: NoScript, HTTPS-Only mode

### 👁 I2P (Invisible Internet Project)

**Source:** [geti2p.net](https://geti2p.net)

- **Router Console**: Web-based management interface
- **Configurable Logging**: `/configlogging` for buffer size, date format
- **Tunnel Management**: Create and manage tunnels via UI
- **Bandwidth Charts**: Real-time network stats

### 🔗 Nym Mixnet

**Source:** [nymtech.net](https://nymtech.net)

- **SOCKS5 Client**: `localhost:1080` proxy interface
- **NymConnect GUI**: Click-to-connect for wallets/messaging
- **Sphinx Packets**: Traffic fragmented and mixed
- **Transitioning to NymVPN**: New all-traffic approach

### 🔒 Lokinet

**Source:** [lokinet.org](https://lokinet.org)

- **Connection Stats Tab**: Speed/quality charts
- **Refactored Log Subsystem**: Clean, readable logs in GUI
- **Exit Node Selection**: Choose .loki exit points
- **Onion Routing**: No single node knows full path

### 🌐 IPFS

**Source:** [ipfs.tech](https://ipfs.tech)

- **Desktop App**: Complete node + Web UI in one
- **Gateway Links**: `localhost:8080` for HTTP access
- **Shareable URLs**: Generate gateway links for non-IPFS users
- **File Management**: Familiar drag-drop interface

### ⚡ ZeroNet

**Source:** [zeronet.io](https://zeronet.io)

- **Offline Access**: View previously visited sites offline
- **Real-time Updates**: Changes propagate through P2P
- **Bitcoin Crypto Auth**: No passwords, wallet-based login
- **Namecoin Integration**: `.bit` domains
- **Tor Support**: Optional IP hiding

### 🕊️ Freenet (Hyphanet)

**Source:** [hyphanet.org](https://hyphanet.org)

- **FProxy Web Interface**: Browse freesites in browser
- **FCP API (Port 9481)**: External app integration
- **Security**: JS disabled, whitelisted HTML only
- **Key Types**: CHK, SSK, USK, KSK for content

### 🐃 GNUnet

**Source:** [gnunet.org](https://gnunet.org)

- **gnunet-gtk GUI**: Config and file sharing
- **GNS**: Decentralized DNS replacement
- **DHT-based Routing**: No central services
- **Anonymous File Sharing**: Built-in

### 🛡️ Psiphon

**Source:** [psiphon.ca](https://psiphon.ca)

- **One-Touch Connect**: Big connect button
- **Split Tunneling**: Choose which apps use VPN
- **Server Selection**: Filter by country
- **Stats Tab**: Connection logs and usage
- **No Registration**: Use immediately

### 🔮 Mysterium Network

**Source:** [mysterium.network](https://mysterium.network)

- **Node Filtering**: Speed, quality, price, location
- **Favorites System**: Quick reconnect to preferred nodes
- **Pay-as-You-Go**: Crypto payments, balance estimates
- **Kill Switch**: Auto-disconnect on VPN drop
- **Night Mode**: Dark theme toggle
- **Electron + React Native**: Cross-platform

### 🌳 Yggdrasil Mesh

**Source:** [yggdrasil-network.github.io](https://yggdrasil-network.github.io)

- **TUN Interface**: Layer 3 virtual network
- **IPv6 Addressing**: Unique `200:` addresses per node
- **Configurable MTU**: Performance tuning
- **Headless Mode**: Router-only operation
- **Firewall Recommended**: Services exposed to mesh

### 🔐 WireGuard

**Source:** [wireguard.com](https://wireguard.com)

- **Minimalist Design**: Fast, simple, lean
- **QR Code Import**: Easy mobile configuration
- **Toggle Activation**: Simple on/off
- **Handshake Status**: Confirm connection
- **Web Admin UIs**: Third-party management panels

---

## Meta-Analysis: Common Patterns

| Pattern | Protocols Using | Chimera Implementation |
|---------|-----------------|------------------------|
| **SOCKS5 Proxy** | Tor, I2P, Nym, Psiphon, V2Ray | Smart routing on port 9050 ✅ |
| **Connection Stats** | I2P, Lokinet, IPFS, Mysterium | Traffic panel in dashboard ✅ |
| **Kill Switch** | Mysterium, Psiphon | TODO: Implement |
| **Split Tunneling** | Psiphon, WireGuard | TODO: Per-app routing |
| **Offline Access** | ZeroNet, IPFS | TODO: Cache layer |
| **Dark Mode** | All modern apps | TailwindCSS dark theme ✅ |
| **QR Config** | WireGuard, Mysterium | TODO: Mobile sharing |
| **No Registration** | Tor, Psiphon, ZeroNet | No account required ✅ |
| **Real-time Logs** | I2P, Lokinet | TODO: Log viewer panel |
| **Node Selection** | Mysterium, Sentinel | TODO: Protocol picker |

---

## Key Takeaways for Chimera GUI

1. **Privacy First**: Minimal logging, clear data on exit
2. **One-Click Connect**: Big prominent Start button ✅
3. **Smart Routing**: Auto-detect `.onion`, `.i2p`, `.loki` ✅
4. **Real-time Stats**: Show Tx/Rx, connections, latency
5. **Configurable Logging**: Allow users to adjust verbosity
6. **Cross-Platform**: Electron/Tauri pattern works ✅
7. **Kill Switch**: Essential for VPN-like behavior
8. **Favorites/Presets**: Save protocol combinations
