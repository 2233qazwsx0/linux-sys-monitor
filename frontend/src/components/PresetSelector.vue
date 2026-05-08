<template>
  <div class="preset-selector">
    <div class="preset-grid">
      <div
        v-for="preset in presets"
        :key="preset.id"
        :class="['preset-card', { active: modelValue === preset.id }]"
        @click="$emit('update:modelValue', preset.id)"
      >
        <div class="preset-icon">{{ preset.icon }}</div>
        <div class="preset-name">{{ preset.name }}</div>
        <div class="preset-desc">{{ preset.desc }}</div>
      </div>
    </div>
  </div>
</template>

<script setup>
defineProps({
  modelValue: { type: String, default: 'default' },
  presets: {
    type: Array,
    default: () => [
      { id: 'default', name: 'Default', icon: '📊', desc: 'Balanced view' },
      { id: 'minimal', name: 'Minimal', icon: '⚡', desc: 'CPU & Memory only' },
      { id: 'detailed', name: 'Detailed', icon: '📈', desc: 'Full gauges' },
      { id: 'performance', name: 'Performance', icon: '🚀', desc: 'I/O focused' }
    ]
  }
})

defineEmits(['update:modelValue'])
</script>

<style scoped>
.preset-selector {
  padding: 0.5rem;
}

.preset-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 0.75rem;
}

.preset-card {
  background: var(--bg-secondary);
  border: 2px solid var(--border);
  border-radius: 0.75rem;
  padding: 1rem;
  cursor: pointer;
  transition: all 0.2s;
  text-align: center;
}

.preset-card:hover {
  border-color: var(--accent);
}

.preset-card.active {
  border-color: var(--accent);
  background: rgba(99, 102, 241, 0.1);
}

.preset-icon {
  font-size: 1.5rem;
  margin-bottom: 0.5rem;
}

.preset-name {
  font-weight: 600;
  font-size: 0.9rem;
  color: var(--text-primary);
  margin-bottom: 0.25rem;
}

.preset-desc {
  font-size: 0.75rem;
  color: var(--text-secondary);
}
</style>
