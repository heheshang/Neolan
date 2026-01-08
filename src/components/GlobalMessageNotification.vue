<template>
  <Teleport to="body">
    <TransitionGroup name="notification">
      <div
        v-for="notification in notifications"
        :key="notification.id"
        class="message-notification"
        :class="`notification-${notification.type}`"
      >
        <div class="notification-header">
          <span class="notification-icon">
            {{ notification.type === 'received' ? '📨' : '📤' }}
          </span>
          <span class="notification-title">{{ notification.title }}</span>
          <button class="notification-close" @click="removeNotification(notification.id)">
            &times;
          </button>
        </div>
        <div class="notification-content">
          {{ notification.content }}
        </div>
        <div class="notification-meta">
          <span class="notification-time">{{ formatTime(notification.timestamp) }}</span>
          <span v-if="notification.senderIp" class="notification-sender">{{ notification.senderIp }}</span>
        </div>
      </div>
    </TransitionGroup>
  </Teleport>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { listen } from '@tauri-apps/api/event';

interface Notification {
  id: string;
  type: 'received' | 'sent';
  title: string;
  content: string;
  senderIp?: string;
  timestamp: number;
}

const notifications = ref<Notification[]>([]);

let notificationIdCounter = 0;
let unlisteners: (() => void)[] = [];

function generateId(): string {
  return `notification-${++notificationIdCounter}-${Date.now()}`;
}

function formatTime(timestamp: number): string {
  const date = new Date(timestamp);
  return date.toLocaleTimeString('en-US', {
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit',
  });
}

function addNotification(notification: Omit<Notification, 'id'>) {
  const id = generateId();
  notifications.value.push({
    ...notification,
    id,
  });

  // Auto-remove after 5 seconds
  setTimeout(() => {
    removeNotification(id);
  }, 5000);
}

function removeNotification(id: string) {
  const index = notifications.value.findIndex((n) => n.id === id);
  if (index >= 0) {
    notifications.value.splice(index, 1);
  }
}

onMounted(async () => {
  // Listen for received messages
  const unlistenReceived = await listen("message-received", (event) => {
    const payload = event.payload as any;
    addNotification({
      type: 'received',
      title: `New message from ${payload.senderName}`,
      content: payload.content,
      senderIp: payload.senderIp,
      timestamp: payload.sentAt || Date.now(),
    });
  });

  // Listen for sent messages
  const unlistenSent = await listen("message-sent", (event) => {
    const payload = event.payload as any;
    addNotification({
      type: 'sent',
      title: 'Message sent',
      content: `Message sent to ${payload.receiverIp}`,
      senderIp: payload.receiverIp,
      timestamp: Date.now(),
    });
  });

  unlisteners = [unlistenReceived, unlistenSent];
});

onUnmounted(() => {
  // Cleanup listeners
  unlisteners.forEach((fn) => fn());
  unlisteners = [];
});
</script>

<style scoped>
.message-notification {
  position: fixed;
  top: 20px;
  right: 20px;
  min-width: 300px;
  max-width: 400px;
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  z-index: 10000;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  gap: 8px;
  padding: 12px;
}

.notification-received {
  border-left: 4px solid #4caf50;
}

.notification-sent {
  border-left: 4px solid #2196f3;
}

.notification-header {
  display: flex;
  align-items: center;
  gap: 8px;
  font-weight: 600;
}

.notification-icon {
  font-size: 18px;
}

.notification-title {
  flex: 1;
  color: var(--text);
  font-size: 14px;
}

.notification-close {
  background: none;
  border: none;
  font-size: 20px;
  color: var(--text-secondary);
  cursor: pointer;
  padding: 0;
  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 4px;
  transition: background 0.2s;
}

.notification-close:hover {
  background: var(--surface-hover);
}

.notification-content {
  color: var(--text);
  font-size: 13px;
  line-height: 1.4;
  word-wrap: break-word;
  white-space: pre-wrap;
  max-height: 100px;
  overflow-y: auto;
}

.notification-meta {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 8px;
  font-size: 11px;
  color: var(--text-secondary);
}

.notification-time {
  font-family: monospace;
}

.notification-sender {
  font-family: monospace;
  background: var(--surface-alt);
  padding: 2px 6px;
  border-radius: 4px;
}

/* Animation */
.notification-enter-active {
  animation: slideIn 0.3s ease-out;
}

.notification-leave-active {
  animation: slideOut 0.3s ease-in;
}

@keyframes slideIn {
  from {
    transform: translateX(100%);
    opacity: 0;
  }
  to {
    transform: translateX(0);
    opacity: 1;
  }
}

@keyframes slideOut {
  from {
    transform: translateX(0);
    opacity: 1;
  }
  to {
    transform: translateX(100%);
    opacity: 0;
  }
}

/* Dark mode support */
@media (prefers-color-scheme: dark) {
  .message-notification {
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.4);
  }
}
</style>
