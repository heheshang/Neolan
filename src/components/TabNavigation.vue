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
  background: var(--color-bg-secondary);
  border-bottom: 1px solid var(--color-border-subtle);
  overflow-x: auto;
  overflow-y: hidden;
  -webkit-overflow-scrolling: touch;
}

.tab-list {
  display: flex;
  gap: var(--spacing-1);
  padding: var(--spacing-3) var(--spacing-4);
  min-width: min-content;
}

.tab-button {
  display: flex;
  align-items: center;
  gap: var(--spacing-2);
  padding: var(--spacing-2) var(--spacing-4);
  background: transparent;
  border: 1px solid transparent;
  border-radius: var(--radius-md);
  color: var(--color-text-secondary);
  font-family: var(--font-sans);
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  letter-spacing: var(--letter-spacing-normal);
  cursor: pointer;
  transition: all var(--transition-fast);
  white-space: nowrap;
  position: relative;
}

.tab-button:hover {
  background: var(--color-primary-subtle);
  border-color: var(--color-border);
  color: var(--color-text-primary);
}

.tab-button.active {
  background: var(--color-primary-subtle);
  border-color: var(--color-primary);
  color: var(--color-primary);
}

.tab-button.active::after {
  content: '';
  position: absolute;
  bottom: -13px;
  left: 0;
  right: 0;
  height: 2px;
  background: var(--gradient-primary);
  box-shadow: var(--shadow-primary);
}

.tab-button.focused {
  outline: 2px solid var(--color-primary);
  outline-offset: 2px;
}

.tab-icon {
  font-size: 14px;
  opacity: 0.7;
  transition: opacity var(--transition-fast);
}

.tab-button.active .tab-icon {
  opacity: 1;
}

.tab-label {
  text-transform: capitalize;
}

/* Scrollbar Styling */
.tab-navigation::-webkit-scrollbar {
  height: 4px;
}

.tab-navigation::-webkit-scrollbar-track {
  background: transparent;
}

.tab-navigation::-webkit-scrollbar-thumb {
  background: var(--color-border);
  border-radius: 2px;
  transition: background var(--transition-fast);
}

.tab-navigation::-webkit-scrollbar-thumb:hover {
  background: var(--color-border-strong);
}

/* Responsive */
@media (max-width: 768px) {
  .tab-list {
    padding: var(--spacing-2) var(--spacing-3);
    gap: var(--spacing-0);
  }

  .tab-button {
    padding: var(--spacing-2) var(--spacing-3);
    font-size: var(--font-size-xs);
    gap: var(--spacing-1);
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
