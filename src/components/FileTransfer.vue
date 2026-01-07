<template>
  <div class="file-transfer-panel">
    <div class="transfer-header">
      <h3 class="transfer-title">File Transfers</h3>
      <button class="refresh-btn" @click="loadTransfers" :disabled="loading" title="Refresh">
        <svg v-if="!loading" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M23 4v6h-6M1 20v-6h6"/>
          <path d="M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15"/>
        </svg>
        <svg v-else width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="spin">
          <path d="M21 12a9 9 0 1 1-6.219-8.56"/>
        </svg>
      </button>
    </div>

    <div class="transfer-list">
      <div v-if="loading" class="transfer-loading">Loading transfers...</div>
      <div v-else-if="error" class="transfer-error">{{ error }}</div>
      <div v-else-if="transfers.length === 0" class="transfer-empty">
        No file transfers in progress
      </div>
      <div v-else>
        <div
          v-for="transfer in sortedTransfers"
          :key="transfer.id"
          class="transfer-item"
          :class="getStatusClass(transfer.status)"
        >
          <!-- Transfer info -->
          <div class="transfer-info">
            <div class="transfer-header-row">
              <span class="transfer-filename">{{ transfer.fileName }}</span>
              <span class="transfer-status" :class="`status-${transfer.status}`">
                {{ formatStatus(transfer.status) }}
              </span>
            </div>
            <div class="transfer-meta">
              <span class="transfer-direction" :class="`direction-${transfer.direction}`">
                {{ transfer.direction === 'upload' ? '↑ Upload' : '↓ Download' }}
              </span>
              <span class="transfer-peer">{{ transfer.peerIp }}</span>
            </div>
          </div>

          <!-- Progress bar -->
          <div class="transfer-progress-section">
            <div class="progress-bar-container">
              <div class="progress-bar" :style="{ width: `${transfer.progress * 100}%` }"></div>
            </div>
            <div class="progress-text">
              <span class="bytes-text">{{ formatBytes(transfer.transferredBytes) }} / {{ formatBytes(transfer.fileSize) }}</span>
              <span class="percent-text">{{ Math.round(transfer.progress * 100) }}%</span>
            </div>
          </div>

          <!-- Speed and ETA (for active transfers) -->
          <div v-if="transfer.status === 'active'" class="transfer-stats">
            <span class="transfer-speed">
              <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z"/>
              </svg>
              {{ getTransferSpeed(transfer) }}
            </span>
            <span class="transfer-eta">
              <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <circle cx="12" cy="12" r="10"/>
                <path d="M12 6v6l4 2"/>
              </svg>
              {{ getETA(transfer) }}
            </span>
          </div>

          <!-- Actions -->
          <div class="transfer-actions">
            <button
              v-if="transfer.status === 'active' || transfer.status === 'pending'"
              class="action-btn cancel-btn"
              @click="handleCancel(transfer.id)"
              title="Cancel transfer"
            >
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M18 6L6 18M6 6l12 12"/>
              </svg>
            </button>
            <span v-else-if="transfer.error" class="error-message" :title="transfer.error">
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <circle cx="12" cy="12" r="10"/>
                <path d="M12 8v4M12 16h.01"/>
              </svg>
            </span>
            <span v-else-if="transfer.status === 'completed'" class="success-icon">
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M20 6L9 17l-5-5"/>
              </svg>
            </span>
          </div>
        </div>
      </div>
    </div>

    <!-- Incoming file transfer request dialog -->
    <div v-if="incomingRequest" class="transfer-request-overlay">
      <div class="transfer-request-dialog">
        <h3 class="request-title">Incoming File Transfer</h3>
        <div class="request-info">
          <p><strong>From:</strong> {{ incomingRequest.senderName }} ({{ incomingRequest.senderIp }})</p>
          <p><strong>File:</strong> {{ incomingRequest.fileName }}</p>
          <p><strong>Size:</strong> {{ formatBytes(incomingRequest.fileSize) }}</p>
          <p><strong>MD5:</strong> {{ incomingRequest.md5 }}</p>
        </div>
        <div class="request-actions">
          <button class="request-btn reject-btn" @click="handleRejectRequest">
            Reject
          </button>
          <button class="request-btn accept-btn" @click="handleAcceptRequest">
            Accept
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed } from 'vue';
import { listen } from '@tauri-apps/api/event';
import type { TaskDto, FileTransferRequestEvent } from '@/api';
import * as api from '@/api';

