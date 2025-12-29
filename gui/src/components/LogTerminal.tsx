import { useRef, useEffect } from 'react';
import { LogEntry } from '../hooks/useChimera';
import { Terminal, Trash2 } from 'lucide-react';

interface LogTerminalProps {
    logs: LogEntry[];
    onClear: () => void;
}

export function LogTerminal({ logs, onClear }: LogTerminalProps) {
    const bottomRef = useRef<HTMLDivElement>(null);

    useEffect(() => {
        bottomRef.current?.scrollIntoView({ behavior: 'smooth' });
    }, [logs]);

    return (
        <div className="glass-panel rounded-2xl overflow-hidden flex flex-col h-[400px]">
            <div className="bg-zinc-900/50 p-3 border-b border-cyber-border flex justify-between items-center">
                <div className="flex items-center gap-2 text-zinc-400">
                    <Terminal size={16} />
                    <span className="font-mono text-xs font-bold uppercase">System Logs</span>
                </div>
                <button
                    onClick={onClear}
                    className="p-1 hover:bg-zinc-800 rounded transition-colors text-zinc-500 hover:text-cyber-danger"
                    title="Clear Logs"
                >
                    <Trash2 size={16} />
                </button>
            </div>

            <div className="flex-1 overflow-y-auto p-4 font-mono text-xs space-y-1 bg-black/40">
                {logs.length === 0 && (
                    <div className="h-full flex items-center justify-center text-zinc-700 italic">
                        No logs available...
                    </div>
                )}
                {logs.map((log, i) => (
                    <div key={i} className="flex gap-3 animate-in fade-in slide-in-from-left-2 duration-200">
                        <span className="text-zinc-600 shrink-0 select-none">{log.timestamp}</span>
                        <span className={`uppercase font-bold shrink-0 w-16 ${log.level === 'ERROR' ? 'text-cyber-danger' :
                                log.level === 'WARN' ? 'text-yellow-500' :
                                    'text-cyber-accent'
                            }`}>
                            [{log.level}]
                        </span>
                        <span className="text-zinc-300 break-all">{log.message}</span>
                    </div>
                ))}
                <div ref={bottomRef} />
            </div>
        </div>
    );
}
