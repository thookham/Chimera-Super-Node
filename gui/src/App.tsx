import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./index.css";

// All 24 protocols with default enabled state
const defaultProtocols = [
  // Core anonymity networks - enabled by default
  { name: "Tor", icon: "🧅", port: 9052, defaultEnabled: true },
  { name: "I2P", icon: "👁", port: 4447, defaultEnabled: true },
  { name: "Nym", icon: "🔗", port: 1080, defaultEnabled: true },
  { name: "Lokinet", icon: "🔒", port: 1090, defaultEnabled: true },
  // Decentralized web
  { name: "IPFS", icon: "🌐", port: 8080, defaultEnabled: true },
  { name: "ZeroNet", icon: "⚡", port: 43110, defaultEnabled: true },
  { name: "Freenet", icon: "🕊️", port: 8888, defaultEnabled: true },
  { name: "GNUnet", icon: "🐃", port: 2086, defaultEnabled: true },
  // P2P networks
  { name: "RetroShare", icon: "💬", port: 0, defaultEnabled: true },
  { name: "Tribler", icon: "📺", port: 0, defaultEnabled: true },
  // Mesh networks
  { name: "Yggdrasil", icon: "🌳", port: 0, defaultEnabled: true },
  { name: "CJDNS", icon: "🕸️", port: 0, defaultEnabled: true },
  // Censorship bypass
  { name: "Psiphon", icon: "🛡️", port: 1081, defaultEnabled: true },
  { name: "Lantern", icon: "🏮", port: 1082, defaultEnabled: true },
  { name: "Outline", icon: "📦", port: 1083, defaultEnabled: true },
  // Tor transports
  { name: "Snowflake", icon: "❄️", port: 0, defaultEnabled: true },
  { name: "Obfs4", icon: "🎭", port: 0, defaultEnabled: true },
  { name: "Meek", icon: "🪞", port: 0, defaultEnabled: true },
  // Decentralized VPN
  { name: "Mysterium", icon: "🔮", port: 1084, defaultEnabled: true },
  { name: "Sentinel", icon: "🛰️", port: 1085, defaultEnabled: true },
  // Anti-censorship
  { name: "Trojan", icon: "🐴", port: 1086, defaultEnabled: true },
  { name: "V2Ray", icon: "🚀", port: 1087, defaultEnabled: true },
  // VPN & Messaging
  { name: "WireGuard", icon: "🔐", port: 0, defaultEnabled: true },
  { name: "Session", icon: "💬", port: 0, defaultEnabled: true },
];

// Load settings from localStorage
const loadSettings = (): Record<string, boolean> => {
  try {
    const saved = localStorage.getItem("chimera_protocol_settings");
    if (saved) return JSON.parse(saved);
  } catch { }
  // Default: all enabled
  return Object.fromEntries(defaultProtocols.map(p => [p.name, p.defaultEnabled]));
};

// Save settings to localStorage
const saveSettings = (settings: Record<string, boolean>) => {
  localStorage.setItem("chimera_protocol_settings", JSON.stringify(settings));
};

interface ProtocolCardProps {
  name: string;
  icon: string;
  port: number;
  enabled: boolean;
  running: boolean;
}

function ProtocolCard({ name, icon, port, enabled, running }: ProtocolCardProps) {
  const status = !enabled ? "disabled" : running ? "online" : "offline";

  return (
    <div className={`glass-card p-4 flex flex-col items-center gap-2 min-w-[120px] ${!enabled ? 'opacity-50' : ''}`}>
      <span className="text-3xl">{icon}</span>
      <span className="font-semibold text-sm">{name}</span>
      <div className="flex items-center gap-1">
        <div className={`status-dot ${status}`}></div>
        <span className="text-xs text-gray-400 uppercase">{status}</span>
      </div>
      {port > 0 && <span className="text-xs text-gray-500">:{port}</span>}
    </div>
  );
}

interface SettingsModalProps {
  settings: Record<string, boolean>;
  proxyPort: number;
  onSave: (settings: Record<string, boolean>, proxyPort: number) => void;
  onClose: () => void;
}

