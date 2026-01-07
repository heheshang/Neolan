<template>
  <div class="wechat-window">
    <!-- Header -->
    <header class="window-header">
      <div class="header-info">
        <span class="peer-name">{{ peer.displayName || peer.ip }}</span>
        <span v-if="peer.status === 'online'" class="peer-status">在线</span>
      </div>
      <div class="header-actions">
        <button class="action-btn" @click="showTransfer = true" title="发送文件">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M21.44 11.05l-9.19 9.19a6 6 0 0 1-8.49-8.49l9.19-9.19a4 4 0 0 1 5.66 5.66l-9.2 9.19a2 2 0 0 1-2.83-2.83l8.49-8.48"/>
          </svg>
        </button>
        <button class="action-btn" @click="$emit('close')" title="关闭">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M18 6L6 18M6 6l12 12"/>
          </svg>
        </button>
      </div>
    </header>

    <!-- Messages -->
    <div class="window-messages" ref="messagesContainer">
      <div v-if="loading" class="loading-state">加载中...</div>
      <div v-else-if="messages.length === 0" class="empty-state">
        <p>暂无消息，开始聊天吧</p>
      </div>
      <div v-else class="message-list">
        <div
          v-for="msg in messages"
          :key="msg.id"
          class="message-item"
          :class="isSentMessage(msg) ? 'message-sent' : 'message-received'"
        >
          <div class="message-bubble">
            <div class="message-sender" v-if="!isSentMessage(msg)">
              {{ msg.senderName }}
            </div>
            <div class="message-content">{{ msg.content }}</div>
            <div class="message-time">{{ formatMessageTime(msg.sentAt) }}</div>
          </div>
        </div>
      </div>
    </div>

    <!-- Input Area -->
    <div class="window-input">
      <div class="input-toolbar">
        <button class="toolbar-btn" @click="showTransfer = true" title="发送文件">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M21.44 11.05l-9.19 9.19a6 6 0 0 1-8.49-8.49l9.19-9.19a4 4 0 0 1 5.66 5.66l-9.2 9.19a2 2 0 0 1-2.83-2.83l8.49-8.48"/>
          </svg>
        </button>
        <button class="toolbar-btn" title="表情">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="12" cy="12" r="10"/>
            <path d="M8 14s1.5 2 4 2 4-2 4-2"/>
            <line x1="9" y1="9" x2="9.01" y2="9"/>
            <line x1="15" y1="9" x2="15.01" y2="9"/>
          </svg>
        </button>
      </div>
      <textarea
        v-model="newMessage"
        class="input-textarea"
        placeholder="请输入消息..."
        rows="1"
        @keydown.enter.exact.prevent="sendMessage"
        @keydown.enter.shift.exact="newMessage += '\n'"
        ref="textareaRef"
      />
      <div class="input-footer">
        <span class="input-hint">按 Enter 发送，Shift + Enter 换行</span>
        <button
          class="send-btn"
          @click="sendMessage"
          :disabled="!newMessage.trim() || sending"
        >
          发送
        </button>
      </div>
    </div>

    <!-- File Transfer Modal -->
    <div v-if="showTransfer" class="transfer-modal" @click.self="showTransfer = false">
      <div class="transfer-content">
        <h3>发送文件</h3>
        <p class="transfer-peer">发送给: {{ peer.displayName || peer.ip }}</p>
        <input
          type="file"
          ref="fileInput"
          @change="handleFileSelect"
          class="transfer-input"
        />
        <div class="transfer-actions">
          <button class="transfer-btn cancel" @click="showTransfer = false">取消</button>
          <button class="transfer-btn confirm" @click="sendFile" :disabled="!selectedFile">
            发送
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, nextTick, watch } from 'vue';
import { listen } from '@tauri-apps/api/event';
import type { PeerDto, MessageDto } from '@/stores';
import * as api from '@/api';

interface Props {
  peer: PeerDto;
}

const props = defineProps<Props>();
defineEmits<{
  (e: 'close'): void;
}>();

const messages = ref<MessageDto[]>([]);
const newMessage = ref('');
const loading = ref(false);
const sending = ref(false);
const messagesContainer = ref<HTMLElement | null>(null);
const textareaRef = ref<HTMLTextAreaElement | null>(null);
const showTransfer = ref(false);
const selectedFile = ref<File | null>(null);
const fileInput = ref<HTMLInputElement | null>(null);

let unlistenMessage: (() => void) | null = null;

