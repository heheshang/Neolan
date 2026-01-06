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

    <!-- Settings Form -->
    <main class="settings-main">
      <!-- Loading state -->
      <div v-if="configStore.loading && !localConfig" class="loading-state">
        <div class="loading-spinner"></div>
        <p class="loading-text">INITIALIZING CONFIGURATION MODULE...</p>
      </div>

      <!-- Settings sections -->
      <div v-else class="settings-content">
        <!-- Section 1: User Identity -->
        <section class="config-section">
          <div class="section-header">
            <span class="section-icon">◆</span>
            <h2 class="section-title">USER IDENTITY</h2>
            <span class="section-line"></span>
          </div>
          <div class="section-body">
            <div class="form-row">
              <div class="form-field">
                <label class="field-label">
                  <span class="label-text">USERNAME</span>
                  <span class="label-required">*</span>
                </label>
                <input
                  v-model="formData.username"
                  type="text"
                  class="cyber-input"
                  :class="{ error: errors.username }"
                  @blur="validateField('username')"
                  placeholder="ENTER_USERNAME"
                  maxlength="50"
                />
                <span v-if="errors.username" class="field-error">{{ errors.username }}</span>
              </div>
              <div class="form-field">
                <label class="field-label">
                  <span class="label-text">HOSTNAME</span>
                </label>
                <input
                  v-model="formData.hostname"
                  type="text"
                  class="cyber-input"
                  placeholder="SYSTEM_HOSTNAME"
                  maxlength="50"
                />
              </div>
            </div>
          </div>
        </section>

        <!-- Section 2: Network Configuration -->
        <section class="config-section">
          <div class="section-header">
            <span class="section-icon">◈</span>
            <h2 class="section-title">NETWORK PROTOCOLS</h2>
            <span class="section-line"></span>
          </div>
          <div class="section-body">
            <div class="form-row">
              <div class="form-field">
                <label class="field-label">
                  <span class="label-text">BIND ADDRESS</span>
                </label>
                <input
                  v-model="formData.bindIp"
                  type="text"
                  class="cyber-input"
                  placeholder="0.0.0.0"
                />
              </div>
              <div class="form-field">
                <label class="field-label">
                  <span class="label-text">UDP PORT</span>
                  <span class="label-required">*</span>
                </label>
                <input
                  v-model.number="formData.udpPort"
                  type="number"
                  class="cyber-input"
                  :class="{ error: errors.udpPort }"
                  @blur="validateField('udpPort')"
                  min="1024"
                  max="65535"
                />
                <span v-if="errors.udpPort" class="field-error">{{ errors.udpPort }}</span>
              </div>
            </div>

            <div class="form-row">
              <div class="form-field field-range">
                <label class="field-label">
                  <span class="label-text">TCP PORT RANGE</span>
                  <span class="label-required">*</span>
                </label>
                <div class="range-inputs">
                  <div class="range-field">
                    <input
                      v-model.number="formData.tcpPortStart"
                      type="number"
                      class="cyber-input"
                      :class="{ error: errors.tcpPortStart }"
                      @blur="validateField('tcpPortStart')"
                      min="1024"
                      max="65535"
                      placeholder="START"
                    />
                    <span class="range-separator">—</span>
                    <input
                      v-model.number="formData.tcpPortEnd"
                      type="number"
                      class="cyber-input"
                      :class="{ error: errors.tcpPortEnd }"
                      @blur="validateField('tcpPortEnd')"
                      min="1024"
                      max="65535"
                      placeholder="END"
                    />
                  </div>
                </div>
                <span v-if="errors.tcpPortStart || errors.tcpPortEnd" class="field-error">
                  {{ errors.tcpPortStart || errors.tcpPortEnd }}
                </span>
              </div>
            </div>

            <div class="form-row">
              <div class="form-field">
                <label class="field-label">
                  <span class="label-text">HEARTBEAT INTERVAL (SEC)</span>
                  <span class="label-required">*</span>
                </label>
                <input
                  v-model.number="formData.heartbeatInterval"
                  type="number"
                  class="cyber-input"
                  :class="{ error: errors.heartbeatInterval }"
                  @blur="validateField('heartbeatInterval')"
                  min="10"
                  max="3600"
                />
                <span v-if="errors.heartbeatInterval" class="field-error">{{ errors.heartbeatInterval }}</span>
              </div>
              <div class="form-field">
                <label class="field-label">
                  <span class="label-text">PEER TIMEOUT (SEC)</span>
                  <span class="label-required">*</span>
                </label>
                <input
                  v-model.number="formData.peerTimeout"
                  type="number"
                  class="cyber-input"
                  :class="{ error: errors.peerTimeout }"
                  @blur="validateField('peerTimeout')"
                  min="10"
                  max="7200"
                />
                <span v-if="errors.peerTimeout" class="field-error">{{ errors.peerTimeout }}</span>
              </div>
            </div>
          </div>
        </section>

        <!-- Section 3: Security -->
        <section class="config-section">
          <div class="section-header">
            <span class="section-icon">⌘</span>
            <h2 class="section-title">SECURITY PROTOCOLS</h2>
            <span class="section-line"></span>
          </div>
          <div class="section-body">
            <div class="form-row">
              <div class="form-field">
                <div class="toggle-field">
                  <label class="field-label">ENCRYPTION ENABLED</label>
                  <div class="cyber-toggle" :class="{ active: formData.encryptionEnabled }" @click="formData.encryptionEnabled = !formData.encryptionEnabled">
                    <span class="toggle-slider"></span>
                    <span class="toggle-label">{{ formData.encryptionEnabled ? 'ACTIVE' : 'INACTIVE' }}</span>
                  </div>
                </div>
              </div>
            </div>
            <div v-if="formData.encryptionEnabled" class="form-row">
              <div class="form-field">
                <label class="field-label">
                  <span class="label-text">ENCRYPTION KEY (BASE64)</span>
                </label>
                <input
                  v-model="formData.encryptionKey"
                  type="text"
                  class="cyber-input"
                  placeholder="ENTER_ENCRYPTION_KEY"
                />
                <span class="field-hint">32-byte key encoded in Base64</span>
              </div>
            </div>
          </div>
        </section>

        <!-- Section 4: Messages -->
        <section class="config-section">
          <div class="section-header">
            <span class="section-icon">✉</span>
            <h2 class="section-title">MESSAGE PROTOCOLS</h2>
            <span class="section-line"></span>
          </div>
          <div class="section-body">
            <div class="form-row">
              <div class="form-field">
                <label class="field-label">
                  <span class="label-text">OFFLINE MESSAGE RETENTION (DAYS)</span>
                  <span class="label-required">*</span>
                </label>
                <input
                  v-model.number="formData.offlineMessageRetentionDays"
                  type="number"
                  class="cyber-input"
                  :class="{ error: errors.offlineMessageRetentionDays }"
                  @blur="validateField('offlineMessageRetentionDays')"
                  min="1"
                  max="365"
                />
                <span v-if="errors.offlineMessageRetentionDays" class="field-error">{{ errors.offlineMessageRetentionDays }}</span>
              </div>
            </div>
          </div>
        </section>

        <!-- Section 5: File Transfer -->
        <section class="config-section">
          <div class="section-header">
            <span class="section-icon">⇅</span>
            <h2 class="section-title">FILE TRANSFER</h2>
            <span class="section-line"></span>
          </div>
          <div class="section-body">
            <div class="form-row">
              <div class="form-field">
                <div class="toggle-field">
                  <label class="field-label">AUTO ACCEPT FILES</label>
                  <div class="cyber-toggle" :class="{ active: formData.autoAcceptFiles }" @click="formData.autoAcceptFiles = !formData.autoAcceptFiles">
                    <span class="toggle-slider"></span>
                    <span class="toggle-label">{{ formData.autoAcceptFiles ? 'ENABLED' : 'DISABLED' }}</span>
                  </div>
                </div>
              </div>
            </div>
            <div class="form-row">
              <div class="form-field">
                <label class="field-label">
                  <span class="label-text">FILE SAVE DIRECTORY</span>
                </label>
                <div class="path-input">
                  <input
                    v-model="formData.fileSaveDir"
                    type="text"
                    class="cyber-input"
                    placeholder="/path/to/downloads"
                  />
                </div>
              </div>
            </div>
          </div>
        </section>

        <!-- Section 6: System -->
        <section class="config-section">
          <div class="section-header">
            <span class="section-icon">⚙</span>
            <h2 class="section-title">SYSTEM PARAMETERS</h2>
            <span class="section-line"></span>
          </div>
          <div class="section-body">
            <div class="form-row">
              <div class="form-field">
                <label class="field-label">
                  <span class="label-text">LOG LEVEL</span>
                </label>
                <div class="cyber-select">
                  <select v-model="formData.logLevel" class="select-input">
                    <option value="trace">TRACE</option>
                    <option value="debug">DEBUG</option>
                    <option value="info">INFO</option>
                    <option value="warn">WARN</option>
                    <option value="error">ERROR</option>
                  </select>
                  <span class="select-arrow">▼</span>
                </div>
              </div>
            </div>
          </div>
        </section>

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
import { ref, reactive, onMounted } from 'vue';
import { useConfigStore } from '@/stores';
import type { ConfigDto } from '@/api';

