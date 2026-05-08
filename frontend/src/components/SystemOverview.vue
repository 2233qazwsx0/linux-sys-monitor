<template>
  <div class="overview" v-if="metrics">
    <div class="stat-card cpu">
      <div class="stat-header">
        <span class="stat-icon">⚡</span>
        <span class="stat-title">{{ lang === 'zh' ? 'CPU' : 'CPU' }}</span>
      </div>
      <div class="stat-value">{{ metrics.cpu.usage.toFixed(1) }}%</div>
      <div class="stat-bar">
        <div class="stat-bar-fill" :style="{ width: metrics.cpu.usage + '%' }"></div>
      </div>
      <div class="stat-footer">
        <span>{{ metrics.cpu.core_count }} {{ lang === 'zh' ? '核心' : 'Cores' }}</span>
        <span>{{ formatLoad(metrics.cpu.per_core) }}</span>
      </div>
      <div class="stat-cpu-name">{{ metrics.cpu.name }}</div>
    </div>

    <div class="stat-card memory">
      <div class="stat-header">
        <span class="stat-icon">🧠</span>
        <span class="stat-title">{{ lang === 'zh' ? '内存' : 'Memory' }}</span>
      </div>
      <div class="stat-value">{{ metrics.memory.usage_percent.toFixed(1) }}%</div>
      <div class="stat-bar">
        <div class="stat-bar-fill" :style="{ width: metrics.memory.usage_percent + '%' }"></div>
      </div>
      <div class="stat-footer">
        <span>{{ formatBytes(metrics.memory.used) }}</span>
        <span>{{ formatBytes(metrics.memory.total) }}</span>
      </div>
      <div class="stat-details" v-if="metrics.memory_details">
        <span>{{ lang === 'zh' ? '缓存' : 'Cached' }}: {{ formatBytes(metrics.memory_details.cached) }}</span>
        <span>{{ lang === 'zh' ? '缓冲' : 'Buffers' }}: {{ formatBytes(metrics.memory_details.buffers) }}</span>
      </div>
    </div>

    <div class="stat-card swap" v-if="metrics.swap.total > 0">
      <div class="stat-header">
        <span class="stat-icon">💿</span>
        <span class="stat-title">{{ lang === 'zh' ? '交换分区' : 'Swap' }}</span>
      </div>
      <div class="stat-value">{{ metrics.swap.usage_percent.toFixed(1) }}%</div>
      <div class="stat-bar">
        <div class="stat-bar-fill swap-fill" :style="{ width: metrics.swap.usage_percent + '%' }"></div>
      </div>
      <div class="stat-footer">
        <span>{{ formatBytes(metrics.swap.used) }}</span>
        <span>{{ formatBytes(metrics.swap.total) }}</span>
      </div>
    </div>

    <div class="stat-card disk">
      <div class="stat-header">
        <span class="stat-icon">💾</span>
        <span class="stat-title">{{ lang === 'zh' ? '磁盘 I/O' : 'Disk I/O' }}</span>
      </div>
      <div class="stat-value duo">
        <span>↓ {{ formatBytes(metrics.disk.read_rate) }}/s</span>
        <span>↑ {{ formatBytes(metrics.disk.write_rate) }}/s</span>
      </div>
    </div>

    <div class="stat-card network" v-if="metrics.network">
      <div class="stat-header">
        <span class="stat-icon">🌐</span>
        <span class="stat-title">{{ lang === 'zh' ? '网络' : 'Network' }}</span>
      </div>
      <div class="stat-value duo">
        <span>↓ {{ formatBytes(metrics.network.rx_rate) }}/s</span>
        <span>↑ {{ formatBytes(metrics.network.tx_rate) }}/s</span>
      </div>
    </div>

    <div class="stat-card battery" v-if="metrics.battery">
      <div class="stat-header">
        <span class="stat-icon">🔋</span>
        <span class="stat-title">{{ lang === 'zh' ? '电池' : 'Battery' }}</span>
      </div>
      <div class="stat-value" :class="{ charging: metrics.battery.is_charging }">
        {{ metrics.battery.charge_percent.toFixed(0) }}%
        <span v-if="metrics.battery.is_charging" class="charging-icon">⚡</span>
      </div>
      <div class="stat-bar">
        <div class="stat-bar-fill battery-fill" :style="{ width: metrics.battery.charge_percent + '%' }"></div>
      </div>
    </div>

    <div class="stat-card temperature" v-if="metrics.temperature">
      <div class="stat-header">
        <span class="stat-icon">🌡️</span>
        <span class="stat-title">{{ lang === 'zh' ? '温度' : 'Temperature' }}</span>
      </div>
      <div class="stat-value">
        {{ metrics.temperature.cpu_temp.toFixed(1) }}°C
      </div>
      <div class="stat-footer" v-if="metrics.temperature.gpu_temp">
        <span>GPU: {{ metrics.temperature.gpu_temp.toFixed(1) }}°C</span>
        <span>Max: {{ metrics.temperature.max_temp.toFixed(1) }}°C</span>
      </div>
    </div>

    <div class="stat-card load-avg" v-if="metrics.load_average">
      <div class="stat-header">
        <span class="stat-icon">📈</span>
        <span class="stat-title">{{ lang === 'zh' ? '负载' : 'Load Avg' }}</span>
      </div>
      <div class="stat-value load-values">
        <span class="load-item">{{ metrics.load_average.load_1.toFixed(2) }}</span>
        <span class="load-item">{{ metrics.load_average.load_5.toFixed(2) }}</span>
        <span class="load-item">{{ metrics.load_average.load_15.toFixed(2) }}</span>
      </div>
      <div class="stat-footer">
        <span>1m</span>
        <span>5m</span>
        <span>15m</span>
      </div>
    </div>

    <div class="stat-card connections" v-if="metrics.network_connections">
      <div class="stat-header">
        <span class="stat-icon">🔌</span>
        <span class="stat-title">{{ lang === 'zh' ? '连接' : 'Connections' }}</span>
      </div>
      <div class="stat-value small">
        <span>TCP: {{ metrics.network_connections.tcp_count }}</span>
      </div>
      <div class="stat-footer duo">
        <span>{{ lang === 'zh' ? '已建立' : 'Est' }}: {{ metrics.network_connections.tcp_established }}</span>
        <span>{{ lang === 'zh' ? '监听' : 'Listen' }}: {{ metrics.network_connections.tcp_listening }}</span>
      </div>
      <div class="stat-footer">
        <span>UDP: {{ metrics.network_connections.udp_count }}</span>
      </div>
    </div>

    <div class="stat-card iowait" v-if="metrics.io_wait">
      <div class="stat-header">
        <span class="stat-icon">⏳</span>
        <span class="stat-title">{{ lang === 'zh' ? 'I/O 等待' : 'I/O Wait' }}</span>
      </div>
      <div class="stat-value">{{ metrics.io_wait.iowait_percent.toFixed(1) }}%</div>
      <div class="stat-bar">
        <div class="stat-bar-fill iowait-fill" :style="{ width: metrics.io_wait.iowait_percent + '%' }"></div>
      </div>
    </div>

    <div class="stat-card container" v-if="metrics.container && metrics.container.is_container">
      <div class="stat-header">
        <span class="stat-icon">📦</span>
        <span class="stat-title">{{ lang === 'zh' ? '容器' : 'Container' }}</span>
      </div>
      <div class="stat-value small">
        {{ metrics.container.container_type || 'Container' }}
      </div>
    </div>

    <div class="stat-card frequencies" v-if="metrics.cpu.frequencies && metrics.cpu.frequencies.length > 0">
      <div class="stat-header">
        <span class="stat-icon">⚙️</span>
        <span class="stat-title">{{ lang === 'zh' ? '核心频率' : 'Core Freq' }}</span>
      </div>
      <div class="freq-grid">
        <div class="freq-item" v-for="(freq, idx) in metrics.cpu.frequencies.slice(0, 8)" :key="idx">
          <span class="freq-core">{{ lang === 'zh' ? '核' : 'C' }}{{ idx }}</span>
          <span class="freq-value">{{ (freq / 1000).toFixed(1) }} GHz</span>
        </div>
      </div>
    </div>

    <div class="stat-card virtual-mem" v-if="metrics.virtual_memory">
      <div class="stat-header">
        <span class="stat-icon">🔄</span>
        <span class="stat-title">{{ lang === 'zh' ? '虚拟内存' : 'VM Stats' }}</span>
      </div>
      <div class="stat-value small">
        <span>{{ lang === 'zh' ? '页错误' : 'Page Faults' }}: {{ formatNumber(metrics.virtual_memory.page_faults) }}</span>
      </div>
      <div class="stat-footer">
        <span>{{ lang === 'zh' ? '主错误' : 'Major' }}: {{ formatNumber(metrics.virtual_memory.major_page_faults) }}</span>
        <span>{{ lang === 'zh' ? '换入' : 'In' }}/{{ lang === 'zh' ? '换出' : 'Out' }}: {{ formatNumber(metrics.virtual_memory.pages_swapped_in) }}/{{ formatNumber(metrics.virtual_memory.pages_swapped_out) }}</span>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'

