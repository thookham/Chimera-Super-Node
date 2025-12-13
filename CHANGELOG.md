# 📜 Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
