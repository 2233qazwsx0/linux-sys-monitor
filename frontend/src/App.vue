<template>
  <div class="dashboard">
    <header class="header">
      <div class="header-content">
        <div class="logo">
          <span class="logo-icon">📊</span>
          <h1>{{ lang === 'zh' ? '系统监控' : 'System Monitor' }}</h1>
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
            <span>{{ connected ? (lang === 'zh' ? '实时' : 'Live') : (lang === 'zh' ? '已断开' : 'Disconnected') }}</span>
          </div>
          <button @click="toggleTheme" class="theme-btn" :title="lang === 'zh' ? '切换主题' : 'Toggle Theme'">
            {{ theme === 'dark' ? '🌙' : '☀️' }}
          </button>
          <button @click="showSettings = !showSettings" class="settings-btn" :title="lang === 'zh' ? '设置' : 'Settings'">
            ⚙️
          </button>
          <select v-model="lang" @change="changeLang" class="lang-select">
            <option value="en">EN</option>
            <option value="zh">中文</option>
          </select>
        </div>
      </div>
    </header>

    <SettingsPanel v-if="showSettings" @close="showSettings = false" />

    <div class="alerts" v-if="alerts.length > 0">
      <div v-for="alert in alerts" :key="alert.timestamp" class="alert-item" :class="alert.alert_type">
        <span class="alert-icon">⚠️</span>
        <span class="alert-text">{{ alert.alert_type === 'cpu' ? 'CPU' : 'Memory' }}: {{ alert.value.toFixed(1) }}%</span>
      </div>
    </div>

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
      <div class="actions">
        <button @click="exportData('json')" class="action-btn">
          📥 {{ lang === 'zh' ? '导出 JSON' : 'Export JSON' }}
        </button>
        <button @click="exportData('csv')" class="action-btn">
          📊 {{ lang === 'zh' ? '导出 CSV' : 'Export CSV' }}
        </button>
      </div>
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
import SettingsPanel from './components/SettingsPanel.vue'

const lang = ref(localStorage.getItem('lang') || (navigator.language.toLowerCase().includes('zh') ? 'zh' : 'en'))
const theme = ref(localStorage.getItem('theme') || 'dark')
const showSettings = ref(false)
const connected = ref(false)
const currentMetrics = ref(null)
const alerts = ref([])
const cpuHistory = ref([])
const memoryHistory = ref([])
const diskHistory = ref([])
const networkHistory = ref([])

function changeLang() {
  localStorage.setItem('lang', lang.value)
  document.documentElement.lang = lang.value
}

function toggleTheme() {
  theme.value = theme.value === 'dark' ? 'light' : 'dark'
  localStorage.setItem('theme', theme.value)
  document.documentElement.setAttribute('data-theme', theme.value)
}

function exportData(format) {
  fetch('/api/history')
    .then(r => r.json())
    .then(data => {
      let content, filename, type
      if (format === 'json') {
        content = JSON.stringify(data, null, 2)
        filename = 'system-monitor-export.json'
        type = 'application/json'
      } else {
        const headers = ['timestamp', 'cpu_usage', 'memory_usage', 'disk_read', 'disk_write']
        const rows = data.map(d => [d.timestamp, d.cpu.usage, d.memory.usage_percent, d.disk.read_rate, d.disk.write_rate])
        content = [headers.join(','), ...rows.map(r => r.join(','))].join('\n')
        filename = 'system-monitor-export.csv'
        type = 'text/csv'
      }
      const blob = new Blob([content], { type })
      const url = URL.createObjectURL(blob)
      const a = document.createElement('a')
      a.href = url
      a.download = filename
      a.click()
      URL.revokeObjectURL(url)
    })
}

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

function fetchAlerts() {
  fetch('/api/alerts')
    .then(r => r.json())
    .then(data => { alerts.value = data.alerts })
    .catch(() => {})
}

onMounted(() => {
  document.documentElement.setAttribute('data-theme', theme.value)
  connect()
  setInterval(fetchAlerts, 5000)
})

onUnmounted(() => {
  if (reconnectTimer) clearTimeout(reconnectTimer)
  if (ws) ws.close()
})
</script>

<style scoped>
.dashboard { min-height: 100vh; }

.header {
  background: rgba(22, 33, 62, 0.9);
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

.header-right { display: flex; align-items: center; gap: 1rem; }

.sysinfo { display: flex; flex-direction: column; align-items: flex-end; }
.hostname { font-weight: 600; font-size: 0.9rem; }
.os { font-size: 0.7rem; color: var(--text-secondary); }

.uptime { display: flex; flex-direction: column; align-items: flex-end; }
.uptime-label { font-size: 0.7rem; color: var(--text-secondary); }
.uptime-value { font-size: 0.9rem; font-weight: 600; }

.status { display: flex; align-items: center; gap: 0.5rem; padding: 0.4rem 0.8rem; border-radius: 9999px; font-size: 0.8rem; }
.status-dot { width: 8px; height: 8px; border-radius: 50%; }
.connected { background: rgba(34, 197, 94, 0.15); color: var(--success); }
.connected .status-dot { background: var(--success); box-shadow: 0 0 10px var(--success); }
.disconnected { background: rgba(239, 68, 68, 0.15); color: var(--danger); }
.disconnected .status-dot { background: var(--danger); }

.theme-btn, .settings-btn {
  background: var(--bg-card);
  border: 1px solid var(--border);
  padding: 0.4rem 0.6rem;
  border-radius: 0.5rem;
  cursor: pointer;
  font-size: 1rem;
  transition: background 0.2s;
}
.theme-btn:hover, .settings-btn:hover { background: var(--bg-secondary); }

.lang-select {
  background: var(--bg-card);
  color: var(--text-primary);
  border: 1px solid var(--border);
  padding: 0.4rem 0.6rem;
  border-radius: 0.5rem;
  font-size: 0.85rem;
  cursor: pointer;
}

.alerts {
  max-width: 1400px;
  margin: 1rem auto;
  padding: 0 2rem;
  display: flex;
  gap: 1rem;
  flex-wrap: wrap;
}

.alert-item {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem 1rem;
  border-radius: 0.5rem;
  font-size: 0.85rem;
  animation: shake 0.5s;
}

.alert-item.cpu { background: rgba(239, 68, 68, 0.2); border: 1px solid var(--danger); color: var(--danger); }
.alert-item.memory { background: rgba(245, 158, 11, 0.2); border: 1px solid var(--warning); color: var(--warning); }

@keyframes shake {
  0%, 100% { transform: translateX(0); }
  25% { transform: translateX(-5px); }
  75% { transform: translateX(5px); }
}

.main { max-width: 1400px; margin: 0 auto; padding: 2rem; }

.charts-grid { display: grid; grid-template-columns: repeat(2, 1fr); gap: 1.5rem; margin-bottom: 2rem; }

.actions {
  display: flex;
  gap: 1rem;
  justify-content: center;
  margin-top: 2rem;
}

.action-btn {
  background: var(--bg-card);
  border: 1px solid var(--border);
  color: var(--text-primary);
  padding: 0.75rem 1.5rem;
  border-radius: 0.75rem;
  cursor: pointer;
  font-size: 0.9rem;
  transition: all 0.2s;
}
.action-btn:hover { background: var(--accent); border-color: var(--accent); }

@media (max-width: 1024px) { .charts-grid { grid-template-columns: 1fr; } }
</style>
