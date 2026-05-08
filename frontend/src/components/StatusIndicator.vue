<template>
  <div class="status-indicator" :class="statusClass">
    <span class="status-dot" :class="{ pulse }"></span>
    <span class="status-text" v-if="showText">{{ text }}</span>
  </div>
</template>

<script setup>
import { computed } from 'vue'

const props = defineProps({
  status: {
    type: String,
    default: 'normal',
    validator: (v) => ['normal', 'warning', 'danger', 'offline'].includes(v)
  },
  showText: { type: Boolean, default: true },
  pulse: { type: Boolean, default: false },
  text: { type: String, default: '' }
})

const statusClass = computed(() => `status-${props.status}`)

const text = computed(() => {
  if (props.text) return props.text
  const texts = {
    normal: 'Normal',
    warning: 'Warning',
    danger: 'Danger',
    offline: 'Offline'
  }
  return texts[props.status]
})
</script>

<style scoped>
.status-indicator {
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.25rem 0.75rem;
  border-radius: 9999px;
  font-size: 0.75rem;
  font-weight: 500;
}

.status-normal {
  background: rgba(34, 197, 94, 0.15);
  color: var(--success);
}

.status-warning {
  background: rgba(245, 158, 11, 0.15);
  color: var(--warning);
}

.status-danger {
  background: rgba(239, 68, 68, 0.15);
  color: var(--danger);
}

.status-offline {
  background: rgba(107, 114, 128, 0.15);
  color: #6b7280;
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: currentColor;
}

.status-normal .status-dot {
  box-shadow: 0 0 8px var(--success);
}

.status-dot.pulse {
  animation: pulse 2s infinite;
}

@keyframes pulse {
  0%, 100% { transform: scale(1); opacity: 1; }
  50% { transform: scale(1.3); opacity: 0.7; }
}
</style>
