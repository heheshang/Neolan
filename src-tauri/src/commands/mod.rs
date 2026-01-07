// Tauri commands - IPC interface layer between frontend and backend

pub mod peer;
pub mod config;
pub mod events;
pub mod message;
pub mod file_transfer;

// Re-export all commands for easier importing
pub use peer::{get_peers, get_online_peers, get_peer_by_ip, get_peer_stats, PeerDto, PeerStats};
pub use config::{get_config, set_config, reset_config, get_config_value, set_config_value, ConfigDto};
pub use events::poll_events;
pub use message::{send_message, send_text_message, get_messages, MessageDto};
pub use file_transfer::{
    accept_file_transfer, reject_file_transfer, get_file_transfers, cancel_file_transfer, TaskDto,
};

// Re-export AppEvent from state module for frontend use
pub use crate::state::AppEvent as FrontendEvent;
