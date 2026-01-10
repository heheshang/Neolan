<template>
  <div class="wechat-chat">
    <!-- Left Sidebar - Conversation List -->
    <aside class="chat-sidebar">
      <ConversationList
        :peers="onlinePeers"
        :selected-peer="selectedPeer"
        @select="handleSelectPeer"
      />
    </aside>

    <!-- Right Panel - Chat Window -->
    <main class="chat-main">
      <ChatWindow
        v-if="selectedPeer"
        :peer="selectedPeer"
        @close="selectedPeer = null"
      />
    </main>

    <!-- File Transfer Panel (Hidden by default) -->
    <FileTransferPanel
      v-if="showTransferPanel && selectedPeer"
      :peer="selectedPeer"
      @close="showTransferPanel = false"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { usePeerStore } from '@/stores';
import type { PeerDto } from '@/stores';
import ConversationList from '@/components/ConversationList.vue';
import ChatWindow from '@/components/WeChatWindow.vue';
import FileTransferPanel from '@/components/FileTransferPanel.vue';

const peerStore = usePeerStore();
const selectedPeer = ref<PeerDto | null>(null);
const showTransferPanel = ref(false);

const onlinePeers = computed(() =>
  peerStore.peers.filter(p => p.status === 'online')
);

function handleSelectPeer(peer: PeerDto) {
  selectedPeer.value = peer;
}

onMounted(async () => {
  await peerStore.fetchPeers();
});
</script>

<style scoped>
.wechat-chat {
  display: flex;
  height: 100vh;
  background: var(--color-bg-primary);
  overflow: hidden;
}

.chat-sidebar {
  width: var(--sidebar-width);
  background: var(--color-bg-secondary);
  border-right: 1px solid var(--color-border-subtle);
  flex-shrink: 0;
  transition: transform var(--transition-normal);
}

.chat-main {
  flex: 1;
  display: flex;
  flex-direction: column;
  background: var(--color-bg-primary);
  position: relative;
  overflow: hidden;
}

/* Responsive */
@media (max-width: 768px) {
  .chat-sidebar {
    position: fixed;
    left: 0;
    top: var(--header-height);
    height: calc(100vh - var(--header-height));
    z-index: var(--z-fixed);
    transform: translateX(-100%);
  }

  .chat-sidebar.open {
    transform: translateX(0);
  }
}
</style>
