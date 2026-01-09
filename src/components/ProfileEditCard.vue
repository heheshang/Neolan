<template>
  <section class="profile-edit-card">
    <div class="card-header">
      <span class="card-icon">✎</span>
      <h2 class="card-title">EDIT PROFILE</h2>
      <span class="card-line"></span>
    </div>
    <div class="card-body">
      <!-- Avatar Section -->
      <div class="avatar-section">
        <div
          class="avatar-display"
          :class="{ clickable: !showAvatarPicker }"
          @click="handleAvatarClick"
        >
          <span class="avatar-large">{{ displayAvatar }}</span>
          <span v-if="!showAvatarPicker" class="avatar-hint">Click to change</span>
        </div>
        <AvatarPicker
          v-if="showAvatarPicker"
          :current-avatar="localForm.avatar"
          @select="handleAvatarSelect"
          @cancel="showAvatarPicker = false"
        />
      </div>

      <!-- Edit Form -->
      <div class="edit-form">
        <!-- Username -->
        <div class="form-field">
          <label class="field-label">
            <span class="label-text">USERNAME</span>
            <span class="label-required">*</span>
          </label>
          <input
            ref="usernameInput"
            v-model="localForm.username"
            type="text"
            class="cyber-input"
            :class="{ error: errors.username }"
            @blur="validateField('username')"
            @keydown.enter="handleSave"
            placeholder="ENTER_USERNAME"
            maxlength="50"
          />
          <span v-if="errors.username" class="field-error">{{ errors.username }}</span>
        </div>

        <!-- Hostname -->
        <div class="form-field">
          <label class="field-label">
            <span class="label-text">HOSTNAME</span>
          </label>
          <input
            v-model="localForm.hostname"
            type="text"
            class="cyber-input"
            :class="{ error: errors.hostname }"
            @blur="validateField('hostname')"
            @keydown.enter="handleSave"
            placeholder="SYSTEM_HOSTNAME"
            maxlength="50"
          />
          <span v-if="errors.hostname" class="field-error">{{ errors.hostname }}</span>
        </div>

        <!-- Status Message -->
        <div class="form-field">
          <label class="field-label">
            <span class="label-text">STATUS MESSAGE</span>
            <span class="label-hint-inline">{{ localForm.status.length }} / 200</span>
          </label>
          <textarea
            v-model="localForm.status"
            class="cyber-textarea"
            :class="{ error: errors.status }"
            @blur="validateField('status')"
            placeholder="What's on your mind?"
            rows="3"
            maxlength="200"
          />
          <span v-if="errors.status" class="field-error">{{ errors.status }}</span>
        </div>
      </div>

      <!-- Action Buttons -->
      <div class="edit-actions">
        <button
          @click="handleSave"
          class="cyber-btn save-btn"
          :class="{ loading: isSaving }"
          :disabled="isSaving"
        >
          <span v-if="!isSaving" class="btn-icon">✓</span>
          <span v-else class="btn-spinner"></span>
          <span class="btn-text">{{ isSaving ? 'SAVING...' : 'SAVE CHANGES' }}</span>
        </button>
        <button
          @click="handleCancel"
          class="cyber-btn cancel-btn"
          :disabled="isSaving"
        >
          <span class="btn-icon">✕</span>
          <span class="btn-text">CANCEL</span>
        </button>
      </div>

      <!-- Keyboard Hint -->
      <div class="keyboard-hint">
        <span class="hint-key">Ctrl+S</span> to save · <span class="hint-key">Esc</span> to cancel
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted, nextTick, onUnmounted } from 'vue';
import AvatarPicker from './AvatarPicker.vue';

interface Props {
  username: string;
  hostname: string;
  status?: string;
  avatar?: string | null;
}

interface Emits {
  (e: 'save', data: { username: string; hostname: string; status: string; avatar: string | null }): void;
  (e: 'cancel'): void;
}

const props = withDefaults(defineProps<Props>(), {
  status: '',
  avatar: null,
});

const emit = defineEmits<Emits>();

// Local form state
const localForm = reactive({
  username: props.username,
  hostname: props.hostname,
  status: props.status || '',
  avatar: props.avatar || null,
});

// Original values for cancel
const originalValues = {
  username: props.username,
  hostname: props.hostname,
  status: props.status || '',
  avatar: props.avatar || null,
};

// UI state
const isSaving = ref(false);
const showAvatarPicker = ref(false);
const usernameInput = ref<HTMLInputElement | null>(null);
const errors = reactive<Record<string, string>>({});

// Computed
const displayAvatar = computed(() => localForm.avatar || '👤');

