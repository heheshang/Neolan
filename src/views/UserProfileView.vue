<template>
  <div class="user-profile-container">
    <!-- Scanline overlay -->
    <div class="scanline-overlay"></div>

    <!-- Header -->
    <header class="profile-header">
      <div class="header-content">
        <h1 class="page-title">
          <span class="title-bracket">[</span>
          <span class="title-text">USER PROFILE</span>
          <span class="title-bracket">]</span>
        </h1>
        <p class="page-subtitle">// CURRENT USER INFORMATION</p>
      </div>
    </header>

    <!-- Main Content -->
    <main class="profile-main">
      <!-- Loading state -->
      <div v-if="configStore.loading && !localConfig" class="loading-state">
        <div class="loading-spinner"></div>
        <p class="loading-text">LOADING USER PROFILE...</p>
      </div>

      <!-- Profile Content -->
      <div v-else class="profile-content">
        <!-- Edit Mode: ProfileEditCard -->
        <ProfileEditCard
          v-if="isEditing"
          :username="config.username || ''"
          :hostname="config.hostname || ''"
          :status="userStatus"
          :avatar="userAvatar"
          @save="handleSaveProfile"
          @cancel="handleCancelEdit"
        />

        <!-- Display Mode: Profile Cards -->
        <template v-else>
          <!-- User Identity Card -->
          <section class="profile-card">
            <div class="card-header">
              <span class="card-icon">◆</span>
              <h2 class="card-title">IDENTITY</h2>
              <span class="card-line"></span>
            </div>
            <div class="card-body">
              <div class="identity-display">
                <div class="identity-avatar">
                  <span class="avatar-large">{{ displayAvatar }}</span>
                </div>
                <div class="identity-info">
                  <div class="identity-name">{{ displayName }}</div>
                  <div class="identity-host">@{{ hostname }}</div>
                  <div v-if="userStatus" class="identity-status">{{ userStatus }}</div>
                </div>
              </div>
            </div>
          </section>

        <!-- Network Configuration -->
        <section class="profile-card">
          <div class="card-header">
            <span class="card-icon">◈</span>
            <h2 class="card-title">NETWORK CONFIGURATION</h2>
            <span class="card-line"></span>
          </div>
          <div class="card-body">
            <div class="info-grid">
              <div class="info-item">
                <span class="info-label">BIND ADDRESS</span>
                <span class="info-value">{{ config.bindIp }}</span>
              </div>
              <div class="info-item">
                <span class="info-label">UDP PORT</span>
                <span class="info-value">{{ config.udpPort }}</span>
              </div>
              <div class="info-item">
                <span class="info-label">TCP PORT RANGE</span>
                <span class="info-value">{{ config.tcpPortStart }} – {{ config.tcpPortEnd }}</span>
              </div>
              <div class="info-item">
                <span class="info-label">HEARTBEAT INTERVAL</span>
                <span class="info-value">{{ config.heartbeatInterval }} sec</span>
              </div>
              <div class="info-item">
                <span class="info-label">PEER TIMEOUT</span>
                <span class="info-value">{{ config.peerTimeout }} sec</span>
              </div>
            </div>
          </div>
        </section>

        <!-- System Information -->
        <section class="profile-card">
          <div class="card-header">
            <span class="card-icon">⚙</span>
            <h2 class="card-title">SYSTEM PARAMETERS</h2>
            <span class="card-line"></span>
          </div>
          <div class="card-body">
            <div class="info-grid">
              <div class="info-item">
                <span class="info-label">LOG LEVEL</span>
                <span class="info-value badge">{{ config.logLevel.toUpperCase() }}</span>
              </div>
              <div class="info-item">
                <span class="info-label">ENCRYPTION</span>
                <span class="info-value" :class="config.encryptionEnabled ? 'enabled' : 'disabled'">
                  {{ config.encryptionEnabled ? 'ENABLED' : 'DISABLED' }}
                </span>
              </div>
              <div class="info-item">
                <span class="info-label">OFFLINE MESSAGE RETENTION</span>
                <span class="info-value">{{ config.offlineMessageRetentionDays }} days</span>
              </div>
              <div class="info-item">
                <span class="info-label">AUTO ACCEPT FILES</span>
                <span class="info-value" :class="config.autoAcceptFiles ? 'enabled' : 'disabled'">
                  {{ config.autoAcceptFiles ? 'ENABLED' : 'DISABLED' }}
                </span>
              </div>
            </div>
          </div>
        </section>

        <!-- Action Buttons -->
        <div class="action-bar">
          <button @click="startEditing" class="cyber-btn edit-btn">
            <span class="btn-icon">✎</span>
            <span class="btn-text">EDIT PROFILE</span>
          </button>
          <button @click="goToSettings" class="cyber-btn settings-btn">
            <span class="btn-icon">⚙</span>
            <span class="btn-text">ADVANCED SETTINGS</span>
          </button>
        </div>
        </template>
      </div>
    </main>

    <!-- Toast Notification -->
    <Transition name="toast">
      <div v-if="toast.show" class="toast-notification" :class="toast.type">
        <span class="toast-icon">{{ toast.type === 'success' ? '✓' : '⚠' }}</span>
        <span class="toast-message">{{ toast.message }}</span>
      </div>
    </Transition>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { useConfigStore } from '@/stores';
