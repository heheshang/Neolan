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
      <div
        v-show="!loading && messages.length > 0"
        class="message-list"
      >
        <div
          v-for="msg in messages"
          :key="msg.msgId || msg.id || String(msg.sentAt)"
          class="message-item"
          :class="isSentMessage(msg) ? 'message-sent' : 'message-received'"
        >
          <div class="message-bubble">
            <div class="message-sender" v-if="!isSentMessage(msg)">
              {{ msg.senderName }}
            </div>
            <div class="message-content">{{ msg.content }}</div>
            <div class="message-time">{{ formatMessageTime(msg.sentAt) }}</div>
            <div v-if="msg.delivered" class="message-delivered">✓ 已送达</div>
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
let unlistenReceiptAck: (() => void) | null = null;

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

    // Don't overwrite if we already have messages (prevent clearing real-time messages)
    if (result.length > 0 && messages.value.length === 0) {
      messages.value = result;
    }

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
  showTransfer.value = false;
  selectedFile.value = null;
  if (fileInput.value) {
    fileInput.value.value = '';
  }
}

async function setupEventListener() {
  try {
    unlistenMessage = await listen<any>('message-received', (event) => {
      // Tauri sends the enum variant, so we need to extract the actual message data
      const payload = event.payload;
      const messageData = payload.MessageReceived || payload;

      // Convert to MessageDto format
      const msg: MessageDto = {
        id: messageData.id || 0,
        msgId: messageData.msgId,
        senderIp: messageData.senderIp,
        senderName: messageData.senderName,
        receiverIp: messageData.receiverIp,
        msgType: messageData.msgType,
        content: messageData.content,
        isEncrypted: messageData.isEncrypted,
        isOffline: messageData.isOffline,
        sentAt: messageData.sentAt,
        receivedAt: messageData.receivedAt,
        createdAt: messageData.createdAt,
        delivered: false  // Initialize as not delivered
      };

      messages.value.push(msg);

      // Force Vue reactivity update
      messages.value = [...messages.value];

      scrollToBottom();
    });

    // Listen for message receipt acknowledgments
    unlistenReceiptAck = await listen<any>('message-receipt-ack', (event) => {
      const payload = event.payload;
      const ackData = payload.MessageReceiptAck || payload;

      // Find the message by msgId and mark as delivered
      const msgIndex = messages.value.findIndex(m => m.msgId === ackData.msgId);
      if (msgIndex !== -1) {
        messages.value[msgIndex].delivered = true;
        // Force reactivity
        messages.value = [...messages.value];
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
  if (unlistenReceiptAck) {
    unlistenReceiptAck();
  }
});

watch(() => props.peer.ip, async () => {
  if (unlistenMessage) {
    unlistenMessage();
  }
  if (unlistenReceiptAck) {
    unlistenReceiptAck();
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
  background: var(--color-bg-primary);
}

/* ==================== Header ==================== */
.window-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--spacing-3) var(--spacing-4);
  background: var(--color-bg-secondary);
  border-bottom: 1px solid var(--color-border-subtle);
}

.header-info {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.peer-name {
  font-size: var(--font-size-md);
  font-weight: var(--font-weight-medium);
  color: var(--color-text-primary);
}

.peer-status {
  font-size: var(--font-size-sm);
  color: var(--color-success);
  font-weight: var(--font-weight-medium);
}

.header-actions {
  display: flex;
  gap: var(--spacing-2);
}

.action-btn {
  width: 36px;
  height: 36px;
  border: none;
  background: transparent;
  border-radius: var(--radius-md);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--color-text-secondary);
  transition: all var(--transition-fast);
}

.action-btn:hover {
  background: var(--color-bg-tertiary);
  color: var(--color-text-primary);
}

.action-btn svg {
  width: 20px;
  height: 20px;
}

/* ==================== Messages ==================== */
.window-messages {
  flex: 1;
  overflow-y: auto;
  padding: var(--spacing-4);
  display: flex;
  flex-direction: column;
  gap: var(--spacing-3);
}

.loading-state,
.empty-state {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: var(--color-text-tertiary);
  font-size: var(--font-size-base);
}

.message-list {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-3);
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
  padding: var(--spacing-2) var(--spacing-3);
  border-radius: var(--radius-lg);
  position: relative;
  box-shadow: var(--shadow-sm);
  transition: transform var(--transition-fast);
}

.message-item:hover .message-bubble {
  transform: scale(1.01);
}

.message-item.message-sent .message-bubble {
  background: var(--color-success);
  color: var(--color-dark-text-primary);
}

.message-item.message-received .message-bubble {
  background: var(--color-bg-elevated);
  color: var(--color-text-primary);
  border: 1px solid var(--color-border-subtle);
}

.message-sender {
  font-size: var(--font-size-sm);
  color: var(--color-primary);
  margin-bottom: var(--spacing-1);
  font-weight: var(--font-weight-medium);
}

.message-content {
  word-wrap: break-word;
  white-space: pre-wrap;
  line-height: var(--line-height-normal);
  font-size: var(--font-size-base);
}

.message-time {
  font-size: var(--font-size-xs);
  color: var(--color-text-tertiary);
  margin-top: var(--spacing-1);
  text-align: right;
}

.message-delivered {
  font-size: var(--font-size-xs);
  color: var(--color-success);
  margin-top: 2px;
  text-align: right;
  font-weight: var(--font-weight-medium);
}

/* ==================== Input Area ==================== */
.window-input {
  background: var(--color-bg-secondary);
  border-top: 1px solid var(--color-border-subtle);
  padding: var(--spacing-3) var(--spacing-4);
  display: flex;
  flex-direction: column;
  gap: var(--spacing-2);
}

.input-toolbar {
  display: flex;
  gap: var(--spacing-3);
}

.toolbar-btn {
  width: 32px;
  height: 32px;
  border: none;
  background: transparent;
  border-radius: var(--radius-md);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--color-text-secondary);
  transition: all var(--transition-fast);
}

.toolbar-btn:hover {
  background: var(--color-bg-tertiary);
  color: var(--color-text-primary);
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
  font-size: var(--font-size-base);
  font-family: inherit;
  line-height: var(--line-height-normal);
  color: var(--color-text-primary);
  background: transparent;
  min-height: 20px;
  max-height: 120px;
}

.input-textarea::placeholder {
  color: var(--color-text-tertiary);
}

.input-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.input-hint {
  font-size: var(--font-size-sm);
  color: var(--color-text-tertiary);
}

.send-btn {
  padding: var(--spacing-1) var(--spacing-5);
  background: var(--color-bg-tertiary);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  font-size: var(--font-size-base);
  color: var(--color-primary);
  cursor: pointer;
  transition: all var(--transition-fast);
  font-weight: var(--font-weight-medium);
}

.send-btn:hover:not(:disabled) {
  background: var(--color-primary);
  border-color: var(--color-primary);
  color: var(--color-text-inverse);
  box-shadow: var(--shadow-primary);
  transform: translateY(-1px);
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
  background: var(--color-bg-overlay);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: var(--z-modal);
  backdrop-filter: blur(4px);
}

.transfer-content {
  background: var(--color-bg-elevated);
  border-radius: var(--radius-xl);
  padding: var(--spacing-6);
  width: 90%;
  max-width: 400px;
  box-shadow: var(--shadow-2xl);
  border: 1px solid var(--color-border-subtle);
}

.transfer-content h3 {
  margin: 0 0 var(--spacing-2) 0;
  font-size: var(--font-size-xl);
  color: var(--color-text-primary);
  font-weight: var(--font-weight-semibold);
}

.transfer-peer {
  font-size: var(--font-size-base);
  color: var(--color-text-secondary);
  margin: 0 0 var(--spacing-5) 0;
}

.transfer-input {
  width: 100%;
  padding: var(--spacing-2) var(--spacing-3);
  background: var(--color-bg-tertiary);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  margin-bottom: var(--spacing-5);
  font-size: var(--font-size-base);
  color: var(--color-text-primary);
  font-family: var(--font-sans);
  transition: all var(--transition-fast);
}

.transfer-input:focus {
  outline: none;
  border-color: var(--color-primary);
  box-shadow: 0 0 0 3px var(--color-primary-subtle);
}

.transfer-actions {
  display: flex;
  gap: var(--spacing-3);
  justify-content: flex-end;
}

.transfer-btn {
  padding: var(--spacing-2) var(--spacing-5);
  border: none;
  border-radius: var(--radius-md);
  font-size: var(--font-size-base);
  cursor: pointer;
  transition: all var(--transition-fast);
  font-weight: var(--font-weight-medium);
}

.transfer-btn.cancel {
  background: var(--color-bg-tertiary);
  color: var(--color-text-secondary);
  border: 1px solid var(--color-border);
}

.transfer-btn.cancel:hover {
  background: var(--color-bg-elevated);
  border-color: var(--color-border-strong);
}

.transfer-btn.confirm {
  background: var(--color-primary);
  color: var(--color-text-inverse);
}

.transfer-btn.confirm:hover:not(:disabled) {
  background: var(--color-primary-dark);
  box-shadow: var(--shadow-primary);
  transform: translateY(-1px);
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
  background: var(--color-border);
  border-radius: 3px;
}

.window-messages::-webkit-scrollbar-thumb:hover {
  background: var(--color-border-strong);
}
</style>
