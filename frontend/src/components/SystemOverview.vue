<template>
  <div class="overview" v-if="metrics">
    <div class="stat-card cpu">
      <div class="stat-header">
        <span class="stat-icon">⚡</span>
        <span class="stat-title">CPU</span>
      </div>
      <div class="stat-value">{{ metrics.cpu.usage.toFixed(1) }}%</div>
      <div class="stat-bar">
        <div class="stat-bar-fill" :style="{ width: metrics.cpu.usage + '%' }"></div>
      </div>
      <div class="stat-footer">
        <span>{{ metrics.cpu.core_count }} Cores</span>
        <span>{{ formatLoad(metrics.cpu.per_core) }}</span>
      </div>
    </div>

    <div class="stat-card memory">
      <div class="stat-header">
        <span class="stat-icon">🧠</span>
        <span class="stat-title">Memory</span>
      </div>
      <div class="stat-value">{{ metrics.memory.usage_percent.toFixed(1) }}%</div>
      <div class="stat-bar">
        <div class="stat-bar-fill" :style="{ width: metrics.memory.usage_percent + '%' }"></div>
      </div>
      <div class="stat-footer">
        <span>{{ formatBytes(metrics.memory.used) }}</span>
        <span>{{ formatBytes(metrics.memory.total) }}</span>
      </div>
    </div>

    <div class="stat-card disk">
      <div class="stat-header">
        <span class="stat-icon">💾</span>
        <span class="stat-title">Disk I/O</span>
      </div>
      <div class="stat-value duo">
        <span>R: {{ formatBytes(metrics.disk.read_rate) }}/s</span>
        <span>W: {{ formatBytes(metrics.disk.write_rate) }}/s</span>
      </div>
      <div class="stat-footer">
        <span>Read</span>
        <span>Write</span>
      </div>
    </div>

    <div class="stat-card network" v-if="metrics.network">
      <div class="stat-header">
        <span class="stat-icon">🌐</span>
        <span class="stat-title">Network</span>
      </div>
      <div class="stat-value duo">
        <span>↓ {{ formatBytes(metrics.network.rx_rate) }}/s</span>
        <span>↑ {{ formatBytes(metrics.network.tx_rate) }}/s</span>
      </div>
      <div class="stat-footer">
        <span>Download</span>
        <span>Upload</span>
      </div>
    </div>
  </div>
</template>

<script setup>
defineProps({
  metrics: Object
})

function formatBytes(bytes) {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i]
}

function formatLoad(perCore) {
  if (!perCore || perCore.length === 0) return '0%'
  const avg = perCore.reduce((a, b) => a + b, 0) / perCore.length
  return avg.toFixed(0) + '% avg'
}
</script>

<style scoped>
.overview {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
  gap: 1rem;
  margin-bottom: 2rem;
}

.stat-card {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: 1rem;
  padding: 1.25rem;
  transition: transform 0.2s, box-shadow 0.2s;
}

.stat-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 30px rgba(0, 0, 0, 0.3);
}

.stat-header {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  margin-bottom: 0.75rem;
}

.stat-icon {
  font-size: 1.25rem;
}

.stat-title {
  font-size: 0.85rem;
  font-weight: 600;
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.stat-value {
  font-size: 2rem;
  font-weight: 700;
  margin-bottom: 0.75rem;
  font-variant-numeric: tabular-nums;
}

.stat-value.duo {
  display: flex;
  justify-content: space-between;
  font-size: 1.1rem;
}

.stat-bar {
  height: 6px;
  background: var(--bg-secondary);
  border-radius: 3px;
  overflow: hidden;
  margin-bottom: 0.75rem;
}

.stat-bar-fill {
  height: 100%;
  border-radius: 3px;
  transition: width 0.3s ease;
}

.cpu .stat-bar-fill {
  background: linear-gradient(90deg, #6366f1, #a855f7);
}

.memory .stat-bar-fill {
  background: linear-gradient(90deg, #22c55e, #10b981);
}

.stat-footer {
  display: flex;
  justify-content: space-between;
  font-size: 0.75rem;
  color: var(--text-secondary);
}
</style>
