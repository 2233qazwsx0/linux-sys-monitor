<template>
  <div class="threshold-colors">
    <div v-for="metric in metrics" :key="metric.key" class="threshold-item">
      <label>{{ metric.label }}</label>
      <div class="threshold-inputs">
        <div class="threshold-input-group">
          <span class="threshold-label warning">⚠</span>
          <input
            type="number"
            v-model.number="thresholds[metric.key].warning"
            min="0"
            max="100"
            @change="emitUpdate"
          />
        </div>
        <div class="threshold-input-group">
          <span class="threshold-label danger">🔴</span>
          <input
            type="number"
            v-model.number="thresholds[metric.key].danger"
            min="0"
            max="100"
            @change="emitUpdate"
          />
        </div>
      </div>
      <div class="threshold-preview">
        <div
          class="threshold-bar"
          :style="{
            background: `linear-gradient(90deg, var(--success) 0%, var(--success) ${thresholds[metric.key].warning}%, var(--warning) ${thresholds[metric.key].warning}%, var(--warning) ${thresholds[metric.key].danger}%, var(--danger) ${thresholds[metric.key].danger}%, var(--danger) 100%)`
          }"
        ></div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, watch } from 'vue'

const props = defineProps({
  modelValue: {
    type: Object,
    default: () => ({
      cpu: { warning: 60, danger: 80 },
      memory: { warning: 70, danger: 85 },
      disk: { warning: 70, danger: 90 },
      network: { warning: 70, danger: 90 }
    })
  }
})

const emit = defineEmits(['update:modelValue'])

const thresholds = ref({ ...props.modelValue })

const metrics = [
  { key: 'cpu', label: 'CPU' },
  { key: 'memory', label: 'Memory' },
  { key: 'disk', label: 'Disk' },
  { key: 'network', label: 'Network' }
]

watch(() => props.modelValue, (val) => {
  thresholds.value = { ...val }
}, { deep: true })

function emitUpdate() {
  emit('update:modelValue', { ...thresholds.value })
}
</script>

<style scoped>
.threshold-colors {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.threshold-item {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.threshold-item label {
  font-size: 0.85rem;
  font-weight: 600;
  color: var(--text-primary);
}

.threshold-inputs {
  display: flex;
  gap: 1rem;
}

.threshold-input-group {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.threshold-label {
  font-size: 0.9rem;
}

.threshold-input-group input {
  width: 60px;
  background: var(--bg-secondary);
  border: 1px solid var(--border);
  color: var(--text-primary);
  padding: 0.4rem;
  border-radius: 0.5rem;
  text-align: center;
  font-size: 0.85rem;
}

.threshold-preview {
  height: 8px;
  border-radius: 4px;
  overflow: hidden;
  background: var(--bg-secondary);
}

.threshold-bar {
  height: 100%;
  border-radius: 4px;
  transition: background 0.3s;
}
</style>