import ProfileEditCard from '@/components/ProfileEditCard.vue';
import type { ConfigDto } from '@/api';

const router = useRouter();
const configStore = useConfigStore();

// State
const localConfig = ref<ConfigDto | null>(null);
const isEditing = ref(false);
const toast = reactive({
  show: false,
  message: '',
  type: 'success' as 'success' | 'error',
});

let toastTimer: ReturnType<typeof setTimeout> | null = null;

// Computed
const config = computed(() => {
  return localConfig.value || configStore.currentConfig;
});

const displayName = computed(() => {
  return config.value?.username || 'User';
});

const hostname = computed(() => {
  return config.value?.hostname || 'localhost';
});

const userAvatar = computed(() => {
  return config.value?.avatar || null;
});

const userStatus = computed(() => {
  return config.value?.status || '';
});

const displayAvatar = computed(() => {
  return userAvatar.value || '👤';
});

// Actions
function startEditing() {
  isEditing.value = true;
}

function goToSettings() {
  router.push({ name: 'Settings', query: { tab: 'identity' } });
}

async function handleSaveProfile(data: {
  username: string;
  hostname: string;
  status: string;
  avatar: string | null;
}) {
  try {
    // Update config with new values
    const updatedConfig = {
      ...config.value,
      username: data.username,
      hostname: data.hostname,
      status: data.status,
      avatar: data.avatar,
    };

    const success = await configStore.saveConfig(updatedConfig);
    if (success) {
      localConfig.value = updatedConfig as ConfigDto;
      isEditing.value = false;
      showToast('Profile saved successfully', 'success');
    } else {
      showToast(configStore.error || 'Failed to save profile', 'error');
    }
  } catch (error) {
    showToast('An error occurred while saving', 'error');
  }
}

function handleCancelEdit() {
  isEditing.value = false;
}

function showToast(message: string, type: 'success' | 'error') {
  toast.message = message;
  toast.type = type;
  toast.show = true;

  if (toastTimer) clearTimeout(toastTimer);
  toastTimer = setTimeout(() => {
    toast.show = false;
  }, 3000);
}

// Initialize
onMounted(async () => {
  const loadedConfig = await configStore.fetchConfig();
  if (loadedConfig) {
    localConfig.value = loadedConfig;
  }
});
</script>

<style scoped>
.user-profile-container {
  --neon-cyan: #00f3ff;
  --neon-magenta: #ff00ff;
  --neon-green: #00ff88;
  --neon-red: #ff3366;
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
}

