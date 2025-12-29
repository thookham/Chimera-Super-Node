use chimera_node::config::Settings;
use chimera_node::health_monitor::Protocol;
use chimera_node::process_manager::ProcessManager;
use chimera_node::socks5::Socks5Server;
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tauri::State;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

/// Log entry for frontend display
#[derive(Serialize, Deserialize, Clone)]
pub struct LogEntry {
    timestamp: String,
    level: String,
    message: String,
}

/// Application state shared across Tauri commands
pub struct AppState {
    pub process_manager: Option<ProcessManager>,
    pub socks5_handle: Option<JoinHandle<()>>,
    pub running: bool,
    pub proxy_port: u16,
    pub logs: Vec<LogEntry>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            process_manager: None,
            socks5_handle: None,
            running: false,
            proxy_port: 9050,
            logs: Vec::new(),
        }
    }
}

/// Protocol status for frontend
#[derive(Serialize, Deserialize)]
pub struct ProtocolStatus {
    name: String,
    running: bool,
    port: u16,
}

/// Get current proxy configuration
#[tauri::command]
async fn get_proxy_config(
    state: State<'_, Arc<Mutex<AppState>>>,
) -> Result<HashMap<String, String>, String> {
    let app_state = state.lock().await;
    let mut config = HashMap::new();
    config.insert("port".to_string(), app_state.proxy_port.to_string());
    config.insert(
        "address".to_string(),
        format!("127.0.0.1:{}", app_state.proxy_port),
    );
    config.insert("running".to_string(), app_state.running.to_string());
    Ok(config)
}

/// Set proxy port (requires restart)
#[tauri::command]
async fn set_proxy_port(
    state: State<'_, Arc<Mutex<AppState>>>,
    port: u16,
) -> Result<String, String> {
    let mut app_state = state.lock().await;
    if app_state.running {
        return Err("Stop daemon first to change port".into());
    }
    app_state.proxy_port = port;
    Ok(format!("Proxy port set to {}", port))
}

/// Start all protocols and the SOCKS5 proxy server
/// Start all protocols and the SOCKS5 proxy server
#[tauri::command]
async fn start_daemon(
    state: State<'_, Arc<Mutex<AppState>>>,
    protocols: Vec<String>,
) -> Result<String, String> {
    let mut app_state = state.lock().await;

    if app_state.running {
        return Err("Daemon already running".into());
    }

    // Load settings
    let settings = Settings::new().map_err(|e| format!("Config error: {}", e))?;
    let proxy_port = app_state.proxy_port;

    // Build enabled protocols set
    let mut enabled_protocols = HashSet::new();
    for p in protocols {
        match p.as_str() {
            "tor" => { enabled_protocols.insert(Protocol::Tor); },
            "i2p" => { enabled_protocols.insert(Protocol::I2p); },
            "nym" => { enabled_protocols.insert(Protocol::Nym); },
            "lokinet" => { enabled_protocols.insert(Protocol::Lokinet); },
            "ipfs" => { enabled_protocols.insert(Protocol::Ipfs); },
            "zeronet" => { enabled_protocols.insert(Protocol::ZeroNet); },
            "freenet" => { enabled_protocols.insert(Protocol::Freenet); },
            "retroshare" => { enabled_protocols.insert(Protocol::RetroShare); },
            "gnunet" => { enabled_protocols.insert(Protocol::GnuNet); },
            "tribler" => { enabled_protocols.insert(Protocol::Tribler); },
            _ => { eprintln!("Unknown protocol requested: {}", p); }
        }
    }

    // Create ProcessManager
    let pm = ProcessManager::new(
        settings.chain_mode.clone(),
        enabled_protocols,
        settings.tor.clone(),
        settings.i2p.clone(),
        settings.nym.clone(),
        settings.lokinet.clone(),
        settings.ipfs.clone(),
        settings.zeronet.clone(),
        settings.freenet.clone(),
        settings.retroshare.clone(),
        settings.gnunet.clone(),
        settings.tribler.clone(),
    );

    // Start all protocol processes
    pm.start_processes()
        .await
        .map_err(|e| format!("Start error: {}", e))?;

    // Create and start SOCKS5 proxy server
    let socks5_server = Socks5Server::new(
        proxy_port,
        settings.tor.socks_port,
        settings.i2p.socks_port,
        settings.lokinet.socks_port,
        settings.nym.socks_port,
        settings.ipfs.gateway_port,
        settings.zeronet.port,
        settings.freenet.fcp_port,
        settings.gnunet.socks_port,
        settings.retroshare.api_url.clone(),
        settings.tribler.api_url.clone(),
    );

    // Spawn SOCKS5 server in background task
    let handle = tokio::spawn(async move {
        if let Err(e) = socks5_server.run().await {
            eprintln!("SOCKS5 server error: {}", e);
        }
    });

    app_state.process_manager = Some(pm);
    app_state.socks5_handle = Some(handle);
    app_state.running = true;

    Ok(format!("Daemon started on 127.0.0.1:{}", proxy_port))
}

