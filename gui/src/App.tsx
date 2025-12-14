import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./index.css";

interface ProtocolStatus {
  name: string;
  icon: string;
  status: "online" | "offline" | "pending";
  port: number;
}

const protocols: ProtocolStatus[] = [
  { name: "Tor", icon: "🧅", status: "offline", port: 9052 },
  { name: "I2P", icon: "👁", status: "offline", port: 4447 },
  { name: "Nym", icon: "🔗", status: "offline", port: 1080 },
  { name: "Lokinet", icon: "🔒", status: "offline", port: 1090 },
  { name: "IPFS", icon: "🌐", status: "offline", port: 8080 },
  { name: "ZeroNet", icon: "⚡", status: "offline", port: 43110 },
];

function ProtocolCard({ protocol, daemonRunning }: { protocol: ProtocolStatus; daemonRunning: boolean }) {
  const status = daemonRunning ? "online" : protocol.status;

  return (
    <div className="glass-card p-6 flex flex-col items-center gap-3 min-w-[140px]">
      <span className="text-4xl">{protocol.icon}</span>
      <span className="font-semibold text-lg">{protocol.name}</span>
      <div className="flex items-center gap-2">
        <div className={`status-dot ${status}`}></div>
        <span className="text-sm text-gray-400 uppercase">{status}</span>
      </div>
      <span className="text-xs text-gray-500">:{protocol.port}</span>
    </div>
  );
}

function SettingsModal({ onClose }: { onClose: () => void }) {
  const [chainMode, setChainMode] = useState("none");
  const [torEnabled, setTorEnabled] = useState(true);
  const [i2pEnabled, setI2pEnabled] = useState(true);
  const [nymEnabled, setNymEnabled] = useState(false);

  return (
    <div className="fixed inset-0 bg-black/60 flex items-center justify-center z-50" onClick={onClose}>
      <div className="glass-card p-8 w-[500px] max-h-[80vh] overflow-auto" onClick={(e) => e.stopPropagation()}>
        <div className="flex justify-between items-center mb-6">
          <h2 className="text-xl font-bold">⚙️ Settings</h2>
          <button onClick={onClose} className="text-gray-400 hover:text-white text-2xl">&times;</button>
        </div>

        {/* Chain Mode */}
        <div className="mb-6">
          <label className="block text-sm font-medium text-gray-300 mb-2">Protocol Chain Mode</label>
          <select
            value={chainMode}
            onChange={(e) => setChainMode(e.target.value)}
            aria-label="Protocol Chain Mode"
            className="w-full bg-gray-800 border border-gray-600 rounded-lg p-3 text-white"
          >
            <option value="none">None (Direct)</option>
            <option value="tor_over_nym">Tor over Nym</option>
            <option value="nym_over_tor">Nym over Tor</option>
          </select>
        </div>

        {/* Protocol Toggles */}
        <div className="mb-6">
          <label className="block text-sm font-medium text-gray-300 mb-3">Enabled Protocols</label>
          <div className="space-y-3">
            <label className="flex items-center gap-3 cursor-pointer">
              <input type="checkbox" checked={torEnabled} onChange={(e) => setTorEnabled(e.target.checked)} className="w-5 h-5" />
              <span>🧅 Tor</span>
            </label>
            <label className="flex items-center gap-3 cursor-pointer">
              <input type="checkbox" checked={i2pEnabled} onChange={(e) => setI2pEnabled(e.target.checked)} className="w-5 h-5" />
              <span>👁 I2P</span>
            </label>
            <label className="flex items-center gap-3 cursor-pointer">
              <input type="checkbox" checked={nymEnabled} onChange={(e) => setNymEnabled(e.target.checked)} className="w-5 h-5" />
              <span>🔗 Nym</span>
            </label>
          </div>
        </div>

        {/* Save Button */}
        <div className="flex justify-end gap-3">
          <button onClick={onClose} className="btn-secondary">Cancel</button>
          <button onClick={onClose} className="btn-primary">Save</button>
        </div>
      </div>
    </div>
  );
}

function App() {
  const [daemonRunning, setDaemonRunning] = useState(false);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [showSettings, setShowSettings] = useState(false);

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

  return (
    <div className="min-h-screen p-8">
      {/* Settings Modal */}
      {showSettings && <SettingsModal onClose={() => setShowSettings(false)} />}

      {/* Header */}
      <header className="flex items-center justify-between mb-8">
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
        <div className="glass-card p-4 mb-6 border-red-500 border text-red-400">
          ⚠️ {error}
        </div>
      )}

      {/* Protocol Grid */}
      <section className="mb-8">
        <h2 className="text-lg font-semibold mb-4 text-gray-300">Protocols</h2>
        <div className="flex flex-wrap gap-4">
          {protocols.map((p) => (
            <ProtocolCard key={p.name} protocol={p} daemonRunning={daemonRunning} />
          ))}
        </div>
      </section>

      {/* Stats */}
      <section className="glass-card p-6 mb-8">
        <h2 className="text-lg font-semibold mb-4 text-gray-300">📊 Traffic</h2>
        <div className="flex gap-8 text-center">
          <div>
            <p className="text-3xl font-bold text-green-400">↑ 0 KB/s</p>
            <p className="text-sm text-gray-400">Upload</p>
          </div>
          <div>
            <p className="text-3xl font-bold text-blue-400">↓ 0 KB/s</p>
            <p className="text-sm text-gray-400">Download</p>
          </div>
          <div>
            <p className="text-3xl font-bold text-purple-400">0</p>
            <p className="text-sm text-gray-400">Active Connections</p>
          </div>
        </div>
      </section>

      {/* Footer */}
      <footer className="flex gap-4">
        <button className="btn-secondary" onClick={() => setShowSettings(true)}>⚙️ Settings</button>
        <button className="btn-secondary">📜 Logs</button>
        <button className="btn-secondary">ℹ️ About</button>
      </footer>
    </div>
  );
}

export default App;

