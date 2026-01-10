<template>
  <div class="user-profile-container">
    <!-- Scanline overlay -->
    <div class="scanline-overlay"></div>

    <!-- Header -->
    <header class="profile-header">
      <div class="header-content">
        <h1 class="page-title">User Profile</h1>
        <p class="page-subtitle">View and edit your profile</p>
      </div>
    </header>

    <!-- Main Content -->
    <main class="profile-main">
      <!-- Loading state -->
      <div v-if="configStore.loading && !localConfig" class="loading-state">
        <div class="loading-spinner"></div>
        <p class="loading-text">Loading profile...</p>
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
              <h2 class="card-title">Identity</h2>
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
            <h2 class="card-title">Network Configuration</h2>
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
            <h2 class="card-title">System Settings</h2>
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
            <span class="btn-text">Edit Profile</span>
          </button>
          <button @click="goToSettings" class="cyber-btn settings-btn">
            <span class="btn-icon">⚙</span>
            <span class="btn-text">Advanced Settings</span>
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
  min-height: 100vh;
  background: var(--color-bg-primary);
  color: var(--color-text-primary);
  font-family: var(--font-sans);
  position: relative;
}

/* Header */
.profile-header {
  position: relative;
  z-index: 1;
  background: var(--color-bg-secondary);
  border-bottom: 1px solid var(--color-border-subtle);
  padding: var(--spacing-6) var(--spacing-8);
}

.header-content {
  max-width: 900px;
  margin: 0 auto;
}

.page-title {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: var(--spacing-3);
  font-size: var(--font-size-3xl);
  font-weight: var(--font-weight-bold);
  color: var(--color-text-primary);
  margin: 0 0 var(--spacing-2) 0;
  letter-spacing: var(--letter-spacing-tight);
}

.page-subtitle {
  text-align: center;
  font-size: var(--font-size-sm);
  color: var(--color-text-secondary);
  letter-spacing: var(--letter-spacing-wide);
  margin: 0;
  text-transform: uppercase;
}

/* Main Content */
.profile-main {
  position: relative;
  z-index: 1;
  padding: var(--spacing-8);
  max-width: 900px;
  margin: 0 auto;
}

.loading-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 80px var(--spacing-10);
  text-align: center;
}

.loading-spinner {
  width: 48px;
  height: 48px;
  border: 3px solid var(--color-border-subtle);
  border-top-color: var(--color-primary);
  border-radius: var(--radius-full);
  animation: spin 0.8s linear infinite;
  margin-bottom: var(--spacing-6);
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.loading-text {
  font-size: var(--font-size-sm);
  color: var(--color-text-secondary);
  letter-spacing: var(--letter-spacing-wide);
}

/* Profile Content */
.profile-content {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-6);
}

/* Profile Card */
.profile-card {
  background: var(--color-bg-elevated);
  border: 1px solid var(--color-border-subtle);
  border-radius: var(--radius-lg);
  overflow: hidden;
  position: relative;
  box-shadow: var(--shadow-sm);
}

.card-header {
  display: flex;
  align-items: center;
  gap: var(--spacing-3);
  padding: var(--spacing-4) var(--spacing-5);
  background: var(--color-bg-tertiary);
  border-bottom: 1px solid var(--color-border-subtle);
}

.card-icon {
  font-size: 16px;
  color: var(--color-primary);
}

.card-title {
  font-size: var(--font-size-base);
  font-weight: var(--font-weight-semibold);
  color: var(--color-primary);
  letter-spacing: var(--letter-spacing-wide);
  margin: 0;
}

.card-line {
  flex: 1;
  height: 1px;
  background: linear-gradient(90deg, var(--color-border-subtle), transparent);
}

.card-body {
  padding: var(--spacing-5);
}

/* Identity Display */
.identity-display {
  display: flex;
  align-items: center;
  gap: var(--spacing-5);
  padding: var(--spacing-5);
}

.identity-avatar {
  width: 80px;
  height: 80px;
  background: var(--gradient-primary-subtle);
  border: 2px solid var(--color-border);
  border-radius: var(--radius-2xl);
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
  gap: var(--spacing-2);
}

.identity-name {
  font-size: var(--font-size-3xl);
  font-weight: var(--font-weight-bold);
  color: var(--color-text-primary);
  letter-spacing: var(--letter-spacing-normal);
}

