<template>
  <div class="process-list">
    <div class="process-header">
      <h3>📋 {{ lang === 'zh' ? '热门进程' : 'Top Processes' }}</h3>
      <div class="sort-controls">
        <span class="sort-label">{{ lang === 'zh' ? '排序' : 'Sort' }}:</span>
        <button 
          :class="{ active: sortBy === 'cpu' }" 
          @click="sortBy = 'cpu'"
        >
          {{ lang === 'zh' ? 'CPU' : 'CPU' }}
        </button>
        <button 
          :class="{ active: sortBy === 'memory' }" 
          @click="sortBy = 'memory'"
        >
          {{ lang === 'zh' ? '内存' : 'Memory' }}
        </button>
      </div>
    </div>
    <table>
      <thead>
        <tr>
          <th>{{ lang === 'zh' ? 'PID' : 'PID' }}</th>
          <th>{{ lang === 'zh' ? '名称' : 'Name' }}</th>
          <th>{{ lang === 'zh' ? 'CPU %' : 'CPU %' }}</th>
          <th>{{ lang === 'zh' ? '内存 %' : 'Memory %' }}</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="proc in sortedProcesses.slice(0, 10)" :key="proc.pid">
          <td>{{ proc.pid }}</td>
          <td class="name">{{ proc.name }}</td>
          <td :class="{ highlight: sortBy === 'cpu' }">{{ proc.cpu.toFixed(1) }}%</td>
          <td :class="{ highlight: sortBy === 'memory' }">{{ proc.memory.toFixed(1) }}%</td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue'

const lang = ref(window.i18n?.isZh ? 'zh' : 'en')
const sortBy = ref('cpu')

const props = defineProps({
  processes: Array
})

const sortedProcesses = computed(() => {
  if (!props.processes) return []
  return [...props.processes].sort((a, b) => {
    if (sortBy.value === 'memory') {
      return b.memory - a.memory
    }
    return b.cpu - a.cpu
  })
})
</script>

<style scoped>
.process-list {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: 1rem;
  padding: 1.25rem;
}

.process-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
}

h3 {
  margin: 0;
  font-size: 1rem;
  color: var(--text-primary);
}

.sort-controls {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.sort-label {
  font-size: 0.75rem;
  color: var(--text-secondary);
}

.sort-controls button {
  background: var(--bg-secondary);
  border: 1px solid var(--border);
  color: var(--text-secondary);
  padding: 0.25rem 0.75rem;
  border-radius: 0.5rem;
  cursor: pointer;
  font-size: 0.75rem;
  transition: all 0.2s;
}

.sort-controls button:hover {
  background: var(--bg-card);
  color: var(--text-primary);
}

.sort-controls button.active {
  background: var(--accent);
  border-color: var(--accent);
  color: white;
}

table {
  width: 100%;
  border-collapse: collapse;
}

th, td {
  text-align: left;
  padding: 0.75rem 0.5rem;
  border-bottom: 1px solid var(--border);
}

th {
  font-size: 0.75rem;
  font-weight: 600;
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

td {
  font-size: 0.85rem;
  color: var(--text-primary);
  font-variant-numeric: tabular-nums;
}

td.name {
  max-width: 200px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

td.highlight {
  color: var(--accent);
  font-weight: 600;
}

tr:last-child td {
  border-bottom: none;
}

tr:hover td {
  background: rgba(99, 102, 241, 0.1);
}
</style>
