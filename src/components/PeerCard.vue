<template>
  <div
    class="peer-card"
    :class="{
      'peer-online': peer.status === 'online',
      'peer-offline': peer.status === 'offline',
      'peer-away': peer.status === 'away',
    }"
    @click="$emit('select', peer)"
  >
    <!-- Scanline overlay -->
    <div class="scanline"></div>

    <!-- Grid overlay -->
    <div class="grid-overlay"></div>

    <!-- Status indicator -->
    <div class="status-ring">
      <div class="status-dot" :class="`status-${peer.status}`">
        <span class="pulse"></span>
      </div>
    </div>

    <!-- Avatar / Identity -->
    <div class="peer-identity">
      <div class="peer-avatar">
        <span class="avatar-text">{{ avatarText }}</span>
        <div class="avatar-glow"></div>
      </div>
    </div>

    <!-- Peer info -->
    <div class="peer-info">
      <h3 class="peer-name">{{ peer.displayName }}</h3>
      <div class="peer-meta">
        <span class="peer-ip">{{ peer.ip }}</span>
        <span class="peer-hostname" v-if="peer.hostname">{{ peer.hostname }}</span>
      </div>
      <div class="peer-groups" v-if="peer.groups.length > 0">
        <span v-for="group in peer.groups" :key="group" class="group-tag">
          {{ group }}
        </span>
      </div>
    </div>

    <!-- Status badge -->
    <div class="status-badge" :class="`badge-${peer.status}`">
      <span class="badge-text">{{ statusText }}</span>
    </div>

    <!-- Connection lines (decorative) -->
    <svg class="connection-lines" viewBox="0 0 200 200" xmlns="http://www.w3.org/2000/svg">
      <line x1="0" y1="0" x2="200" y2="0" class="conn-line top" />
      <line x1="0" y1="200" x2="200" y2="200" class="conn-line bottom" />
      <line x1="0" y1="0" x2="0" y2="200" class="conn-line left" />
      <line x1="200" y1="0" x2="200" y2="200" class="conn-line right" />
    </svg>

    <!-- Corner decorations -->
    <div class="corner top-left"></div>
    <div class="corner top-right"></div>
    <div class="corner bottom-left"></div>
    <div class="corner bottom-right"></div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import type { PeerDto } from '@/stores';

interface Props {
  peer: PeerDto;
}

const props = defineProps<Props>();
defineEmits<{
  (e: 'select', peer: PeerDto): void;
}>();

const avatarText = computed(() => {
  return props.peer.displayName?.charAt(0)?.toUpperCase() || '?';
});

const statusText = computed(() => {
  switch (props.peer.status) {
    case 'online': return 'ONLINE';
    case 'offline': return 'OFFLINE';
    case 'away': return 'AWAY';
    default: return 'UNKNOWN';
  }
});
</script>

<style scoped>
/* ==================== Cyberpunk Theme Variables ==================== */
.peer-card {
  --neon-cyan: #00f3ff;
  --neon-magenta: #ff00ff;
  --neon-green: #00ff88;
  --neon-red: #ff3366;
  --neon-amber: #ffaa00;
  --bg-dark: #0a0a12;
  --bg-card: #12121a;
  --border-dim: #2a2a3a;
  --text-primary: #e0e0ff;
  --text-secondary: #8888aa;
  --text-muted: #4a4a5a;
}

/* ==================== Card Container ==================== */
.peer-card {
  position: relative;
  background: var(--bg-card);
  border: 1px solid var(--border-dim);
  border-radius: 12px;
  padding: 20px;
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  overflow: hidden;
  min-height: 180px;
  display: flex;
  flex-direction: column;
}

.peer-card:hover {
  border-color: var(--neon-cyan);
  box-shadow:
    0 0 20px rgba(0, 243, 255, 0.3),
    0 0 40px rgba(0, 243, 255, 0.1),
    inset 0 0 20px rgba(0, 243, 255, 0.05);
  transform: translateY(-4px);
}

