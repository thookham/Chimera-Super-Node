import { motion } from 'framer-motion';
import { Activity, ShieldCheck, ShieldAlert } from 'lucide-react';

interface HeroStatusProps {
    running: boolean;
}

export function HeroStatus({ running }: HeroStatusProps) {
    return (
        <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            className="relative overflow-hidden rounded-2xl glass-panel p-8 mb-6"
        >
            <div className="absolute top-0 right-0 p-4 opacity-10">
                <Activity size={120} />
            </div>

            <div className="flex items-center gap-6 relative z-10">
                <div className={`p-4 rounded-full ${running ? 'bg-cyber-success/20 text-cyber-success' : 'bg-cyber-danger/20 text-cyber-danger'}`}>
                    {running ? <ShieldCheck size={48} /> : <ShieldAlert size={48} />}
                </div>

                <div>
                    <h2 className="text-zinc-400 text-sm font-mono uppercase tracking-widest mb-1">System Status</h2>
                    <div className="flex items-center gap-3">
                        <h1 className={`text-4xl font-bold tracking-tight ${running ? 'text-white neon-text' : 'text-zinc-500'}`}>
                            {running ? 'SYSTEM ONLINE' : 'SYSTEM OFFLINE'}
                        </h1>
                        {running && (
                            <span className="relative flex h-3 w-3">
                                <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-cyber-success opacity-75"></span>
                                <span className="relative inline-flex rounded-full h-3 w-3 bg-cyber-success"></span>
                            </span>
                        )}
                    </div>
                    <p className="text-zinc-500 mt-2 font-mono text-sm">
                        {running ? 'All anonymity protocols active and routing.' : 'Daemon is stopped. No traffic is being proxied.'}
                    </p>
                </div>
            </div>
        </motion.div>
    );
}
