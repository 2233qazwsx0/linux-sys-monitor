<template>
  <div class="process-list">
    <div class="process-header">
      <h3>📋 {{ lang === 'zh' ? '热门进程' : 'Top Processes' }}</h3>
      <div class="process-stats">
        <span class="stat-badge">
          {{ processes.length }} {{ lang === 'zh' ? '进程' : 'Processes' }}
        </span>
        <span v-if="searchQuery" class="search-indicator">
          🔍 {{ lang === 'zh' ? '已过滤' : 'Filtered' }}
        </span>
      </div>
    </div>
    <table>
      <thead>
        <tr>
          <th class="sortable" @click="sortBy('pid')">
            PID
            <span v-if="sortKey === 'pid'" class="sort-icon">{{ sortOrder === 'asc' ? '↑' : '↓' }}</span>
          </th>
          <th class="sortable" @click="sortBy('name')">
            {{ lang === 'zh' ? '名称' : 'Name' }}
            <span v-if="sortKey === 'name'" class="sort-icon">{{ sortOrder === 'asc' ? '↑' : '↓' }}</span>
          </th>
          <th class="sortable cpu-col" @click="sortBy('cpu')">
            CPU %
            <span v-if="sortKey === 'cpu'" class="sort-icon">{{ sortOrder === 'asc' ? '↑' : '↓' }}</span>
          </th>
          <th class="sortable mem-col" @click="sortBy('memory')">
            MEM %
            <span v-if="sortKey === 'memory'" class="sort-icon">{{ sortOrder === 'asc' ? '↑' : '↓' }}</span>
          </th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="proc in sortedProcesses" :key="proc.pid" class="process-row">
          <td class="pid">{{ proc.pid }}</td>
          <td class="name" :title="proc.name">{{ proc.name }}</td>
          <td class="cpu-col">
            <span class="cpu-value" :class="getCpuClass(proc.cpu)">{{ proc.cpu.toFixed(1) }}%</span>
          </td>
          <td class="mem-col">
            <span class="mem-value" :class="getMemClass(proc.memory)">{{ proc.memory.toFixed(1) }}%</span>
          </td>
        </tr>
        <tr v-if="sortedProcesses.length === 0">
          <td colspan="4" class="empty-row">
            {{ lang === 'zh' ? '没有找到匹配的进程' : 'No matching processes found' }}
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue'

const props = defineProps({
  processes: { type: Array, default: () => [] },
  searchQuery: { type: String, default: '' }
})

const lang = ref(localStorage.getItem('lang') || 'en')
const sortKey = ref('cpu')
const sortOrder = ref('desc')

const sortedProcesses = computed(() => {
  const filtered = props.searchQuery
    ? props.processes.filter(p =>
        p.name.toLowerCase().includes(props.searchQuery.toLowerCase()) ||
        String(p.pid).includes(props.searchQuery)
      )
    : props.processes

  return [...filtered].sort((a, b) => {
    const aVal = a[sortKey.value]
    const bVal = b[sortKey.value]
    const modifier = sortOrder.value === 'asc' ? 1 : -1
    if (typeof aVal === 'string') {
      return aVal.localeCompare(bVal) * modifier
    }
    return (aVal - bVal) * modifier
  })
})

function sortBy(key) {
  if (sortKey.value === key) {
    sortOrder.value = sortOrder.value === 'asc' ? 'desc' : 'asc'
  } else {
    sortKey.value = key
    sortOrder.value = 'desc'
  }
}

function getCpuClass(value) {
  if (value >= 80) return 'danger'
  if (value >= 50) return 'warning'
  return 'normal'
}

function getMemClass(value) {
  if (value >= 80) return 'danger'
  if (value >= 50) return 'warning'
  return 'normal'
}
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

.process-stats {
  display: flex;
  gap: 0.75rem;
  align-items: center;
}

.stat-badge {
  background: var(--bg-secondary);
  padding: 0.25rem 0.75rem;
  border-radius: 9999px;
  font-size: 0.75rem;
  color: var(--text-secondary);
}

.search-indicator {
  color: var(--accent);
  font-size: 0.8rem;
}

table {
  width: 100%;
  border-collapse: collapse;
}

thead {
  position: sticky;
  top: 0;
  background: var(--bg-card);
}

th {
  text-align: left;
  padding: 0.75rem 0.5rem;
  border-bottom: 2px solid var(--border);
  font-size: 0.75rem;
  font-weight: 600;
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

th.sortable {
  cursor: pointer;
  user-select: none;
}

th.sortable:hover {
  color: var(--accent);
}

.sort-icon {
  margin-left: 0.25rem;
}

td {
  padding: 0.75rem 0.5rem;
  border-bottom: 1px solid var(--border);
  font-size: 0.85rem;
  color: var(--text-primary);
  font-variant-numeric: tabular-nums;
}

.process-row:hover td {
  background: rgba(99, 102, 241, 0.05);
}

td.name {
  max-width: 200px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

td.pid {
  color: var(--text-secondary);
  font-family: monospace;
}

.cpu-col, .mem-col {
  text-align: right;
  min-width: 80px;
}

.cpu-value, .mem-value {
  display: inline-block;
  padding: 0.2rem 0.5rem;
  border-radius: 0.25rem;
  font-weight: 500;
}

.cpu-value.normal, .mem-value.normal {
  background: rgba(34, 197, 94, 0.15);
  color: var(--success);
}

.cpu-value.warning, .mem-value.warning {
  background: rgba(245, 158, 11, 0.15);
  color: var(--warning);
}

.cpu-value.danger, .mem-value.danger {
  background: rgba(239, 68, 68, 0.15);
  color: var(--danger);
}

.empty-row {
  text-align: center;
  padding: 2rem;
  color: var(--text-secondary);
  font-style: italic;
}

tr:last-child td {
  border-bottom: none;
}
</style>
