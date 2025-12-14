use chimera_node::config::Settings;
use chimera_node::process_manager::ProcessManager;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tauri::State;
use tokio::sync::Mutex;

/// Application state shared across Tauri commands
pub struct AppState {
    pub process_manager: Option<ProcessManager>,
    pub running: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            process_manager: None,
            running: false,
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

/// Start all protocols (ProcessManager::start_processes)
#[tauri::command]
async fn start_daemon(state: State<'_, Arc<Mutex<AppState>>>) -> Result<String, String> {
    let mut app_state = state.lock().await;

    if app_state.running {
        return Err("Daemon already running".into());
    }

    // Load settings
    let settings = Settings::new().map_err(|e| format!("Config error: {}", e))?;

    // Create ProcessManager
    let pm = ProcessManager::new(
        settings.chain_mode.clone(),
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

    // Start all processes
    pm.start_processes()
        .await
        .map_err(|e| format!("Start error: {}", e))?;

    app_state.process_manager = Some(pm);
    app_state.running = true;

    Ok("Daemon started".into())
}

/// Stop all protocols
#[tauri::command]
async fn stop_daemon(state: State<'_, Arc<Mutex<AppState>>>) -> Result<String, String> {
    let mut app_state = state.lock().await;

    if !app_state.running {
        return Err("Daemon not running".into());
    }

    // Note: ProcessManager doesn't have stop_processes yet, we'll just mark as stopped
    // TODO: Implement proper shutdown
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

    // TODO: Query individual adapter health when HealthMonitor is integrated
    status.insert("tor".to_string(), app_state.running);
    status.insert("i2p".to_string(), app_state.running);
    status.insert("nym".to_string(), false); // Nym often disabled by default
    status.insert("lokinet".to_string(), false);
    status.insert("ipfs".to_string(), false);
    status.insert("zeronet".to_string(), false);

    Ok(status)
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
            get_status
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
