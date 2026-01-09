<template>
  <div class="settings-tab">
    <div class="form-row">
      <div class="form-field">
        <label class="field-label">
          <span class="label-text">BIND ADDRESS</span>
        </label>
        <input
          v-model="localData.bindIp"
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
          v-model.number="localData.udpPort"
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
              v-model.number="localData.tcpPortStart"
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
              v-model.number="localData.tcpPortEnd"
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
          v-model.number="localData.heartbeatInterval"
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
          v-model.number="localData.peerTimeout"
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
                <span class="label-text">CUSTOM BROADCAST ADDRESS</span>
              </label>
              <input
                type="text"
                class="cyber-input"
                placeholder="255.255.255.255"
              />
              <span class="field-hint">Leave empty for default broadcast</span>
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
    case 'udpPort':
      if (!localData.udpPort || localData.udpPort < 1024 || localData.udpPort > 65535) {
        errors.udpPort = 'PORT_RANGE_1024_65535';
        emit('update:valid', false);
        return false;
      }
      delete errors.udpPort;
      break;

    case 'tcpPortStart':
      if (!localData.tcpPortStart || localData.tcpPortStart < 1024 || localData.tcpPortStart > 65535) {
        errors.tcpPortStart = 'PORT_RANGE_1024_65535';
        emit('update:valid', false);
        return false;
      }
      if (localData.tcpPortStart >= localData.tcpPortEnd) {
        errors.tcpPortStart = 'START_MUST_BE_LESS_THAN_END';
        emit('update:valid', false);
        return false;
      }
      delete errors.tcpPortStart;
      delete errors.tcpPortEnd;
      break;

    case 'tcpPortEnd':
      if (!localData.tcpPortEnd || localData.tcpPortEnd < 1024 || localData.tcpPortEnd > 65535) {
        errors.tcpPortEnd = 'PORT_RANGE_1024_65535';
        emit('update:valid', false);
        return false;
      }
      if (localData.tcpPortEnd <= localData.tcpPortStart) {
        errors.tcpPortEnd = 'END_MUST_BE_GREATER_THAN_START';
        emit('update:valid', false);
        return false;
      }
      delete errors.tcpPortStart;
      delete errors.tcpPortEnd;
      break;

    case 'heartbeatInterval':
      if (!localData.heartbeatInterval || localData.heartbeatInterval < 10 || localData.heartbeatInterval > 3600) {
        errors.heartbeatInterval = 'HEARTBEAT_RANGE_10_3600';
        emit('update:valid', false);
        return false;
      }
      delete errors.heartbeatInterval;
      if (localData.peerTimeout && localData.peerTimeout <= localData.heartbeatInterval) {
        errors.peerTimeout = 'TIMEOUT_MUST_BE_GREATER';
        emit('update:valid', false);
      }
      break;

    case 'peerTimeout':
      if (!localData.peerTimeout || localData.peerTimeout < 10 || localData.peerTimeout > 7200) {
        errors.peerTimeout = 'TIMEOUT_RANGE_10_7200';
        emit('update:valid', false);
        return false;
      }
      if (localData.peerTimeout <= localData.heartbeatInterval) {
        errors.peerTimeout = 'TIMEOUT_MUST_EXCEED_HEARTBEAT';
        emit('update:valid', false);
        return false;
      }
      delete errors.peerTimeout;
      break;
  }
  emit('update:valid', Object.keys(errors).length === 0);
  return true;
}

function validateAll(): boolean {
  const fields: (keyof typeof localData)[] = ['udpPort', 'tcpPortStart', 'tcpPortEnd', 'heartbeatInterval', 'peerTimeout'];
  for (const field of fields) {
    validateField(field);
  }
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

.field-range {
  grid-column: 1 / -1;
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
}

.advanced-enter-active,
.advanced-leave-active {
  transition: all 0.3s ease;
  max-height: 200px;
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

  .range-field {
    flex-wrap: wrap;
  }

  .range-separator {
    width: 100%;
    text-align: center;
  }
}
</style>
