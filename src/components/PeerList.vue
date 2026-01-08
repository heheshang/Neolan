<template>
  <div class="peer-list-container">
    <!-- Digital rain background effect -->
    <canvas ref="matrixCanvas" class="matrix-rain"></canvas>

    <!-- Header -->
    <header class="list-header">
      <div class="header-content">
        <h1 class="page-title">
          <span class="title-icon">◈</span>
          <span class="title-text">NETWORK NODES</span>
          <span class="title-icon">◈</span>
        </h1>
        <p class="page-subtitle">PEER-TO-PEER MESSAGING PROTOCOL</p>
      </div>

      <!-- Stats bar -->
      <div class="stats-bar">
        <div class="stat-item">
          <span class="stat-label">TOTAL</span>
          <span class="stat-value">{{ stats.total }}</span>
        </div>
        <div class="stat-divider">|</div>
        <div class="stat-item stat-online">
          <span class="stat-label">ONLINE</span>
          <span class="stat-value">{{ stats.online }}</span>
        </div>
        <div class="stat-divider">|</div>
        <div class="stat-item stat-offline">
          <span class="stat-label">OFFLINE</span>
          <span class="stat-value">{{ stats.offline }}</span>
        </div>
      </div>

      <!-- Controls -->
      <div class="controls">
        <button
          @click="refreshPeers"
          class="control-btn refresh-btn"
          :class="{ refreshing: loading }"
          title="Refresh nodes"
        >
          <span class="btn-icon">⟳</span>
          <span class="btn-text">SCAN</span>
        </button>

        <div class="filter-group">
          <button
            v-for="filter in filters"
            :key="filter.key"
            @click="currentFilter = filter.key"
            class="filter-btn"
            :class="{ active: currentFilter === filter.key }"
          >
            <span class="filter-dot" :class="`dot-${filter.key}`"></span>
            <span class="filter-label">{{ filter.label }}</span>
          </button>
        </div>
      </div>
    </header>

    <!-- Peer grid -->
    <main class="peer-grid-container">
      <div v-if="loading && peers.length === 0" class="loading-state">
        <div class="loading-spinner"></div>
        <p class="loading-text">INITIALIZING NETWORK SCAN...</p>
      </div>

      <div v-else-if="filteredPeers.length === 0" class="empty-state">
        <div class="empty-icon">⬡</div>
        <h2 class="empty-title">NO NODES DETECTED</h2>
        <p class="empty-text">Waiting for peers to come online...</p>
      </div>

      <TransitionGroup
        v-else
        name="peer-card"
        tag="div"
        class="peer-grid"
      >
        <PeerCard
          v-for="peer in filteredPeers"
          :key="peer.ip"
          :peer="peer"
          @select="selectPeer"
        />
      </TransitionGroup>
    </main>

    <!-- Footer -->
    <footer class="list-footer">
      <div class="footer-content">
        <span class="footer-text">NEOLAN v0.1.0</span>
        <span class="footer-divider">◆</span>
        <span class="footer-text" v-if="lastUpdated">
          LAST UPDATE: {{ formatTime(lastUpdated) }}
        </span>
      </div>
      <div class="status-indicator">
        <span class="status-dot" :class="{ active: eventStore.polling }"></span>
        <span class="status-text">{{ eventStore.polling ? 'MONITORING ACTIVE' : 'MONITORING PAUSED' }}</span>
      </div>
    </footer>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';
import { usePeerStore, useEventStore } from '@/stores';
import PeerCard from './PeerCard.vue';
import type { PeerDto } from '@/stores';

const peerStore = usePeerStore();
const eventStore = useEventStore();

const matrixCanvas = ref<HTMLCanvasElement | null>(null);
const currentFilter = ref<'all' | 'online' | 'offline' | 'away'>('all');

const peers = computed(() => peerStore.peers);
const loading = computed(() => peerStore.loading);
const lastUpdated = computed(() => peerStore.lastUpdated);
const stats = computed(() => peerStore.stats);

const filters: Array<{ key: 'all' | 'online' | 'offline' | 'away'; label: string }> = [
  { key: 'all', label: 'ALL' },
  { key: 'online', label: 'ONLINE' },
  { key: 'away', label: 'AWAY' },
  { key: 'offline', label: 'OFFLINE' },
];

const filteredPeers = computed(() => {
  if (currentFilter.value === 'all') return peers.value;
  return peers.value.filter(p => p.status === currentFilter.value);
});

async function refreshPeers() {
  await peerStore.fetchPeers();
}

function selectPeer(_peer: PeerDto) {
  // TODO: Navigate to chat or show peer details
}

function formatTime(timestamp: number | null): string {
  if (!timestamp) return 'NEVER';
  const date = new Date(timestamp);
  return date.toLocaleTimeString('en-US', { hour12: false });
}

// Matrix rain effect
let animationFrameId: number | null = null;

