<template>
  <PeerList />
</template>

<script setup lang="ts">
import PeerList from '@/components/PeerList.vue';
</script>

<style scoped>
/* Full viewport height for the peer list */
:deep(.peer-list-container) {
  height: 100vh;
  display: flex;
  flex-direction: column;
}
</style>