const lang = ref(window.i18n?.isZh ? 'zh' : 'en')

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
  return avg.toFixed(0) + '%'
}

function formatNumber(num) {
  if (num >= 1000000) return (num / 1000000).toFixed(1) + 'M'
  if (num >= 1000) return (num / 1000).toFixed(1) + 'K'
  return num.toString()
}
</script>

<style scoped>
.overview {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
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

.stat-value.small {
  font-size: 1rem;
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.stat-value.charging {
  color: var(--success);
}

.charging-icon {
  margin-left: 0.5rem;
  animation: blink 1s infinite;
}

@keyframes blink {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.5; }
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

.swap-fill {
  background: linear-gradient(90deg, #f59e0b, #ef4444);
}

.iowait-fill {
  background: linear-gradient(90deg, #f59e0b, #ef4444);
}

.battery-fill {
  background: linear-gradient(90deg, #22c55e, #10b981);
}

.stat-footer {
  display: flex;
  justify-content: space-between;
  font-size: 0.75rem;
  color: var(--text-secondary);
}

.stat-footer.duo {
  flex-direction: column;
  gap: 0.25rem;
}

.stat-details {
  margin-top: 0.5rem;
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
  font-size: 0.7rem;
  color: var(--text-secondary);
}

.stat-cpu-name {
  margin-top: 0.5rem;
  font-size: 0.7rem;
  color: var(--text-secondary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.load-values {
  display: flex;
  justify-content: space-between;
  font-size: 1.25rem;
}

.load-item {
  font-variant-numeric: tabular-nums;
}

.freq-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 0.5rem;
}

.freq-item {
  display: flex;
  justify-content: space-between;
  font-size: 0.75rem;
}

.freq-core {
  color: var(--text-secondary);
}

.freq-value {
  font-variant-numeric: tabular-nums;
}
</style>