function SettingsModal({ settings, proxyPort, onSave, onClose }: SettingsModalProps) {
  const [localSettings, setLocalSettings] = useState({ ...settings });
  const [localProxyPort, setLocalProxyPort] = useState(proxyPort);
  const [chainMode, setChainMode] = useState("none");

  const toggleProtocol = (name: string) => {
    setLocalSettings(prev => ({ ...prev, [name]: !prev[name] }));
  };

  const enableAll = () => {
    setLocalSettings(Object.fromEntries(defaultProtocols.map(p => [p.name, true])));
  };

  const disableAll = () => {
    setLocalSettings(Object.fromEntries(defaultProtocols.map(p => [p.name, false])));
  };

  const handleSave = () => {
    onSave(localSettings, localProxyPort);
    onClose();
  };

  return (
    <div className="fixed inset-0 bg-black/60 flex items-center justify-center z-50" onClick={onClose}>
      <div className="glass-card p-6 w-[600px] max-h-[85vh] overflow-auto" onClick={(e) => e.stopPropagation()}>
        <div className="flex justify-between items-center mb-4">
          <h2 className="text-xl font-bold">⚙️ Settings</h2>
          <button onClick={onClose} className="text-gray-400 hover:text-white text-2xl">&times;</button>
        </div>

        {/* Proxy Configuration */}
        <div className="mb-4 p-3 bg-gray-800/50 rounded-lg">
          <label className="block text-sm font-medium text-gray-300 mb-2">🌐 SOCKS5 Proxy Port</label>
          <div className="flex gap-2 items-center">
            <input
              type="number"
              value={localProxyPort}
              onChange={(e) => setLocalProxyPort(parseInt(e.target.value) || 9050)}
              min={1024}
              max={65535}
              className="w-24 bg-gray-800 border border-gray-600 rounded-lg p-2 text-white"
            />
            <span className="text-sm text-gray-400">Configure browser to: 127.0.0.1:{localProxyPort}</span>
          </div>
        </div>

        {/* Chain Mode */}
        <div className="mb-4">
          <label className="block text-sm font-medium text-gray-300 mb-2">Protocol Chain Mode</label>
          <select
            value={chainMode}
            onChange={(e) => setChainMode(e.target.value)}
            aria-label="Protocol Chain Mode"
            className="w-full bg-gray-800 border border-gray-600 rounded-lg p-2 text-white"
          >
            <option value="none">None (Direct)</option>
            <option value="tor_over_nym">Tor over Nym</option>
            <option value="nym_over_tor">Nym over Tor</option>
          </select>
        </div>

        {/* Quick Actions */}
        <div className="flex gap-2 mb-4">
          <button onClick={enableAll} className="btn-secondary text-sm">Enable All</button>
          <button onClick={disableAll} className="btn-secondary text-sm">Disable All</button>
        </div>

        {/* Protocol Toggles */}
        <div className="mb-4">
          <label className="block text-sm font-medium text-gray-300 mb-2">Enabled Protocols (Sticky)</label>
          <div className="grid grid-cols-3 gap-2 max-h-[300px] overflow-auto">
            {defaultProtocols.map(p => (
              <label key={p.name} className="flex items-center gap-2 cursor-pointer p-2 rounded hover:bg-white/5">
                <input
                  type="checkbox"
                  checked={localSettings[p.name] ?? true}
                  onChange={() => toggleProtocol(p.name)}
                  className="w-4 h-4"
                />
                <span className="text-sm">{p.icon} {p.name}</span>
              </label>
            ))}
          </div>
        </div>

        {/* Save/Cancel */}
        <div className="flex justify-end gap-2">
          <button onClick={onClose} className="btn-secondary">Cancel</button>
          <button onClick={handleSave} className="btn-primary">Save</button>
        </div>
      </div>
    </div>
  );
}