/* Scanline Overlay */
.scanline-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  pointer-events: none;
  z-index: 100;
  background: repeating-linear-gradient(
    0deg,
    rgba(0, 0, 0, 0.1) 0px,
    rgba(0, 0, 0, 0.1) 1px,
    transparent 1px,
    transparent 2px
  );
  opacity: 0.3;
}

/* Header */
.profile-header {
  position: relative;
  z-index: 1;
  background: var(--bg-panel);
  backdrop-filter: blur(20px);
  border-bottom: 1px solid var(--border-dim);
  padding: 24px 32px;
}

.header-content {
  max-width: 900px;
  margin: 0 auto;
}

.page-title {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 12px;
  font-size: 24px;
  font-weight: 700;
  color: var(--neon-cyan);
  text-shadow: 0 0 20px var(--neon-cyan), 0 0 40px rgba(0, 243, 255, 0.5);
  margin: 0 0 8px 0;
  letter-spacing: 3px;
}

.title-bracket {
  animation: title-pulse 2s ease-in-out infinite alternate;
}

@keyframes title-pulse {
  0% { opacity: 0.5; }
  100% { opacity: 1; }
}

.page-subtitle {
  text-align: center;
  font-size: 10px;
  color: var(--text-secondary);
  letter-spacing: 2px;
  margin: 0;
}

/* Main Content */
.profile-main {
  position: relative;
  z-index: 1;
  padding: 32px;
  max-width: 900px;
  margin: 0 auto;
}

