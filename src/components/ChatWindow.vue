<template>
  <div class="chat-window">
    <!-- Header -->
    <div class="chat-header">
      <div class="chat-peer-info">
        <h3 class="chat-peer-name">{{ peer?.displayName || 'Unknown' }}</h3>
        <span class="chat-peer-ip">{{ peer?.ip || '' }}</span>
      </div>
      <button class="chat-close-btn" @click="$emit('close')">&times;</button>
    </div>

    <!-- Messages area -->
    <div class="chat-messages" ref="messagesContainer">
      <div v-if="loading" class="chat-loading">Loading messages...</div>
      <div v-else-if="error" class="chat-error">{{ error }}</div>
      <div v-else-if="messages.length === 0" class="chat-empty">
        No messages yet. Start the conversation!
      </div>
      <div v-else class="chat-message-list">
        <div
          v-for="msg in messages"
          :key="msg.id"
          class="chat-message"
          :class="isSentMessage(msg) ? 'message-sent' : 'message-received'"
        >
          <div class="message-sender">{{ getSenderName(msg) }}</div>
          <div class="message-content">{{ msg.content }}</div>
          <div class="message-time">{{ formatTime(msg.sentAt) }}</div>
        </div>
      </div>
    </div>

    <!-- Input area -->
    <div class="chat-input-area">
      <textarea
        v-model="newMessage"
        class="chat-input"
        placeholder="Type a message..."
        rows="1"
        @keydown.enter.exact.prevent="sendMessage"
        @keydown.enter.shift.exact="newMessage += '\n'"
      />
      <button
        class="chat-send-btn"
        @click="sendMessage"
        :disabled="!newMessage.trim() || sending"
      >
        {{ sending ? 'Sending...' : 'Send' }}
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, nextTick, watch } from 'vue';
import { listen } from '@tauri-apps/api/event';
import type { PeerDto, MessageDto } from '@/stores';
import * as api from '../api';

interface Props {
  peer: PeerDto;
}

const props = defineProps<Props>();
defineEmits<{
  (e: 'close'): void;
}>();

// State
const messages = ref<MessageDto[]>([]);
const newMessage = ref('');
const loading = ref(false);
const sending = ref(false);
const error = ref<string | null>(null);
const messagesContainer = ref<HTMLElement | null>(null);

// Event listener cleanup function
let unlistenMessage: (() => void) | null = null;

// Computed
const isSentMessage = (msg: MessageDto) => msg.senderIp === props.peer.ip;

const getSenderName = (msg: MessageDto) => {
  return isSentMessage(msg) ? 'You' : msg.senderName;
};

// Methods
const formatTime = (timestamp: number) => {
  const date = new Date(timestamp);
  return date.toLocaleTimeString('en-US', {
    hour: '2-digit',
    minute: '2-digit',
  });
};

const scrollToBottom = () => {
  nextTick(() => {
    if (messagesContainer.value) {
      messagesContainer.value.scrollTop = messagesContainer.value.scrollHeight;
    }
  });
};

const loadMessages = async () => {
  loading.value = true;
  error.value = null;
  try {
    const result = await api.getMessages(props.peer.ip, 50);
    messages.value = result;
    scrollToBottom();
  } catch (err) {
    error.value = err instanceof Error ? err.message : String(err);
    console.error('Failed to load messages:', err);
  } finally {
    loading.value = false;
  }
};

const sendMessage = async () => {
  const content = newMessage.value.trim();
  if (!content || sending.value) return;

  sending.value = true;
  try {
    await api.sendMessage(props.peer.ip, content);
    newMessage.value = '';
    // Message will be received via event, no need to reload
  } catch (err) {
    error.value = err instanceof Error ? err.message : String(err);
    console.error('Failed to send message:', err);
  } finally {
    sending.value = false;
  }
};

// Setup event listener for real-time messages
const setupEventListener = async () => {
  try {
    unlistenMessage = await listen<MessageDto>('message-received', (event) => {
      const msg = event.payload;
      // Only add message if it's from the current peer
      if (msg.senderIp === props.peer.ip || msg.receiverIp === props.peer.ip) {
        messages.value.push(msg);
        scrollToBottom();
      }
    });
  } catch (err) {
    console.error('Failed to setup event listener:', err);
  }
};

// Lifecycle
onMounted(async () => {
  await loadMessages();
  await setupEventListener();
});

onUnmounted(() => {
  // Clean up event listener
  if (unlistenMessage) {
    unlistenMessage();
    unlistenMessage = null;
  }
});

// Watch for peer changes
watch(() => props.peer.ip, async () => {
  // Clean up old listener
  if (unlistenMessage) {
    unlistenMessage();
    unlistenMessage = null;
  }
  // Load new messages and setup new listener
  await loadMessages();
  await setupEventListener();
});
</script>

<style scoped>
.chat-window {
  display: flex;
  flex-direction: column;
  width: 100%;
  height: 100%;
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: 8px;
  overflow: hidden;
}

.chat-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  background: var(--surface-alt);
  border-bottom: 1px solid var(--border);
}

.chat-peer-info {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.chat-peer-name {
  font-size: 16px;
  font-weight: 600;
  color: var(--text);
  margin: 0;
}

.chat-peer-ip {
  font-size: 12px;
  color: var(--text-secondary);
}

.chat-close-btn {
  background: none;
  border: none;
  font-size: 24px;
  color: var(--text-secondary);
  cursor: pointer;
  padding: 0;
  width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 4px;
  transition: background 0.2s;
}

.chat-close-btn:hover {
  background: var(--surface-hover);
}

.chat-messages {
  flex: 1;
  overflow-y: auto;
  padding: 16px;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.chat-loading,
.chat-error,
.chat-empty {
  text-align: center;
  padding: 20px;
  color: var(--text-secondary);
}

.chat-error {
  color: var(--error);
}

.chat-message-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.chat-message {
  display: flex;
  flex-direction: column;
  max-width: 70%;
  padding: 10px 14px;
  border-radius: 12px;
  gap: 4px;
}

.message-sent {
  align-self: flex-end;
  background: var(--primary);
  color: white;
}

.message-received {
  align-self: flex-start;
  background: var(--surface-alt);
  color: var(--text);
}

.message-sender {
  font-size: 12px;
  font-weight: 600;
  opacity: 0.9;
}

.message-content {
  word-wrap: break-word;
  white-space: pre-wrap;
}

.message-time {
  font-size: 11px;
  opacity: 0.7;
  align-self: flex-end;
}

.chat-input-area {
  display: flex;
  gap: 8px;
  padding: 12px 16px;
  background: var(--surface-alt);
  border-top: 1px solid var(--border);
}

.chat-input {
  flex: 1;
  padding: 10px 14px;
  border: 1px solid var(--border);
  border-radius: 6px;
  background: var(--surface);
  color: var(--text);
  font-size: 14px;
  resize: none;
  min-height: 40px;
  max-height: 120px;
}

.chat-input:focus {
  outline: none;
  border-color: var(--primary);
}

.chat-send-btn {
  padding: 10px 20px;
  background: var(--primary);
  color: white;
  border: none;
  border-radius: 6px;
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
  transition: opacity 0.2s;
}

.chat-send-btn:hover:not(:disabled) {
  opacity: 0.9;
}

.chat-send-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
</style>
