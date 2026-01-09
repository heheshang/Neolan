<template>
  <div class="avatar-picker-overlay" @click="handleOverlayClick">
    <div class="avatar-picker" @click.stop>
      <div class="picker-header">
        <h3 class="picker-title">Choose Your Avatar</h3>
        <button @click="$emit('cancel')" class="close-btn">✕</button>
      </div>

      <div class="picker-body">
        <!-- Emoji Grid -->
        <div class="emoji-grid">
          <button
            v-for="emoji in emojiList"
            :key="emoji"
            class="emoji-btn"
            :class="{ active: emoji === currentAvatar }"
            @click="$emit('select', emoji)"
          >
            {{ emoji }}
          </button>
        </div>
      </div>

      <div class="picker-footer">
        <button @click="$emit('cancel')" class="cyber-btn cancel-btn">
          Cancel
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
interface Props {
  currentAvatar?: string | null;
}

interface Emits {
  (e: 'select', emoji: string): void;
  (e: 'cancel'): void;
}

defineProps<Props>();
const emit = defineEmits<Emits>();

function handleOverlayClick() {
  emit('cancel');
}

// Common emojis for avatars
const emojiList = [
  // People
  '👤', '👨', '👩', '🧑', '👶', '👴', '👵',
  '👨‍💼', '👩‍💼', '🧑‍💼', '👨‍💻', '👩‍💻', '🧑‍💻',
  '👨‍🎨', '👩‍🎨', '🧑‍🎨', '👨‍🚀', '👩‍🚀', '🧑‍🚀',
  '🦸', '🦸‍♂️', '🦸‍♀️', '🦹', '🦹‍♂️', '🦹‍♀️',

  // Animals
  '🐱', '🐶', '🐭', '🐹', '🐰', '🦊', '🐻', '🐼',
  '🐨', '🐯', '🦁', '🐮', '🐷', '🐸', '🐵', '🐔',

  // Nature
  '🌟', '⭐', '💫', '✨', '☀️', '🌙', '🔥', '💧',
  '🌸', '🌺', '🌻', '🌷', '🍀', '🌲', '🌵', '🍄',

  // Objects
  '💎', '🔮', '🎯', '🎨', '🎭', '🎪', '🎢', '🎡',
  '🚀', '🛸', '🪐', '⌚', '💻', '📱', '💡', '🔦',

  // Symbols
  '❤️', '🧡', '💛', '💚', '💙', '💜', '🖤', '💯',
  '✅', '❌', '⚡', '🔋', '🔌', '💾', '📀', '🎵',
];
</script>

<style scoped>
.avatar-picker-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.8);
  backdrop-filter: blur(4px);
  z-index: 1000;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 20px;
}

.avatar-picker {
  --neon-magenta: #ff00ff;
  --neon-cyan: #00f3ff;
  --neon-green: #00ff88;
  --bg-dark: #0a0a12;
  --bg-panel: rgba(18, 18, 26, 0.98);
  --border-dim: #2a2a3a;
  --text-primary: #e0e0ff;
  --text-secondary: #8888aa;

  background: var(--bg-panel);
  border: 1px solid var(--border-dim);
  border-radius: 12px;
  box-shadow: 0 0 40px rgba(255, 0, 255, 0.2);
  max-width: 500px;
  width: 100%;
  max-height: 80vh;
  display: flex;
  flex-direction: column;
  animation: picker-appear 0.2s ease-out;
}

@keyframes picker-appear {
  from {
    opacity: 0;
    transform: scale(0.95);
  }
  to {
    opacity: 1;
    transform: scale(1);
  }
}

.picker-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 20px;
  border-bottom: 1px solid var(--border-dim);
}

.picker-title {
  font-size: 14px;
  font-weight: 700;
  color: var(--neon-magenta);
  letter-spacing: 2px;
  margin: 0;
}

.close-btn {
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: transparent;
  border: 1px solid var(--border-dim);
  border-radius: 4px;
  color: var(--text-secondary);
  font-size: 16px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.close-btn:hover {
  background: rgba(255, 0, 255, 0.1);
  border-color: var(--neon-magenta);
  color: var(--neon-magenta);
}

.picker-body {
  padding: 20px;
  overflow-y: auto;
  flex: 1;
}

.emoji-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(48px, 1fr));
  gap: 8px;
}

.emoji-btn {
  width: 48px;
  height: 48px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(0, 0, 0, 0.3);
  border: 1px solid var(--border-dim);
  border-radius: 8px;
  font-size: 28px;
  cursor: pointer;
  transition: all 0.2s ease;
  position: relative;
}

.emoji-btn:hover {
  background: rgba(255, 0, 255, 0.1);
  border-color: var(--neon-magenta);
  transform: scale(1.1);
  box-shadow: 0 0 15px rgba(255, 0, 255, 0.3);
}

.emoji-btn.active {
  background: rgba(0, 255, 136, 0.1);
  border-color: var(--neon-green);
  box-shadow: 0 0 15px rgba(0, 255, 136, 0.3);
}

.emoji-btn.active::after {
  content: '✓';
  position: absolute;
  bottom: 2px;
  right: 2px;
  font-size: 10px;
  color: var(--neon-green);
}

.picker-footer {
  padding: 16px 20px;
  border-top: 1px solid var(--border-dim);
  display: flex;
  justify-content: flex-end;
}

.cyber-btn {
  padding: 10px 20px;
  background: transparent;
  border: 1px solid var(--border-dim);
  border-radius: 4px;
  color: var(--text-secondary);
  font-family: 'Courier New', monospace;
  font-size: 11px;
  font-weight: 600;
  letter-spacing: 1px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.cancel-btn:hover {
  background: rgba(255, 0, 255, 0.1);
  border-color: var(--neon-magenta);
  color: var(--neon-magenta);
}

/* Scrollbar Styling */
.picker-body::-webkit-scrollbar {
  width: 8px;
}

.picker-body::-webkit-scrollbar-track {
  background: rgba(0, 0, 0, 0.2);
  border-radius: 4px;
}

.picker-body::-webkit-scrollbar-thumb {
  background: var(--border-dim);
  border-radius: 4px;
}

.picker-body::-webkit-scrollbar-thumb:hover {
  background: var(--text-secondary);
}

/* Responsive */
@media (max-width: 768px) {
  .avatar-picker {
    max-height: 90vh;
  }

  .emoji-grid {
    grid-template-columns: repeat(auto-fill, minmax(44px, 1fr));
    gap: 6px;
  }

  .emoji-btn {
    width: 44px;
    height: 44px;
    font-size: 24px;
  }
}
</style>