// Methods
function validateField(field: keyof typeof localForm): boolean {
  switch (field) {
    case 'username':
      if (!localForm.username || localForm.username.trim().length === 0) {
        errors.username = 'USERNAME_REQUIRED';
        return false;
      }
      if (localForm.username.length > 50) {
        errors.username = 'MAX_50_CHARS';
        return false;
      }
      delete errors.username;
      break;

    case 'hostname':
      if (localForm.hostname && localForm.hostname.length > 50) {
        errors.hostname = 'MAX_50_CHARS';
        return false;
      }
      delete errors.hostname;
      break;

    case 'status':
      if (localForm.status && localForm.status.length > 200) {
        errors.status = 'MAX_200_CHARS';
        return false;
      }
      delete errors.status;
      break;
  }
  return true;
}

function validateAll(): boolean {
  let valid = true;
  const fields: (keyof typeof localForm)[] = ['username', 'hostname', 'status'];
  for (const field of fields) {
    if (!validateField(field)) {
      valid = false;
    }
  }
  return valid;
}

function handleAvatarClick() {
  if (!showAvatarPicker.value) {
    showAvatarPicker.value = true;
  }
}

function handleAvatarSelect(emoji: string) {
  localForm.avatar = emoji;
  showAvatarPicker.value = false;
}

async function handleSave() {
  if (!validateAll()) {
    // Focus first error field
    if (errors.username) {
      usernameInput.value?.focus();
    }
    return;
  }

  isSaving.value = true;
  try {
    await emit('save', {
      username: localForm.username.trim(),
      hostname: localForm.hostname.trim(),
      status: localForm.status.trim(),
      avatar: localForm.avatar,
    });
  } finally {
    isSaving.value = false;
  }
}

function handleCancel() {
  // Check if there are unsaved changes
  const hasChanges =
    localForm.username !== originalValues.username ||
    localForm.hostname !== originalValues.hostname ||
    localForm.status !== originalValues.status ||
    localForm.avatar !== originalValues.avatar;

  if (hasChanges) {
    if (confirm('Discard unsaved changes?')) {
      emit('cancel');
    }
  } else {
    emit('cancel');
  }
}

// Keyboard shortcuts
function handleKeyDown(event: KeyboardEvent) {
  if ((event.ctrlKey || event.metaKey) && event.key === 's') {
    event.preventDefault();
    handleSave();
  } else if (event.key === 'Escape') {
    event.preventDefault();
    handleCancel();
  }
}

// Lifecycle
onMounted(() => {
  document.addEventListener('keydown', handleKeyDown);
  nextTick(() => {
    usernameInput.value?.focus();
  });
});

onUnmounted(() => {
  document.removeEventListener('keydown', handleKeyDown);
});
</script>

<style scoped>
.profile-edit-card {
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

  background: rgba(0, 0, 0, 0.4);
  border: 1px solid var(--border-dim);
  border-radius: 8px;
  overflow: hidden;
  position: relative;
}

.profile-edit-card::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 1px;
  background: linear-gradient(90deg, transparent, var(--neon-magenta), transparent);
  opacity: 0.5;
}

.card-header {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 16px 20px;
  background: rgba(255, 0, 255, 0.05);
  border-bottom: 1px solid var(--border-dim);
}

.card-icon {
  font-size: 16px;
  color: var(--neon-magenta);
  text-shadow: 0 0 10px var(--neon-magenta);
}

.card-title {
  font-size: 14px;
  font-weight: 700;
  color: var(--neon-magenta);
  letter-spacing: 2px;
  margin: 0;
}

.card-line {
  flex: 1;
  height: 1px;
  background: linear-gradient(90deg, var(--border-dim), transparent);
}

.card-body {
  padding: 24px;
}

/* Avatar Section */
.avatar-section {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 16px;
  margin-bottom: 24px;
}

.avatar-display {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
}

.avatar-display.clickable {
  cursor: pointer;
}

.avatar-display.clickable:hover .avatar-large {
  transform: scale(1.05);
  box-shadow: 0 0 20px rgba(255, 0, 255, 0.5);
}

.avatar-large {
  font-size: 64px;
  width: 100px;
  height: 100px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, rgba(0, 243, 255, 0.1), rgba(255, 0, 255, 0.1));
  border: 2px solid var(--border-dim);
  border-radius: 12px;
  transition: all 0.3s ease;
}

.avatar-hint {
  font-size: 10px;
  color: var(--text-secondary);
  letter-spacing: 1px;
  opacity: 0;
  transition: opacity 0.3s ease;
}

.avatar-display.clickable:hover .avatar-hint {
  opacity: 1;
}

