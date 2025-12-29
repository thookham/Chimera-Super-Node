import { useState, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';

export interface LogEntry {
    timestamp: string;
    level: string;
    message: string;
}

export interface AppStatus {
    daemon: boolean;
    proxy: boolean;
    [key: string]: boolean;
}

export function useChimera() {
    const [status, setStatus] = useState<AppStatus>({ daemon: false, proxy: false });
    const [logs, setLogs] = useState<LogEntry[]>([]);
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const [selectedProtocols, setSelectedProtocols] = useState<string[]>([
        'tor', 'i2p', 'nym', 'lokinet', 'ipfs', 'zeronet', 'freenet', 'retroshare', 'gnunet', 'tribler'
    ]);

    const fetchStatus = useCallback(async () => {
        try {
            const s = await invoke<AppStatus>('get_status');
            setStatus(s);
        } catch (e: any) {
            console.error('Failed to fetch status:', e);
        }
    }, []);

    const fetchLogs = useCallback(async () => {
        try {
            const l = await invoke<LogEntry[]>('get_logs');
            setLogs(l);
        } catch (e: any) {
            console.error('Failed to fetch logs:', e);
        }
    }, []);

    const startDaemon = async () => {
        setLoading(true);
        setError(null);
        try {
            await invoke('start_daemon', { protocols: selectedProtocols });
            await fetchStatus();
        } catch (e: any) {
            setError(e.toString());
        } finally {
            setLoading(false);
        }
    };

    const stopDaemon = async () => {
        setLoading(true);
        setError(null);
        try {
            await invoke('stop_daemon');
            await fetchStatus();
        } catch (e: any) {
            setError(e.toString());
        } finally {
            setLoading(false);
        }
    };

    const clearLogs = async () => {
        try {
            await invoke('clear_logs');
            setLogs([]);
        } catch (e: any) {
            console.error(e);
        }
    };

    // Poll status and logs
    useEffect(() => {
        fetchStatus();
        const interval = setInterval(() => {
            fetchStatus();
            fetchLogs();
        }, 1000);
        return () => clearInterval(interval);
    }, [fetchStatus, fetchLogs]);

    return {
        status,
        logs,
        loading,
        error,
        startDaemon,
        stopDaemon,
        clearLogs,
        selectedProtocols,
        setSelectedProtocols
    };
}
