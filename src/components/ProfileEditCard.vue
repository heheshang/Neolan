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
            placeholder="Username"
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
            placeholder="Hostname"
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
            placeholder="Your status"
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
  background: var(--color-bg-elevated);
  border: 1px solid var(--color-border-subtle);
  border-radius: var(--radius-xl);
  overflow: hidden;
  position: relative;
  box-shadow: var(--shadow-md);
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
  color: var(--color-text-primary);
  letter-spacing: var(--letter-spacing-normal);
  margin: 0;
}

.card-line {
  flex: 1;
  height: 1px;
  background: linear-gradient(90deg, var(--color-border-subtle), transparent);
}

.card-body {
  padding: var(--spacing-6);
}

/* Avatar Section */
.avatar-section {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--spacing-4);
  margin-bottom: var(--spacing-6);
}

.avatar-display {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--spacing-2);
}

.avatar-display.clickable {
  cursor: pointer;
}

.avatar-display.clickable:hover .avatar-large {
  transform: scale(1.05);
  box-shadow: var(--shadow-primary);
}

.avatar-large {
  font-size: 64px;
  width: 100px;
  height: 100px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--gradient-primary-subtle);
  border: 2px solid var(--color-border);
  border-radius: var(--radius-xl);
  transition: all var(--transition-normal);
}

.avatar-hint {
  font-size: var(--font-size-xs);
  color: var(--color-text-secondary);
  letter-spacing: var(--letter-spacing-normal);
  opacity: 0;
  transition: opacity var(--transition-fast);
}

.avatar-display.clickable:hover .avatar-hint {
  opacity: 1;
}

/* Edit Form */
.edit-form {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-5);
  margin-bottom: var(--spacing-6);
}

.form-field {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-2);
}

.field-label {
  display: flex;
  align-items: center;
  justify-content: space-between;
  font-size: var(--font-size-xs);
  font-weight: var(--font-weight-semibold);
  color: var(--color-text-secondary);
  letter-spacing: var(--letter-spacing-normal);
}

.label-required {
  color: var(--color-error);
}

.label-hint-inline {
  color: var(--color-text-tertiary);
  font-size: var(--font-size-xs);
}

.cyber-input {
  width: 100%;
  padding: var(--spacing-2) var(--spacing-4);
  background: var(--color-bg-primary);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  color: var(--color-text-primary);
  font-family: var(--font-sans);
  font-size: var(--font-size-base);
  letter-spacing: var(--letter-spacing-normal);
  transition: all var(--transition-fast);
  outline: none;
}

.cyber-input:hover {
  border-color: var(--color-border-strong);
}

.cyber-input:focus {
  border-color: var(--color-primary);
  box-shadow: 0 0 0 3px var(--color-primary-subtle);
}

.cyber-input.error {
  border-color: var(--color-error);
  box-shadow: 0 0 0 3px var(--color-error-light);
}

.cyber-textarea {
  width: 100%;
  padding: var(--spacing-2) var(--spacing-4);
  background: var(--color-bg-primary);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  color: var(--color-text-primary);
  font-family: var(--font-sans);
  font-size: var(--font-size-base);
  letter-spacing: var(--letter-spacing-normal);
  transition: all var(--transition-fast);
  outline: none;
  resize: vertical;
  min-height: 80px;
}

.cyber-textarea:hover {
  border-color: var(--color-border-strong);
}

.cyber-textarea:focus {
  border-color: var(--color-primary);
  box-shadow: 0 0 0 3px var(--color-primary-subtle);
}

.cyber-textarea.error {
  border-color: var(--color-error);
  box-shadow: 0 0 0 3px var(--color-error-light);
}

.field-error {
  font-size: var(--font-size-xs);
  color: var(--color-error);
  letter-spacing: var(--letter-spacing-normal);
}

/* Action Buttons */
.edit-actions {
  display: flex;
  gap: var(--spacing-3);
  justify-content: center;
  padding-top: var(--spacing-4);
  border-top: 1px solid var(--color-border-subtle);
}

.cyber-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: var(--spacing-2);
  padding: var(--spacing-3) var(--spacing-6);
  border: 1px solid;
  border-radius: var(--radius-md);
  font-family: var(--font-sans);
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-semibold);
  letter-spacing: var(--letter-spacing-normal);
  cursor: pointer;
  transition: all var(--transition-normal);
  outline: none;
  min-width: 140px;
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
  transform: translateY(-1px);
}

.save-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.cancel-btn {
  background: transparent;
  border-color: var(--color-border);
  color: var(--color-text-secondary);
}

.cancel-btn:hover:not(:disabled) {
  background: var(--color-bg-tertiary);
  border-color: var(--color-border-strong);
  color: var(--color-text-primary);
  transform: translateY(-1px);
}

.btn-icon {
  font-size: 14px;
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

@keyframes spin {
  to { transform: rotate(360deg); }
}

/* Keyboard Hint */
.keyboard-hint {
  text-align: center;
  font-size: var(--font-size-xs);
  color: var(--color-text-tertiary);
  letter-spacing: var(--letter-spacing-normal);
  margin-top: var(--spacing-4);
}

.hint-key {
  display: inline-block;
  padding: 2px var(--spacing-2);
  background: var(--color-bg-tertiary);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  font-family: var(--font-mono);
  color: var(--color-text-secondary);
  font-size: var(--font-size-xs);
}

/* Responsive */
@media (max-width: 768px) {
  .card-body {
    padding: var(--spacing-4);
  }

  .edit-actions {
    flex-direction: column;
  }

  .cyber-btn {
    width: 100%;
  }
}
</style>