// Check if message was sent by us (not received from peer)
// If receiverIp equals peer.ip, it means we sent this message to the peer
const isSentMessage = (msg: MessageDto) => msg.receiverIp === props.peer.ip;

function formatMessageTime(timestamp: number): string {
  const date = new Date(timestamp);
  return date.toLocaleTimeString('zh-CN', { hour: '2-digit', minute: '2-digit' });
}

async function loadMessages() {
  loading.value = true;
  try {
    const result = await api.getMessages(props.peer.ip, 50);
    messages.value = result;
    scrollToBottom();
  } catch (err) {
    console.error('Failed to load messages:', err);
  } finally {
    loading.value = false;
  }
}

async function sendMessage() {
  const content = newMessage.value.trim();
  if (!content || sending.value) return;

  sending.value = true;
  try {
    await api.sendMessage(props.peer.ip, content);
    newMessage.value = '';
    if (textareaRef.value) {
      adjustTextareaHeight(textareaRef.value);
    }
  } catch (err) {
    console.error('Failed to send message:', err);
  } finally {
    sending.value = false;
  }
}

function scrollToBottom() {
  nextTick(() => {
    if (messagesContainer.value) {
      messagesContainer.value.scrollTop = messagesContainer.value.scrollHeight;
    }
  });
}

function adjustTextareaHeight(textarea: HTMLTextAreaElement) {
  textarea.style.height = 'auto';
  textarea.style.height = Math.min(textarea.scrollHeight, 120) + 'px';
}

async function handleFileSelect(event: Event) {
  const target = event.target as HTMLInputElement;
  if (target.files && target.files[0]) {
    selectedFile.value = target.files[0];
  }
}

async function sendFile() {
  if (!selectedFile.value) return;
  // TODO: Implement file sending logic
  console.log('Sending file:', selectedFile.value.name);
  showTransfer.value = false;
  selectedFile.value = null;
  if (fileInput.value) {
    fileInput.value.value = '';
  }
}

async function setupEventListener() {
  try {
    unlistenMessage = await listen<MessageDto>('message-received', (event) => {
      const msg = event.payload;
      if (msg.senderIp === props.peer.ip || msg.receiverIp === props.peer.ip) {
        messages.value.push(msg);
        scrollToBottom();
      }
    });
  } catch (err) {
    console.error('Failed to setup event listener:', err);
  }
}

onMounted(async () => {
  await loadMessages();
  await setupEventListener();

  if (textareaRef.value) {
    textareaRef.value.addEventListener('input', function() {
      adjustTextareaHeight(this);
    });
  }
});

onUnmounted(() => {
  if (unlistenMessage) {
    unlistenMessage();
  }
});

watch(() => props.peer.ip, async () => {
  if (unlistenMessage) {
    unlistenMessage();
  }
  await loadMessages();
  await setupEventListener();
});
</script>

<style scoped>
.wechat-window {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: #f5f5f5;
}

/* ==================== Header ==================== */
.window-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  background: #fff;
  border-bottom: 1px solid #e7e7e7;
}

