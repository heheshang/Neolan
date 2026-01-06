// Application state management
//
// Provides a centralized state management structure for the Tauri application.

use crate::config::AppConfig;
use crate::modules::peer::{PeerManager, PeerNode};
use crate::Result;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

/// Application state
///
/// This struct holds all the global state for the Tauri application.
/// It is wrapped in Arc<Mutex<>> to allow thread-safe access across commands.
#[derive(Clone)]
pub struct AppState {
    /// Peer manager (when initialized)
    peer_manager: Arc<Mutex<Option<PeerManager>>>,

    /// Current application configuration
    config: Arc<Mutex<AppConfig>>,

    /// Event emitter for state changes
    event_emitter: Arc<Mutex<super::events::AppEventEmitter>>,
}

impl AppState {
    /// Create a new application state
    pub fn new(config: AppConfig) -> Self {
        Self {
            peer_manager: Arc::new(Mutex::new(None)),
            config: Arc::new(Mutex::new(config)),
            event_emitter: Arc::new(Mutex::new(super::events::AppEventEmitter::new())),
        }
    }

    /// Get the current configuration
    pub fn get_config(&self) -> AppConfig {
        self.config.lock().unwrap().clone()
    }

    /// Set the configuration
    pub fn set_config(&self, config: AppConfig) {
        *self.config.lock().unwrap() = config;
        self.emit_event(super::events::AppEvent::ConfigChanged);
    }

    /// Update configuration fields
    pub fn update_config<F>(&self, updater: F) -> Result<()>
    where
        F: FnOnce(&mut AppConfig),
    {
        let mut config = self.config.lock().unwrap();
        updater(&mut config);
        drop(config);
        self.emit_event(super::events::AppEvent::ConfigChanged);
        Ok(())
    }

    /// Initialize the peer manager
    ///
    /// This should be called once during application startup.
    pub fn init_peer_manager(&self, peer_manager: PeerManager) {
        let mut pm = self.peer_manager.lock().unwrap();
        *pm = Some(peer_manager);
        self.emit_event(super::events::AppEvent::Initialized);
    }

    /// Get peer list (all peers)
    pub fn get_peers(&self) -> Vec<PeerNode> {
        if let Some(manager) = self.peer_manager.lock().unwrap().as_ref() {
            manager.get_all_peers()
        } else {
            Vec::new()
        }
    }

    /// Get online peers
    pub fn get_online_peers(&self) -> Vec<PeerNode> {
        if let Some(manager) = self.peer_manager.lock().unwrap().as_ref() {
            manager.get_online_peers()
        } else {
            Vec::new()
        }
    }

    /// Get peer by IP
    pub fn get_peer(&self, ip: std::net::IpAddr) -> Option<PeerNode> {
        if let Some(manager) = self.peer_manager.lock().unwrap().as_ref() {
            manager.get_peer(ip)
        } else {
            None
        }
    }

    /// Get peer statistics
    pub fn get_peer_stats(&self) -> PeerStats {
        if let Some(manager) = self.peer_manager.lock().unwrap().as_ref() {
            let all = manager.get_all_peers();
            let online_count = all.iter().filter(|p| p.is_online()).count();
            PeerStats {
                total: all.len(),
                online: online_count,
                offline: all.len() - online_count,
            }
        } else {
            PeerStats {
                total: 0,
                online: 0,
                offline: 0,
            }
        }
    }

    /// Emit an event
    pub fn emit_event(&self, event: super::events::AppEvent) {
        if let Ok(mut emitter) = self.event_emitter.try_lock() {
            emitter.emit(event);
        }
    }

    /// Get pending events and clear the buffer
    pub fn drain_events(&self) -> Vec<super::events::AppEvent> {
        if let Ok(mut emitter) = self.event_emitter.try_lock() {
            emitter.drain()
        } else {
            Vec::new()
        }
    }
}

/// Peer statistics
#[derive(Clone, Debug, serde::Serialize)]
pub struct PeerStats {
    pub total: usize,
    pub online: usize,
    pub offline: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state_creation() {
        let config = AppConfig::default();
        let state = AppState::new(config);

        // Verify initial state
        let peers = state.get_peers();
        assert_eq!(peers.len(), 0);

        let stats = state.get_peer_stats();
        assert_eq!(stats.total, 0);
        assert_eq!(stats.online, 0);
        assert_eq!(stats.offline, 0);
    }

    #[test]
    fn test_config_access() {
        let config = AppConfig::default();
        let state = AppState::new(config.clone());

        // Get config
        let retrieved = state.get_config();
        assert_eq!(retrieved.udp_port, config.udp_port);

        // Update config
        let mut new_config = config;
        new_config.udp_port = 2426;
        state.set_config(new_config);

        let updated = state.get_config();
        assert_eq!(updated.udp_port, 2426);
    }

    #[test]
    fn test_update_config() {
        let config = AppConfig::default();
        let state = AppState::new(config);

        // Update config field
        state
            .update_config(|c| {
                c.udp_port = 2500;
                c.log_level = "debug".to_string();
            })
            .unwrap();

        let updated = state.get_config();
        assert_eq!(updated.udp_port, 2500);
        assert_eq!(updated.log_level, "debug");
    }
}
