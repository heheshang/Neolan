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
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-3) var(--spacing-5);
  background: var(--color-bg-secondary);
  border-bottom: 1px solid var(--color-border-subtle);
  cursor: pointer;
  transition: all var(--transition-normal);
  position: relative;
}

.user-profile-header::before {
  content: '';
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  height: 1px;
  background: var(--gradient-primary);
  transform: scaleX(0);
  transition: transform var(--transition-normal);
}

.user-profile-header:hover::before {
  transform: scaleX(1);
}

.user-profile-header:hover {
  background: var(--color-primary-subtle);
}

.user-profile-header:hover .user-name {
  color: var(--color-primary);
}

.user-profile-header:hover .indicator-arrow {
  transform: translateX(4px);
  color: var(--color-primary);
}

.user-info {
  display: flex;
  align-items: center;
  gap: var(--spacing-3);
}

.user-avatar {
  width: 40px;
  height: 40px;
  background: var(--gradient-primary);
  border: 2px solid var(--color-border);
  border-radius: var(--radius-lg);
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all var(--transition-normal);
  box-shadow: var(--shadow-sm);
}

.user-profile-header:hover .user-avatar {
  transform: scale(1.05);
  box-shadow: var(--shadow-primary);
}

.avatar-icon {
  font-size: 20px;
}

.user-details {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.user-name {
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-semibold);
  color: var(--color-text-primary);
  letter-spacing: var(--letter-spacing-normal);
  transition: color var(--transition-fast);
}

.user-host {
  font-size: var(--font-size-xs);
  color: var(--color-text-tertiary);
  font-family: var(--font-mono);
}

.header-indicator {
  display: flex;
  align-items: center;
}

.indicator-arrow {
  font-size: 20px;
  color: var(--color-text-tertiary);
  transition: all var(--transition-normal);
}

@media (max-width: 768px) {
  .user-profile-header {
    padding: var(--spacing-2) var(--spacing-4);
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
