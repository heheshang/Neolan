<script setup lang="ts">
import { RouterView } from "vue-router";
import { onMounted, onUnmounted } from "vue";
import { listen } from "@tauri-apps/api/event";
import { useEventStore } from "./stores/eventStore";

const eventStore = useEventStore();

// Application lifecycle
onMounted(async () => {
  // Start polling for backend events (for peer events, config changes, etc.)
  eventStore.startPolling(100); // Poll every 100ms

  // Setup global message event listeners using Tauri's event system
  const unlistenMessageReceived = await listen("message-received", (event) => {
    console.log("=".repeat(80));
    console.log("📨 [APP - GLOBAL] ========== MESSAGE RECEIVED IN APP.VUE ==========");
    console.log("📨 [APP - GLOBAL] Raw event payload:", event.payload);
    console.log("📨 [APP - GLOBAL] Event type:", typeof event.payload);

    // Tauri sends the enum variant as the payload, so we need to extract the data
    const payload = event.payload as any;
    const messageData = payload.MessageReceived || payload;

    console.log("📨 [APP - GLOBAL] Extracted messageData:", messageData);
    console.log("📨 [APP - GLOBAL] Message details:", {
      msgId: messageData.msgId,
      senderName: messageData.senderName,
      senderIp: messageData.senderIp,
      content: messageData.content,
      sentAt: messageData.sentAt,
      timestamp: new Date().toISOString()
    });
    console.log("=".repeat(80));
  });

  const unlistenMessageSent = await listen("message-sent", (event) => {
    console.log("📤 [FRONTEND] Message Sent via Tauri event:", event.payload);
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
    <RouterView />
  </div>
</template>

<style>
:root {
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f5f5f5;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

.app {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #e0e0e0;
    background-color: #1a1a1a;
  }
}
</style>
