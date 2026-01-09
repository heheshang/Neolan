<template>
  <div class="settings-tab">
    <div class="form-row">
      <div class="form-field">
        <label class="field-label">
          <span class="label-text">OFFLINE MESSAGE RETENTION (DAYS)</span>
          <span class="label-required">*</span>
        </label>
        <input
          v-model.number="localData.offlineMessageRetentionDays"
          type="number"
          class="cyber-input"
          :class="{ error: errors.offlineMessageRetentionDays }"
          @blur="validateField('offlineMessageRetentionDays')"
          min="1"
          max="365"
        />
        <span v-if="errors.offlineMessageRetentionDays" class="field-error">
          {{ errors.offlineMessageRetentionDays }}
        </span>
      </div>
    </div>

    <!-- Advanced Options -->
    <div class="advanced-section">
      <button
        @click="showAdvanced = !showAdvanced"
        class="advanced-toggle"
        :class="{ active: showAdvanced }"
      >
        <span class="toggle-icon">{{ showAdvanced ? '▼' : '▶' }}</span>
        <span class="toggle-text">{{ showAdvanced ? 'Hide' : 'Show' }} Advanced Options</span>
      </button>

      <Transition name="advanced">
        <div v-if="showAdvanced" class="advanced-content">
          <div class="form-row">
            <div class="form-field">
              <div class="toggle-field">
                <label class="field-label">MESSAGE RECEIPTS</label>
                <div class="cyber-toggle active">
                  <span class="toggle-slider"></span>
                  <span class="toggle-label">ENABLED</span>
                </div>
              </div>
            </div>
          </div>

          <div class="form-row">
            <div class="form-field">
              <div class="toggle-field">
                <label class="field-label">READ RECEIPTS</label>
                <div class="cyber-toggle">
                  <span class="toggle-slider"></span>
                  <span class="toggle-label">DISABLED</span>
                </div>
              </div>
            </div>
          </div>

          <div class="form-row">
            <div class="form-field">
              <label class="field-label">
                <span class="label-text">MESSAGE HISTORY LIMIT</span>
              </label>
              <input
                type="number"
                class="cyber-input"
                value="1000"
                min="100"
                max="10000"
              />
              <span class="field-hint">Maximum messages to store per conversation</span>
            </div>
          </div>
        </div>
      </Transition>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, watch } from 'vue';
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
const showAdvanced = ref(false);

// Validation
function validateField(field: keyof typeof localData): boolean {
  switch (field) {
    case 'offlineMessageRetentionDays':
      if (!localData.offlineMessageRetentionDays || localData.offlineMessageRetentionDays < 1 || localData.offlineMessageRetentionDays > 365) {
        errors.offlineMessageRetentionDays = 'RETENTION_RANGE_1_365';
        emit('update:valid', false);
        return false;
      }
      delete errors.offlineMessageRetentionDays;
      break;
  }
  emit('update:valid', Object.keys(errors).length === 0);
  return true;
}

function validateAll(): boolean {
  validateField('offlineMessageRetentionDays');
  return Object.keys(errors).length === 0;
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

.toggle-field {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 16px;
  background: rgba(0, 0, 0, 0.3);
  border: 1px solid var(--border-dim);
  border-radius: 8px;
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

/* Advanced Options */
.advanced-section {
  border-top: 1px solid var(--border-dim);
  padding-top: 16px;
}

.advanced-toggle {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 0;
  background: transparent;
  border: none;
  color: var(--text-secondary);
  font-family: 'Courier New', monospace;
  font-size: 11px;
  font-weight: 600;
  letter-spacing: 1px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.advanced-toggle:hover {
  color: var(--neon-cyan);
}

.advanced-toggle.active .toggle-text {
  color: var(--neon-cyan);
}

.toggle-icon {
  font-size: 10px;
  transition: transform 0.2s ease;
}

.advanced-toggle.active .toggle-icon {
  transform: rotate(90deg);
}

.advanced-content {
  padding-top: 16px;
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.advanced-enter-active,
.advanced-leave-active {
  transition: all 0.3s ease;
  max-height: 400px;
  overflow: hidden;
}

.advanced-enter-from,
.advanced-leave-to {
  max-height: 0;
  opacity: 0;
}

/* Responsive */
@media (max-width: 768px) {
  .form-row {
    grid-template-columns: 1fr;
  }
}
</style>
