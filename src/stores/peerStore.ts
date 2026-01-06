import { defineStore } from "pinia";
import { ref, computed } from "vue";
import * as api from "../api";

export const usePeerStore = defineStore("peer", () => {
  // ==================== State ====================

  const peers = ref<api.PeerDto[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);
  const lastUpdated = ref<number | null>(null);

  // ==================== Getters ====================

  const onlinePeers = computed(() =>
    peers.value.filter((p) => p.status === "online")
  );

  const offlinePeers = computed(() =>
    peers.value.filter((p) => p.status === "offline")
  );

  const awayPeers = computed(() =>
    peers.value.filter((p) => p.status === "away")
  );

  const peerCount = computed(() => peers.value.length);
  const onlineCount = computed(() => onlinePeers.value.length);
  const offlineCount = computed(() => offlinePeers.value.length);

  const getPeerByIp = (ip: string) => {
    return peers.value.find((p) => p.ip === ip);
  };

  const getPeersByStatus = (status: "online" | "offline" | "away") => {
    return peers.value.filter((p) => p.status === status);
  };

  const stats = computed(() => ({
    total: peerCount.value,
    online: onlineCount.value,
    offline: offlineCount.value,
  }));

  // ==================== Actions ====================

  async function fetchPeers() {
    loading.value = true;
    error.value = null;
    try {
      const result = await api.getPeers();
      peers.value = result;
      lastUpdated.value = Date.now();
    } catch (err) {
      error.value = err instanceof Error ? err.message : String(err);
      console.error("Failed to fetch peers:", err);
    } finally {
      loading.value = false;
    }
  }

  async function fetchOnlinePeers() {
    loading.value = true;
    error.value = null;
    try {
      const result = await api.getOnlinePeers();
      // Update online peers, keep offline peers in memory
      const offlineIpSet = new Set(
        peers.value.filter((p) => p.status === "offline").map((p) => p.ip)
      );
      const existingOnlineIps = new Set(result.map((p) => p.ip));

      // Merge: keep offline peers, update online peers
      peers.value = [
        ...peers.value.filter((p) => offlineIpSet.has(p.ip) && !existingOnlineIps.has(p.ip)),
        ...result,
      ];
      lastUpdated.value = Date.now();
    } catch (err) {
      error.value = err instanceof Error ? err.message : String(err);
      console.error("Failed to fetch online peers:", err);
    } finally {
      loading.value = false;
    }
  }

  async function fetchPeerByIp(ip: string) {
    loading.value = true;
    error.value = null;
    try {
      const result = await api.getPeerByIp(ip);
      if (result) {
        const index = peers.value.findIndex((p) => p.ip === ip);
        if (index >= 0) {
          peers.value[index] = result;
        } else {
          peers.value.push(result);
        }
        lastUpdated.value = Date.now();
      }
      return result;
    } catch (err) {
      error.value = err instanceof Error ? err.message : String(err);
      console.error(`Failed to fetch peer ${ip}:`, err);
      return null;
    } finally {
      loading.value = false;
    }
  }

  async function fetchStats() {
    try {
      const result = await api.getPeerStats();
      return result;
    } catch (err) {
      console.error("Failed to fetch peer stats:", err);
      return { total: 0, online: 0, offline: 0 };
    }
  }

  function updatePeer(peer: api.PeerDto) {
    const index = peers.value.findIndex((p) => p.ip === peer.ip);
    if (index >= 0) {
      peers.value[index] = peer;
    } else {
      peers.value.push(peer);
    }
    lastUpdated.value = Date.now();
  }

  function updatePeerStatus(ip: string, status: "online" | "offline" | "away") {
    const peer = peers.value.find((p) => p.ip === ip);
    if (peer) {
      peer.status = status;
      peer.lastSeen = Date.now();
      lastUpdated.value = Date.now();
    }
  }

  function removePeer(ip: string) {
    const index = peers.value.findIndex((p) => p.ip === ip);
    if (index >= 0) {
      peers.value.splice(index, 1);
      lastUpdated.value = Date.now();
    }
  }

  function clearPeers() {
    peers.value = [];
    lastUpdated.value = null;
  }

  // ==================== Return ====================

  return {
    // State
    peers,
    loading,
    error,
    lastUpdated,

    // Getters
    onlinePeers,
    offlinePeers,
    awayPeers,
    peerCount,
    onlineCount,
    offlineCount,
    stats,
    getPeerByIp,
    getPeersByStatus,

    // Actions
    fetchPeers,
    fetchOnlinePeers,
    fetchPeerByIp,
    fetchStats,
    updatePeer,
    updatePeerStatus,
    removePeer,
    clearPeers,
  };
});