.loading-state {
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

/* Profile Content */
.profile-content {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

/* Profile Card */
.profile-card {
  background: rgba(0, 0, 0, 0.4);
  border: 1px solid var(--border-dim);
  border-radius: 8px;
  overflow: hidden;
  position: relative;
}

.profile-card::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 1px;
  background: linear-gradient(90deg, transparent, var(--neon-cyan), transparent);
  opacity: 0.5;
}

.card-header {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 16px 20px;
  background: rgba(0, 243, 255, 0.05);
  border-bottom: 1px solid var(--border-dim);
}

.card-icon {
  font-size: 16px;
  color: var(--neon-cyan);
  text-shadow: 0 0 10px var(--neon-cyan);
}

.card-title {
  font-size: 14px;
  font-weight: 700;
  color: var(--neon-cyan);
  letter-spacing: 2px;
  margin: 0;
}

.card-line {
  flex: 1;
  height: 1px;
  background: linear-gradient(90deg, var(--border-dim), transparent);
}

.card-body {
  padding: 20px;
}

/* Identity Display */
.identity-display {
  display: flex;
  align-items: center;
  gap: 20px;
  padding: 20px;
}

.identity-avatar {
  width: 80px;
  height: 80px;
  background: linear-gradient(135deg, rgba(0, 243, 255, 0.1), rgba(255, 0, 255, 0.1));
  border: 2px solid var(--border-dim);
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.avatar-large {
  font-size: 40px;
  opacity: 0.8;
}

.identity-info {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.identity-name {
  font-size: 24px;
  font-weight: 700;
  color: var(--text-primary);
  letter-spacing: 1px;
}

.identity-host {
  font-size: 14px;
  color: var(--text-secondary);
  font-family: 'Courier New', monospace;
}

.identity-status {
  font-size: 12px;
  color: var(--text-muted);
  font-style: italic;
  max-width: 400px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

/* Info Grid */
.info-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 16px;
}

.info-item {
  display: flex;
  flex-direction: column;
  gap: 6px;
  padding: 12px;
  background: rgba(0, 0, 0, 0.3);
  border: 1px solid var(--border-dim);
  border-radius: 4px;
}

.info-label {
  font-size: 10px;
  font-weight: 600;
  color: var(--text-secondary);
  letter-spacing: 1px;
  text-transform: uppercase;
}

.info-value {
  font-size: 13px;
  color: var(--text-primary);
  letter-spacing: 0.5px;
}

.info-value.badge {
  display: inline-block;
  padding: 4px 12px;
  background: rgba(0, 243, 255, 0.1);
  border: 1px solid var(--neon-cyan);
  border-radius: 4px;
  font-size: 11px;
  font-weight: 600;
  color: var(--neon-cyan);
  text-align: center;
}

.info-value.enabled {
  color: var(--neon-green);
}

.info-value.disabled {
  color: var(--text-muted);
}

/* Action Bar */
.action-bar {
  display: flex;
  gap: 16px;
  justify-content: center;
  margin-top: 32px;
}

.cyber-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 10px;
  padding: 14px 32px;
  border: 1px solid;
  border-radius: 4px;
  font-family: inherit;
  font-size: 12px;
  font-weight: 700;
  letter-spacing: 2px;
  cursor: pointer;
  transition: all 0.3s ease;
  outline: none;
  min-width: 180px;
}

.settings-btn {
  background: linear-gradient(135deg, rgba(0, 243, 255, 0.1), rgba(0, 243, 255, 0.05));
  border-color: var(--neon-cyan);
  color: var(--neon-cyan);
}

.settings-btn:hover {
  background: linear-gradient(135deg, rgba(0, 243, 255, 0.2), rgba(0, 243, 255, 0.1));
  box-shadow: 0 0 25px rgba(0, 243, 255, 0.5);
  transform: translateY(-2px);
}

.edit-btn {
  background: linear-gradient(135deg, rgba(0, 255, 136, 0.1), rgba(0, 255, 136, 0.05));
  border-color: var(--neon-green);
  color: var(--neon-green);
}

.edit-btn:hover {
  background: linear-gradient(135deg, rgba(0, 255, 136, 0.2), rgba(0, 255, 136, 0.1));
  box-shadow: 0 0 25px rgba(0, 255, 136, 0.5);
  transform: translateY(-2px);
}

.btn-icon {
  font-size: 16px;
}

.btn-text {
  font-size: 12px;
}

/* Responsive */
@media (max-width: 768px) {
  .profile-header {
    padding: 20px 16px;
  }

  .page-title {
    font-size: 18px;
  }

  .profile-main {
    padding: 20px 16px;
  }

  .identity-display {
    flex-direction: column;
    text-align: center;
  }

  .identity-name {
    font-size: 20px;
  }

  .info-grid {
    grid-template-columns: 1fr;
  }

  .action-bar {
    flex-direction: column;
  }

  .cyber-btn {
    width: 100%;
  }

  .toast-notification {
    right: 16px;
    left: 16px;
    bottom: 16px;
  }
}

/* Toast Notification */
.toast-notification {
  position: fixed;
  bottom: 32px;
  right: 32px;
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 16px 24px;
  border-radius: 4px;
  z-index: 1000;
  animation: slide-in 0.3s ease-out;
}

.toast-notification.success {
  background: rgba(0, 255, 136, 0.1);
  border: 1px solid var(--neon-green);
  box-shadow: 0 0 20px rgba(0, 255, 136, 0.4);
}

.toast-notification.error {
  background: rgba(255, 51, 102, 0.1);
  border: 1px solid var(--neon-red);
  box-shadow: 0 0 20px rgba(255, 51, 102, 0.4);
}

.toast-icon {
  font-size: 18px;
}

.toast-notification.success .toast-icon {
  color: var(--neon-green);
}

.toast-notification.error .toast-icon {
  color: var(--neon-red);
}

.toast-message {
  font-size: 12px;
  font-weight: 600;
  letter-spacing: 1px;
  color: var(--text-primary);
}

.toast-enter-active,
.toast-leave-active {
  transition: all 0.3s ease;
}

.toast-enter-from,
.toast-leave-to {
  opacity: 0;
  transform: translateX(100px);
}

@keyframes slide-in {
  from {
    opacity: 0;
    transform: translateX(100px);
  }
  to {
    opacity: 1;
    transform: translateX(0);
  }
}
</style>
