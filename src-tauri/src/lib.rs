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
use crate::network::UdpTransport;
use crate::modules::peer::{PeerManager, discovery::PeerDiscovery};
use crate::modules::message::handler::MessageHandler;
use crate::modules::peer::manager::MessageRouteRequest;
use std::thread;
use std::time::Duration;

// Re-export commonly used types
pub use error::{NeoLanError, Result};
pub use state::AppState;
pub use state::app_state::{TauriEvent, PeerDiscoveredDto};

// Import Tauri commands from submodules
use commands::peer::{get_peers, get_online_peers, get_peer_by_ip, get_peer_stats};
use commands::config::{get_config, set_config, reset_config, get_config_value, set_config_value};
use commands::events::poll_events;
use commands::message::{send_message, send_text_message, get_messages};
use commands::file_transfer::{accept_file_transfer, reject_file_transfer, get_file_transfers, cancel_file_transfer};
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

    // Clone app_state before moving into setup
    let app_state_for_setup = app_state.clone();

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
                        TauriEvent::FileTransferRequest { .. } => {
                            if let Err(e) = app_handle.emit("file-transfer-request", &event) {
                                tracing::error!("Failed to emit file-transfer-request event: {}", e);
                            }
                        }
                        TauriEvent::PeersDiscovered { .. } => {
                            if let Err(e) = app_handle.emit("peers-discovered", &event) {
                                tracing::error!("Failed to emit peers-discovered event: {}", e);
                            }
                        }
                    }
                }
                tracing::info!("Event listener task ended");
            });

            // Initialize PeerManager
            tracing::info!("Initializing PeerManager...");

            // Get UDP port from config
            let config = app_state_for_setup.get_config();
            let udp_port = config.udp_port;

            // Bind UDP transport for receiving (PeerManager)
            let udp_recv = match UdpTransport::bind(udp_port) {
                Ok(u) => {
                    tracing::info!("UDP transport bound to port {}", udp_port);
                    u
                }
                Err(e) => {
                    tracing::error!("Failed to bind UDP transport: {}", e);
                    return Err(Box::new(e));
                }
            };

            // Bind UDP transport for sending (MessageHandler) - any available port
            let udp_send = match UdpTransport::bind(0) {
                Ok(u) => {
                    tracing::info!("UDP send transport bound to port {}", u.port());
                    u
                }
                Err(e) => {
                    tracing::error!("Failed to bind UDP send transport: {}", e);
                    return Err(Box::new(e));
                }
            };

            // Create channel for routing messages from PeerManager to MessageHandler
            let (message_route_tx, message_route_rx) = mpsc::channel::<MessageRouteRequest>();

            // Initialize MessageHandler
            tracing::info!("Initializing MessageHandler...");
            let app_state_for_handler = app_state_for_setup.clone();
            let app_state_arc = std::sync::Arc::new(app_state_for_handler);
            let message_handler = MessageHandler::new(udp_send, config.clone())
                .with_app_state(app_state_arc);
            app_state_for_setup.init_message_handler(message_handler);
            tracing::info!("MessageHandler initialized");

            // Spawn background task to handle routed messages from PeerManager
            let app_state_for_messages = app_state_for_setup.clone();
            let local_ip = config.bind_ip.parse().unwrap_or_else(|_| std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)));
            thread::spawn(move || {
                tracing::info!("Message handler task started");
                for route_request in message_route_rx {
                    let sender_ip = route_request.sender.ip();
                    if let Err(e) = app_state_for_messages.handle_routed_message(
                        &route_request.message,
                        sender_ip,
                        local_ip,
                    ) {
                        tracing::error!("Failed to handle incoming message: {:?}", e);
                    }
                }
                tracing::info!("Message handler task ended");
            });

            // Create PeerDiscovery with system defaults
            let discovery = PeerDiscovery::with_defaults(udp_recv);
            tracing::info!("PeerDiscovery created: {}@{}", discovery.username(), discovery.hostname());

            // Create PeerManager
            let peer_manager = PeerManager::new(discovery);
            // Connect message routing channel to PeerManager
            peer_manager.set_message_handler_channel(message_route_tx);
            app_state_for_setup.init_peer_manager(peer_manager);
            tracing::info!("PeerManager initialized");

            // Spawn peer manager in background thread
            let app_state_for_thread = app_state_for_setup.clone();
            thread::spawn(move || {
                tracing::info!("PeerManager thread started");

                // Start peer discovery (blocking call)
                if let Err(e) = app_state_for_thread.start_peer_manager() {
                    tracing::error!("PeerManager failed: {}", e);
                }

                tracing::info!("PeerManager thread ended");
            });

            // Wait a bit for peers to be discovered
            let app_state_for_discovery = app_state_for_setup.clone();
            thread::spawn(move || {
                thread::sleep(Duration::from_secs(3));
                tracing::info!("Emitting initial peers discovery");
                app_state_for_discovery.emit_peers_discovered();
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
            accept_file_transfer,
            reject_file_transfer,
            get_file_transfers,
            cancel_file_transfer,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
