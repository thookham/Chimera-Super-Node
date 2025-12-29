# Chimera Super Node - System Architecture

## 1. High-Level Overview

**Chimera Super Node** is a unified anonymity orchestrator that allows users to route traffic through multiple privacy networks simultaneously. It encapsulates complex daemon management (Tor, I2P, Nym, etc.) behind a robust, user-friendly interface.

The system is built as a **hybrid desktop application**:
- **Frontend**: A modern, reactive GUI built with React 19, TypeScript, and TailwindCSS v4.
- **Backend**: A high-performance Rust core managed by Tauri, responsible for process supervision, health monitoring, and SOCKS5 proxying.

---

## 2. System Context

```mermaid
graph TD
    User((User))
    
    subgraph "Chimera Super Node"
        UI[React Frontend]
        Tauri[Tauri Core (Rust)]
        PM[Process Manager]
        Proxy[SOCKS5 Server]
    end
    
    subgraph "Privacy Networks (Sidecars)"
        Tor[Tor Daemon]
        I2P[I2PD]
        Nym[Nym Client]
        Lokinet
        IPFS
    end
    
    User -->|Interacts| UI
    UI <-->|Commands/Status| Tauri
    Tauri -->|Controls| PM
    PM -->|Spawns/Monitors| Tor
    PM -->|Spawns/Monitors| I2P
    PM -->|Spawns/Monitors| Nym
    
    User -->|Traffic| Proxy
    Proxy -->|Routes| Tor
    Proxy -->|Routes| I2P
```

---

## 3. Core Components

### 3.1 Frontend Layer (`gui/`)
The frontend is the visual control center. It uses a component-based architecture:
*   **HeroStatus**: Visual heartbeat and global system state.
*   **ProtocolGrid**: Interactive command matrix for toggling individual networks.
*   **ControlPanel**: Master switches for daemon initialization and termination.
*   **LogTerminal**: Real-time log streamer via Tauri events.

**State Management**:
*   `useChimera.ts`: A custom hook that bridges React state (loading, error, logs) with Tauri commands (`invoke`).

### 3.2 Backend Layer (`src-tauri/`)
This layer connects the GUI to the operating system.
*   **Commands**:
    *   `start_daemon(protocols)`: Receives user configuration, validates it, and triggers the `ProcessManager`.
    *   `stop_daemon()`: Gracefully halts all child processes.
    *   `get_status()`: Returns a map of protocol health (verified via `HealthMonitor`).

### 3.3 Logic Core (`src/`)
This is the "Brain" of the node, written in Rust.

#### **ProcessManager** (`process_manager.rs`)
The central supervisor. It:
1.  **Orchestrates Startup**: Respects dependencies (e.g., "Tor over Nym" chaining modes).
2.  **Manages Lifecycle**: Holds handles to child processes and ensures they are killed when the app exits.
3.  **Selective Startup**: Filters protocols based on the `enabled_protocols` Set provided by the user.

#### **Protocol Adapters** (`adapters/`)
An abstraction layer (`ProtocolAdapter` trait) that standardizes interaction with diverse binaries (Tor, I2PD, Nym, etc.).
*   **Responsibility**: Building command-line arguments, parsing config files, and detecting startup failure.
*   **Implementations**: `TorAdapter`, `I2pAdapter`, `NymAdapter`, etc.

#### **Health Monitor** (`health_monitor.rs`)
A background surveillance system suitable for a "Robust" application.
*   **Mechanism**: Spawns async tasks for every active protocol.
*   **Checks**: Periodically polls the adapter (checking PID existence or port connectivity).
*   **Reporting**: Updates a shared `RwLock<HashMap>` which the Frontend polls for real-time status.

---

## 4. Data Flow

### 4.1 Startup Sequence
1.  **User** selects protocols (e.g., Tor, Nym) in `ProtocolGrid`.
2.  **Frontend** calls `invoke('start_daemon', { protocols: ['tor', 'nym'] })`.
3.  **Tauri** creates a `ProcessManager` with the allowed list.
4.  **ProcessManager** iterates through adapters:
    *   IF protocol is in `enabled_protocols`:
        *   Launch binary.
        *   Spawn `run_health_monitor` task.
    *   ELSE: Skip.
5.  **Result** returned to UI ("Daemon started").

### 4.2 Health Polling
1.  **Background Tasks** (Rust) run every ~5 seconds.
    *   Probe: "Is Tor PID 1234 still alive?" -> Result: `true`.
    *   Write: Update `HealthState` map.
2.  **Frontend Hook** (React) runs every ~1-2 seconds.
    *   Call `get_status`.
    *   Receive JSON: `{ "daemon": true, "tor": true, "i2p": false }`.
    *   Update Red/Green shield icons.

---

## 5. Security & Isolation

*   **Sidecars**: All network daemons run as child processes. They are not embedded in the Chimera binary, ensuring modularity.
*   **Memory Safety**: Rust's ownership model guarantees thread safety when sharing status strings and health states across async boundaries.
