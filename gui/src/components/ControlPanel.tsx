import { Play, Square } from 'lucide-react';

interface ControlPanelProps {
    running: boolean;
    loading: boolean;
    onStart: () => void;
    onStop: () => void;
}

export function ControlPanel({ running, loading, onStart, onStop }: ControlPanelProps) {
    return (
        <div className="glass-panel p-6 rounded-2xl mb-6 flex items-center justify-between">
            <div>
                <h3 className="text-zinc-100 font-bold text-lg mb-1">Control Center</h3>
                <p className="text-zinc-500 text-sm">Manage the Chimera daemon lifecycle.</p>
            </div>

            <div className="flex gap-4">
                {!running ? (
                    <button
                        onClick={onStart}
                        disabled={loading}
                        className="flex items-center gap-2 px-6 py-3 bg-cyber-primary hover:bg-violet-600 active:bg-violet-700 text-white font-bold rounded-lg transition-all disabled:opacity-50 disabled:cursor-not-allowed shadow-[0_0_20px_rgba(139,92,246,0.3)] hover:shadow-[0_0_30px_rgba(139,92,246,0.5)]"
                    >
                        {loading ? (
                            <div className="h-5 w-5 border-2 border-white/30 border-t-white rounded-full animate-spin" />
                        ) : (
                            <Play size={20} fill="currentColor" />
                        )}
                        INITIALIZE
                    </button>
                ) : (
                    <button
                        onClick={onStop}
                        disabled={loading}
                        className="flex items-center gap-2 px-6 py-3 bg-zinc-800 hover:bg-red-900/30 text-zinc-300 hover:text-red-400 border border-zinc-700 hover:border-red-500/50 font-bold rounded-lg transition-all disabled:opacity-50 disabled:cursor-not-allowed"
                    >
                        {loading ? (
                            <div className="h-5 w-5 border-2 border-zinc-500/30 border-t-zinc-500 rounded-full animate-spin" />
                        ) : (
                            <Square size={20} fill="currentColor" />
                        )}
                        TERMINATE
                    </button>
                )}
            </div>
        </div>
    );
}
