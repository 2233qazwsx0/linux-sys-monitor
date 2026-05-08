<template>
  <div class="historical-panel" v-if="show">
    <div class="panel-header">
      <h3>📊 {{ lang === 'zh' ? '历史对比' : 'Historical Comparison' }}</h3>
      <button @click="$emit('close')">✕</button>
    </div>
    <div class="comparison-content">
      <div class="comparison-section">
        <h4>{{ lang === 'zh' ? '当前 vs 1小时前' : 'Current vs 1h ago' }}</h4>
        <div class="metrics-row">
          <div class="metric">
            <span class="metric-name">CPU</span>
            <span class="current">{{ comparison?.current?.cpu?.usage?.toFixed(1) || 0 }}%</span>
            <span class="past" v-if="comparison?.one_hour_ago">
              ({{ comparison.one_hour_ago.cpu.usage.toFixed(1) }}%)
            </span>
            <span class="change" :class="getChangeClass(comparison?.cpu_change)">
              {{ formatChange(comparison?.cpu_change) }}
            </span>
          </div>
          <div class="metric">
            <span class="metric-name">Memory</span>
            <span class="current">{{ comparison?.current?.memory?.usage_percent?.toFixed(1) || 0 }}%</span>
            <span class="past" v-if="comparison?.one_hour_ago">
              ({{ comparison.one_hour_ago.memory.usage_percent.toFixed(1) }}%)
            </span>
            <span class="change" :class="getChangeClass(comparison?.memory_change)">
              {{ formatChange(comparison?.memory_change) }}
            </span>
          </div>
        </div>
      </div>
      <div class="comparison-section">
        <h4>{{ lang === 'zh' ? '当前 vs 24小时前' : 'Current vs 24h ago' }}</h4>
        <div class="metrics-row">
          <div class="metric">
            <span class="metric-name">CPU</span>
            <span class="current">{{ comparison?.current?.cpu?.usage?.toFixed(1) || 0 }}%</span>
            <span class="past" v-if="comparison?.twenty_four_hours_ago">
              ({{ comparison.twenty_four_hours_ago.cpu.usage.toFixed(1) }}%)
            </span>
          </div>
          <div class="metric">
            <span class="metric-name">Memory</span>
            <span class="current">{{ comparison?.current?.memory?.usage_percent?.toFixed(1) || 0 }}%</span>
            <span class="past" v-if="comparison?.twenty_four_hours_ago">
              ({{ comparison.twenty_four_hours_ago.memory.usage_percent.toFixed(1) }}%)
            </span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, watch } from 'vue'

defineProps({ show: Boolean })
defineEmits(['close'])

const lang = ref(localStorage.getItem('lang') || (navigator.language.toLowerCase().includes('zh') ? 'zh' : 'en'))
const comparison = ref(null)

function fetchComparison() {
  fetch('/api/historical')
    .then(r => r.json())
    .then(data => { comparison.value = data })
    .catch(() => {})
}

function formatChange(value) {
  if (value === undefined || value === null) return ''
  const sign = value >= 0 ? '+' : ''
  return `${sign}${value.toFixed(1)}%`
}

function getChangeClass(value) {
  if (value === undefined || value === null) return ''
  if (value > 5) return 'increase'
  if (value < -5) return 'decrease'
  return 'stable'
}

onMounted(fetchComparison)
watch(() => lang.value, fetchComparison)
</script>

<style scoped>
.historical-panel {
  position: fixed;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: 1rem;
  width: 500px;
  max-width: 90vw;
  z-index: 300;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.5);
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem 1.5rem;
  border-bottom: 1px solid var(--border);
}

.panel-header h3 { margin: 0; font-size: 1.1rem; }
.panel-header button { background: none; border: none; font-size: 1.25rem; cursor: pointer; color: var(--text-secondary); }

.comparison-content { padding: 1.5rem; }

.comparison-section { margin-bottom: 1.5rem; }
.comparison-section:last-child { margin-bottom: 0; }

.comparison-section h4 {
  margin: 0 0 0.75rem 0;
  font-size: 0.85rem;
  color: var(--text-secondary);
  font-weight: 600;
}

.metrics-row {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.metric {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.5rem;
  background: var(--bg-secondary);
  border-radius: 0.5rem;
}

.metric-name {
  font-weight: 600;
  font-size: 0.85rem;
  min-width: 60px;
}

.current {
  font-weight: 700;
  font-size: 1rem;
  font-variant-numeric: tabular-nums;
}

.past {
  color: var(--text-secondary);
  font-size: 0.85rem;
}

.change {
  margin-left: auto;
  font-weight: 600;
  font-size: 0.85rem;
}

.change.increase { color: var(--danger); }
.change.decrease { color: var(--success); }
.change.stable { color: var(--text-secondary); }
</style>