.header-info {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.peer-name {
  font-size: 16px;
  font-weight: 500;
  color: #333;
}

.peer-status {
  font-size: 12px;
  color: #07C160;
}

.header-actions {
  display: flex;
  gap: 8px;
}

.action-btn {
  width: 36px;
  height: 36px;
  border: none;
  background: transparent;
  border-radius: 6px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #666;
  transition: all 0.2s;
}

.action-btn:hover {
  background: #f5f5f5;
  color: #333;
}

.action-btn svg {
  width: 20px;
  height: 20px;
}

/* ==================== Messages ==================== */
.window-messages {
  flex: 1;
  overflow-y: auto;
  padding: 16px;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.loading-state,
.empty-state {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: #999;
  font-size: 14px;
}

.message-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.message-item {
  display: flex;
}

.message-item.message-sent {
  justify-content: flex-end;
}

.message-item.message-received {
  justify-content: flex-start;
}

.message-bubble {
  max-width: 70%;
  padding: 10px 14px;
  border-radius: 8px;
  position: relative;
}

.message-item.message-sent .message-bubble {
  background: #95ec69;
  color: #333;
}

.message-item.message-received .message-bubble {
  background: #fff;
  color: #333;
}

.message-sender {
  font-size: 12px;
  color: #07C160;
  margin-bottom: 4px;
  font-weight: 500;
}

.message-content {
  word-wrap: break-word;
  white-space: pre-wrap;
  line-height: 1.5;
  font-size: 15px;
}

.message-time {
  font-size: 11px;
  color: #999;
  margin-top: 4px;
  text-align: right;
}

/* ==================== Input Area ==================== */
.window-input {
  background: #fff;
  border-top: 1px solid #e7e7e7;
  padding: 12px 16px;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.input-toolbar {
  display: flex;
  gap: 12px;
}

.toolbar-btn {
  width: 32px;
  height: 32px;
  border: none;
  background: transparent;
  border-radius: 4px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #666;
  transition: all 0.2s;
}

.toolbar-btn:hover {
  background: #f5f5f5;
  color: #333;
}

.toolbar-btn svg {
  width: 20px;
  height: 20px;
}

.input-textarea {
  width: 100%;
  border: none;
  outline: none;
  resize: none;
  font-size: 15px;
  font-family: inherit;
  line-height: 1.5;
  color: #333;
  background: transparent;
  min-height: 20px;
  max-height: 120px;
}

.input-textarea::placeholder {
  color: #ccc;
}

.input-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.input-hint {
  font-size: 12px;
  color: #999;
}

.send-btn {
  padding: 6px 20px;
  background: #e9e9e9;
  border: none;
  border-radius: 4px;
  font-size: 14px;
  color: #07C160;
  cursor: pointer;
  transition: all 0.2s;
}

.send-btn:hover:not(:disabled) {
  background: #07C160;
  color: #fff;
}

.send-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

/* ==================== Transfer Modal ==================== */
.transfer-modal {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.transfer-content {
  background: #fff;
  border-radius: 8px;
  padding: 24px;
  width: 90%;
  max-width: 400px;
}

.transfer-content h3 {
  margin: 0 0 8px 0;
  font-size: 18px;
  color: #333;
}

.transfer-peer {
  font-size: 14px;
  color: #666;
  margin: 0 0 20px 0;
}

.transfer-input {
  width: 100%;
  padding: 10px;
  border: 1px solid #e7e7e7;
  border-radius: 6px;
  margin-bottom: 20px;
  font-size: 14px;
}

.transfer-actions {
  display: flex;
  gap: 12px;
  justify-content: flex-end;
}

.transfer-btn {
  padding: 8px 20px;
  border: none;
  border-radius: 6px;
  font-size: 14px;
  cursor: pointer;
  transition: all 0.2s;
}

.transfer-btn.cancel {
  background: #f5f5f5;
  color: #666;
}

.transfer-btn.cancel:hover {
  background: #e7e7e7;
}

.transfer-btn.confirm {
  background: #07C160;
  color: #fff;
}

.transfer-btn.confirm:hover:not(:disabled) {
  background: #06ae56;
}

.transfer-btn.confirm:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

/* ==================== Scrollbar ==================== */
.window-messages::-webkit-scrollbar {
  width: 6px;
}

.window-messages::-webkit-scrollbar-track {
  background: transparent;
}

.window-messages::-webkit-scrollbar-thumb {
  background: #ccc;
  border-radius: 3px;
}

.window-messages::-webkit-scrollbar-thumb:hover {
  background: #999;
}

/* ==================== Dark Mode ==================== */
@media (prefers-color-scheme: dark) {
  .wechat-window {
    background: #1a1a1a;
  }

  .window-header {
    background: #2a2a2a;
    border-bottom-color: #3a3a3a;
  }

  .peer-name {
    color: #e0e0e0;
  }

  .action-btn:hover {
    background: #3a3a3a;
    color: #e0e0e0;
  }

  .message-item.message-received .message-bubble {
    background: #3a3a3a;
    color: #e0e0e0;
  }

  .message-item.message-sent .message-bubble {
    background: #3a5a3a;
    color: #e0e0e0;
  }

  .window-input {
    background: #2a2a2a;
    border-top-color: #3a3a3a;
  }

  .input-textarea {
    color: #e0e0e0;
  }

  .input-textarea::placeholder {
    color: #666;
  }

  .send-btn {
    background: #3a3a3a;
  }

  .transfer-content {
    background: #2a2a2a;
  }

  .transfer-content h3 {
    color: #e0e0e0;
  }

  .transfer-input {
    background: #3a3a3a;
    border-color: #4a4a4a;
    color: #e0e0e0;
  }

  .transfer-btn.cancel {
    background: #3a3a3a;
    color: #888;
  }

  .transfer-btn.cancel:hover {
    background: #4a4a4a;
  }
}
</style>
