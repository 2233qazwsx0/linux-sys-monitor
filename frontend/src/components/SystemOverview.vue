<template>
  <div class="overview" v-if="metrics">
    <div class="stat-card">
      <div class="stat-icon">🔥</div>
      <div class="stat-info">
        <div class="stat-label">CPU Usage</div>
        <div class="stat-value">{{ metrics.cpu.usage.toFixed(1) }}%</div>
      </div>
    </div>

    <div class="stat-card">
      <div class="stat-icon">💾</div>
      <div class="stat-info">
        <div class="stat-label">Memory</div>
        <div class="stat-value">{{ metrics.memory.usage_percent.toFixed(1) }}%</div>
        <div class="stat-detail">{{ formatBytes(metrics.memory.used) }} / {{ formatBytes(metrics.memory.total) }}</div>
      </div>
    </div>

    <div class="stat-card">
      <div class="stat-icon">📊</div>
      <div class="stat-info">
        <div class="stat-label">Disk I/O</div>
        <div class="stat-value">{{ formatBytes(metrics.disk.read_rate) }}/s</div>
        <div class="stat-detail">Write: {{ formatBytes(metrics.disk.write_rate) }}/s</div>
      </div>
    </div>

    <div class="stat-card">
      <div class="stat-icon">🖥️</div>
      <div class="stat-info">
        <div class="stat-label">CPU Cores</div>
        <div class="stat-value">{{ metrics.cpu.core_count }}</div>
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
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
}
</script>

<style scoped>
.overview {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 20px;
  margin-bottom: 30px;
}

.stat-card {
  background: white;
  border-radius: 12px;
  padding: 20px;
  display: flex;
  align-items: center;
  gap: 15px;
  box-shadow: 0 2px 10px rgba(0,0,0,0.1);
}

.stat-icon {
  font-size: 32px;
}

.stat-label {
  font-size: 12px;
  color: #7f8c8d;
  text-transform: uppercase;
  letter-spacing: 1px;
}

.stat-value {
  font-size: 24px;
  font-weight: bold;
  color: #2c3e50;
}

.stat-detail {
  font-size: 12px;
  color: #95a5a6;
  margin-top: 4px;
}
</style>