// State
const transfers = ref<TaskDto[]>([]);
const incomingRequest = ref<FileTransferRequestEvent | null>(null);
const loading = ref(false);
const error = ref<string | null>(null);

// Track speed for each transfer
const transferSpeeds = ref<Map<string, { bytes: number; time: number }[]>>(new Map());
const lastUpdateTimes = ref<Map<string, number>>(new Map());

// Event listener cleanup functions
const unlistenFns: Array<() => void> = [];

// Computed
const sortedTransfers = computed(() => {
  return [...transfers.value].sort((a, b) => {
    // Sort by status: active first, then pending, then others
    const statusOrder = { active: 0, pending: 1, paused: 2, completed: 3, failed: 4, cancelled: 5 };
    const aStatus = statusOrder[a.status as keyof typeof statusOrder] ?? 99;
    const bStatus = statusOrder[b.status as keyof typeof statusOrder] ?? 99;
    if (aStatus !== bStatus) return aStatus - bStatus;
    // Then by update time (newest first)
    return b.updatedAt - a.updatedAt;
  });
});

// Methods
const formatBytes = (bytes: number): string => {
  if (bytes === 0) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return `${(bytes / Math.pow(k, i)).toFixed(i === 0 ? 0 : 1)} ${sizes[i]}`;
};

const formatStatus = (status: string): string => {
  const statusMap: Record<string, string> = {
    pending: 'Pending',
    active: 'Transferring',
    paused: 'Paused',
    completed: 'Completed',
    failed: 'Failed',
    cancelled: 'Cancelled',
  };
  return statusMap[status] || status;
};

const getStatusClass = (status: string): string => {
  return `transfer-item-${status}`;
};

const getTransferSpeed = (transfer: TaskDto): string => {
  const history = transferSpeeds.value.get(transfer.id);
  if (!history || history.length < 2) return '...';

  const recent = history.slice(-3); // Use last 3 data points
  if (recent.length < 2) return '...';

  const totalTime = recent[recent.length - 1].time - recent[0].time;
  const totalBytes = recent[recent.length - 1].bytes - recent[0].bytes;

  if (totalTime <= 0 || totalBytes <= 0) return '...';

  const bytesPerSec = totalBytes / (totalTime / 1000);
  return `${formatBytes(bytesPerSec)}/s`;
};

const getETA = (transfer: TaskDto): string => {
  const history = transferSpeeds.value.get(transfer.id);
  if (!history || history.length < 2) return '--:--';

  const recent = history.slice(-3);
  const totalTime = recent[recent.length - 1].time - recent[0].time;
  const totalBytes = recent[recent.length - 1].bytes - recent[0].bytes;

  if (totalTime <= 0 || totalBytes <= 0) return '--:--';

  const bytesPerSec = totalBytes / (totalTime / 1000);
  const remainingBytes = transfer.fileSize - transfer.transferredBytes;

  if (bytesPerSec <= 0 || remainingBytes <= 0) return '--:--';

  const etaSeconds = remainingBytes / bytesPerSec;
  if (etaSeconds < 60) return `${Math.round(etaSeconds)}s`;
  if (etaSeconds < 3600) return `${Math.floor(etaSeconds / 60)}m ${Math.round(etaSeconds % 60)}s`;
  return `${Math.floor(etaSeconds / 3600)}h ${Math.floor((etaSeconds % 3600) / 60)}m`;
};

const loadTransfers = async () => {
  loading.value = true;
  error.value = null;
  try {
    const result = await api.get_file_transfers();
    transfers.value = result;

    // Initialize speed tracking for new transfers
    const now = Date.now();
    result.forEach(transfer => {
      if (!lastUpdateTimes.value.has(transfer.id)) {
        lastUpdateTimes.value.set(transfer.id, now);
        transferSpeeds.value.set(transfer.id, [{ bytes: transfer.transferredBytes, time: now }]);
      }
    });
  } catch (err) {
    error.value = err instanceof Error ? err.message : String(err);
    console.error('Failed to load transfers:', err);
  } finally {
    loading.value = false;
  }
};

