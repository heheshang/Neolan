<template>
  <div class="settings-container">
    <!-- Scanline overlay -->
    <div class="scanline-overlay"></div>

    <!-- Header -->
    <header class="settings-header">
      <div class="header-content">
        <h1 class="page-title">Settings</h1>
        <p class="page-subtitle">Configure your application</p>
      </div>
    </header>

    <!-- Tab Navigation -->
    <TabNavigation
      :tabs="tabs"
      v-model="currentTab"
    />

    <!-- Main Content -->
    <main class="settings-main">
      <!-- Loading state -->
      <div v-if="configStore.loading && !localConfig" class="loading-state">
        <div class="loading-spinner"></div>
        <p class="loading-text">Loading settings...</p>
      </div>

      <!-- Settings Tabs -->
      <div v-else class="tab-content">
        <!-- Identity Tab -->
        <Transition name="tab" mode="out-in">
          <IdentitySettings
            v-if="currentTab === 'identity'"
            v-model="formData"
            @update:valid="handleTabValid('identity', $event)"
            ref="identityTab"
          />
          <NetworkSettings
            v-else-if="currentTab === 'network'"
            v-model="formData"
            @update:valid="handleTabValid('network', $event)"
            ref="networkTab"
          />
          <SecuritySettings
            v-else-if="currentTab === 'security'"
            v-model="formData"
            @update:valid="handleTabValid('security', $event)"
            ref="securityTab"
          />
          <MessageSettings
            v-else-if="currentTab === 'messages'"
            v-model="formData"
            @update:valid="handleTabValid('messages', $event)"
            ref="messagesTab"
          />
          <FileSettings
            v-else-if="currentTab === 'files'"
            v-model="formData"
            @update:valid="handleTabValid('files', $event)"
            ref="filesTab"
          />
          <SystemSettings
            v-else-if="currentTab === 'system'"
            v-model="formData"
            @update:valid="handleTabValid('system', $event)"
            ref="systemTab"
          />
        </Transition>

        <!-- Action Buttons -->
        <div class="action-bar">
          <button
            @click="handleSave"
            class="cyber-btn save-btn"
            :class="{ loading: configStore.saving }"
            :disabled="configStore.saving"
          >
            <span v-if="!configStore.saving" class="btn-icon">✓</span>
            <span v-else class="btn-spinner"></span>
            <span class="btn-text">{{ configStore.saving ? 'Saving...' : 'Save Changes' }}</span>
          </button>
          <button
            @click="handleReset"
            class="cyber-btn reset-btn"
            :disabled="configStore.saving"
          >
            <span class="btn-icon">↺</span>
            <span class="btn-text">Reset to Defaults</span>
          </button>
        </div>
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
import { ref, reactive, computed, watch, onMounted, onUnmounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { useConfigStore } from '@/stores';
import TabNavigation, { type Tab } from '@/components/TabNavigation.vue';
import IdentitySettings from '@/components/settings/IdentitySettings.vue';
import NetworkSettings from '@/components/settings/NetworkSettings.vue';
import SecuritySettings from '@/components/settings/SecuritySettings.vue';
import MessageSettings from '@/components/settings/MessageSettings.vue';
import FileSettings from '@/components/settings/FileSettings.vue';
import SystemSettings from '@/components/settings/SystemSettings.vue';
import type { ConfigDto } from '@/api';

const route = useRoute();
const router = useRouter();
const configStore = useConfigStore();

// Tab definitions
const tabs: Tab[] = [
  { id: 'identity', label: 'Identity', icon: '◆' },
  { id: 'network', label: 'Network', icon: '◈' },
  { id: 'security', label: 'Security', icon: '⌘' },
  { id: 'messages', label: 'Messages', icon: '✉' },
  { id: 'files', label: 'Files', icon: '⇅' },
  { id: 'system', label: 'System', icon: '⚙' },
];

// State
const localConfig = ref<ConfigDto | null>(null);
const currentTab = ref('identity');
const formData = reactive<ConfigDto>({
  username: '',
  hostname: '',
  avatar: null,
  status: '',
  bindIp: '0.0.0.0',
  udpPort: 2425,
  tcpPortStart: 8000,
  tcpPortEnd: 9000,
  heartbeatInterval: 60,
  peerTimeout: 180,
  encryptionEnabled: false,
  encryptionKey: undefined,
  offlineMessageRetentionDays: 30,
  autoAcceptFiles: false,
  fileSaveDir: '',
  logLevel: 'info',
});

// Track validity per tab
const tabValidity = reactive<Record<string, boolean>>({
  identity: true,
  network: true,
  security: true,
  messages: true,
  files: true,
  system: true,
});

// Track unsaved changes per tab
const unsavedChanges = reactive<Record<string, boolean>>({
  identity: false,
  network: false,
  security: false,
  messages: false,
  files: false,
  system: false,
});

// Toast notification
const toast = reactive({
  show: false,
  message: '',
  type: 'success' as 'success' | 'error',
});

let toastTimer: ReturnType<typeof setTimeout> | null = null;
let originalConfig: ConfigDto | null = null;

// Tab refs for validation
const identityTab = ref();
const networkTab = ref();
const securityTab = ref();
const messagesTab = ref();
const filesTab = ref();
const systemTab = ref();

// Get current tab ref
const currentTabRef = computed(() => {
  switch (currentTab.value) {
    case 'identity': return identityTab.value;
    case 'network': return networkTab.value;
    case 'security': return securityTab.value;
    case 'messages': return messagesTab.value;
    case 'files': return filesTab.value;
    case 'system': return systemTab.value;
    default: return null;
  }
});

// Initialize from URL query param
onMounted(() => {
  const tabParam = route.query.tab as string;
  if (tabParam && tabs.some(t => t.id === tabParam)) {
    currentTab.value = tabParam;
  }
  loadConfig();
});

// Watch for tab changes in URL
watch(() => route.query.tab, (newTab) => {
  if (newTab && tabs.some(t => t.id === newTab)) {
    switchTab(newTab as string);
  }
});

// Watch for tab changes
watch(currentTab, (newTab, oldTab) => {
  if (newTab !== oldTab) {
    // Update URL without full navigation
    router.replace({ query: { tab: newTab } });
  }
});

// Load configuration
async function loadConfig() {
  const config = await configStore.fetchConfig();
  if (config) {
    localConfig.value = config;
    originalConfig = { ...config };
    Object.assign(formData, config);
  }
}

// Handle tab validity updates
function handleTabValid(tab: string, valid: boolean) {
  tabValidity[tab] = valid;
}

// Switch tabs with unsaved changes check
async function switchTab(newTab: string) {
  const oldTab = currentTab.value;

  // Check for unsaved changes
  if (unsavedChanges[oldTab]) {
    const confirmed = confirm('You have unsaved changes. Do you want to save them before switching tabs?');
    if (confirmed) {
      const saved = await saveTabData();
      if (!saved) {
        return; // Don't switch if save failed
      }
    } else {
      // Discard changes
      if (originalConfig) {
        Object.assign(formData, originalConfig);
      }
    }
    unsavedChanges[oldTab] = false;
  }

  currentTab.value = newTab;
}

// Save current tab data
async function saveTabData(): Promise<boolean> {
  const tabRef = currentTabRef.value;
  if (tabRef?.validate) {
    const isValid = tabRef.validate();
    if (!isValid) {
      return false;
    }
  }

  const success = await configStore.saveConfig({ ...formData });
  if (success) {
    localConfig.value = { ...formData };
    originalConfig = { ...formData };
    unsavedChanges[currentTab.value] = false;
    showToast('Configuration saved', 'success');
  } else {
    showToast(configStore.error || 'Failed to save configuration', 'error');
  }
  return success;
}

// Handle save button
async function handleSave() {
  await saveTabData();
}

// Handle reset button
async function handleReset() {
  if (confirm('RESET ALL SETTINGS TO DEFAULT VALUES?')) {
    const config = await configStore.resetToDefault();
    if (config) {
      localConfig.value = config;
      originalConfig = { ...config };
      Object.assign(formData, config);
      Object.keys(unsavedChanges).forEach(key => unsavedChanges[key as keyof typeof unsavedChanges] = false);
      showToast('Configuration reset', 'success');
    }
  }
}

// Show toast notification
function showToast(message: string, type: 'success' | 'error') {
  toast.message = message;
  toast.type = type;
  toast.show = true;

  if (toastTimer) clearTimeout(toastTimer);
  toastTimer = setTimeout(() => {
    toast.show = false;
  }, 3000);
}

// Prevent navigation with unsaved changes
onUnmounted(() => {
  // Clean up timer
  if (toastTimer) clearTimeout(toastTimer);
});
</script>

<style scoped>
.settings-container {
  min-height: 100vh;
  background: var(--color-bg-primary);
  color: var(--color-text-primary);
  font-family: var(--font-sans);
  position: relative;
}

/* Header */
.settings-header {
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
.settings-main {
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

/* Tab Content */
.tab-content {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-8);
}

.tab-enter-active,
.tab-leave-active {
  transition: all var(--transition-normal);
}

.tab-enter-from {
  opacity: 0;
  transform: translateX(20px);
}

.tab-leave-to {
  opacity: 0;
  transform: translateX(-20px);
}

/* Action Buttons */
.action-bar {
  display: flex;
  gap: var(--spacing-4);
  justify-content: center;
  padding-top: var(--spacing-4);
  border-top: 1px solid var(--color-border-subtle);
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

.save-btn {
  background: var(--color-primary);
  border-color: var(--color-primary);
  color: var(--color-text-inverse);
}

.save-btn:hover:not(:disabled) {
  background: var(--color-primary-dark);
  border-color: var(--color-primary-dark);
  box-shadow: var(--shadow-primary);
  transform: translateY(-2px);
}

.save-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.reset-btn {
  background: transparent;
  border-color: var(--color-border);
  color: var(--color-text-secondary);
}

.reset-btn:hover:not(:disabled) {
  background: var(--color-bg-tertiary);
  border-color: var(--color-border-strong);
  color: var(--color-text-primary);
  transform: translateY(-2px);
}

.btn-icon {
  font-size: 16px;
}

.btn-text {
  font-size: var(--font-size-sm);
}

.btn-spinner {
  width: 16px;
  height: 16px;
  border: 2px solid transparent;
  border-top-color: currentColor;
  border-radius: var(--radius-full);
  animation: spin 0.8s linear infinite;
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

/* Responsive */
@media (max-width: 768px) {
  .settings-header {
    padding: var(--spacing-5) var(--spacing-4);
  }

  .page-title {
    font-size: var(--font-size-2xl);
  }

  .settings-main {
    padding: var(--spacing-5) var(--spacing-4);
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
</style>
