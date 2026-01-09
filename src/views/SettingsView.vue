<template>
  <div class="settings-container">
    <!-- Scanline overlay -->
    <div class="scanline-overlay"></div>

    <!-- Header -->
    <header class="settings-header">
      <div class="header-content">
        <h1 class="page-title">
          <span class="title-bracket">[</span>
          <span class="title-text">SYSTEM CONFIG</span>
          <span class="title-bracket">]</span>
        </h1>
        <p class="page-subtitle">// NEURAL NETWORK PARAMETERS</p>
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
        <p class="loading-text">INITIALIZING CONFIGURATION MODULE...</p>
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
            <span class="btn-text">{{ configStore.saving ? 'SAVING...' : 'SAVE CONFIG' }}</span>
          </button>
          <button
            @click="handleReset"
            class="cyber-btn reset-btn"
            :disabled="configStore.saving"
          >
            <span class="btn-icon">↺</span>
            <span class="btn-text">RESET DEFAULTS</span>
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
.settings-header {
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
.settings-main {
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

/* Tab Content */
.tab-content {
  display: flex;
  flex-direction: column;
  gap: 32px;
}

.tab-enter-active,
.tab-leave-active {
  transition: all 0.3s ease;
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
  gap: 16px;
  justify-content: center;
  padding-top: 16px;
  border-top: 1px solid var(--border-dim);
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

.save-btn {
  background: linear-gradient(135deg, rgba(0, 243, 255, 0.1), rgba(0, 243, 255, 0.05));
  border-color: var(--neon-cyan);
  color: var(--neon-cyan);
}

.save-btn:hover:not(:disabled) {
  background: linear-gradient(135deg, rgba(0, 243, 255, 0.2), rgba(0, 243, 255, 0.1));
  box-shadow: 0 0 25px rgba(0, 243, 255, 0.5);
  transform: translateY(-2px);
}

.save-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.reset-btn {
  background: linear-gradient(135deg, rgba(255, 0, 255, 0.1), rgba(255, 0, 255, 0.05));
  border-color: var(--neon-magenta);
  color: var(--neon-magenta);
}

.reset-btn:hover:not(:disabled) {
  background: linear-gradient(135deg, rgba(255, 0, 255, 0.2), rgba(255, 0, 255, 0.1));
  box-shadow: 0 0 25px rgba(255, 0, 255, 0.5);
  transform: translateY(-2px);
}

.btn-icon {
  font-size: 16px;
}

.btn-text {
  font-size: 12px;
}

.btn-spinner {
  width: 16px;
  height: 16px;
  border: 2px solid transparent;
  border-top-color: currentColor;
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
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

/* Responsive */
@media (max-width: 768px) {
  .settings-header {
    padding: 20px 16px;
  }

  .page-title {
    font-size: 18px;
  }

  .settings-main {
    padding: 20px 16px;
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
</style>