const configStore = useConfigStore();

// Local form data
const localConfig = ref<ConfigDto | null>(null);
const formData = reactive<ConfigDto>({
  username: '',
  hostname: '',
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

// Validation errors
const errors = reactive<Record<string, string>>({});

// Toast notification
const toast = reactive({
  show: false,
  message: '',
  type: 'success' as 'success' | 'error',
});

let toastTimer: ReturnType<typeof setTimeout> | null = null;

// Initialize
onMounted(async () => {
  const config = await configStore.fetchConfig();
  if (config) {
    localConfig.value = config;
    Object.assign(formData, config);
  }
});

// Validation
function validateField(field: keyof typeof formData): boolean {
  switch (field) {
    case 'username':
      if (!formData.username || formData.username.trim().length === 0) {
        errors.username = 'USERNAME_REQUIRED';
        return false;
      }
      if (formData.username.length > 50) {
        errors.username = 'MAX_50_CHARS';
        return false;
      }
      delete errors.username;
      break;

    case 'udpPort':
      if (!formData.udpPort || formData.udpPort < 1024 || formData.udpPort > 65535) {
        errors.udpPort = 'PORT_RANGE_1024_65535';
        return false;
      }
      delete errors.udpPort;
      break;

    case 'tcpPortStart':
      if (!formData.tcpPortStart || formData.tcpPortStart < 1024 || formData.tcpPortStart > 65535) {
        errors.tcpPortStart = 'PORT_RANGE_1024_65535';
        return false;
      }
      if (formData.tcpPortStart >= formData.tcpPortEnd) {
        errors.tcpPortStart = 'START_MUST_BE_LESS_THAN_END';
        return false;
      }
      delete errors.tcpPortStart;
      delete errors.tcpPortEnd;
      break;

    case 'tcpPortEnd':
      if (!formData.tcpPortEnd || formData.tcpPortEnd < 1024 || formData.tcpPortEnd > 65535) {
        errors.tcpPortEnd = 'PORT_RANGE_1024_65535';
        return false;
      }
      if (formData.tcpPortEnd <= formData.tcpPortStart) {
        errors.tcpPortEnd = 'END_MUST_BE_GREATER_THAN_START';
        return false;
      }
      delete errors.tcpPortStart;
      delete errors.tcpPortEnd;
      break;

    case 'heartbeatInterval':
      if (!formData.heartbeatInterval || formData.heartbeatInterval < 10 || formData.heartbeatInterval > 3600) {
        errors.heartbeatInterval = 'HEARTBEAT_RANGE_10_3600';
        return false;
      }
      delete errors.heartbeatInterval;
      if (formData.peerTimeout && formData.peerTimeout <= formData.heartbeatInterval) {
        errors.peerTimeout = 'TIMEOUT_MUST_BE_GREATER';
      }
      break;

    case 'peerTimeout':
      if (!formData.peerTimeout || formData.peerTimeout < 10 || formData.peerTimeout > 7200) {
        errors.peerTimeout = 'TIMEOUT_RANGE_10_7200';
        return false;
      }
      if (formData.peerTimeout <= formData.heartbeatInterval) {
        errors.peerTimeout = 'TIMEOUT_MUST_EXCEED_HEARTBEAT';
        return false;
      }
      delete errors.peerTimeout;
      break;

    case 'offlineMessageRetentionDays':
      if (!formData.offlineMessageRetentionDays || formData.offlineMessageRetentionDays < 1 || formData.offlineMessageRetentionDays > 365) {
        errors.offlineMessageRetentionDays = 'RETENTION_RANGE_1_365';
        return false;
      }
      delete errors.offlineMessageRetentionDays;
      break;
  }
  return true;
}

function validateAll(): boolean {
  let valid = true;
  const fields: (keyof typeof formData)[] = ['username', 'udpPort', 'tcpPortStart', 'tcpPortEnd', 'heartbeatInterval', 'peerTimeout', 'offlineMessageRetentionDays'];
  for (const field of fields) {
    if (!validateField(field)) {
      valid = false;
    }
  }
  return valid;
}

// Actions
async function handleSave() {
  if (!validateAll()) {
    showToast('VALIDATION_FAILED', 'error');
    return;
  }

  const success = await configStore.saveConfig({ ...formData });
  if (success) {
    localConfig.value = { ...formData };
    showToast('CONFIGURATION_SAVED', 'success');
  } else {
    showToast(configStore.error || 'SAVE_FAILED', 'error');
  }
}

async function handleReset() {
  if (confirm('RESET_ALL_SETTINGS_TO_DEFAULT_VALUES?')) {
    const config = await configStore.resetToDefault();
    if (config) {
      localConfig.value = config;
      Object.assign(formData, config);
      Object.keys(errors).forEach(key => delete errors[key]);
      showToast('CONFIGURATION_RESET', 'success');
    }
  }
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
</script>

<style scoped>
/* ==================== Cyberpunk Theme ==================== */
.settings-container {
  --neon-cyan: #00f3ff;
  --neon-magenta: #ff00ff;
  --neon-green: #00ff88;
  --neon-red: #ff3366;
  --neon-amber: #ffaa00;
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

/* ==================== Scanline Overlay ==================== */
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

/* ==================== Header ==================== */
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

/* ==================== Main Content ==================== */
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

/* ==================== Config Sections ==================== */
.settings-content {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.config-section {
  background: rgba(0, 0, 0, 0.4);
  border: 1px solid var(--border-dim);
  border-radius: 8px;
  overflow: hidden;
  position: relative;
}

.config-section::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 1px;
  background: linear-gradient(90deg, transparent, var(--neon-cyan), transparent);
  opacity: 0.5;
}

.section-header {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 16px 20px;
  background: rgba(0, 243, 255, 0.05);
  border-bottom: 1px solid var(--border-dim);
}

.section-icon {
  font-size: 16px;
  color: var(--neon-cyan);
  text-shadow: 0 0 10px var(--neon-cyan);
}

.section-title {
  font-size: 14px;
  font-weight: 700;
  color: var(--neon-cyan);
  letter-spacing: 2px;
  margin: 0;
}

.section-line {
  flex: 1;
  height: 1px;
  background: linear-gradient(90deg, var(--border-dim), transparent);
}

.section-body {
  padding: 20px;
}

.form-row {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 20px;
  margin-bottom: 16px;
}

.form-row:last-child {
  margin-bottom: 0;
}

/* ==================== Form Fields ==================== */
.form-field {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.field-label {
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 10px;
  font-weight: 600;
  color: var(--text-secondary);
  letter-spacing: 1px;
}

.label-required {
  color: var(--neon-red);
}

.cyber-input {
  width: 100%;
  padding: 12px 16px;
  background: rgba(0, 0, 0, 0.5);
  border: 1px solid var(--border-dim);
  border-radius: 4px;
  color: var(--text-primary);
  font-family: inherit;
  font-size: 12px;
  letter-spacing: 1px;
  transition: all 0.3s ease;
  outline: none;
}

.cyber-input:hover {
  border-color: var(--border-bright);
}

.cyber-input:focus {
  border-color: var(--neon-cyan);
  box-shadow: 0 0 15px rgba(0, 243, 255, 0.3), inset 0 0 10px rgba(0, 243, 255, 0.1);
}

.cyber-input.error {
  border-color: var(--neon-red);
  box-shadow: 0 0 15px rgba(255, 51, 102, 0.3);
}

.cyber-input::placeholder {
  color: var(--text-muted);
  opacity: 0.5;
}

.field-error {
  font-size: 10px;
  color: var(--neon-red);
  letter-spacing: 1px;
}

.field-hint {
  font-size: 10px;
  color: var(--text-muted);
  letter-spacing: 1px;
}

/* ==================== Range Input ==================== */
.field-range {
  grid-column: 1 / -1;
}

.range-inputs {
  display: flex;
  gap: 12px;
}

.range-field {
  display: flex;
  align-items: center;
  gap: 12px;
  flex: 1;
}

.range-separator {
  color: var(--neon-cyan);
  font-size: 18px;
}

/* ==================== Toggle Switch ==================== */
.toggle-field {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.cyber-toggle {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 8px 16px;
  background: rgba(0, 0, 0, 0.5);
  border: 1px solid var(--border-dim);
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.3s ease;
}

.cyber-toggle:hover {
  border-color: var(--border-bright);
}

.cyber-toggle.active {
  border-color: var(--neon-green);
  box-shadow: 0 0 15px rgba(0, 255, 136, 0.3);
}

.toggle-slider {
  width: 40px;
  height: 20px;
  background: var(--border-dim);
  border-radius: 10px;
  position: relative;
  transition: all 0.3s ease;
}

.cyber-toggle.active .toggle-slider {
  background: var(--neon-green);
}

.toggle-slider::after {
  content: '';
  position: absolute;
  top: 2px;
  left: 2px;
  width: 16px;
  height: 16px;
  background: var(--text-primary);
  border-radius: 50%;
  transition: all 0.3s ease;
}

.cyber-toggle.active .toggle-slider::after {
  left: 22px;
  background: var(--bg-dark);
}

.toggle-label {
  font-size: 10px;
  font-weight: 600;
  letter-spacing: 1px;
  color: var(--text-secondary);
}

.cyber-toggle.active .toggle-label {
  color: var(--neon-green);
}

/* ==================== Select Dropdown ==================== */
.cyber-select {
  position: relative;
}

.select-input {
  width: 100%;
  padding: 12px 40px 12px 16px;
  background: rgba(0, 0, 0, 0.5);
  border: 1px solid var(--border-dim);
  border-radius: 4px;
  color: var(--text-primary);
  font-family: inherit;
  font-size: 12px;
  letter-spacing: 1px;
  cursor: pointer;
  appearance: none;
  outline: none;
  transition: all 0.3s ease;
}

.select-input:hover {
  border-color: var(--border-bright);
}

.select-input:focus {
  border-color: var(--neon-cyan);
  box-shadow: 0 0 15px rgba(0, 243, 255, 0.3);
}

.select-arrow {
  position: absolute;
  right: 16px;
  top: 50%;
  transform: translateY(-50%);
  color: var(--neon-cyan);
  font-size: 10px;
  pointer-events: none;
}

/* ==================== Action Buttons ==================== */
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

/* ==================== Toast Notification ==================== */
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

/* ==================== Responsive ==================== */
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

  .form-row {
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
</style>
