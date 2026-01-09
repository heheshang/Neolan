// Heartbeat monitor - periodic heartbeat sending and timeout detection
//
// This module handles:
// - Sending periodic heartbeat packets to LAN
// - Detecting offline peers (no heartbeat within timeout)
// - Maintaining peer online status

use crate::Result;
use crate::config::AppConfig;
use crate::modules::peer::{types::PeerNode, discovery::PeerDiscovery};
use std::collections::HashMap;
use std::io::{self, Error as IoError, ErrorKind};
use std::net::{IpAddr, SocketAddr};
use std::sync::{Arc, Mutex, PoisonError};
use std::time::{Duration, SystemTime};
use tracing::{info, warn, debug};

/// Default heartbeat send interval (from AppConfig)
const DEFAULT_HEARTBEAT_INTERVAL: u64 = AppConfig::DEFAULT_HEARTBEAT_INTERVAL;

/// Default peer timeout (from AppConfig)
const DEFAULT_PEER_TIMEOUT: u64 = AppConfig::DEFAULT_PEER_TIMEOUT;

/// Convert lock poison error to io error
fn lock_error<T>(_: PoisonError<T>) -> io::Error {
    IoError::new(ErrorKind::Other, "Mutex lock poisoned")
}

/// Heartbeat monitor
///
/// Periodically sends heartbeat packets and detects offline peers.
pub struct HeartbeatMonitor {
    /// Peer discovery service (for sending heartbeats)
    discovery: PeerDiscovery,

    /// Peer list (shared with PeerManager)
    peers: Arc<Mutex<HashMap<IpAddr, PeerNode>>>,

    /// Whether the monitor is running
    running: Arc<Mutex<bool>>,

    /// Heartbeat send interval (seconds)
    heartbeat_interval: u64,

    /// Peer timeout (seconds)
    peer_timeout: u64,
}

impl HeartbeatMonitor {
    /// Create a new heartbeat monitor
    ///
    /// # Arguments
    /// * `discovery` - Peer discovery service
    /// * `peers` - Shared peer list
    ///
    /// # Returns
    /// * `HeartbeatMonitor` - New heartbeat monitor instance
    pub fn new(
        discovery: PeerDiscovery,
        peers: Arc<Mutex<HashMap<IpAddr, PeerNode>>>,
    ) -> Self {
        info!("Creating HeartbeatMonitor");

        Self {
            discovery,
            peers,
            running: Arc::new(Mutex::new(false)),
            heartbeat_interval: DEFAULT_HEARTBEAT_INTERVAL,
            peer_timeout: DEFAULT_PEER_TIMEOUT,
        }
    }

    /// Create with custom intervals
    ///
    /// # Arguments
    /// * `discovery` - Peer discovery service
    /// * `peers` - Shared peer list
    /// * `heartbeat_interval` - Heartbeat send interval (seconds)
    /// * `peer_timeout` - Peer timeout (seconds)
    ///
    /// # Returns
    /// * `HeartbeatMonitor` - New heartbeat monitor instance
    pub fn with_intervals(
        discovery: PeerDiscovery,
        peers: Arc<Mutex<HashMap<IpAddr, PeerNode>>>,
        heartbeat_interval: u64,
        peer_timeout: u64,
    ) -> Self {
        info!("Creating HeartbeatMonitor with custom intervals");

        Self {
            discovery,
            peers,
            running: Arc::new(Mutex::new(false)),
            heartbeat_interval,
            peer_timeout,
        }
    }

    /// Start the heartbeat monitor
    ///
    /// This starts a background thread that:
    /// 1. Sends heartbeat packets every `heartbeat_interval` seconds
    /// 2. Checks for offline peers every `heartbeat_interval` seconds
    ///
    /// Note: This spawns a new thread and returns immediately.
    ///
    /// # Returns
    /// * `Ok(())` - Monitor started successfully
    /// * `Err(NeoLanError)` - Start failed
    pub fn start(&self) -> Result<()> {
        // Check if already running
        {
            let running = self.running.lock()
                .map_err(lock_error)?;
            if *running {
                warn!("HeartbeatMonitor already running");
                return Ok(());
            }
            drop(running);
            let mut running = self.running.lock()
                .map_err(lock_error)?;
            *running = true;
        }

        info!("Starting HeartbeatMonitor");

        // Clone Arc values for the new thread
        let discovery = self.discovery.clone();
        let peers = Arc::clone(&self.peers);
        let running = Arc::clone(&self.running);
        let heartbeat_interval = self.heartbeat_interval;
        let peer_timeout = self.peer_timeout;

        // Spawn heartbeat thread
        std::thread::spawn(move || {
            info!("Heartbeat thread started");

            loop {
                // Check if still running
                {
                    let is_running = running.lock()
                        .map(|r| *r)
                        .unwrap_or(false);
                    if !is_running {
                        info!("Heartbeat thread stopping");
                        break;
                    }
                }

                // Send heartbeat
                if let Err(e) = Self::send_heartbeat(&discovery, &std::net::SocketAddr::new(
                    std::net::IpAddr::V4(std::net::Ipv4Addr::new(255, 255, 255, 255)),
                    2425,
                )) {
                    warn!("Failed to send heartbeat: {:?}", e);
                }

                // Check for offline peers
                if let Err(e) = Self::check_offline_peers(&peers, peer_timeout) {
                    warn!("Failed to check offline peers: {:?}", e);
                }

                // Sleep until next heartbeat
                std::thread::sleep(Duration::from_secs(heartbeat_interval));
            }

            info!("Heartbeat thread stopped");
        });

        Ok(())
    }

    /// Stop the heartbeat monitor
    pub fn stop(&self) {
        if let Ok(mut running) = self.running.lock() {
            *running = false;
            info!("Stopping HeartbeatMonitor");
        }
    }

