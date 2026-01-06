import { defineStore } from "pinia";
import { ref, computed } from "vue";
import * as api from "../api";

export const useConfigStore = defineStore("config", () => {
  // ==================== State ====================

  const config = ref<api.ConfigDto | null>(null);
  const loading = ref(false);
  const saving = ref(false);
  const error = ref<string | null>(null);
  const lastUpdated = ref<number | null>(null);

  // Default config (fallback)
  const defaultConfig: api.ConfigDto = {
    username: "User",
    hostname: "localhost",
    bindIp: "0.0.0.0",
    udpPort: 2425,
    tcpPortStart: 8000,
    tcpPortEnd: 9000,
    heartbeatInterval: 60,
    peerTimeout: 180,
    encryptionEnabled: false,
    encryptionKey: undefined,
    offlineMessageRetentionDays: 30,
    autoAcceptFiles: false,
    fileSaveDir: "",
    logLevel: "info",
  };

  // ==================== Getters ====================

  const currentConfig = computed(() => config.value || defaultConfig);

  const networkConfig = computed(() => ({
    bindIp: currentConfig.value.bindIp,
    udpPort: currentConfig.value.udpPort,
    tcpPortStart: currentConfig.value.tcpPortStart,
    tcpPortEnd: currentConfig.value.tcpPortEnd,
    heartbeatInterval: currentConfig.value.heartbeatInterval,
    peerTimeout: currentConfig.value.peerTimeout,
  }));

  const securityConfig = computed(() => ({
    encryptionEnabled: currentConfig.value.encryptionEnabled,
    encryptionKey: currentConfig.value.encryptionKey,
  }));

  const messageConfig = computed(() => ({
    offlineMessageRetentionDays: currentConfig.value.offlineMessageRetentionDays,
  }));

  const fileTransferConfig = computed(() => ({
    autoAcceptFiles: currentConfig.value.autoAcceptFiles,
    fileSaveDir: currentConfig.value.fileSaveDir,
  }));

  const appConfig = computed(() => ({
    username: currentConfig.value.username,
    hostname: currentConfig.value.hostname,
    logLevel: currentConfig.value.logLevel,
  }));

  const isConfigLoaded = computed(() => config.value !== null);

  // ==================== Actions ====================

  async function fetchConfig() {
    loading.value = true;
    error.value = null;
    try {
      const result = await api.getConfig();
      config.value = result;
      lastUpdated.value = Date.now();
      return result;
    } catch (err) {
      error.value = err instanceof Error ? err.message : String(err);
      console.error("Failed to fetch config:", err);
      return null;
    } finally {
      loading.value = false;
    }
  }

  async function saveConfig(newConfig: api.ConfigDto) {
    saving.value = true;
    error.value = null;
    try {
      await api.setConfig(newConfig);
      config.value = newConfig;
      lastUpdated.value = Date.now();
      return true;
    } catch (err) {
      error.value = err instanceof Error ? err.message : String(err);
      console.error("Failed to save config:", err);
      return false;
    } finally {
      saving.value = false;
    }
  }

  async function resetToDefault() {
    saving.value = true;
    error.value = null;
    try {
      const result = await api.resetConfig();
      config.value = result;
      lastUpdated.value = Date.now();
      return result;
    } catch (err) {
      error.value = err instanceof Error ? err.message : String(err);
      console.error("Failed to reset config:", err);
      return null;
    } finally {
      saving.value = false;
    }
  }

  async function getValue(key: keyof api.ConfigDto): Promise<string | null> {
    try {
      const result = await api.getConfigValue(key);
      return result;
    } catch (err) {
      console.error(`Failed to get config value ${key}:`, err);
      return null;
    }
  }

  async function setValue(key: keyof api.ConfigDto, value: string): Promise<boolean> {
    error.value = null;
    try {
      await api.setConfigValue(key, value);
      // Reload config to get updated values
      await fetchConfig();
      return true;
    } catch (err) {
      error.value = err instanceof Error ? err.message : String(err);
      console.error(`Failed to set config value ${key}:`, err);
      return false;
    }
  }

  function updateLocalConfig(updater: (config: api.ConfigDto) => api.ConfigDto) {
    if (config.value) {
      config.value = updater(config.value);
    }
  }

  function clearConfig() {
    config.value = null;
    lastUpdated.value = null;
  }

  // ==================== Return ====================

  return {
    // State
    config,
    loading,
    saving,
    error,
    lastUpdated,

    // Getters
    currentConfig,
    networkConfig,
    securityConfig,
    messageConfig,
    fileTransferConfig,
    appConfig,
    isConfigLoaded,

    // Actions
    fetchConfig,
    saveConfig,
    resetToDefault,
    getValue,
    setValue,
    updateLocalConfig,
    clearConfig,
  };
});