function initMatrixRain() {
  const canvas = matrixCanvas.value;
  if (!canvas) return;

  const ctx = canvas.getContext('2d');
  if (!ctx) return;

  let width = canvas.width = canvas.offsetWidth;
  let height = canvas.height = canvas.offsetHeight;

  const chars = '01アイウエオカキクケコサシスセソタチツテトナニヌネノハヒフヘホマミムメモヤユヨラリルレロワヲン';
  const fontSize = 14;
  const columns = Math.floor(width / fontSize);
  const drops: number[] = [];

  for (let i = 0; i < columns; i++) {
    drops.push(Math.random() * -100);
  }

  function draw() {
    if (!ctx) return;
    ctx.fillStyle = 'rgba(10, 10, 18, 0.05)';
    ctx.fillRect(0, 0, width, height);

    ctx.fillStyle = '#00f3ff';
    ctx.font = `${fontSize}px monospace`;

    for (let i = 0; i < drops.length; i++) {
      const text = chars[Math.floor(Math.random() * chars.length)];
      const x = i * fontSize;
      const y = drops[i] * fontSize;

      ctx.globalAlpha = 0.3;
      ctx.fillText(text, x, y);
      ctx.globalAlpha = 1;

      if (y > height && Math.random() > 0.975) {
        drops[i] = 0;
      }
      drops[i]++;
    }

    animationFrameId = requestAnimationFrame(draw);
  }

  draw();

  // Handle resize
  window.addEventListener('resize', () => {
    if (canvas) {
      width = canvas.width = canvas.offsetWidth;
      height = canvas.height = canvas.offsetHeight;
    }
  });
}

// Lifecycle
onMounted(async () => {
  await refreshPeers();
  eventStore.startPolling(100);
  initMatrixRain();

  // Listen for peer events
  eventStore.onPeerOnline(() => refreshPeers());
  eventStore.onPeerOffline(() => refreshPeers());
  eventStore.onPeerStatusChanged(() => refreshPeers());
});

onUnmounted(() => {
  eventStore.stopPolling();
  if (animationFrameId !== null) {
    cancelAnimationFrame(animationFrameId);
  }
});
</script>

<style scoped>
/* ==================== Cyberpunk Theme ==================== */
.peer-list-container {
  --neon-cyan: #00f3ff;
  --neon-magenta: #ff00ff;
  --neon-green: #00ff88;
  --neon-red: #ff3366;
  --neon-amber: #ffaa00;
  --bg-dark: #0a0a12;
  --bg-darker: #050508;
  --bg-panel: rgba(18, 18, 26, 0.95);
  --border-dim: #2a2a3a;
  --border-bright: #3a3a4a;
  --text-primary: #e0e0ff;
  --text-secondary: #8888aa;
  --text-muted: #4a4a5a;

  min-height: 100vh;
  background: var(--bg-dark);
  color: var(--text-primary);
  font-family: 'Courier New', monospace;
  position: relative;
  overflow-x: hidden;
}

/* ==================== Matrix Rain Background ==================== */
.matrix-rain {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  pointer-events: none;
  z-index: 0;
  opacity: 0.15;
}

/* ==================== Header ==================== */
.list-header {
  position: relative;
  z-index: 1;
  background: var(--bg-panel);
  backdrop-filter: blur(20px);
  border-bottom: 1px solid var(--border-dim);
  padding: 24px 32px;
}

.header-content {
  max-width: 1400px;
  margin: 0 auto;
}

.page-title {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 16px;
  font-size: 28px;
  font-weight: 700;
  color: var(--neon-cyan);
  text-shadow: 0 0 20px var(--neon-cyan), 0 0 40px rgba(0, 243, 255, 0.5);
  margin: 0 0 8px 0;
  letter-spacing: 4px;
}

.title-icon {
  animation: title-pulse 2s ease-in-out infinite alternate;
}

@keyframes title-pulse {
  0% { opacity: 0.5; }
  100% { opacity: 1; }
}

.page-subtitle {
  text-align: center;
  font-size: 11px;
  color: var(--text-secondary);
  letter-spacing: 3px;
  margin: 0 0 24px 0;
}

/* ==================== Stats Bar ==================== */
.stats-bar {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 24px;
  padding: 16px;
  background: rgba(0, 0, 0, 0.5);
  border: 1px solid var(--border-dim);
  border-radius: 8px;
  margin-bottom: 20px;
}

.stat-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
}

.stat-label {
  font-size: 10px;
  color: var(--text-muted);
  letter-spacing: 2px;
}

.stat-value {
  font-size: 24px;
  font-weight: 700;
  color: var(--text-primary);
}

.stat-item.stat-online .stat-value {
  color: var(--neon-green);
  text-shadow: 0 0 10px var(--neon-green);
}

.stat-item.stat-offline .stat-value {
  color: var(--neon-red);
}

.stat-divider {
  color: var(--border-dim);
  font-size: 20px;
}

/* ==================== Controls ==================== */
.controls {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
  flex-wrap: wrap;
}

.control-btn {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 20px;
  background: linear-gradient(135deg, rgba(0, 243, 255, 0.1), rgba(255, 0, 255, 0.1));
  border: 1px solid var(--neon-cyan);
  border-radius: 6px;
  color: var(--neon-cyan);
  font-family: inherit;
  font-size: 11px;
  font-weight: 700;
  letter-spacing: 2px;
  cursor: pointer;
  transition: all 0.3s ease;
}

