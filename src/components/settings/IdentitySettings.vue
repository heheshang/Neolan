<template>
  <div class="settings-tab">
    <div class="form-row">
      <div class="form-field">
        <label class="field-label">
          <span class="label-text">USERNAME</span>
          <span class="label-required">*</span>
        </label>
        <input
          v-model="localData.username"
          type="text"
          class="cyber-input"
          :class="{ error: errors.username }"
          @blur="validateField('username')"
          placeholder="Username"
          maxlength="50"
        />
        <span v-if="errors.username" class="field-error">{{ errors.username }}</span>
      </div>
      <div class="form-field">
        <label class="field-label">
          <span class="label-text">HOSTNAME</span>
        </label>
        <input
          v-model="localData.hostname"
          type="text"
          class="cyber-input"
          placeholder="Hostname"
          maxlength="50"
        />
      </div>
    </div>

    <div class="form-row">
      <div class="form-field">
        <label class="field-label">
          <span class="label-text">AVATAR</span>
        </label>
        <div class="avatar-display" @click="showAvatarPicker = !showAvatarPicker">
          <span class="avatar-emoji">{{ displayAvatar }}</span>
          <span class="avatar-hint">Click to change</span>
        </div>
        <AvatarPicker
          v-if="showAvatarPicker"
          :current-avatar="localData.avatar"
          @select="handleAvatarSelect"
          @cancel="showAvatarPicker = false"
        />
      </div>
    </div>

    <div class="form-row">
      <div class="form-field field-full-width">
        <label class="field-label">
          <span class="label-text">STATUS MESSAGE</span>
          <span class="label-hint-inline">{{ (localData.status || '').length }} / 200</span>
        </label>
        <textarea
          v-model="localData.status"
          class="cyber-textarea"
          placeholder="Your status message"
          rows="3"
          maxlength="200"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, watch } from 'vue';
import AvatarPicker from '@/components/AvatarPicker.vue';
import type { ConfigDto } from '@/api';

interface Props {
  modelValue: ConfigDto;
}

interface Emits {
  (e: 'update:modelValue', value: ConfigDto): void;
  (e: 'update:valid', value: boolean): void;
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();

// Local form data
const localData = reactive<ConfigDto>({ ...props.modelValue });
const errors = reactive<Record<string, string>>({});
const showAvatarPicker = ref(false);

// Computed
const displayAvatar = computed(() => localData.avatar || '👤');

// Validation
function validateField(field: keyof typeof localData): boolean {
  switch (field) {
    case 'username':
      if (!localData.username || localData.username.trim().length === 0) {
        errors.username = 'USERNAME_REQUIRED';
        emit('update:valid', false);
        return false;
      }
      if (localData.username.length > 50) {
        errors.username = 'MAX_50_CHARS';
        emit('update:valid', false);
        return false;
      }
      delete errors.username;
      break;
  }
  emit('update:valid', Object.keys(errors).length === 0);
  return true;
}

function validateAll(): boolean {
  validateField('username');
  return Object.keys(errors).length === 0;
}

function handleAvatarSelect(emoji: string) {
  localData.avatar = emoji;
  showAvatarPicker.value = false;
  emitData();
}

function emitData() {
  emit('update:modelValue', { ...localData });
}

// Watch for changes
watch(() => props.modelValue, (newValue) => {
  Object.assign(localData, newValue);
}, { deep: true });

watch(localData, () => {
  emitData();
}, { deep: true });

// Expose validation method
defineExpose({
  validate: validateAll,
  isValid: () => Object.keys(errors).length === 0,
});
</script>

<style scoped>
.settings-tab {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-5);
}

.form-row {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: var(--spacing-5);
}

.form-field {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-2);
}

.field-full-width {
  grid-column: 1 / -1;
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

.field-error {
  font-size: var(--font-size-xs);
  color: var(--color-error);
  letter-spacing: var(--letter-spacing-normal);
}

.avatar-display {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--spacing-2);
  padding: var(--spacing-4);
  background: var(--color-bg-tertiary);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  cursor: pointer;
  transition: all var(--transition-normal);
  width: fit-content;
}

.avatar-display:hover {
  border-color: var(--color-primary);
  box-shadow: var(--shadow-primary);
}

.avatar-emoji {
  font-size: 48px;
}

.avatar-hint {
  font-size: var(--font-size-xs);
  color: var(--color-text-secondary);
  opacity: 0;
  transition: opacity var(--transition-fast);
}

.avatar-display:hover .avatar-hint {
  opacity: 1;
}

/* Responsive */
@media (max-width: 768px) {
  .form-row {
    grid-template-columns: 1fr;
  }
}
</style>
