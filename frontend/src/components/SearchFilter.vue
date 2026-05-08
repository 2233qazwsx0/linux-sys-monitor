<template>
  <div class="search-filter">
    <div class="search-input-wrapper">
      <span class="search-icon">🔍</span>
      <input
        ref="inputRef"
        v-model="query"
        type="text"
        :placeholder="placeholder"
        class="search-input"
        @input="handleInput"
        @focus="focused = true"
        @blur="focused = false"
        @keydown.escape="clear"
      />
      <button v-if="query" class="clear-btn" @click="clear">✕</button>
    </div>
    <div class="search-results" v-if="focused && query && results.length > 0">
      <div
        v-for="(result, index) in results"
        :key="index"
        class="result-item"
        @click="selectResult(result)"
      >
        <slot name="result" :result="result">{{ result }}</slot>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, watch } from 'vue'

const props = defineProps({
  modelValue: { type: String, default: '' },
  items: { type: Array, default: () => [] },
  filterKey: { type: String, default: 'name' },
  placeholder: { type: String, default: 'Search...' }
})

const emit = defineEmits(['update:modelValue', 'filter', 'select'])

const query = ref(props.modelValue)
const focused = ref(false)
const inputRef = ref(null)

watch(() => props.modelValue, (val) => {
  query.value = val
})

const results = computed(() => {
  if (!query.value) return []
  const q = query.value.toLowerCase()
  return props.items.filter(item => {
    const value = typeof item === 'object' ? item[props.filterKey] : item
    return String(value).toLowerCase().includes(q)
  })
})

function handleInput() {
  emit('update:modelValue', query.value)
  emit('filter', query.value)
}

function clear() {
  query.value = ''
  emit('update:modelValue', '')
  emit('filter', '')
  inputRef.value?.focus()
}

function selectResult(result) {
  emit('select', result)
  focused.value = false
}

defineExpose({ focus: () => inputRef.value?.focus() })
</script>

<style scoped>
.search-filter {
  position: relative;
  width: 100%;
}

.search-input-wrapper {
  display: flex;
  align-items: center;
  background: var(--bg-secondary);
  border: 1px solid var(--border);
  border-radius: 0.5rem;
  padding: 0.5rem 0.75rem;
  gap: 0.5rem;
}

.search-input-wrapper:focus-within {
  border-color: var(--accent);
  box-shadow: 0 0 0 3px var(--accent-glow);
}

.search-icon {
  font-size: 0.9rem;
  opacity: 0.6;
}

.search-input {
  flex: 1;
  background: none;
  border: none;
  color: var(--text-primary);
  font-size: 0.9rem;
  outline: none;
}

.search-input::placeholder {
  color: var(--text-secondary);
}

.clear-btn {
  background: none;
  border: none;
  color: var(--text-secondary);
  cursor: pointer;
  padding: 0.25rem;
  font-size: 0.75rem;
}

.clear-btn:hover {
  color: var(--text-primary);
}

.search-results {
  position: absolute;
  top: 100%;
  left: 0;
  right: 0;
  margin-top: 0.5rem;
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: 0.5rem;
  max-height: 300px;
  overflow-y: auto;
  z-index: 100;
  box-shadow: 0 10px 40px rgba(0, 0, 0, 0.3);
}

.result-item {
  padding: 0.75rem 1rem;
  cursor: pointer;
  transition: background 0.2s;
}

.result-item:hover {
  background: var(--bg-secondary);
}
</style>