    /// Send heartbeat packet to LAN
    fn send_heartbeat(discovery: &PeerDiscovery, _broadcast_addr: &SocketAddr) -> Result<()> {
        debug!("Sending heartbeat to LAN");

        // Use announce_online as heartbeat - it already:
        // 1. Enables broadcast mode
        // 2. Sends STATUS_ONLINE message (serves as heartbeat)
        // 3. Is compatible with IPMsg protocol
        discovery.announce_online()?;

        debug!("Heartbeat sent (via STATUS_ONLINE)");
        Ok(())
    }

    /// Check for offline peers and mark them
    fn check_offline_peers(
        peers: &Arc<Mutex<HashMap<IpAddr, PeerNode>>>,
        timeout_seconds: u64,
    ) -> Result<()> {
        let now = SystemTime::now();
        let timeout_duration = Duration::from_secs(timeout_seconds);

        let mut peers = peers.lock()
            .map_err(lock_error)?;

        let mut offline_count = 0;

        for (ip, peer) in peers.iter_mut() {
            // Calculate time since last seen
            if let Ok(duration) = now.duration_since(peer.last_seen) {
                if duration > timeout_duration {
                    // Peer has timed out
                    if peer.is_online() {
                        info!("Peer timeout: {} (last seen {:?} ago)", ip, duration);
                        peer.mark_offline();
                        offline_count += 1;
                    }
                }
            }
        }

        if offline_count > 0 {
            info!("Marked {} peers as offline", offline_count);
        }

        Ok(())
    }

    /// Check if monitor is running
    ///
    /// # Returns
    /// * `bool` - true if running
    pub fn is_running(&self) -> bool {
        self.running.lock()
            .map(|r| *r)
            .unwrap_or(false)
    }

    /// Get heartbeat interval
    ///
    /// # Returns
    /// * `u64` - Heartbeat interval in seconds
    pub fn heartbeat_interval(&self) -> u64 {
        self.heartbeat_interval
    }

    /// Get peer timeout
    ///
    /// # Returns
    /// * `u64` - Peer timeout in seconds
    pub fn peer_timeout(&self) -> u64 {
        self.peer_timeout
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::network::UdpTransport;

    #[test]
    fn test_heartbeat_monitor_creation() {
        let udp = UdpTransport::bind(0).unwrap();
        let discovery = PeerDiscovery::new(
            udp,
            "TestUser".to_string(),
            "test-host".to_string(),
        );
        let peers = Arc::new(Mutex::new(HashMap::new()));

        let monitor = HeartbeatMonitor::new(discovery, peers);

        assert_eq!(monitor.heartbeat_interval(), DEFAULT_HEARTBEAT_INTERVAL);
        assert_eq!(monitor.peer_timeout(), DEFAULT_PEER_TIMEOUT);
        assert!(!monitor.is_running());
    }

    #[test]
    fn test_heartbeat_monitor_with_custom_intervals() {
        let udp = UdpTransport::bind(0).unwrap();
        let discovery = PeerDiscovery::new(
            udp,
            "TestUser".to_string(),
            "test-host".to_string(),
        );
        let peers = Arc::new(Mutex::new(HashMap::new()));

        let monitor = HeartbeatMonitor::with_intervals(
            discovery,
            peers,
            15,  // 15 second heartbeat
            45,  // 45 second timeout
        );

        assert_eq!(monitor.heartbeat_interval(), 15);
        assert_eq!(monitor.peer_timeout(), 45);
    }

    #[test]
    fn test_heartbeat_monitor_start_stop() {
        let udp = UdpTransport::bind(0).unwrap();
        let discovery = PeerDiscovery::new(
            udp,
            "TestUser".to_string(),
            "test-host".to_string(),
        );
        let peers = Arc::new(Mutex::new(HashMap::new()));

        let monitor = HeartbeatMonitor::new(discovery, peers);

        // Start monitor
        let result = monitor.start();
        assert!(result.is_ok());

        // Should be running
        assert!(monitor.is_running());

        // Start again should return Ok but not spawn another thread
        let result = monitor.start();
        assert!(result.is_ok());

        // Stop monitor
        monitor.stop();
        assert!(!monitor.is_running());
    }

    #[test]
    fn test_check_offline_peers() {
        let udp = UdpTransport::bind(0).unwrap();
        let discovery = PeerDiscovery::new(
            udp,
            "TestUser".to_string(),
            "test-host".to_string(),
        );

        // Create a peer list with one online and one stale peer
        let mut peer_map = HashMap::new();
        let ip1 = "192.168.1.100".parse().unwrap();
        let ip2 = "192.168.1.101".parse().unwrap();

        // Recent peer (should remain online)
        let mut peer1 = PeerNode::new(ip1, 2425);
        peer1.last_seen = SystemTime::now();
        peer_map.insert(ip1, peer1);

        // Stale peer (should be marked offline)
        let mut peer2 = PeerNode::new(ip2, 2425);
        // Set last_seen to 2 minutes ago
        let past = SystemTime::now() - Duration::from_secs(120);
        peer2.last_seen = past;
        peer_map.insert(ip2, peer2);

        let peers = Arc::new(Mutex::new(peer_map));

        // Check for offline peers with 60 second timeout
        let result = HeartbeatMonitor::check_offline_peers(&peers, 60);
        assert!(result.is_ok());

        // Verify peer2 is now offline
        let peers_ref = peers.lock().unwrap();
        assert!(peers_ref[&ip1].is_online());
        assert!(!peers_ref[&ip2].is_online());
    }

    #[test]
    fn test_constants() {
        assert_eq!(DEFAULT_HEARTBEAT_INTERVAL, 30);
        assert_eq!(DEFAULT_PEER_TIMEOUT, 60);
    }
}
