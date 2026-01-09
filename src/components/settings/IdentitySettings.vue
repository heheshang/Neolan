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
          v-model="localData.hostname"
          type="text"
          class="cyber-input"
          placeholder="SYSTEM_HOSTNAME"
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
          placeholder="What's on your mind?"
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
  gap: 20px;
}

.form-row {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 20px;
}

.form-field {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.field-full-width {
  grid-column: 1 / -1;
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
  border-color: var(--neon-cyan);
  box-shadow: 0 0 15px rgba(0, 243, 255, 0.3), inset 0 0 10px rgba(0, 243, 255, 0.1);
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
  border-color: var(--neon-cyan);
  box-shadow: 0 0 15px rgba(0, 243, 255, 0.3), inset 0 0 10px rgba(0, 243, 255, 0.1);
}

.field-error {
  font-size: 10px;
  color: var(--neon-red);
  letter-spacing: 1px;
}

.avatar-display {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  padding: 16px;
  background: rgba(0, 0, 0, 0.3);
  border: 1px solid var(--border-dim);
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.3s ease;
  width: fit-content;
}

.avatar-display:hover {
  border-color: var(--neon-cyan);
  box-shadow: 0 0 15px rgba(0, 243, 255, 0.3);
}

.avatar-emoji {
  font-size: 48px;
}

.avatar-hint {
  font-size: 10px;
  color: var(--text-secondary);
  opacity: 0;
  transition: opacity 0.3s ease;
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
