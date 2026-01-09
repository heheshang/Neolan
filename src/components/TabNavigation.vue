<template>
  <nav class="tab-navigation" :class="{ scrollable: isScrollable }" ref="tabContainer">
    <div class="tab-list" ref="tabList">
      <button
        v-for="tab in tabs"
        :key="tab.id"
        :class="['tab-button', { active: tab.id === modelValue, focused: tab.id === focusedTab }]"
        :aria-selected="tab.id === modelValue"
        :role="'tab'"
        :tabindex="tab.id === modelValue ? 0 : -1"
        @click="selectTab(tab.id)"
        @keydown="handleKeydown($event, tab.id)"
      >
        <span class="tab-icon">{{ tab.icon }}</span>
        <span class="tab-label">{{ tab.label }}</span>
      </button>
    </div>
  </nav>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, nextTick } from 'vue';

export interface Tab {
  id: string;
  label: string;
  icon: string;
}

interface Props {
  tabs: Tab[];
  modelValue: string;
}

interface Emits {
  (e: 'update:modelValue', value: string): void;
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();

// Refs
const tabContainer = ref<HTMLElement | null>(null);
const tabList = ref<HTMLElement | null>(null);
const focusedTab = ref<string | null>(null);

// Computed
const isScrollable = computed(() => {
  if (!tabList.value || !tabContainer.value) return false;
  return tabList.value.scrollWidth > tabContainer.value.clientWidth;
});

// Methods
function selectTab(tabId: string) {
  emit('update:modelValue', tabId);
}

function handleKeydown(event: KeyboardEvent, tabId: string) {
  const tabIndex = props.tabs.findIndex(t => t.id === tabId);
  if (tabIndex === -1) return;

  let nextIndex = tabIndex;

  switch (event.key) {
    case 'ArrowRight':
      event.preventDefault();
      nextIndex = (tabIndex + 1) % props.tabs.length;
      break;
    case 'ArrowLeft':
      event.preventDefault();
      nextIndex = (tabIndex - 1 + props.tabs.length) % props.tabs.length;
      break;
    case 'Home':
      event.preventDefault();
      nextIndex = 0;
      break;
    case 'End':
      event.preventDefault();
      nextIndex = props.tabs.length - 1;
      break;
    case 'Enter':
    case ' ':
      event.preventDefault();
      selectTab(tabId);
      return;
    default:
      return;
  }

  const nextTab = props.tabs[nextIndex];
  if (nextTab) {
    focusedTab.value = nextTab.id;
    nextTick(() => {
      const button = tabList.value?.querySelector(`[data-tab-id="${nextTab.id}"]`) as HTMLButtonElement;
      button?.focus();
    });
  }
}

function checkScrollable() {
  if (!tabList.value || !tabContainer.value) return;
  const needsScroll = tabList.value.scrollWidth > tabContainer.value.clientWidth;
  if (needsScroll) {
    tabContainer.value.classList.add('scrollable');
  } else {
    tabContainer.value.classList.remove('scrollable');
  }
}

// Lifecycle
onMounted(() => {
  checkScrollable();
  window.addEventListener('resize', checkScrollable);
});

onUnmounted(() => {
  window.removeEventListener('resize', checkScrollable);
});

// Expose method for programmatic scrolling
defineExpose({
  scrollToTab: (tabId: string) => {
    const button = tabList.value?.querySelector(`[data-tab-id="${tabId}"]`) as HTMLElement;
    if (button && tabList.value) {
      const containerWidth = tabContainer.value?.clientWidth || 0;
      const buttonLeft = button.offsetLeft;
      const buttonWidth = button.offsetWidth;
      const scrollLeft = buttonLeft - (containerWidth - buttonWidth) / 2;
      tabList.value.scrollTo({ left: scrollLeft, behavior: 'smooth' });
    }
  },
});
</script>

<style scoped>
.tab-navigation {
  --neon-cyan: #00f3ff;
  --border-dim: #2a2a3a;
  --bg-dark: #0a0a12;

  background: rgba(0, 0, 0, 0.4);
  border-bottom: 1px solid var(--border-dim);
  overflow-x: auto;
  overflow-y: hidden;
  -webkit-overflow-scrolling: touch;
}

.tab-list {
  display: flex;
  gap: 4px;
  padding: 12px 16px;
  min-width: min-content;
}

.tab-button {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 16px;
  background: transparent;
  border: 1px solid transparent;
  border-radius: 6px;
  color: var(--text-secondary);
  font-family: 'Courier New', monospace;
  font-size: 11px;
  font-weight: 600;
  letter-spacing: 1px;
  cursor: pointer;
  transition: all 0.2s ease;
  white-space: nowrap;
  position: relative;
}

.tab-button:hover {
  background: rgba(0, 243, 255, 0.05);
  border-color: rgba(0, 243, 255, 0.2);
  color: var(--text-primary);
}

.tab-button.active {
  background: rgba(0, 243, 255, 0.1);
  border-color: var(--neon-cyan);
  color: var(--neon-cyan);
}

.tab-button.active::after {
  content: '';
  position: absolute;
  bottom: -13px;
  left: 0;
  right: 0;
  height: 2px;
  background: var(--neon-cyan);
  box-shadow: 0 0 10px var(--neon-cyan);
}

.tab-button.focused {
  outline: 2px solid var(--neon-cyan);
  outline-offset: 2px;
}

.tab-icon {
  font-size: 14px;
  opacity: 0.8;
}

.tab-button.active .tab-icon {
  opacity: 1;
}

.tab-label {
  text-transform: uppercase;
}

/* Scrollbar Styling */
.tab-navigation::-webkit-scrollbar {
  height: 4px;
}

.tab-navigation::-webkit-scrollbar-track {
  background: transparent;
}

.tab-navigation::-webkit-scrollbar-thumb {
  background: var(--border-dim);
  border-radius: 2px;
}

.tab-navigation::-webkit-scrollbar-thumb:hover {
  background: var(--text-secondary);
}

/* Responsive */
@media (max-width: 768px) {
  .tab-list {
    padding: 10px 12px;
    gap: 2px;
  }

  .tab-button {
    padding: 8px 12px;
    font-size: 10px;
    gap: 6px;
  }

  .tab-icon {
    font-size: 12px;
  }

  .tab-button.active::after {
    bottom: -10px;
    height: 2px;
  }
}
</style>
