<template>
  <div class="keyboard-shortcuts">
    <h4>{{ lang === 'zh' ? '键盘快捷键' : 'Keyboard Shortcuts' }}</h4>
    <div class="shortcuts-list">
      <div v-for="shortcut in shortcuts" :key="shortcut.key" class="shortcut-item">
        <kbd>{{ shortcut.key }}</kbd>
        <span>{{ shortcut.label }}</span>
      </div>
    </div>
  </div>
</template>

<script setup>
import { computed } from 'vue'

const props = defineProps({
  shortcuts: {
    type: Array,
    default: () => [
      { key: 'Ctrl+Shift+D', label: 'Toggle dark/light theme' },
      { key: 'Ctrl+Shift+S', label: 'Open settings' },
      { key: 'Ctrl+Shift+R', label: 'Refresh data' },
      { key: 'Ctrl+F', label: 'Focus search' },
      { key: 'Escape', label: 'Close dialogs' },
      { key: '1-6', label: 'Switch dashboard presets' }
    ]
  }
})

const lang = computed(() => localStorage.getItem('lang') || 'en')
</script>

<style scoped>
.keyboard-shortcuts {
  padding: 0.5rem;
}

.keyboard-shortcuts h4 {
  margin: 0 0 1rem 0;
  font-size: 0.9rem;
  color: var(--text-primary);
}

.shortcuts-list {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.shortcut-item {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  font-size: 0.8rem;
}

kbd {
  display: inline-block;
  padding: 0.25rem 0.5rem;
  background: var(--bg-secondary);
  border: 1px solid var(--border);
  border-radius: 0.3rem;
  font-family: monospace;
  font-size: 0.75rem;
  color: var(--accent);
  min-width: 80px;
  text-align: center;
}

.shortcut-item span {
  color: var(--text-secondary);
}
</style>
