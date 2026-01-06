// Peer management module - node discovery, heartbeat, group management

pub mod discovery;
pub mod types;
pub mod manager;
pub mod heartbeat;

// Re-export commonly used types
pub use discovery::PeerDiscovery;
pub use types::{PeerNode, PeerStatus, PeerInfo, PeerEvent};
pub use manager::PeerManager;
pub use heartbeat::HeartbeatMonitor;