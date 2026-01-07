<template>
  <div class="transfer-panel">
    <div class="panel-header">
      <h3>文件传输</h3>
      <button class="close-btn" @click="$emit('close')">&times;</button>
    </div>
    <div class="panel-content">
      <FileTransfer />
    </div>
  </div>
</template>

<script setup lang="ts">
import FileTransfer from './FileTransfer.vue';
import type { PeerDto } from '@/stores';

defineProps<{
  peer?: PeerDto;
}>();

defineEmits<{
  (e: 'close'): void;
}>();
</script>

<style scoped>
.transfer-panel {
  position: absolute;
  top: 0;
  right: 0;
  bottom: 0;
  width: 400px;
  background: #fff;
  border-left: 1px solid #e7e7e7;
  box-shadow: -2px 0 8px rgba(0, 0, 0, 0.1);
  z-index: 10;
  display: flex;
  flex-direction: column;
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px;
  border-bottom: 1px solid #e7e7e7;
}

.panel-header h3 {
  margin: 0;
  font-size: 16px;
  font-weight: 500;
  color: #333;
}

.close-btn {
  width: 32px;
  height: 32px;
  border: none;
  background: transparent;
  border-radius: 4px;
  cursor: pointer;
  font-size: 20px;
  color: #999;
  display: flex;
  align-items: center;
  justify-content: center;
}

.close-btn:hover {
  background: #f5f5f5;
  color: #333;
}

.panel-content {
  flex: 1;
  overflow: hidden;
}

@media (prefers-color-scheme: dark) {
  .transfer-panel {
    background: #2a2a2a;
    border-left-color: #3a3a3a;
  }

  .panel-header {
    border-bottom-color: #3a3a3a;
  }

  .panel-header h3 {
    color: #e0e0e0;
  }

  .close-btn:hover {
    background: #3a3a3a;
    color: #e0e0e0;
  }
}
</style>
