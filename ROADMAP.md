# 🗺️ Chimera Roadmap

This document outlines the development plan for **Chimera**, the unified anonymity Super Node.

## 🏁 Phase 1: MVP (Current)

**Goal**: A functional CLI daemon that routes traffic to Tor and I2P.

- [x] **Core**: Rust daemon initialization.
- [x] **Proxy**: Basic SOCKS5 implementation.
- [x] **Tor**: Integration via `arti` or C-Tor control port.
- [x] **I2P**: Integration via `i2pd` (static lib or sidecar).
- [x] **Routing**: Basic TLD-based routing (`.onion`, `.i2p`).

## 🚀 Phase 2: Expanded Protocols (v0.5)

**Goal**: Support for Lokinet and Nym.

- [x] **Lokinet**: Integration via Tun/Tap or LLARP library. *(Experimental - SOCKS5 port configured)*
- [x] **Nym**: Integration via native Rust client. *(SDK integrated, requires upstream_provider)*
- [x] **Unified Config**: Single `chimera.toml` for all networks.

## 🕸️ Phase 2.5: Decentralized Web & Storage (v0.8) - **Completed**

**Goal**: Integrate content and social networks.

- [x] **IPFS Integration**:
  - [x] Adapter to talk to Kubo (local IPFS node)
  - [x] Routing for `ipfs://` and `ipns://` (via TLD heuristics)
- [x] **ZeroNet Integration**:
  - [x] Adapter for ZeroNet daemon
  - [x] `.bit` TLD routing
- [x] **Freenet/Hyphanet Integration**:
  - [x] Freenet Client Protocol (FCP) adapter
  - [x] Routing for `USK@`, `SSK@`, `.freenet`
- [ ] **Deferred (Future)**:
  - [ ] RetroShare (JSON API)
  - [ ] GNUnet (C bindings)
  - [ ] Tribler (REST API)

## 🧪 Phase 3: Rigorous Testing & Debugging (v0.9) - **Completed**

**Goal**: Ensure stability and correctness via comprehensive test suite.

- [x] **Unit Testing**: Coverage for routing logic and config parsing.
- [x] **Integration Testing**: Mocked upstream verification for full SOCKS5 flow.
- [x] **Architecture**: Split into Library/Binary structure for better testability.

## 🔗 Phase 4: The "Super" Protocol (v1.0) - **Completed**

**Goal**: Advanced features and protocol chaining.

- [x] **Protocol Chaining**: Route Tor over Nym (`User -> Nym -> Tor -> Dest`).
- [x] **Health Monitor**: Background task for failover support.
- [ ] **Smart Fallback (Failover)**: If one network is down, automatically switch to another.
- [ ] **Bonded Mode**: Split traffic across multiple networks for speed/redundancy.
- [ ] **Round-Robin**: Rotate exit networks for each request to maximize anonymity set.
- [ ] **Identity Vault**: Encrypted storage for private keys across all networks.
- [ ] **Identity Vault**: Encrypted storage for private keys across all networks.

## 🖥️ Phase 5: User Experience (v1.5)

**Goal**: GUI and ease of use.

- [ ] **GUI**: Cross-platform desktop app (Tauri or Iced).
- [ ] **Dashboard**: Real-time traffic stats and network status.
- [ ] **One-Click Install**: Bundled binaries for Windows, macOS, Linux.

## 📱 Phase 6: Mobile (v2.0)

**Goal**: Android and iOS support.

- [ ] **Android**: Port Chimera core to Android (JNI).
- [ ] **iOS**: Port to iOS (Swift/Rust bridge).