/* Edit Form */
.edit-form {
  display: flex;
  flex-direction: column;
  gap: 20px;
  margin-bottom: 24px;
}

.form-field {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.field-label {
  display: flex;
  align-items: center;
  justify-content: space-between;
  font-size: 10px;
  font-weight: 600;
  color: var(--text-secondary);
  letter-spacing: 1px;
}

.label-required {
  color: var(--neon-red);
}

.label-hint-inline {
  color: var(--text-muted);
  font-size: 9px;
}

.cyber-input {
  width: 100%;
  padding: 12px 16px;
  background: rgba(0, 0, 0, 0.5);
  border: 1px solid var(--border-dim);
  border-radius: 4px;
  color: var(--text-primary);
  font-family: 'Courier New', monospace;
  font-size: 12px;
  letter-spacing: 1px;
  transition: all 0.3s ease;
  outline: none;
}

.cyber-input:hover {
  border-color: var(--border-bright);
}

.cyber-input:focus {
  border-color: var(--neon-magenta);
  box-shadow: 0 0 15px rgba(255, 0, 255, 0.3), inset 0 0 10px rgba(255, 0, 255, 0.1);
}

.cyber-input.error {
  border-color: var(--neon-red);
  box-shadow: 0 0 15px rgba(255, 51, 102, 0.3);
}

.cyber-textarea {
  width: 100%;
  padding: 12px 16px;
  background: rgba(0, 0, 0, 0.5);
  border: 1px solid var(--border-dim);
  border-radius: 4px;
  color: var(--text-primary);
  font-family: 'Courier New', monospace;
  font-size: 12px;
  letter-spacing: 1px;
  transition: all 0.3s ease;
  outline: none;
  resize: vertical;
  min-height: 80px;
}

.cyber-textarea:hover {
  border-color: var(--border-bright);
}

.cyber-textarea:focus {
  border-color: var(--neon-magenta);
  box-shadow: 0 0 15px rgba(255, 0, 255, 0.3), inset 0 0 10px rgba(255, 0, 255, 0.1);
}

.cyber-textarea.error {
  border-color: var(--neon-red);
  box-shadow: 0 0 15px rgba(255, 51, 102, 0.3);
}

.field-error {
  font-size: 10px;
  color: var(--neon-red);
  letter-spacing: 1px;
}

/* Action Buttons */
.edit-actions {
  display: flex;
  gap: 12px;
  justify-content: center;
  padding-top: 16px;
  border-top: 1px solid var(--border-dim);
}

.cyber-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 12px 24px;
  border: 1px solid;
  border-radius: 4px;
  font-family: 'Courier New', monospace;
  font-size: 11px;
  font-weight: 700;
  letter-spacing: 1px;
  cursor: pointer;
  transition: all 0.3s ease;
  outline: none;
  min-width: 140px;
}

.save-btn {
  background: linear-gradient(135deg, rgba(0, 255, 136, 0.1), rgba(0, 255, 136, 0.05));
  border-color: var(--neon-green);
  color: var(--neon-green);
}

.save-btn:hover:not(:disabled) {
  background: linear-gradient(135deg, rgba(0, 255, 136, 0.2), rgba(0, 255, 136, 0.1));
  box-shadow: 0 0 20px rgba(0, 255, 136, 0.4);
  transform: translateY(-1px);
}

.save-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.cancel-btn {
  background: linear-gradient(135deg, rgba(136, 136, 170, 0.1), rgba(136, 136, 170, 0.05));
  border-color: var(--text-secondary);
  color: var(--text-secondary);
}

.cancel-btn:hover:not(:disabled) {
  background: linear-gradient(135deg, rgba(136, 136, 170, 0.2), rgba(136, 136, 170, 0.1));
  border-color: var(--text-primary);
  color: var(--text-primary);
}

.btn-icon {
  font-size: 14px;
}

.btn-text {
  font-size: 11px;
}

.btn-spinner {
  width: 14px;
  height: 14px;
  border: 2px solid transparent;
  border-top-color: currentColor;
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

/* Keyboard Hint */
.keyboard-hint {
  text-align: center;
  font-size: 10px;
  color: var(--text-muted);
  letter-spacing: 1px;
  margin-top: 16px;
}

.hint-key {
  display: inline-block;
  padding: 2px 6px;
  background: rgba(0, 0, 0, 0.3);
  border: 1px solid var(--border-dim);
  border-radius: 3px;
  font-family: 'Courier New', monospace;
  color: var(--text-secondary);
}

/* Responsive */
@media (max-width: 768px) {
  .card-body {
    padding: 16px;
  }

  .edit-actions {
    flex-direction: column;
  }

  .cyber-btn {
    width: 100%;
  }
}
</style>
