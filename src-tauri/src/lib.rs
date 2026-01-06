// Module declarations
mod commands;
mod migration;
mod modules;
pub mod network;
mod storage;
mod config;
pub mod state;
pub mod utils;
mod error;

// Import Emitter trait for event emission
use tauri::Emitter;

// Re-export commonly used types
pub use error::{NeoLanError, Result};
pub use state::AppState;
pub use state::app_state::TauriEvent;

// Import Tauri commands from submodules
use commands::peer::{get_peers, get_online_peers, get_peer_by_ip, get_peer_stats};
use commands::config::{get_config, set_config, reset_config, get_config_value, set_config_value};
use commands::events::poll_events;
use commands::message::{send_message, send_text_message, get_messages};
use std::sync::mpsc;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize logging system first
    utils::logger::init_logger();

    // Log application startup
    tracing::info!("NeoLan starting...");

    // Create default application configuration
    let default_config = config::AppConfig::default();

    // Initialize application state
    let app_state = AppState::new(default_config);

    // Create channel for event forwarding
    let (event_tx, event_rx) = mpsc::channel::<TauriEvent>();

    // Set the event sender in AppState
    app_state.set_event_sender(event_tx);

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(move |app| {
            // Get AppHandle for emitting events
            let app_handle = app.handle().clone();

            // Spawn background task to listen for events and forward to frontend
            tauri::async_runtime::spawn(async move {
                tracing::info!("Event listener task started");
                for event in event_rx {
                    match &event {
                        TauriEvent::MessageReceived { .. } => {
                            if let Err(e) = app_handle.emit("message-received", &event) {
                                tracing::error!("Failed to emit message-received event: {}", e);
                            }
                        }
                        TauriEvent::PeerOnline { .. } => {
                            if let Err(e) = app_handle.emit("peer-online", &event) {
                                tracing::error!("Failed to emit peer-online event: {}", e);
                            }
                        }
                        TauriEvent::PeerOffline { .. } => {
                            if let Err(e) = app_handle.emit("peer-offline", &event) {
                                tracing::error!("Failed to emit peer-offline event: {}", e);
                            }
                        }
                    }
                }
                tracing::info!("Event listener task ended");
            });

            Ok(())
        })
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            greet,
            get_peers,
            get_online_peers,
            get_peer_by_ip,
            get_peer_stats,
            get_config,
            set_config,
            reset_config,
            get_config_value,
            set_config_value,
            poll_events,
            send_message,
            send_text_message,
            get_messages,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
