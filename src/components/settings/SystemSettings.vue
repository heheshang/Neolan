<template>
  <div class="settings-tab">
    <div class="form-row">
      <div class="form-field">
        <label class="field-label">
          <span class="label-text">LOG LEVEL</span>
        </label>
        <div class="cyber-select">
          <select v-model="localData.logLevel" class="select-input">
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
              <label class="field-label">
                <span class="label-text">LOG FILE SIZE LIMIT (MB)</span>
              </label>
              <input
                type="number"
                class="cyber-input"
                value="100"
                min="1"
                max="1000"
              />
            </div>
          </div>

          <div class="form-row">
            <div class="form-field">
              <div class="toggle-field">
                <label class="field-label">DEBUG MODE</label>
                <div class="cyber-toggle">
                  <span class="toggle-slider"></span>
                  <span class="toggle-label">DISABLED</span>
                </div>
              </div>
            </div>
          </div>

          <div class="form-row">
            <div class="form-field">
              <div class="toggle-field">
                <label class="field-label">TELEMETRY</label>
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
                <span class="label-text">UPDATE CHANNEL</span>
              </label>
              <div class="cyber-select">
                <select class="select-input" value="stable">
                  <option value="stable">STABLE</option>
                  <option value="beta">BETA</option>
                  <option value="nightly">NIGHTLY</option>
                </select>
                <span class="select-arrow">▼</span>
              </div>
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
const showAdvanced = ref(false);

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
  validate: () => true,
  isValid: () => true,
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
  font-size: 10px;
  font-weight: 600;
  color: var(--text-secondary);
  letter-spacing: 1px;
}

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
  font-family: 'Courier New', monospace;
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