.identity-host {
  font-size: var(--font-size-base);
  color: var(--color-text-secondary);
  font-family: var(--font-mono);
}

.identity-status {
  font-size: var(--font-size-sm);
  color: var(--color-text-tertiary);
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
  gap: var(--spacing-4);
}

.info-item {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-1);
  padding: var(--spacing-3);
  background: var(--color-bg-tertiary);
  border: 1px solid var(--color-border-subtle);
  border-radius: var(--radius-md);
}

.info-label {
  font-size: var(--font-size-xs);
  font-weight: var(--font-weight-semibold);
  color: var(--color-text-secondary);
  letter-spacing: var(--letter-spacing-normal);
  text-transform: uppercase;
}

.info-value {
  font-size: var(--font-size-sm);
  color: var(--color-text-primary);
  letter-spacing: var(--letter-spacing-normal);
}

.info-value.badge {
  display: inline-block;
  padding: var(--spacing-1) var(--spacing-3);
  background: var(--color-primary-subtle);
  border: 1px solid var(--color-primary);
  border-radius: var(--radius-md);
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-semibold);
  color: var(--color-primary);
  text-align: center;
}

.info-value.enabled {
  color: var(--color-success);
}

.info-value.disabled {
  color: var(--color-text-tertiary);
}

/* Action Bar */
.action-bar {
  display: flex;
  gap: var(--spacing-4);
  justify-content: center;
  margin-top: var(--spacing-8);
}

.cyber-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: var(--spacing-2);
  padding: var(--spacing-3) var(--spacing-8);
  border: 1px solid;
  border-radius: var(--radius-md);
  font-family: var(--font-sans);
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-semibold);
  letter-spacing: var(--letter-spacing-wide);
  cursor: pointer;
  transition: all var(--transition-normal);
  outline: none;
  min-width: 180px;
}

.settings-btn {
  background: var(--color-primary);
  border-color: var(--color-primary);
  color: var(--color-text-inverse);
}

.settings-btn:hover {
  background: var(--color-primary-dark);
  border-color: var(--color-primary-dark);
  box-shadow: var(--shadow-primary);
  transform: translateY(-2px);
}

.edit-btn {
  background: var(--color-success);
  border-color: var(--color-success);
  color: var(--color-dark-text-primary);
}

.edit-btn:hover {
  background: var(--color-success-dark);
  border-color: var(--color-success-dark);
  box-shadow: var(--shadow-success);
  transform: translateY(-2px);
}

.btn-icon {
  font-size: 16px;
}

.btn-text {
  font-size: var(--font-size-sm);
}

.btn-spinner {
  width: 14px;
  height: 14px;
  border: 2px solid transparent;
  border-top-color: currentColor;
  border-radius: var(--radius-full);
  animation: spin 0.8s linear infinite;
}

/* Responsive */
@media (max-width: 768px) {
  .profile-header {
    padding: var(--spacing-5) var(--spacing-4);
  }

  .page-title {
    font-size: var(--font-size-2xl);
  }

  .profile-main {
    padding: var(--spacing-5) var(--spacing-4);
  }

  .identity-display {
    flex-direction: column;
    text-align: center;
  }

  .identity-name {
    font-size: var(--font-size-2xl);
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
    right: var(--spacing-4);
    left: var(--spacing-4);
    bottom: var(--spacing-4);
  }
}

/* Toast Notification */
.toast-notification {
  position: fixed;
  bottom: var(--spacing-8);
  right: var(--spacing-8);
  display: flex;
  align-items: center;
  gap: var(--spacing-3);
  padding: var(--spacing-4) var(--spacing-6);
  border-radius: var(--radius-md);
  z-index: var(--z-notification);
  animation: slideIn var(--transition-normal) ease-out;
  box-shadow: var(--shadow-lg);
}

.toast-notification.success {
  background: var(--color-success-light);
  border: 1px solid var(--color-success);
  color: var(--color-success);
}

.toast-notification.error {
  background: var(--color-error-light);
  border: 1px solid var(--color-error);
  color: var(--color-error);
}

.toast-icon {
  font-size: 18px;
}

.toast-message {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-semibold);
  letter-spacing: var(--letter-spacing-normal);
  color: var(--color-text-primary);
}

.toast-enter-active,
.toast-leave-active {
  transition: all var(--transition-normal);
}

.toast-enter-from,
.toast-leave-to {
  opacity: 0;
  transform: translateX(100px);
}
</style>