/* Status-based glow effects */
.peer-card.peer-online {
  border-color: rgba(0, 255, 136, 0.3);
}

.peer-card.peer-online:hover {
  border-color: var(--neon-green);
  box-shadow:
    0 0 20px rgba(0, 255, 136, 0.4),
    0 0 40px rgba(0, 255, 136, 0.2),
    inset 0 0 20px rgba(0, 255, 136, 0.1);
}

.peer-card.peer-away {
  border-color: rgba(255, 170, 0, 0.3);
}

.peer-card.peer-away:hover {
  border-color: var(--neon-amber);
  box-shadow:
    0 0 20px rgba(255, 170, 0, 0.4),
    0 0 40px rgba(255, 170, 0, 0.2),
    inset 0 0 20px rgba(255, 170, 0, 0.1);
}

.peer-card.peer-offline {
  opacity: 0.6;
  filter: grayscale(0.5);
}

/* ==================== Scanline Overlay ==================== */
.scanline {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 2px;
  background: linear-gradient(
    90deg,
    transparent,
    var(--neon-cyan),
    transparent
  );
  opacity: 0;
  animation: scanline 3s linear infinite;
  pointer-events: none;
}

@keyframes scanline {
  0% {
    top: 0;
    opacity: 0;
  }
  50% {
    opacity: 0.5;
  }
  100% {
    top: 100%;
    opacity: 0;
  }
}

/* ==================== Grid Overlay ==================== */
.grid-overlay {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-image:
    linear-gradient(rgba(0, 243, 255, 0.03) 1px, transparent 1px),
    linear-gradient(90deg, rgba(0, 243, 255, 0.03) 1px, transparent 1px);
  background-size: 20px 20px;
  pointer-events: none;
  opacity: 0;
  transition: opacity 0.3s ease;
}

.peer-card:hover .grid-overlay {
  opacity: 1;
}

/* ==================== Status Ring & Dot ==================== */
.status-ring {
  position: absolute;
  top: 16px;
  right: 16px;
  width: 24px;
  height: 24px;
  border-radius: 50%;
  background: var(--bg-dark);
  border: 2px solid var(--border-dim);
  display: flex;
  align-items: center;
  justify-content: center;
}

.status-dot {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  position: relative;
}

.status-dot.status-online {
  background: var(--neon-green);
  box-shadow: 0 0 10px var(--neon-green);
}

.status-dot.status-away {
  background: var(--neon-amber);
  box-shadow: 0 0 10px var(--neon-amber);
}

.status-dot.status-offline {
  background: var(--neon-red);
  box-shadow: none;
}

.pulse {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  width: 100%;
  height: 100%;
  border-radius: 50%;
  animation: pulse 2s ease-out infinite;
}

.status-online .pulse {
  background: var(--neon-green);
}

.status-away .pulse {
  background: var(--neon-amber);
}

@keyframes pulse {
  0% {
    transform: translate(-50%, -50%) scale(1);
    opacity: 0.8;
  }
  100% {
    transform: translate(-50%, -50%) scale(2.5);
    opacity: 0;
  }
}

/* ==================== Avatar ==================== */
.peer-identity {
  display: flex;
  justify-content: center;
  margin-bottom: 16px;
}

.peer-avatar {
  position: relative;
  width: 72px;
  height: 72px;
  border-radius: 50%;
  background: linear-gradient(135deg, var(--neon-cyan), var(--neon-magenta));
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 32px;
  font-weight: 700;
  color: var(--bg-dark);
  text-shadow: 0 0 20px rgba(255, 255, 255, 0.5);
}

.avatar-glow {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  width: 100%;
  height: 100%;
  border-radius: 50%;
  background: inherit;
  filter: blur(20px);
  opacity: 0.5;
  z-index: -1;
  animation: glow-pulse 3s ease-in-out infinite alternate;
}

@keyframes glow-pulse {
  0% {
    opacity: 0.3;
    transform: translate(-50%, -50%) scale(0.9);
  }
  100% {
    opacity: 0.6;
    transform: translate(-50%, -50%) scale(1.1);
  }
}

