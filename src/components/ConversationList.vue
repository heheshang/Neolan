<template>
  <div class="conversation-list">
    <!-- Header -->
    <header class="list-header">
      <div class="header-search">
        <svg class="search-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="11" cy="11" r="8"/>
          <path d="M21 21l-4.35-4.35"/>
        </svg>
        <input
          type="text"
          v-model="searchQuery"
          placeholder="搜索"
          class="search-input"
        />
      </div>
    </header>

    <!-- Conversation List -->
    <div class="list-content">
      <div
        v-for="peer in filteredPeers"
        :key="peer.ip"
        class="conversation-item"
        :class="{ active: isSelected(peer) }"
        @click="$emit('select', peer)"
      >
        <!-- Avatar -->
        <div class="conversation-avatar">
          <span class="avatar-text">{{ getAvatarText(peer) }}</span>
          <div v-if="peer.status === 'online'" class="online-dot"></div>
        </div>

        <!-- Content -->
        <div class="conversation-content">
          <div class="conversation-top">
            <span class="conversation-name">{{ peer.displayName || peer.ip }}</span>
            <span class="conversation-time">{{ formatTime(peer.lastSeen) }}</span>
          </div>
          <div class="conversation-bottom">
            <span class="conversation-preview">{{ getLastMessage(peer) }}</span>
          </div>
        </div>
      </div>

      <!-- Empty State -->
      <div v-if="filteredPeers.length === 0" class="empty-state">
        <div class="empty-icon">🔍</div>
        <p class="empty-text">未找到联系人</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import type { PeerDto } from '@/stores';

interface Props {
  peers: PeerDto[];
  selectedPeer?: PeerDto | null;
}

const props = defineProps<Props>();
defineEmits<{
  (e: 'select', peer: PeerDto): void;
}>();

const searchQuery = ref('');

const filteredPeers = computed(() => {
  if (!searchQuery.value) return props.peers;
  const query = searchQuery.value.toLowerCase();
  return props.peers.filter(p =>
    p.displayName?.toLowerCase().includes(query) ||
    p.ip.includes(query) ||
    p.hostname?.toLowerCase().includes(query)
  );
});

function isSelected(peer: PeerDto): boolean {
  return props.selectedPeer?.ip === peer.ip;
}

function getAvatarText(peer: PeerDto): string {
  return peer.displayName?.charAt(0)?.toUpperCase() || peer.ip.slice(-1);
}

function formatTime(timestamp: number): string {
  const date = new Date(timestamp);
  const now = new Date();
  const diff = now.getTime() - date.getTime();

  // Less than 1 minute
  if (diff < 60000) return '刚刚';

  // Less than 1 hour
  if (diff < 3600000) return `${Math.floor(diff / 60000)}分钟前`;

  // Today
  if (date.toDateString() === now.toDateString()) {
    return date.toLocaleTimeString('zh-CN', { hour: '2-digit', minute: '2-digit' });
  }

  // Yesterday
  const yesterday = new Date(now);
  yesterday.setDate(yesterday.getDate() - 1);
  if (date.toDateString() === yesterday.toDateString()) {
    return '昨天';
  }

  // This week
  if (diff < 604800000) {
    const days = ['日', '一', '二', '三', '四', '五', '六'];
    return `周${days[date.getDay()]}`;
  }

  // Older
  return date.toLocaleDateString('zh-CN', { month: '2-digit', day: '2-digit' });
}

function getLastMessage(peer: PeerDto): string {
  return peer.status === 'online' ? '[在线111]' : '[离线]';
}
</script>

<style scoped>
.conversation-list {
  height: 100%;
  display: flex;
  flex-direction: column;
  background: var(--color-bg-secondary);
}

/* ==================== Header ==================== */
.list-header {
  padding: var(--spacing-3) var(--spacing-4);
  border-bottom: 1px solid var(--color-border-subtle);
}

.header-search {
  position: relative;
  background: var(--color-bg-tertiary);
  border-radius: var(--radius-md);
  padding: var(--spacing-2) var(--spacing-3);
  display: flex;
  align-items: center;
  gap: var(--spacing-2);
  transition: all var(--transition-fast);
}

.header-search:focus-within {
  background: var(--color-bg-elevated);
  box-shadow: var(--shadow-sm);
}

.search-icon {
  width: 16px;
  height: 16px;
  color: var(--color-text-tertiary);
  flex-shrink: 0;
  transition: color var(--transition-fast);
}

.header-search:focus-within .search-icon {
  color: var(--color-primary);
}

.search-input {
  flex: 1;
  border: none;
  background: transparent;
  outline: none;
  font-size: var(--font-size-base);
  color: var(--color-text-primary);
  font-family: var(--font-sans);
}

.search-input::placeholder {
  color: var(--color-text-tertiary);
}

/* ==================== List Content ==================== */
.list-content {
  flex: 1;
  overflow-y: auto;
}

.conversation-item {
  display: flex;
  padding: var(--spacing-3) var(--spacing-4);
  cursor: pointer;
  transition: all var(--transition-fast);
  gap: var(--spacing-3);
  border-bottom: 1px solid transparent;
}

.conversation-item:hover {
  background: var(--color-bg-tertiary);
}

.conversation-item.active {
  background: var(--color-primary-subtle);
  border-bottom-color: var(--color-primary-subtle);
}

/* ==================== Avatar ==================== */
.conversation-avatar {
  position: relative;
  width: 48px;
  height: 48px;
  border-radius: var(--radius-lg);
  background: var(--gradient-primary);
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  box-shadow: var(--shadow-sm);
  transition: transform var(--transition-fast);
}

.conversation-item:hover .conversation-avatar {
  transform: scale(1.05);
  box-shadow: var(--shadow-md);
}

.avatar-text {
  font-size: 20px;
  font-weight: var(--font-weight-semibold);
  color: var(--color-text-inverse);
}

.online-dot {
  position: absolute;
  bottom: -2px;
  right: -2px;
  width: 12px;
  height: 12px;
  background: var(--color-online);
  border: 2px solid var(--color-bg-secondary);
  border-radius: var(--radius-full);
  box-shadow: var(--shadow-xs);
}

/* ==================== Content ==================== */
.conversation-content {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  justify-content: center;
  gap: var(--spacing-1);
}

.conversation-top {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.conversation-name {
  font-size: var(--font-size-base);
  font-weight: var(--font-weight-medium);
  color: var(--color-text-primary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.conversation-item.active .conversation-name {
  color: var(--color-primary);
  font-weight: var(--font-weight-semibold);
}

.conversation-time {
  font-size: var(--font-size-sm);
  color: var(--color-text-tertiary);
  flex-shrink: 0;
}

.conversation-bottom {
  display: flex;
  align-items: center;
}

.conversation-preview {
  font-size: var(--font-size-sm);
  color: var(--color-text-tertiary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

/* ==================== Empty State ==================== */
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: var(--spacing-12) var(--spacing-5);
  gap: var(--spacing-3);
}

.empty-icon {
  font-size: 48px;
  opacity: 0.3;
}

.empty-text {
  font-size: var(--font-size-base);
  color: var(--color-text-tertiary);
  margin: 0;
}

/* ==================== Responsive ==================== */
@media (max-width: 768px) {
  .list-header {
    padding: var(--spacing-2) var(--spacing-3);
  }

  .conversation-item {
    padding: var(--spacing-2) var(--spacing-3);
    gap: var(--spacing-2);
  }

  .conversation-avatar {
    width: 42px;
    height: 42px;
  }

  .avatar-text {
    font-size: 18px;
  }

  .conversation-name {
    font-size: var(--font-size-sm);
  }
}
</style>
