<template>
  <header class="user-profile-header" @click="navigateToProfile">
    <div class="user-info">
      <div class="user-avatar">
        <span class="avatar-icon">👤</span>
      </div>
      <div class="user-details">
        <div class="user-name">{{ displayName }}</div>
        <div class="user-host">@{{ hostname }}</div>
      </div>
    </div>
    <div class="header-indicator">
      <span class="indicator-arrow">›</span>
    </div>
  </header>
</template>

<script setup lang="ts">
import { computed, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { useConfigStore } from '@/stores';

const router = useRouter();
const configStore = useConfigStore();

const displayName = computed(() => {
  const config = configStore.currentConfig;
  return config?.username || 'User';
});

const hostname = computed(() => {
  const config = configStore.currentConfig;
  return config?.hostname || 'localhost';
});

function navigateToProfile() {
  router.push({ name: 'UserProfile' });
}

// Load config on mount
onMounted(async () => {
  if (!configStore.isConfigLoaded) {
    await configStore.fetchConfig();
  }
});
</script>

<style scoped>
.user-profile-header {
  --neon-cyan: #00f3ff;
  --neon-magenta: #ff00ff;
  --bg-dark: #0a0a12;
  --bg-panel: rgba(18, 18, 26, 0.95);
  --border-dim: #2a2a3a;
  --text-primary: #e0e0ff;
  --text-secondary: #8888aa;

  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 20px;
  background: rgba(0, 0, 0, 0.4);
  border-bottom: 1px solid var(--border-dim);
  cursor: pointer;
  transition: all 0.3s ease;
  position: relative;
}

.user-profile-header::before {
  content: '';
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  height: 1px;
  background: linear-gradient(90deg, transparent, var(--neon-cyan), transparent);
  opacity: 0.5;
}

.user-profile-header:hover {
  background: rgba(0, 243, 255, 0.05);
  border-bottom-color: var(--neon-cyan);
}

.user-profile-header:hover .user-name {
  color: var(--neon-cyan);
  text-shadow: 0 0 10px var(--neon-cyan);
}

.user-profile-header:hover .indicator-arrow {
  transform: translateX(4px);
  color: var(--neon-cyan);
}

.user-info {
  display: flex;
  align-items: center;
  gap: 12px;
}

.user-avatar {
  width: 40px;
  height: 40px;
  background: linear-gradient(135deg, rgba(0, 243, 255, 0.1), rgba(255, 0, 255, 0.1));
  border: 1px solid var(--border-dim);
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.3s ease;
}

.user-profile-header:hover .user-avatar {
  border-color: var(--neon-cyan);
  box-shadow: 0 0 15px rgba(0, 243, 255, 0.3);
}

.avatar-icon {
  font-size: 20px;
  opacity: 0.8;
}

.user-details {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.user-name {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
  letter-spacing: 1px;
  transition: all 0.3s ease;
}

.user-host {
  font-size: 11px;
  color: var(--text-secondary);
  letter-spacing: 0.5px;
}

.header-indicator {
  display: flex;
  align-items: center;
}

.indicator-arrow {
  font-size: 20px;
  color: var(--text-secondary);
  transition: all 0.3s ease;
}

@media (max-width: 768px) {
  .user-profile-header {
    padding: 10px 16px;
  }

  .user-avatar {
    width: 36px;
    height: 36px;
  }

  .avatar-icon {
    font-size: 18px;
  }

  .user-name {
    font-size: 13px;
  }

  .user-host {
    font-size: 10px;
  }

  .indicator-arrow {
    font-size: 18px;
  }
}
</style>
