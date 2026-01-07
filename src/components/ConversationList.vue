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
  background: #fff;
}

/* ==================== Header ==================== */
.list-header {
  padding: 12px 16px;
  border-bottom: 1px solid #e7e7e7;
}

.header-search {
  position: relative;
  background: #f5f5f5;
  border-radius: 6px;
  padding: 8px 12px;
  display: flex;
  align-items: center;
  gap: 8px;
}

.search-icon {
  width: 16px;
  height: 16px;
  color: #999;
  flex-shrink: 0;
}

.search-input {
  flex: 1;
  border: none;
  background: transparent;
  outline: none;
  font-size: 14px;
  color: #333;
}

.search-input::placeholder {
  color: #999;
}

/* ==================== List Content ==================== */
.list-content {
  flex: 1;
  overflow-y: auto;
}

.conversation-item {
  display: flex;
  padding: 12px 16px;
  cursor: pointer;
  transition: background 0.2s;
  gap: 12px;
}

.conversation-item:hover {
  background: #f5f5f5;
}

.conversation-item.active {
  background: #e7e7e7;
}

/* ==================== Avatar ==================== */
.conversation-avatar {
  position: relative;
  width: 48px;
  height: 48px;
  border-radius: 6px;
  background: linear-gradient(135deg, #07C160, #06AE56);
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.avatar-text {
  font-size: 20px;
  font-weight: 600;
  color: #fff;
}

.online-dot {
  position: absolute;
  bottom: -2px;
  right: -2px;
  width: 12px;
  height: 12px;
  background: #07C160;
  border: 2px solid #fff;
  border-radius: 50%;
}

/* ==================== Content ==================== */
.conversation-content {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  justify-content: center;
  gap: 4px;
}

.conversation-top {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.conversation-name {
  font-size: 15px;
  font-weight: 500;
  color: #333;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.conversation-time {
  font-size: 12px;
  color: #999;
  flex-shrink: 0;
}

.conversation-bottom {
  display: flex;
  align-items: center;
}

.conversation-preview {
  font-size: 13px;
  color: #999;
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
  padding: 40px 20px;
  gap: 12px;
}

.empty-icon {
  font-size: 48px;
  opacity: 0.5;
}

.empty-text {
  font-size: 14px;
  color: #999;
  margin: 0;
}

/* ==================== Scrollbar ==================== */
.list-content::-webkit-scrollbar {
  width: 6px;
}

.list-content::-webkit-scrollbar-track {
  background: transparent;
}

.list-content::-webkit-scrollbar-thumb {
  background: #ccc;
  border-radius: 3px;
}

.list-content::-webkit-scrollbar-thumb:hover {
  background: #999;
}

/* ==================== Dark Mode ==================== */
@media (prefers-color-scheme: dark) {
  .conversation-list {
    background: #2a2a2a;
  }

  .list-header {
    border-bottom-color: #3a3a3a;
  }

  .header-search {
    background: #3a3a3a;
  }

  .search-input {
    color: #e0e0e0;
  }

  .search-input::placeholder {
    color: #888;
  }

  .conversation-item:hover {
    background: #3a3a3a;
  }

  .conversation-item.active {
    background: #4a4a4a;
  }

  .conversation-name {
    color: #e0e0e0;
  }

  .conversation-preview,
  .conversation-time {
    color: #888;
  }

  .online-dot {
    border-color: #2a2a2a;
  }

  .empty-text {
    color: #888;
  }

  .list-content::-webkit-scrollbar-thumb {
    background: #555;
  }

  .list-content::-webkit-scrollbar-thumb:hover {
    background: #666;
  }
}
</style>
