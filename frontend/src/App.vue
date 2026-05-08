<template>
  <div class="dashboard">
    <header class="header">
      <div class="header-content">
        <div class="logo">
          <span class="logo-icon">📊</span>
          <h1>{{ $t('title') }}</h1>
        </div>
        <div class="header-right">
          <div class="sysinfo" v-if="currentMetrics">
            <span class="hostname">{{ currentMetrics.hostname }}</span>
            <span class="os">{{ currentMetrics.os_version }}</span>
          </div>
          <div class="uptime" v-if="currentMetrics">
            <span class="uptime-label">{{ lang === 'zh' ? '运行时间' : 'Uptime' }}</span>
            <span class="uptime-value">{{ uptime }}</span>
          </div>
          <div class="status" :class="connected ? 'connected' : 'disconnected'">
            <span class="status-dot"></span>
            <span>{{ connected ? $t('live') : $t('disconnected') }}</span>
          </div>
          <select v-model="lang" @change="changeLang" class="lang-select">
            <option value="en">EN</option>
            <option value="zh">中文</option>
          </select>
        </div>
      </div>
    </header>

    <main class="main">
      <SystemOverview :metrics="currentMetrics" />
      
      <DiskList :disks="currentMetrics?.disks" />
      
      <div class="charts-grid">
        <CpuChart :data="cpuHistory" />
        <MemoryChart :data="memoryHistory" />
        <DiskChart :data="diskHistory" />
        <NetworkChart :data="networkHistory" />
      </div>

      <ProcessList v-if="currentMetrics && currentMetrics.processes" :processes="currentMetrics.processes" />
    </main>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, onUnmounted } from 'vue'
import CpuChart from './components/CpuChart.vue'
import MemoryChart from './components/MemoryChart.vue'
import DiskChart from './components/DiskChart.vue'
import NetworkChart from './components/NetworkChart.vue'
import SystemOverview from './components/SystemOverview.vue'
import ProcessList from './components/ProcessList.vue'
import DiskList from './components/DiskList.vue'

const lang = ref(window.i18n?.isZh ? 'zh' : 'en')

function $t(key) {
  const translations = {
    'en': { 'title': 'System Monitor', 'live': 'Live', 'disconnected': 'Disconnected' },
    'zh': { 'title': '系统监控', 'live': '实时', 'disconnected': '已断开' }
  }
  return translations[lang.value][key] || key
}

function changeLang() {
  document.documentElement.lang = lang.value
}

const connected = ref(false)
const currentMetrics = ref(null)
const cpuHistory = ref([])
const memoryHistory = ref([])
const diskHistory = ref([])
const networkHistory = ref([])

const uptime = computed(() => {
  if (!currentMetrics.value?.uptime) return '0s'
  const seconds = currentMetrics.value.uptime
  const days = Math.floor(seconds / 86400)
  const hours = Math.floor((seconds % 86400) / 3600)
  const mins = Math.floor((seconds % 3600) / 60)
  if (days > 0) return `${days}d ${hours}h`
  if (hours > 0) return `${hours}h ${mins}m`
  return `${mins}m`
})

let ws = null
let reconnectTimer = null

function connect() {
  const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:'
  ws = new WebSocket(`${protocol}//${window.location.host}/ws`)
  
  ws.onopen = () => { connected.value = true }
  ws.onclose = () => {
    connected.value = false
    reconnectTimer = setTimeout(connect, 2000)
  }
  ws.onerror = () => { connected.value = false }
  
  ws.onmessage = (event) => {
    const data = JSON.parse(event.data)
    currentMetrics.value = data
    
    const timestamp = new Date(data.timestamp * 1000)
    
    cpuHistory.value.push({ time: timestamp, value: data.cpu.usage })
    memoryHistory.value.push({ time: timestamp, value: data.memory.usage_percent })
    diskHistory.value.push({ time: timestamp, read: data.disk.read_rate, write: data.disk.write_rate })
    
    if (data.network) {
      networkHistory.value.push({ time: timestamp, rx: data.network.rx_rate, tx: data.network.tx_rate })
    }
    
    const maxPoints = 60
    if (cpuHistory.value.length > maxPoints) cpuHistory.value.shift()
    if (memoryHistory.value.length > maxPoints) memoryHistory.value.shift()
    if (diskHistory.value.length > maxPoints) diskHistory.value.shift()
    if (networkHistory.value.length > maxPoints) networkHistory.value.shift()
  }
}

onMounted(() => { connect() })
onUnmounted(() => {
  if (reconnectTimer) clearTimeout(reconnectTimer)
  if (ws) ws.close()
})
</script>

<style scoped>
.dashboard { min-height: 100vh; }

.header {
  background: rgba(22, 33, 62, 0.8);
  backdrop-filter: blur(10px);
  border-bottom: 1px solid var(--border);
  padding: 1rem 2rem;
  position: sticky;
  top: 0;
  z-index: 100;
}

.header-content {
  max-width: 1400px;
  margin: 0 auto;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.logo { display: flex; align-items: center; gap: 0.75rem; }
.logo-icon { font-size: 1.75rem; }
.logo h1 {
  font-size: 1.5rem;
  font-weight: 700;
  background: linear-gradient(135deg, var(--accent), #a855f7);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}

.header-right { display: flex; align-items: center; gap: 1.5rem; }

.sysinfo { display: flex; flex-direction: column; align-items: flex-end; }
.hostname { font-weight: 600; font-size: 0.9rem; }
.os { font-size: 0.7rem; color: var(--text-secondary); }

.uptime { display: flex; flex-direction: column; align-items: flex-end; }
.uptime-label { font-size: 0.7rem; color: var(--text-secondary); text-transform: uppercase; }
.uptime-value { font-size: 0.9rem; font-weight: 600; font-variant-numeric: tabular-nums; }

.status { display: flex; align-items: center; gap: 0.5rem; padding: 0.5rem 1rem; border-radius: 9999px; font-size: 0.85rem; font-weight: 500; }
.status-dot { width: 8px; height: 8px; border-radius: 50%; animation: pulse 2s infinite; }
.connected { background: rgba(34, 197, 94, 0.15); color: var(--success); }
.connected .status-dot { background: var(--success); box-shadow: 0 0 10px var(--success); }
.disconnected { background: rgba(239, 68, 68, 0.15); color: var(--danger); }
.disconnected .status-dot { background: var(--danger); }

.lang-select {
  background: var(--bg-card);
  color: var(--text-primary);
  border: 1px solid var(--border);
  padding: 0.4rem 0.75rem;
  border-radius: 0.5rem;
  font-size: 0.85rem;
  cursor: pointer;
}

@keyframes pulse { 0%, 100% { opacity: 1; } 50% { opacity: 0.5; } }

.main { max-width: 1400px; margin: 0 auto; padding: 2rem; }

.charts-grid { display: grid; grid-template-columns: repeat(2, 1fr); gap: 1.5rem; margin-bottom: 2rem; }

@media (max-width: 1024px) { .charts-grid { grid-template-columns: 1fr; } }
</style>
