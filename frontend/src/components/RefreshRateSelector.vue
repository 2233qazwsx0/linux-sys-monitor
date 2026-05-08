<template>
  <div class="refresh-rate-selector">
    <label>{{ label }}</label>
    <div class="rate-options">
      <button
        v-for="rate in rates"
        :key="rate.value"
        :class="{ active: modelValue === rate.value }"
        @click="$emit('update:modelValue', rate.value)"
      >
        {{ rate.label }}
      </button>
    </div>
  </div>
</template>

<script setup>
defineProps({
  modelValue: { type: Number, default: 1000 },
  label: { type: String, default: 'Refresh Rate' },
  rates: {
    type: Array,
    default: () => [
      { value: 500, label: '500ms' },
      { value: 1000, label: '1s' },
      { value: 2000, label: '2s' },
      { value: 5000, label: '5s' },
      { value: 10000, label: '10s' }
    ]
  }
})

defineEmits(['update:modelValue'])
</script>

<style scoped>
.refresh-rate-selector {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.refresh-rate-selector label {
  font-size: 0.85rem;
  color: var(--text-secondary);
}

.rate-options {
  display: flex;
  gap: 0.5rem;
  flex-wrap: wrap;
}

.rate-options button {
  background: var(--bg-secondary);
  border: 1px solid var(--border);
  color: var(--text-primary);
  padding: 0.4rem 0.75rem;
  border-radius: 0.5rem;
  cursor: pointer;
  font-size: 0.8rem;
  transition: all 0.2s;
}

.rate-options button:hover {
  border-color: var(--accent);
}

.rate-options button.active {
  background: var(--accent);
  border-color: var(--accent);
  color: white;
}
</style>