function App() {
  const [protocolSettings, setProtocolSettings] = useState<Record<string, boolean>>(loadSettings);
  const [proxyPort, setProxyPort] = useState(9050);
  const [daemonRunning, setDaemonRunning] = useState(false);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [showSettings, setShowSettings] = useState(false);

  // Auto-start on mount (all enabled protocols attempt to start)
  useEffect(() => {
    const autoStart = async () => {
      try {
        await invoke("start_daemon");
        setDaemonRunning(true);
      } catch (e) {
        // Silent fail for auto-start
        console.log("Auto-start skipped:", e);
      }
    };
    autoStart();
  }, []);

  const handleSaveSettings = (newSettings: Record<string, boolean>, newPort: number) => {
    setProtocolSettings(newSettings);
    setProxyPort(newPort);
    saveSettings(newSettings);
    localStorage.setItem("chimera_proxy_port", String(newPort));
  };

  const toggleDaemon = async () => {
    setLoading(true);
    setError(null);
    try {
      if (daemonRunning) {
        await invoke("stop_daemon");
        setDaemonRunning(false);
      } else {
        await invoke("start_daemon");
        setDaemonRunning(true);
      }
    } catch (e) {
      setError(String(e));
    }
    setLoading(false);
  };

  const enabledCount = Object.values(protocolSettings).filter(Boolean).length;

  return (
    <div className="min-h-screen p-6">
      {showSettings && (
        <SettingsModal
          settings={protocolSettings}
          proxyPort={proxyPort}
          onSave={handleSaveSettings}
          onClose={() => setShowSettings(false)}
        />
      )}

      {/* Header */}
      <header className="flex items-center justify-between mb-6">
        <div className="flex items-center gap-3">
          <span className="text-4xl">🦁</span>
          <div>
            <h1 className="text-2xl font-bold bg-gradient-to-r from-indigo-400 to-purple-400 bg-clip-text text-transparent">
              CHIMERA SUPER NODE
            </h1>
            <p className="text-sm text-gray-400">Unified Anonymity Gateway • v1.0.0</p>
          </div>
        </div>
        <button
          onClick={toggleDaemon}
          disabled={loading}
          className={daemonRunning ? "btn-secondary" : "btn-primary"}
        >
          {loading ? "..." : daemonRunning ? "⏹ Stop All" : "▶ Start Daemon"}
        </button>
      </header>

      {/* Error Banner */}
      {error && (
        <div className="glass-card p-3 mb-4 border-red-500 border text-red-400 text-sm">
          ⚠️ {error}
        </div>
      )}

      {/* Smart Routing Proxy Info */}
      {daemonRunning && (
        <div className="glass-card p-4 mb-4 border-green-500/50 border">
          <div className="flex items-center justify-between mb-2">
            <div>
              <p className="font-semibold text-green-400">🌐 Smart Routing Proxy Active</p>
              <p className="text-xs text-gray-400">Auto-detects .onion, .i2p, .loki, IPFS, ZeroNet</p>
            </div>
            <code className="bg-gray-800 px-3 py-1 rounded text-lg font-mono text-white">127.0.0.1:{proxyPort}</code>
          </div>
          <div className="text-xs text-gray-500">
            Configure browser SOCKS5 proxy → traffic auto-routes to correct protocol
          </div>
        </div>
      )}

      {/* Protocol Grid */}
      <section className="mb-6">
        <h2 className="text-lg font-semibold mb-3 text-gray-300">
          Protocols ({enabledCount}/{defaultProtocols.length} enabled)
        </h2>
        <div className="grid grid-cols-6 gap-3">
          {defaultProtocols.map(p => (
            <ProtocolCard
              key={p.name}
              name={p.name}
              icon={p.icon}
              port={p.port}
              enabled={protocolSettings[p.name] ?? true}
              running={daemonRunning && (protocolSettings[p.name] ?? true)}
            />
          ))}
        </div>
      </section>

      {/* Stats */}
      <section className="glass-card p-4 mb-6">
        <h2 className="text-sm font-semibold mb-3 text-gray-300">📊 Traffic</h2>
        <div className="flex gap-6 text-center">
          <div>
            <p className="text-2xl font-bold text-green-400">↑ 0 KB/s</p>
            <p className="text-xs text-gray-400">Upload</p>
          </div>
          <div>
            <p className="text-2xl font-bold text-blue-400">↓ 0 KB/s</p>
            <p className="text-xs text-gray-400">Download</p>
          </div>
          <div>
            <p className="text-2xl font-bold text-purple-400">0</p>
            <p className="text-xs text-gray-400">Connections</p>
          </div>
        </div>
      </section>

      {/* Footer */}
      <footer className="flex gap-3">
        <button className="btn-secondary" onClick={() => setShowSettings(true)}>⚙️ Settings</button>
        <button className="btn-secondary">📜 Logs</button>
        <button className="btn-secondary">ℹ️ About</button>
      </footer>
    </div>
  );
}

export default App;