.control-btn:hover {
  background: linear-gradient(135deg, rgba(0, 243, 255, 0.2), rgba(255, 0, 255, 0.2));
  box-shadow: 0 0 20px rgba(0, 243, 255, 0.4);
  transform: translateY(-2px);
}

.control-btn.refreshing {
  animation: refresh-spin 1s linear infinite;
}

@keyframes refresh-spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

.btn-icon {
  font-size: 16px;
}

/* ==================== Filter Buttons ==================== */
.filter-group {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.filter-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 14px;
  background: transparent;
  border: 1px solid var(--border-dim);
  border-radius: 6px;
  color: var(--text-secondary);
  font-family: inherit;
  font-size: 10px;
  font-weight: 600;
  letter-spacing: 1px;
  cursor: pointer;
  transition: all 0.3s ease;
}

.filter-btn:hover {
  border-color: var(--border-bright);
  background: rgba(255, 255, 255, 0.05);
}

.filter-btn.active {
  background: rgba(0, 243, 255, 0.15);
  border-color: var(--neon-cyan);
  color: var(--neon-cyan);
  box-shadow: 0 0 15px rgba(0, 243, 255, 0.3);
}

.filter-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
}

.filter-dot.dot-all {
  background: var(--text-secondary);
}

.filter-dot.dot-online {
  background: var(--neon-green);
  box-shadow: 0 0 6px var(--neon-green);
}

.filter-dot.dot-away {
  background: var(--neon-amber);
  box-shadow: 0 0 6px var(--neon-amber);
}

.filter-dot.dot-offline {
  background: var(--neon-red);
}

/* ==================== Main Grid ==================== */
.peer-grid-container {
  position: relative;
  z-index: 1;
  flex: 1;
  padding: 32px;
}

.peer-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 24px;
  max-width: 1400px;
  margin: 0 auto;
}

/* ==================== Loading & Empty States ==================== */
.loading-state,
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 80px 40px;
  text-align: center;
}

.loading-spinner {
  width: 60px;
  height: 60px;
  border: 3px solid var(--border-dim);
  border-top-color: var(--neon-cyan);
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin-bottom: 24px;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.loading-text {
  font-size: 12px;
  color: var(--text-secondary);
  letter-spacing: 2px;
  animation: blink 1.5s ease-in-out infinite;
}

@keyframes blink {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.5; }
}

.empty-icon {
  font-size: 64px;
  color: var(--border-dim);
  margin-bottom: 16px;
}

.empty-title {
  font-size: 18px;
  color: var(--text-secondary);
  margin: 0 0 8px 0;
  letter-spacing: 2px;
}

.empty-text {
  font-size: 12px;
  color: var(--text-muted);
  margin: 0;
}

/* ==================== Card Transitions ==================== */
.peer-card-enter-active {
  transition: all 0.4s cubic-bezier(0.4, 0, 0.2, 1);
}

.peer-card-leave-active {
  transition: all 0.3s cubic-bezier(0.4, 0, 1, 1);
}

.peer-card-enter-from {
  opacity: 0;
  transform: translateY(20px) scale(0.9);
}

.peer-card-leave-to {
  opacity: 0;
  transform: scale(0.9);
}

.peer-card-move {
  transition: transform 0.4s cubic-bezier(0.4, 0, 0.2, 1);
}

/* ==================== Footer ==================== */
.list-footer {
  position: relative;
  z-index: 1;
  background: var(--bg-panel);
  backdrop-filter: blur(20px);
  border-top: 1px solid var(--border-dim);
  padding: 16px 32px;
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.footer-content {
  display: flex;
  align-items: center;
  gap: 12px;
}

.footer-text {
  font-size: 10px;
  color: var(--text-muted);
  letter-spacing: 1px;
}

.footer-divider {
  color: var(--border-dim);
}

.status-indicator {
  display: flex;
  align-items: center;
  gap: 8px;
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: var(--border-dim);
  transition: all 0.3s ease;
}

.status-dot.active {
  background: var(--neon-green);
  box-shadow: 0 0 10px var(--neon-green);
  animation: status-pulse 2s ease-in-out infinite;
}

@keyframes status-pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.5; }
}

.status-text {
  font-size: 10px;
  color: var(--text-secondary);
  letter-spacing: 1px;
}

/* ==================== Responsive ==================== */
@media (max-width: 768px) {
  .list-header {
    padding: 20px 16px;
  }

  .page-title {
    font-size: 20px;
  }

  .stats-bar {
    flex-wrap: wrap;
    gap: 16px;
  }

  .controls {
    flex-direction: column;
    align-items: stretch;
  }

  .filter-group {
    justify-content: center;
  }

  .peer-grid-container {
    padding: 20px 16px;
  }

  .peer-grid {
    grid-template-columns: 1fr;
  }

  .list-footer {
    flex-direction: column;
    gap: 12px;
    padding: 16px;
  }
}
</style>