/* ==================== Peer Info ==================== */
.peer-info {
  text-align: center;
  flex: 1;
}

.peer-name {
  font-family: 'Courier New', monospace;
  font-size: 18px;
  font-weight: 700;
  color: var(--text-primary);
  margin: 0 0 8px 0;
  letter-spacing: 1px;
  text-transform: uppercase;
}

.peer-meta {
  display: flex;
  flex-direction: column;
  gap: 4px;
  font-family: 'Courier New', monospace;
  font-size: 11px;
  color: var(--text-secondary);
  margin-bottom: 12px;
}

.peer-ip {
  color: var(--neon-cyan);
  text-shadow: 0 0 10px rgba(0, 243, 255, 0.5);
}

.peer-hostname {
  color: var(--text-muted);
}

/* ==================== Group Tags ==================== */
.peer-groups {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
  justify-content: center;
  margin-top: auto;
}

.group-tag {
  font-family: 'Courier New', monospace;
  font-size: 10px;
  padding: 4px 10px;
  background: rgba(0, 243, 255, 0.1);
  border: 1px solid rgba(0, 243, 255, 0.3);
  border-radius: 4px;
  color: var(--neon-cyan);
  text-transform: uppercase;
  letter-spacing: 1px;
}

/* ==================== Status Badge ==================== */
.status-badge {
  position: absolute;
  bottom: 16px;
  right: 16px;
  padding: 4px 12px;
  border-radius: 4px;
  font-family: 'Courier New', monospace;
  font-size: 9px;
  font-weight: 700;
  letter-spacing: 2px;
  text-transform: uppercase;
}

.status-badge.badge-online {
  background: rgba(0, 255, 136, 0.2);
  border: 1px solid var(--neon-green);
  color: var(--neon-green);
  box-shadow: 0 0 10px rgba(0, 255, 136, 0.3);
}

.status-badge.badge-away {
  background: rgba(255, 170, 0, 0.2);
  border: 1px solid var(--neon-amber);
  color: var(--neon-amber);
  box-shadow: 0 0 10px rgba(255, 170, 0, 0.3);
}

.status-badge.badge-offline {
  background: rgba(255, 51, 102, 0.2);
  border: 1px solid var(--neon-red);
  color: var(--neon-red);
}

/* ==================== Connection Lines ==================== */
.connection-lines {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  pointer-events: none;
  opacity: 0;
  transition: opacity 0.3s ease;
}

.peer-card:hover .connection-lines {
  opacity: 1;
}

.conn-line {
  stroke: var(--neon-cyan);
  stroke-width: 1;
  stroke-dasharray: 5, 5;
  opacity: 0.3;
  animation: dash 20s linear infinite;
}

@keyframes dash {
  to {
    stroke-dashoffset: -100;
  }
}

/* ==================== Corner Decorations ==================== */
.corner {
  position: absolute;
  width: 8px;
  height: 8px;
  border: 2px solid var(--neon-cyan);
  opacity: 0;
  transition: opacity 0.3s ease;
}

.peer-card:hover .corner {
  opacity: 1;
}

.corner.top-left {
  top: 4px;
  left: 4px;
  border-right: none;
  border-bottom: none;
}

.corner.top-right {
  top: 4px;
  right: 4px;
  border-left: none;
  border-bottom: none;
}

.corner.bottom-left {
  bottom: 4px;
  left: 4px;
  border-right: none;
  border-top: none;
}

.corner.bottom-right {
  bottom: 4px;
  right: 4px;
  border-left: none;
  border-top: none;
}

/* ==================== Responsive ==================== */
@media (max-width: 640px) {
  .peer-card {
    min-height: 160px;
    padding: 16px;
  }

  .peer-avatar {
    width: 60px;
    height: 60px;
    font-size: 28px;
  }

  .peer-name {
    font-size: 16px;
  }
}

/* ==================== Dark Mode Support ==================== */
@media (prefers-color-scheme: dark) {
  .peer-card {
    /* Already dark by default */
  }
}
</style>
