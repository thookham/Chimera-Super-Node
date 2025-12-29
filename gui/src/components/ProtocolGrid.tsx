import { motion } from 'framer-motion';
import { Globe, Shield, Radio, Server, Layers, Check, X } from 'lucide-react';

interface ProtocolProps {
    status: Record<string, boolean>;
    selected: string[];
    onToggle: (id: string) => void;
    disabled: boolean;
}

const protocols = [
    { id: 'tor', name: 'Tor', icon: Shield, color: 'text-violet-400' },
    { id: 'i2p', name: 'I2P', icon: Globe, color: 'text-yellow-400' },
    { id: 'nym', name: 'Nym', icon: Radio, color: 'text-orange-500' },
    { id: 'lokinet', name: 'Lokinet', icon: Server, color: 'text-green-500' },
    { id: 'ipfs', name: 'IPFS', icon: Layers, color: 'text-cyan-400' },
    { id: 'zeronet', name: 'ZeroNet', icon: Globe, color: 'text-pink-400' },
    { id: 'freenet', name: 'Freenet', icon: Layers, color: 'text-blue-500' },
    { id: 'retroshare', name: 'RetroShare', icon: Shield, color: 'text-emerald-400' },
    { id: 'gnunet', name: 'GNUnet', icon: Server, color: 'text-teal-400' },
    { id: 'tribler', name: 'Tribler', icon: Layers, color: 'text-red-400' },
];

export function ProtocolGrid({ status, selected, onToggle, disabled }: ProtocolProps) {
    return (
        <div className="grid grid-cols-2 lg:grid-cols-5 gap-4 mb-6">
            {protocols.map((p, i) => {
                const isSelected = selected.includes(p.id);
                const isActive = status[p.id];

                return (
                    <motion.div
                        key={p.id}
                        initial={{ opacity: 0, scale: 0.9 }}
                        animate={{ opacity: 1, scale: 1 }}
                        transition={{ delay: i * 0.05 }}
                        onClick={() => !disabled && onToggle(p.id)}
                        className={`relative glass-panel p-4 rounded-xl border transition-all duration-300 cursor-pointer overflow-hidden ${isSelected
                                ? 'border-cyber-primary/30 bg-cyber-card/80'
                                : 'border-zinc-800 opacity-60 grayscale hover:grayscale-0 hover:opacity-100'
                            } ${disabled ? 'cursor-not-allowed' : ''}`}
                    >
                        {/* Status Indicator (Top Right) */}
                        <div className="absolute top-3 right-3">
                            <div className={`h-2 w-2 rounded-full transition-colors duration-500 ${isActive
                                    ? 'bg-emerald-500 shadow-[0_0_8px_rgba(16,185,129,0.8)]'
                                    : isSelected && !disabled // If selected but waiting to start (or stopped)
                                        ? 'bg-zinc-600'
                                        : 'bg-zinc-800'
                                }`} />
                        </div>

                        <div className="mb-3">
                            <p.icon className={`${p.color} ${!isSelected ? 'text-zinc-500' : ''} transition-colors`} size={24} />
                        </div>

                        <div className="flex justify-between items-end">
                            <div>
                                <h3 className={`font-bold text-sm ${isSelected ? 'text-zinc-200' : 'text-zinc-500'}`}>{p.name}</h3>
                                <p className="text-[10px] text-zinc-600 font-mono mt-0.5">
                                    {isActive ? 'ONLINE' : isSelected ? 'ENABLED' : 'DISABLED'}
                                </p>
                            </div>

                            {/* Selection Checkmark */}
                            <div className={`rounded-full p-1 ${isSelected ? 'bg-cyber-primary/20 text-cyber-primary' : 'bg-zinc-800/50 text-zinc-700'}`}>
                                {isSelected ? <Check size={12} strokeWidth={3} /> : <X size={12} />}
                            </div>
                        </div>
                    </motion.div>
                );
            })}
        </div>
    );
}
