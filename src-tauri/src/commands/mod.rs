// Tauri commands - IPC interface layer between frontend and backend

pub mod peer;
pub mod config;
pub mod events;
pub mod message;

// Re-export all commands for easier importing
pub use peer::{get_peers, get_online_peers, get_peer_by_ip, get_peer_stats, PeerDto, PeerStats};
pub use config::{get_config, set_config, reset_config, get_config_value, set_config_value, ConfigDto};
pub use events::poll_events;
pub use message::{send_message, send_text_message, get_messages, MessageDto};
// Re-export AppEvent from state module for frontend use
pub use crate::state::AppEvent as FrontendEvent;
