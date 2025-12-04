# 🗺️ Chimera Roadmap

This document outlines the development plan for **Chimera**, the unified anonymity Super Node.

## 🏁 Phase 1: MVP (Current)
**Goal**: A functional CLI daemon that routes traffic to Tor and I2P.
- [ ] **Core**: Rust daemon initialization.
- [ ] **Proxy**: Basic SOCKS5 implementation.
- [ ] **Tor**: Integration via `arti` or C-Tor control port.
- [ ] **I2P**: Integration via `i2pd` (static lib or sidecar).
- [ ] **Routing**: Basic TLD-based routing (`.onion`, `.i2p`).

## 🚀 Phase 2: Expanded Protocols (v0.5)
**Goal**: Support for Lokinet and Nym.
- [ ] **Lokinet**: Integration via Tun/Tap or LLARP library.
- [ ] **Nym**: Integration via native Rust client.
- [ ] **Unified Config**: Single `chimera.toml` for all networks.

## 🕸️ Phase 2.5: Decentralized Web & Storage (v0.8)
**Goal**: Integrate content and social networks.
- [ ] **Freenet (Hyphanet)**:
    -   Integration via **FCPv2** (Freenet Client Protocol).
    -   Support for `USK@` (Updatable Subspace Keys) and `SSK@` (Signed Subspace Keys).
- [ ] **ZeroNet**:
    -   Integration via **ZeroFrame API** (JSON-based).
    -   Support for `.bit` domains and Zite routing.
    -   Explore `zeronet-rs` for native Rust integration.
- [ ] **RetroShare**:
    -   Integration via **JSON API** (OpenAPI spec).
    -   Support for Friend-to-Friend (F2F) routing and GXS (Generic eXchange System).
- [ ] **GNUnet**:
    -   Integration via C bindings or `gnunet-rs`.
    -   Support for GNS (GNU Name System) and CADET transport.
- [ ] **Tribler**:
    -   Integration via REST API.
    -   Support for anonymous BitTorrent streaming and searching.

## 🔗 Phase 3: The "Super" Protocol (v1.0)
**Goal**: Advanced features and protocol chaining.
- [ ] **Protocol Chaining**: Route Tor over Nym (`User -> Nym -> Tor -> Dest`).
- [ ] **Smart Fallback (Failover)**: If one network is down, automatically switch to another.
- [ ] **Bonded Mode**: Split traffic across multiple networks for speed/redundancy.
- [ ] **Round-Robin**: Rotate exit networks for each request to maximize anonymity set.
- [ ] **Identity Vault**: Encrypted storage for private keys across all networks.

## 🖥️ Phase 4: User Experience (v1.5)
**Goal**: GUI and ease of use.
- [ ] **GUI**: Cross-platform desktop app (Tauri or Iced).
- [ ] **Dashboard**: Real-time traffic stats and network status.
- [ ] **One-Click Install**: Bundled binaries for Windows, macOS, Linux.

## 📱 Phase 5: Mobile (v2.0)
**Goal**: Android and iOS support.
- [ ] **Android**: Port Chimera core to Android (JNI).
- [ ] **iOS**: Port to iOS (Swift/Rust bridge).
