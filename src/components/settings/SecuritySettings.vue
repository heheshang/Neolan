<template>
  <div class="settings-tab">
    <div class="form-row">
      <div class="form-field field-full-width">
        <div class="toggle-field">
          <label class="field-label">ENCRYPTION ENABLED</label>
          <div
            class="cyber-toggle"
            :class="{ active: localData.encryptionEnabled }"
            @click="toggleEncryption"
          >
            <span class="toggle-slider"></span>
            <span class="toggle-label">{{ localData.encryptionEnabled ? 'ACTIVE' : 'INACTIVE' }}</span>
          </div>
        </div>
      </div>
    </div>

    <Transition name="expand">
      <div v-if="localData.encryptionEnabled" class="form-row">
        <div class="form-field field-full-width">
          <label class="field-label">
            <span class="label-text">ENCRYPTION KEY (BASE64)</span>
          </label>
          <input
            v-model="localData.encryptionKey"
            type="password"
            class="cyber-input"
            placeholder="ENTER_ENCRYPTION_KEY"
          />
          <span class="field-hint">32-byte key encoded in Base64</span>
        </div>
      </div>
    </Transition>
  </div>
</template>

<script setup lang="ts">
import { reactive, watch } from 'vue';
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

function toggleEncryption() {
  localData.encryptionEnabled = !localData.encryptionEnabled;
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

.field-full-width {
  grid-column: 1 / -1;
}

.field-label {
  display: flex;
  align-items: center;
  font-size: 10px;
  font-weight: 600;
  color: var(--text-secondary);
  letter-spacing: 1px;
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

/* Expand animation */
.expand-enter-active,
.expand-leave-active {
  transition: all 0.3s ease;
  max-height: 200px;
  overflow: hidden;
}

.expand-enter-from,
.expand-leave-to {
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
