# Chimera Super Node - User Guide

## Introduction

Chimera Super Node is a powerful dashboard for managing multiple anonymity networks (Tor, I2P, Nym, etc.) from a single interface. It allows you to "mix and match" privacy layers to suit your operational security needs.

---

## 2. Interface Overview

### 2.1 Hero Status
The top section of the dashboard shows the global state of the Super Node.
*   **DAEMON ACTIVE (Green)**: The backend Supervisor is running.
*   **DAEMON DISCONNECTED (Red)**: The backend is stopped or unreachable.
*   **Heartbeat**: A pulsing animation indicates the system is alive and processing health checks.

### 2.2 Protocol Grid
This is where you configure your network stack.
*   **Selection**: Click on any card (Tor, I2P, Nym) to toggle it **ENABLED** or **DISABLED**.
*   **Selection State**: A glowing border indicates the protocol is selected for the next startup.
*   **Active State**: A green dot and "ONLINE" text indicate the protocol is currently running and healthy.
*   **Health Failure**: If a protocol crashes, its indicator will turn Red/OFF automatically.

### 2.3 Control Panel
*   **INITIALIZE**: Starts the daemon with the currently selected protocols.
*   **TERMINATE**: Stops all running processes and the SOCKS5 proxy.

---

## 3. Getting Started

1.  **Select Protocols**: Click the cards for the networks you want to use (e.g., Click 'Tor' and 'Nym').
2.  **Launch**: Click the **INITIALIZE** button.
3.  **Wait for Green**: Watch the `HeroStatus` turn green and the individual protocol cards light up.
4.  **Connect**: Configure your browser or application to use the SOCKS5 proxy at `127.0.0.1:9050` (or your configured port).

---

## 4. Troubleshooting

### "Daemon Failed to Start"
*   Check the **Live Logs** terminal at the bottom of the screen.
*   Common issue: Port conflict. Ensure no other Tor/I2P instances are running in the background.

### "Protocol Unhealthy"
*   If a shield turns red, the process may have crashed.
*   Click **TERMINATE** and then **INITIALIZE** to restart the stack.
