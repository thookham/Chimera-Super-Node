# 📜 Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
