# 📜 Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2025-12-14

### Added

- **Phase 5: User Experience (GUI)**
  - **Tauri 2.0 Desktop App**: Cross-platform GUI using React + TypeScript + TailwindCSS.
  - **24 Protocol Adapters**: Full support for Tor, I2P, Nym, Lokinet, IPFS, ZeroNet, Freenet, GNUnet, RetroShare, Tribler, Yggdrasil, CJDNS, Psiphon, Lantern, Outline, Snowflake, Obfs4, Meek, Mysterium, Sentinel, Trojan, V2Ray, WireGuard, Session.
  - **Smart Routing Proxy**: Auto-detects `.onion`, `.i2p`, `.loki`, `.nym`, `.bit`, `.eth`, `.ipfs`, `USK@`, `.gnu`, `retroshare://`, `tribler://` and routes to correct upstream.
  - **Log Viewer**: Real-time log viewer with auto-refresh and color-coded log levels.
  - **Settings Modal**: Configure proxy port and enable/disable protocols with sticky persistence.
  - **Golden Nuggets Research Doc**: UI/UX patterns from 12+ protocol applications.
- **Cross-Platform Builds**: NSIS (Windows), DMG (macOS), AppImage/deb (Linux).
- **CSS Animations**: Glassmorphism, card hover effects, pulse/blink status indicators, loading spinner.
- **System Tray**: Minimize to tray support.

### Changed

- Upgraded project to Tauri 2.0 architecture.
- README updated with documentation section linking to Golden Nuggets.

## [1.0.0] - 2025-12-14

### Added

- **Phase 4: Super Protocol**
  - **Protocol Chaining**: Route Tor over Nym (or vice versa) via `chain_mode` config and `--Socks5Proxy` flag.
  - **Health Monitor**: Background task tracking adapter health for failover support.
  - **ChainMode Enum**: `none`, `tor_over_nym`, `nym_over_tor`.
- **Configuration**:
  - Added `chain_mode` global setting.
  - Added `upstream_proxy` and `fallback_protocol` to `TorSettings`.

### Changed

- **ProcessManager**: Chain-aware startup order (Nym before Tor when chaining).

## [0.9.0] - 2025-12-14

### Changed

- **Architecture**: Refactored `chimera_node` to split core logic into a library (`lib.rs`) and binary (`main.rs`) for better testability.
- **SOCKS5 Server**: Derives `Clone`/`Copy` and supports dependency injection for testing.
- **Routing Logic**: Extracted to pure function `resolve_upstream` for unit testing.

### Added

- **Test Suite**:
  - **Unit Tests**: Full coverage of domain routing logic.
  - **Integration Tests**: End-to-end verification of SOCKS5 handshake and data transfer using Mock Upstreams.

## [0.8.0] - 2025-12-13

### Added

- **Decentralized Web Support (Phase 2.5)**
  - **IPFS**: Added `ipfs` adapter with Kubo RPC integration (via HTTP).
  - **ZeroNet**: Added `zeronet` adapter and `.bit` TLD routing.
  - **Freenet/Hyphanet**: Added `freenet` adapter with FCP support and `.freenet`/`USK@`/`SSK@` routing.
- **Configuration**:
  - Added `[ipfs]`, `[zeronet]`, and `[freenet]` sections to `chimera.toml`.
  - Updated `config.rs` with new settings structs.
- **Dependencies**: Added `reqwest`, `serde_json`, and `url` to `Cargo.toml`.

## [0.8.1] - 2025-12-13

### Added

- **Extended Decentralized Protocols (Deferred Items)**
  - **RetroShare**: Added `retroshare` adapter (JSON API) and routing.
  - **GNUnet**: Added `gnunet` adapter (GNS Proxy) and `.gnu`/`.zkey` routing.
  - **Tribler**: Added `tribler` adapter (REST API) and routing.
- **Configuration**:
  - Added `[retroshare]`, `[gnunet]`, and `[tribler]` sections to `chimera.toml`.

## [0.5.0] - 2025-12-12

### Added

- **Lokinet SOCKS5 Support**: Added `socks_port` configuration for Lokinet adapter.
- **Nym Configuration**: Documented `upstream_provider` requirement in example config.
- **Experimental Warning**: Lokinet adapter now logs experimental status on startup.

### Changed

- Lokinet `get_proxy_addr()` now returns actual SOCKS5 address instead of empty string.
- Updated `chimera.example.toml` with comprehensive configuration documentation.

## [0.3.0] - 2025-12-05

### Added

- **Native Nym Integration**: Replaced `nym-socks5-client` binary dependency with embedded `nym-sdk` (Rust).
- **Lokinet Config**: Automatic generation of `lokinet.ini` for isolated router processes.
- **Configuration**: Added `upstream_provider` to `[nym]` settings (Required for SOCKS5).

## [0.2.0] - 2025-12-05

### Added

- Initial project structure (`TorI2P_SuperNode`).
- Submodules for Tor, I2PD, Lokinet, and Nym.
- Rust project initialization (Pending).
- Documentation: README, ROADMAP, CONTRIBUTING, CODE_OF_CONDUCT.
- Core SOCKS5 server implementation.
- Basic routing logic for `.onion` (Tor) and `.i2p` (I2PD).
- Routing logical for `.loki` (Lokinet) and `.nym` (Nym).
- Configuration system with `chimera.toml`.