const handleCancel = async (taskId: string) => {
  try {
    await api.cancel_file_transfer(taskId);
    await loadTransfers();
  } catch (err) {
    console.error('Failed to cancel transfer:', err);
    error.value = err instanceof Error ? err.message : String(err);
  }
};

const handleAcceptRequest = async () => {
  if (!incomingRequest.value) return;

  try {
    // Use a port in the configured range (8000-9000)
    const tcpPort = 8000 + Math.floor(Math.random() * 1000);
    await api.accept_file_transfer(incomingRequest.value.requestId, tcpPort);
    incomingRequest.value = null;
    await loadTransfers();
  } catch (err) {
    console.error('Failed to accept transfer:', err);
    error.value = err instanceof Error ? err.message : String(err);
  }
};

const handleRejectRequest = async () => {
  if (!incomingRequest.value) return;

  try {
    await api.reject_file_transfer(incomingRequest.value.requestId);
    incomingRequest.value = null;
  } catch (err) {
    console.error('Failed to reject transfer:', err);
    error.value = err instanceof Error ? err.message : String(err);
  }
};

// Setup event listeners
const setupEventListeners = async () => {
  try {
    // Listen for incoming file transfer requests
    const unlistenRequest = await listen<FileTransferRequestEvent>('file-transfer-request', (event) => {
      incomingRequest.value = event.payload;
    });
    unlistenFns.push(unlistenRequest);

    // Listen for transfer progress updates
    const unlistenProgress = await listen<TaskDto>('file-transfer-progress', (event) => {
      const updated = event.payload;

      // Update speed tracking
      const now = Date.now();
      const history = transferSpeeds.value.get(updated.id) || [];
      history.push({ bytes: updated.transferredBytes, time: now });
      // Keep only last 10 data points
      if (history.length > 10) {
        history.shift();
      }
      transferSpeeds.value.set(updated.id, history);

      // Update transfer in list
      const index = transfers.value.findIndex(t => t.id === updated.id);
      if (index >= 0) {
        transfers.value[index] = updated;
      } else {
        transfers.value.push(updated);
      }
    });
    unlistenFns.push(unlistenProgress);

    // Listen for transfer status changes
    const unlistenStatus = await listen<TaskDto>('file-transfer-status', (event) => {
      const updated = event.payload;
      const index = transfers.value.findIndex(t => t.id === updated.id);
      if (index >= 0) {
        transfers.value[index] = updated;
      }
    });
    unlistenFns.push(unlistenStatus);
  } catch (err) {
    console.error('Failed to setup event listeners:', err);
  }
};

// Lifecycle
onMounted(async () => {
  await loadTransfers();
  await setupEventListeners();

  // Poll for updates every 2 seconds
  const pollInterval = setInterval(loadTransfers, 2000);
  unlistenFns.push(() => clearInterval(pollInterval));
});

onUnmounted(() => {
  // Clean up event listeners
  unlistenFns.forEach(fn => fn());
});
</script>

<style scoped>
.file-transfer-panel {
  display: flex;
  flex-direction: column;
  width: 100%;
  height: 100%;
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: 8px;
  overflow: hidden;
}

.transfer-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  background: var(--surface-alt);
  border-bottom: 1px solid var(--border);
}

.transfer-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--text);
  margin: 0;
}

.refresh-btn {
  background: none;
  border: none;
  color: var(--text-secondary);
  cursor: pointer;
  padding: 4px;
  border-radius: 4px;
  transition: background 0.2s;
  display: flex;
  align-items: center;
  justify-content: center;
}

.refresh-btn:hover:not(:disabled) {
  background: var(--surface-hover);
}

