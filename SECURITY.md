# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 1.0.x   | :white_check_mark: |
| < 1.0   | :x:                |

## Reporting a Vulnerability

Please report vulnerabilities by emailing **security@chimera.network**.
We aim to acknowledge reports within 24 hours.

## CISA Alignment & Best Practices

Chimera aligns with **CISA's Secure Software Development Framework (SSDF)** and **Memory Safety Guidance**:

1.  **Memory Safety**: Written in **Rust**, eliminating entire classes of memory safety vulnerabilities (buffer overflows, use-after-free) present in C/C++.
2.  **Least Privilege**: Sidecars (Tor, I2P) run as unprivileged child processes.
3.  **Input Validation**: All configuration inputs and SOCKS5 requests are strictly validated.
4.  **Supply Chain Security**: We use `Cargo.lock` to pin dependencies and recommend verifying `sha256` sums of all sidecar binaries.

## Known Risks

- **Experimental Protocols**: Support for Lokinet, ZeroNet, etc., is experimental. Use standard Tor/I2P for critical anonymity.
- **Sidecar Management**: Chimera manages external processes. If a sidecar crashes, anonymity for that specific circuit may be lost until restart.
