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
  background: #f5f5f5;
  overflow: hidden;
}

.chat-sidebar {
  width: 280px;
  background: #fff;
  border-right: 1px solid #e7e7e7;
  flex-shrink: 0;
}

.chat-main {
  flex: 1;
  display: flex;
  flex-direction: column;
  background: #f5f5f5;
  position: relative;
  overflow: hidden;
}

/* Dark mode */
@media (prefers-color-scheme: dark) {
  .wechat-chat {
    background: #1a1a1a;
  }

  .chat-sidebar {
    background: #2a2a2a;
    border-right-color: #3a3a3a;
  }

  .chat-main {
    background: #1a1a1a;
  }
}
</style>