.refresh-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.refresh-btn svg.spin {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

.transfer-list {
  flex: 1;
  overflow-y: auto;
  padding: 12px;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.transfer-loading,
.transfer-error,
.transfer-empty {
  text-align: center;
  padding: 20px;
  color: var(--text-secondary);
}

.transfer-error {
  color: var(--error);
}

.transfer-item {
  padding: 12px;
  background: var(--surface-alt);
  border: 1px solid var(--border);
  border-radius: 8px;
  display: flex;
  flex-direction: column;
  gap: 10px;
  transition: border-color 0.2s;
}

.transfer-item:hover {
  border-color: var(--border-hover);
}

.transfer-item-active {
  border-left: 3px solid var(--primary);
}

.transfer-item-completed {
  opacity: 0.7;
}

.transfer-item-failed,
.transfer-item-cancelled {
  border-left: 3px solid var(--error);
}

.transfer-info {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.transfer-header-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 8px;
}

.transfer-filename {
  font-weight: 600;
  color: var(--text);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.transfer-status {
  font-size: 12px;
  padding: 2px 8px;
  border-radius: 4px;
  font-weight: 500;
  text-transform: uppercase;
}

.status-pending {
  background: var(--warning-bg);
  color: var(--warning);
}

.status-active {
  background: var(--primary-bg);
  color: var(--primary);
}

.status-completed {
  background: var(--success-bg);
  color: var(--success);
}

.status-failed,
.status-cancelled {
  background: var(--error-bg);
  color: var(--error);
}

.transfer-meta {
  display: flex;
  gap: 12px;
  font-size: 12px;
  color: var(--text-secondary);
}

.transfer-direction {
  font-weight: 500;
}

.direction-upload {
  color: var(--warning);
}

.direction-download {
  color: var(--primary);
}

.transfer-progress-section {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.progress-bar-container {
  width: 100%;
  height: 6px;
  background: var(--surface);
  border-radius: 3px;
  overflow: hidden;
}

.progress-bar {
  height: 100%;
  background: linear-gradient(90deg, var(--primary), var(--primary-light));
  border-radius: 3px;
  transition: width 0.3s ease;
}

.progress-text {
  display: flex;
  justify-content: space-between;
  font-size: 12px;
  color: var(--text-secondary);
}

.transfer-stats {
  display: flex;
  gap: 16px;
  font-size: 12px;
  color: var(--text-secondary);
}

.transfer-stats span {
  display: flex;
  align-items: center;
  gap: 4px;
}

.transfer-stats svg {
  flex-shrink: 0;
}

.transfer-actions {
  display: flex;
  gap: 8px;
  justify-content: flex-end;
}

.action-btn {
  background: var(--surface);
  border: 1px solid var(--border);
  color: var(--text);
  padding: 6px 12px;
  border-radius: 6px;
  cursor: pointer;
  font-size: 13px;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  justify-content: center;
}

.cancel-btn:hover {
  background: var(--error-bg);
  border-color: var(--error);
  color: var(--error);
}

.error-message {
  color: var(--error);
  display: flex;
  align-items: center;
}

.success-icon {
  color: var(--success);
  display: flex;
  align-items: center;
}

/* Transfer request dialog */
.transfer-request-overlay {
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

.transfer-request-dialog {
  background: var(--surface);
  border-radius: 12px;
  padding: 20px;
  max-width: 400px;
  width: 90%;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2);
}

.request-title {
  font-size: 18px;
  font-weight: 600;
  color: var(--text);
  margin: 0 0 16px 0;
}

.request-info {
  display: flex;
  flex-direction: column;
  gap: 8px;
  margin-bottom: 20px;
  color: var(--text);
}

.request-info p {
  margin: 0;
  display: flex;
  gap: 8px;
}

.request-info strong {
  min-width: 60px;
}

.request-actions {
  display: flex;
  gap: 12px;
  justify-content: flex-end;
}

.request-btn {
  padding: 10px 20px;
  border-radius: 6px;
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
  transition: opacity 0.2s;
  border: none;
}

.reject-btn {
  background: var(--surface-alt);
  color: var(--text);
  border: 1px solid var(--border);
}

.reject-btn:hover {
  opacity: 0.8;
}

.accept-btn {
  background: var(--primary);
  color: white;
}

.accept-btn:hover {
  opacity: 0.9;
}
</style>