/// Stop all protocols and SOCKS5 proxy
#[tauri::command]
async fn stop_daemon(state: State<'_, Arc<Mutex<AppState>>>) -> Result<String, String> {
    let mut app_state = state.lock().await;

    if !app_state.running {
        return Err("Daemon not running".into());
    }

    // Abort the SOCKS5 server task
    if let Some(handle) = app_state.socks5_handle.take() {
        handle.abort();
    }

    app_state.process_manager = None;
    app_state.running = false;

    Ok("Daemon stopped".into())
}

/// Get status of all protocols
#[tauri::command]
async fn get_status(
    state: State<'_, Arc<Mutex<AppState>>>,
) -> Result<HashMap<String, bool>, String> {
    let app_state = state.lock().await;

    let mut status = HashMap::new();
    status.insert("daemon".to_string(), app_state.running);
    status.insert("proxy".to_string(), app_state.running);

    if let Some(pm) = &app_state.process_manager {
        let health_map = pm.health_state.read().await;
        
        status.insert("tor".to_string(), *health_map.get(&Protocol::Tor).unwrap_or(&false));
        status.insert("i2p".to_string(), *health_map.get(&Protocol::I2p).unwrap_or(&false));
        status.insert("nym".to_string(), *health_map.get(&Protocol::Nym).unwrap_or(&false));
        status.insert("lokinet".to_string(), *health_map.get(&Protocol::Lokinet).unwrap_or(&false));
        status.insert("ipfs".to_string(), *health_map.get(&Protocol::Ipfs).unwrap_or(&false));
        status.insert("zeronet".to_string(), *health_map.get(&Protocol::ZeroNet).unwrap_or(&false));
        status.insert("freenet".to_string(), *health_map.get(&Protocol::Freenet).unwrap_or(&false));
        status.insert("gnunet".to_string(), *health_map.get(&Protocol::GnuNet).unwrap_or(&false));
        status.insert("retroshare".to_string(), *health_map.get(&Protocol::RetroShare).unwrap_or(&false));
        status.insert("tribler".to_string(), *health_map.get(&Protocol::Tribler).unwrap_or(&false));
    } else {
         // If daemon is stopped, all are false
        let protocols = vec![
            "tor", "i2p", "nym", "lokinet", "ipfs", "zeronet", 
            "freenet", "gnunet", "retroshare", "tribler"
        ];
        for p in protocols {
            status.insert(p.to_string(), false);
        }
    }

    Ok(status)
}

/// Get logs from the log buffer
#[tauri::command]
async fn get_logs(state: State<'_, Arc<Mutex<AppState>>>) -> Result<Vec<LogEntry>, String> {
    let app_state = state.lock().await;
    Ok(app_state.logs.clone())
}

/// Clear logs
#[tauri::command]
async fn clear_logs(state: State<'_, Arc<Mutex<AppState>>>) -> Result<String, String> {
    let mut app_state = state.lock().await;
    app_state.logs.clear();
    Ok("Logs cleared".into())
}

/// Add a log entry (internal helper, also exposed for testing)
fn add_log_entry(logs: &mut Vec<LogEntry>, level: &str, message: &str) {
    let timestamp = Local::now().format("%H:%M:%S").to_string();
    logs.push(LogEntry {
        timestamp,
        level: level.to_string(),
        message: message.to_string(),
    });
    // Keep only last 500 entries
    if logs.len() > 500 {
        logs.remove(0);
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_state = Arc::new(Mutex::new(AppState::default()));

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            start_daemon,
            stop_daemon,
            get_status,
            get_proxy_config,
            set_proxy_port,
            get_logs,
            clear_logs
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
