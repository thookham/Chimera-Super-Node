import { useChimera } from './hooks/useChimera';
import { HeroStatus } from './components/HeroStatus';
import { ProtocolGrid } from './components/ProtocolGrid';
import { ControlPanel } from './components/ControlPanel';
import { LogTerminal } from './components/LogTerminal';

function App() {
  const {
    status,
    logs,
    loading,
    error,
    startDaemon,
    stopDaemon,
    clearLogs,
    selectedProtocols,
    setSelectedProtocols
  } = useChimera();

  const handleToggle = (id: string) => {
    if (selectedProtocols.includes(id)) {
      setSelectedProtocols(selectedProtocols.filter(p => p !== id));
    } else {
      setSelectedProtocols([...selectedProtocols, id]);
    }
  };

  return (
    <div className="min-h-screen bg-cyber-bg p-6 text-zinc-100 selection:bg-cyber-primary selection:text-white">
      <div className="max-w-4xl mx-auto">
        <header className="mb-8 flex justify-between items-center">
          <div>
            <h1 className="text-2xl font-bold tracking-tight mb-1 bg-gradient-to-r from-white to-zinc-500 bg-clip-text text-transparent">
              CHIMERA SUPER NODE
            </h1>
            <p className="text-zinc-500 text-xs font-mono uppercase tracking-widest">
              Unified Anonymity Orchestrator v1.0.0
            </p>
          </div>
          <div className="text-right">
            <div className="text-xs font-mono text-zinc-600">Connected to Localhost</div>
            <div className={`text-xs font-bold ${status.daemon ? 'text-emerald-500' : 'text-red-500'}`}>
              {status.daemon ? 'DAEMON ACTIVE' : 'DAEMON DISCONNECTED'}
            </div>
          </div>
        </header>

        {error && (
          <div className="mb-6 p-4 bg-red-500/10 border border-red-500/20 text-red-500 rounded-xl flex items-center gap-3">
            <span className="font-bold">ERROR:</span> {error}
          </div>
        )}

        <HeroStatus running={status.daemon} />

        <ProtocolGrid
          status={status}
          selected={selectedProtocols}
          onToggle={handleToggle}
          disabled={status.daemon || loading}
        />

        <ControlPanel
          running={status.daemon}
          loading={loading}
          onStart={startDaemon}
          onStop={stopDaemon}
        />

        <LogTerminal logs={logs} onClear={clearLogs} />

        <footer className="mt-8 text-center text-zinc-700 text-xs font-mono">
          &copy; 2025 Antigravity Research. System Safe.
        </footer>
      </div>
    </div>
  );
}

export default App;
