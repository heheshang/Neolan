import { defineStore } from "pinia";
import { ref, computed } from "vue";
import * as api from "../api";

export const useEventStore = defineStore("event", () => {
  // ==================== State ====================

  const events = ref<api.AppEvent[]>([]);
  const polling = ref(false);
  const pollInterval = ref(100); // ms
  const pollTimer = ref<number | null>(null);
  const handlers = ref<Map<string, ((data?: any) => void)[]>>(new Map());

  // ==================== Getters ====================

  const hasEvents = computed(() => events.value.length > 0);

  const eventCount = computed(() => events.value.length);

  const eventsByType = computed(() => {
    const grouped = new Map<string, api.AppEvent[]>();
    for (const event of events.value) {
      const list = grouped.get(event.type) || [];
      list.push(event);
      grouped.set(event.type, list);
    }
    return grouped;
  });

  // ==================== Actions ====================

  function on(eventType: string, handler: (data?: any) => void) {
    if (!handlers.value.has(eventType)) {
      handlers.value.set(eventType, []);
    }
    handlers.value.get(eventType)!.push(handler);

    // Return unsubscribe function
    return () => {
      const list = handlers.value.get(eventType);
      if (list) {
        const index = list.indexOf(handler);
        if (index >= 0) {
          list.splice(index, 1);
        }
      }
    };
  }

  function off(eventType: string, handler?: (data?: any) => void) {
    if (!handler) {
      handlers.value.delete(eventType);
    } else {
      const list = handlers.value.get(eventType);
      if (list) {
        const index = list.indexOf(handler);
        if (index >= 0) {
          list.splice(index, 1);
        }
      }
    }
  }

  function emit(event: api.AppEvent) {
    events.value.push(event);
    dispatchEvent(event);
  }

  function dispatchEvent(event: api.AppEvent) {
    const list = handlers.value.get(event.type);
    if (list) {
      for (const handler of list) {
        try {
          handler(event.data);
        } catch (err) {
          console.error(`Error in event handler for ${event.type}:`, err);
        }
      }
    }
  }

  async function poll() {
    if (polling.value) return;

    polling.value = true;
    try {
      const newEvents = await api.pollEvents();
      if (newEvents.length > 0) {
        for (const event of newEvents) {
          emit(event);
        }
      }
    } catch (err) {
      console.error("Failed to poll events:", err);
    } finally {
      polling.value = false;
    }
  }

  function startPolling(interval?: number) {
    stopPolling();
    if (interval !== undefined) {
      pollInterval.value = interval;
    }

    // Poll immediately
    poll();

    // Set up interval
    pollTimer.value = window.setInterval(() => {
      poll();
    }, pollInterval.value);
  }

  function stopPolling() {
    if (pollTimer.value !== null) {
      clearInterval(pollTimer.value);
      pollTimer.value = null;
    }
    polling.value = false;
  }

  function clearEvents() {
    events.value = [];
  }

  function clearHandlers() {
    handlers.value.clear();
  }

  // ==================== Convenience Event Handlers ====================

  function onPeerOnline(handler: (data: { ip: string; port: number; username?: string }) => void) {
    return on("PeerOnline", handler);
  }

  function onPeerOffline(handler: (data: { ip: string }) => void) {
    return on("PeerOffline", handler);
  }

  function onPeerStatusChanged(handler: (data: { ip: string; status: string }) => void) {
    return on("PeerStatusChanged", handler);
  }

  function onPeerUpdated(handler: (data: { ip: string; username?: string }) => void) {
    return on("PeerUpdated", handler);
  }

  function onConfigChanged(handler: () => void) {
    return on("ConfigChanged", handler);
  }

  function onInitialized(handler: () => void) {
    return on("Initialized", handler);
  }

  function onError(handler: (data: { message: string }) => void) {
    return on("Error", handler);
  }

  function onMessageReceived(handler: (data: {
    msg_id: string;
    sender_ip: string;
    sender_name: string;
    content: string;
    timestamp: number;
  }) => void) {
    return on("MessageReceived", handler);
  }

  function onMessageSent(handler: (data: {
    msg_id: string;
    receiver_ip: string;
  }) => void) {
    return on("MessageSent", handler);
  }

  // ==================== Return ====================

  return {
    // State
    events,
    polling,
    pollInterval,
    hasEvents,
    eventCount,
    eventsByType,

    // Actions
    on,
    off,
    emit,
    poll,
    startPolling,
    stopPolling,
    clearEvents,
    clearHandlers,

    // Convenience handlers
    onPeerOnline,
    onPeerOffline,
    onPeerStatusChanged,
    onPeerUpdated,
    onConfigChanged,
    onInitialized,
    onError,
    onMessageReceived,
    onMessageSent,
  };
});
