<template>
  <div class="file-transfers-view">
    <FileTransfer />
  </div>
</template>

<script setup lang="ts">
import FileTransfer from '@/components/FileTransfer.vue';
</script>

<style scoped>
.file-transfers-view {
  height: calc(100vh - 100px); /* Subtract navbar and padding */
  display: flex;
  flex-direction: column;
}
</style>
