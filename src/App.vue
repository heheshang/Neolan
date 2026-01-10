<script setup lang="ts">
import { RouterView } from "vue-router";
import { onMounted, onUnmounted } from "vue";
import { listen } from "@tauri-apps/api/event";
import { useEventStore } from "./stores/eventStore";
import UserProfileHeader from "./components/UserProfileHeader.vue";

const eventStore = useEventStore();

// Application lifecycle
onMounted(async () => {
  // Start polling for backend events (for peer events, config changes, etc.)
  eventStore.startPolling(100); // Poll every 100ms

  // Setup global message event listeners using Tauri's event system
  const unlistenMessageReceived = await listen("message-received", () => {
    // Global event listener for message reception
  });

  const unlistenMessageSent = await listen("message-sent", () => {
    // Global event listener for message sent confirmation
  });

  // Store unsubscribe functions for cleanup
  (window as any).__tauriEventUnsubscribers = [unlistenMessageReceived, unlistenMessageSent];
});

onUnmounted(() => {
  // Stop polling
  eventStore.stopPolling();

  // Cleanup Tauri event listeners
  const unsubscribers = (window as any).__tauriEventUnsubscribers;
  if (unsubscribers) {
    unsubscribers.forEach((fn: () => void) => fn());
    delete (window as any).__tauriEventUnsubscribers;
  }
});
</script>

<template>
  <div class="app">
    <UserProfileHeader />
    <div class="app-content">
      <RouterView />
    </div>
  </div>
</template>

<style>
.app {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
  background: var(--color-bg-primary);
  color: var(--color-text-primary);
  font-family: var(--font-sans);
  font-size: var(--font-size-base);
  line-height: var(--line-height-normal);
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-rendering: optimizeLegibility;
}

.app-content {
  flex: 1;
  display: flex;
  flex-direction: column;
}
</style>
