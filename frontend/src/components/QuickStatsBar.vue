<template>
  <div class="quick-stats-bar" :class="{ compact }">
    <div class="stat-item" v-for="stat in stats" :key="stat.label">
      <span class="stat-label">{{ stat.label }}</span>
      <span class="stat-value" :class="stat.class">
        {{ stat.value }}{{ stat.unit }}
        <span class="trend" :class="stat.trendClass">{{ stat.trend }}</span>
      </span>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, onUnmounted } from 'vue'

const props = defineProps({
  compact: { type: Boolean, default: false }
})

const trends = ref([])

const stats = computed(() => {
  const cpu = trends.value.find(t => t.metric === 'cpu')
  const mem = trends.value.find(t => t.metric === 'memory')
  return [
    {
      label: 'CPU',
      value: cpu?.current?.toFixed(1) || '0.0',
      unit: '%',
      class: cpu && cpu.current > 80 ? 'warning' : '',
      trend: cpu?.trend || '→',
      trendClass: cpu?.trend === '↑' ? 'up' : cpu?.trend === '↓' ? 'down' : 'neutral'
    },
    {
      label: 'Memory',
      value: mem?.current?.toFixed(1) || '0.0',
      unit: '%',
      class: mem && mem.current > 85 ? 'warning' : '',
      trend: mem?.trend || '→',
      trendClass: mem?.trend === '↑' ? 'up' : mem?.trend === '↓' ? 'down' : 'neutral'
    }
  ]
})

let fetchInterval = null

function fetchTrends() {
  fetch('/api/trends')
    .then(r => r.json())
    .then(data => { trends.value = data })
    .catch(() => {})
}

onMounted(() => {
  fetchTrends()
  fetchInterval = setInterval(fetchTrends, 5000)
})

onUnmounted(() => {
  if (fetchInterval) clearInterval(fetchInterval)
})
</script>

<style scoped>
.quick-stats-bar {
  display: flex;
  gap: 2rem;
  padding: 0.75rem 1.5rem;
  background: var(--bg-card);
  border-bottom: 1px solid var(--border);
  justify-content: center;
}

.quick-stats-bar.compact {
  padding: 0.5rem 1rem;
  gap: 1rem;
}

.stat-item {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.stat-label {
  font-size: 0.75rem;
  color: var(--text-secondary);
  font-weight: 600;
}

.stat-value {
  font-size: 0.9rem;
  font-weight: 700;
  font-variant-numeric: tabular-nums;
  display: flex;
  align-items: center;
  gap: 0.25rem;
}

.compact .stat-label { font-size: 0.65rem; }
.compact .stat-value { font-size: 0.8rem; }

.stat-value.warning { color: var(--warning); }

.trend {
  font-size: 0.85em;
  margin-left: 0.125rem;
}

.trend.up { color: var(--danger); }
.trend.down { color: var(--success); }
.trend.neutral { color: var(--text-secondary); }
</style>
