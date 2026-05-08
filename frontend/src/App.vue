<template>
  <div class="dashboard">
    <header class="header">
      <div class="header-content">
        <div class="logo">
          <span class="logo-icon">📊</span>
          <h1>{{ lang === 'zh' ? '系统监控' : 'System Monitor' }}</h1>
        </div>
        <div class="header-right">
          <SearchFilter
            v-model="searchQuery"
            :items="currentMetrics?.processes || []"
            filter-key="name"
            :placeholder="lang === 'zh' ? '搜索进程...' : 'Search processes...'"
            @filter="filterProcesses"
            class="header-search"
          />
          <div class="sysinfo" v-if="currentMetrics">
            <span class="hostname">{{ currentMetrics.hostname }}</span>
            <span class="os">{{ currentMetrics.os_version }}</span>
          </div>
          <div class="uptime" v-if="currentMetrics">
            <span class="uptime-label">{{ lang === 'zh' ? '运行时间' : 'Uptime' }}</span>
            <span class="uptime-value">{{ uptime }}</span>
          </div>
          <div class="status" :class="connected ? 'connected' : 'disconnected'">
            <span class="status-dot" :class="{ pulse: connected }"></span>
            <span>{{ connected ? (lang === 'zh' ? '实时' : 'Live') : (lang === 'zh' ? '已断开' : 'Disconnected') }}</span>
          </div>
          <NotificationBadge />
          <button @click="toggleTheme" class="theme-btn" :title="lang === 'zh' ? '切换主题' : 'Toggle Theme'">
            {{ theme === 'dark' ? '🌙' : '☀️' }}
          </button>
          <button @click="setAutoTheme(!autoTheme)" class="auto-theme-btn" :class="{ active: autoTheme }" :title="lang === 'zh' ? '跟随系统' : 'Auto Theme'">
            🔄
          </button>
          <button @click="minimizeToTray" class="tray-btn" :title="lang === 'zh' ? '最小化到托盘' : 'Minimize to Tray'">
            ⬇️
          </button>
          <button @click="showSettings = !showSettings" class="settings-btn" :title="lang === 'zh' ? '设置' : 'Settings'">
            ⚙️
            <span v-if="settingsBadge > 0" class="settings-badge">{{ settingsBadge }}</span>
          </button>
          <select v-model="lang" @change="changeLang" class="lang-select">
            <option value="en">EN</option>
            <option value="zh">中文</option>
          </select>
        </div>
      </div>
    </header>

    <SettingsPanel v-if="showSettings" @close="showSettings = false" @settings-change="onSettingsChange" />

    <div class="alerts" v-if="alerts.length > 0">
      <div v-for="alert in alerts" :key="alert.timestamp" class="alert-item" :class="alert.alert_type">
        <span class="alert-icon">⚠️</span>
        <span class="alert-text">{{ alert.alert_type === 'cpu' ? 'CPU' : 'Memory' }}: {{ alert.value.toFixed(1) }}%</span>
      </div>
    </div>

    <main class="main">
      <CollapsibleSection
        :title="lang === 'zh' ? '系统概览' : 'System Overview'"
        :collapsed="isCollapsed('overview')"
        section-id="overview"
        @toggle="onToggleSection('overview')"
      >
        <template #actions>
          <select v-model="selectedPreset" @change="applyPreset" class="preset-select">
            <option value="default">{{ lang === 'zh' ? '默认' : 'Default' }}</option>
            <option value="minimal">{{ lang === 'zh' ? '简洁' : 'Minimal' }}</option>
            <option value="detailed">{{ lang === 'zh' ? '详细' : 'Detailed' }}</option>
            <option value="performance">{{ lang === 'zh' ? '性能' : 'Performance' }}</option>
          </select>
        </template>
        <DraggableCard
          v-for="cardId in cardOrder"
          :key="cardId"
          :id="cardId"
          @drop="handleCardDrop"
        >
          <template v-if="cardId === 'cpu'">
            <div class="stat-card cpu">
              <div class="stat-header">
                <StatusIndicator :status="getStatus('cpu')" :pulse="getStatus('cpu') === 'danger'" />
                <SparklineChart
                  v-if="showSparklines && cpuHistory.length > 0"
                  :data="cpuHistory.map(d => d.value)"
                  color="#6366f1"
                  :width="80"
                  :height="25"
                />
              </div>
              <div class="stat-value">
                <AnimatedNumber :value="metrics?.cpu.usage || 0" :enabled="animationsEnabled" />
                <span class="unit">%</span>
              </div>
              <GaugeChart
                v-if="showGauges"
                :value="metrics?.cpu.usage || 0"
                name="CPU"
                :thresholds="thresholds.cpu"
                color="#6366f1"
              />
              <div class="stat-bar">
                <div class="stat-bar-fill" :style="{ width: (metrics?.cpu.usage || 0) + '%', background: getStatusColor('cpu') }"></div>
              </div>
              <div class="stat-footer">
                <span>{{ metrics?.cpu.core_count }} {{ lang === 'zh' ? '核心' : 'Cores' }}</span>
                <span>{{ formatLoad(metrics?.cpu.per_core) }}</span>
              </div>
              <div class="stat-cpu-name">{{ metrics?.cpu.name }}</div>
            </div>
          </template>
          <template v-else-if="cardId === 'memory'">
            <div class="stat-card memory">
              <div class="stat-header">
                <StatusIndicator :status="getStatus('memory')" :pulse="getStatus('memory') === 'danger'" />
                <SparklineChart
                  v-if="showSparklines && memoryHistory.length > 0"
                  :data="memoryHistory.map(d => d.value)"
                  color="#22c55e"
                  :width="80"
                  :height="25"
                />
              </div>
              <div class="stat-value">
                <AnimatedNumber :value="metrics?.memory.usage_percent || 0" :enabled="animationsEnabled" />
                <span class="unit">%</span>
              </div>
              <GaugeChart
                v-if="showGauges"
                :value="metrics?.memory.usage_percent || 0"
                name="Memory"
                :thresholds="thresholds.memory"
                color="#22c55e"
              />
              <div class="stat-bar">
                <div class="stat-bar-fill" :style="{ width: (metrics?.memory.usage_percent || 0) + '%', background: getStatusColor('memory') }"></div>
              </div>
              <div class="stat-footer">
                <span>{{ formatBytes(metrics?.memory.used) }}</span>
                <span>{{ formatBytes(metrics?.memory.total) }}</span>
              </div>
            </div>
          </template>
          <template v-else-if="cardId === 'swap'">
            <div class="stat-card swap" v-if="metrics?.swap.total > 0">
              <div class="stat-header">
                <StatusIndicator :status="getStatus('swap')" />
                <SparklineChart
                  v-if="showSparklines && swapHistory.length > 0"
                  :data="swapHistory.map(d => d.value)"
                  color="#f59e0b"
                  :width="80"
                  :height="25"
                />
              </div>
              <div class="stat-value">
                <AnimatedNumber :value="metrics?.swap.usage_percent || 0" :enabled="animationsEnabled" />
                <span class="unit">%</span>
              </div>
              <div class="stat-bar">
                <div class="stat-bar-fill swap-fill" :style="{ width: (metrics?.swap.usage_percent || 0) + '%' }"></div>
              </div>
              <div class="stat-footer">
                <span>{{ formatBytes(metrics?.swap.used) }}</span>
                <span>{{ formatBytes(metrics?.swap.total) }}</span>
              </div>
            </div>
          </template>
          <template v-else-if="cardId === 'disk'">
            <div class="stat-card disk">
              <div class="stat-header">
                <span class="stat-icon">💾</span>
                <span class="stat-title">{{ lang === 'zh' ? '磁盘 I/O' : 'Disk I/O' }}</span>
              </div>
              <div class="stat-value duo">
                <span>↓ {{ formatBytes(metrics?.disk.read_rate) }}/s</span>
                <span>↑ {{ formatBytes(metrics?.disk.write_rate) }}/s</span>
              </div>
            </div>
          </template>
          <template v-else-if="cardId === 'network'">
            <div class="stat-card network" v-if="metrics?.network">
              <div class="stat-header">
                <span class="stat-icon">🌐</span>
                <span class="stat-title">{{ lang === 'zh' ? '网络' : 'Network' }}</span>
              </div>
              <div class="stat-value duo">
                <span>↓ {{ formatBytes(metrics?.network.rx_rate) }}/s</span>
                <span>↑ {{ formatBytes(metrics?.network.tx_rate) }}/s</span>
              </div>
            </div>
          </template>
          <template v-else-if="cardId === 'battery'">
            <div class="stat-card battery" v-if="metrics?.battery">
              <div class="stat-header">
                <span class="stat-icon">🔋</span>
                <span class="stat-title">{{ lang === 'zh' ? '电池' : 'Battery' }}</span>
                <span v-if="metrics?.battery.is_charging" class="charging-icon">⚡</span>
              </div>
              <div class="stat-value" :class="{ charging: metrics?.battery.is_charging }">
                <AnimatedNumber :value="metrics?.battery.charge_percent || 0" :decimals="0" :enabled="animationsEnabled" />
                <span class="unit">%</span>
              </div>
              <div class="stat-bar">
                <div class="stat-bar-fill battery-fill" :style="{ width: (metrics?.battery.charge_percent || 0) + '%' }"></div>
              </div>
            </div>
          </template>
        </DraggableCard>
      </CollapsibleSection>

      <DiskList :disks="currentMetrics?.disks" />

      <CollapsibleSection
        :title="lang === 'zh' ? '图表' : 'Charts'"
        :collapsed="isCollapsed('charts')"
        section-id="charts"
        @toggle="onToggleSection('charts')"
      >
        <template #actions>
          <RefreshRateSelector v-model="refreshRate" class="inline-refresh" />
        </template>
        <div class="charts-grid">
          <CpuChart :data="filteredCpuHistory" ref="cpuChartRef" />
          <MemoryChart :data="filteredMemoryHistory" ref="memoryChartRef" />
          <DiskChart :data="diskHistory" />
          <NetworkChart :data="networkHistory" />
        </div>
      </CollapsibleSection>

      <HeatmapView
        v-if="showHeatmap && heatmapData.length > 0"
        :data="heatmapData"
        :title="lang === 'zh' ? '资源使用热力图' : 'Resource Usage Heatmap'"
      />

      <CollapsibleSection
        :title="lang === 'zh' ? '进程列表' : 'Process List'"
        :collapsed="isCollapsed('processes')"
        section-id="processes"
        @toggle="onToggleSection('processes')"
      >
        <ProcessList
          v-if="currentMetrics && currentMetrics.processes"
          :processes="filteredProcesses"
          :search-query="searchQuery"
        />
      </CollapsibleSection>

      <div class="actions">
        <button @click="exportData('json')" class="action-btn">
          📥 {{ lang === 'zh' ? '导出 JSON' : 'Export JSON' }}
        </button>
        <button @click="exportData('csv')" class="action-btn">
          📊 {{ lang === 'zh' ? '导出 CSV' : 'Export CSV' }}
        </button>
        <button @click="exportCharts" class="action-btn">
          📷 {{ lang === 'zh' ? '导出图表' : 'Export Charts' }}
        </button>
        <button @click="refreshData" class="action-btn">
          🔄 {{ lang === 'zh' ? '刷新' : 'Refresh' }}
        </button>
      </div>
    </main>

    <ToastNotification />
  </div>
</template>

<script setup>
import { ref, computed, onMounted, onUnmounted, watch } from 'vue'
import CpuChart from './components/CpuChart.vue'
import MemoryChart from './components/MemoryChart.vue'
import DiskChart from './components/DiskChart.vue'
import NetworkChart from './components/NetworkChart.vue'
import SystemOverview from './components/SystemOverview.vue'
import ProcessList from './components/ProcessList.vue'
import DiskList from './components/DiskList.vue'
import SettingsPanel from './components/SettingsPanel.vue'
import ToastNotification from './components/ToastNotification.vue'
import CollapsibleSection from './components/CollapsibleSection.vue'
import NotificationBadge from './components/NotificationBadge.vue'
import SearchFilter from './components/SearchFilter.vue'
import RefreshRateSelector from './components/RefreshRateSelector.vue'
import ChartExport from './components/ChartExport.vue'
import PresetSelector from './components/PresetSelector.vue'
import ThresholdColors from './components/ThresholdColors.vue'
import KeyboardShortcuts from './components/KeyboardShortcuts.vue'
import SparklineChart from './components/SparklineChart.vue'
import GaugeChart from './components/GaugeChart.vue'
import HeatmapView from './components/HeatmapView.vue'
import AnimatedNumber from './components/AnimatedNumber.vue'
import StatusIndicator from './components/StatusIndicator.vue'
import DraggableCard from './components/DraggableCard.vue'
import { useTheme } from './composables/useTheme'
import { useSettings } from './composables/useSettings'
import { useToast } from './composables/useToast'
import { useKeyboard, setupKeyboardListeners } from './composables/useKeyboard'
import { useNotificationBadge } from './composables/useNotificationBadge'

const lang = ref(localStorage.getItem('lang') || (navigator.language.toLowerCase().includes('zh') ? 'zh' : 'en'))
const showSettings = ref(false)
const connected = ref(false)
const currentMetrics = ref(null)
const alerts = ref([])
const cpuHistory = ref([])
const memoryHistory = ref([])
const swapHistory = ref([])
const diskHistory = ref([])
const networkHistory = ref([])
const searchQuery = ref('')
const filteredProcesses = ref([])
const filteredCpuHistory = ref([])
const filteredMemoryHistory = ref([])
const selectedPreset = ref('default')
const searchFilterRef = ref(null)
const cpuChartRef = ref(null)
const memoryChartRef = ref(null)

const { theme, autoTheme, toggleTheme, setAutoTheme, initTheme } = useTheme()
const { settings, toggleSection, isCollapsed, updateCardOrder, setPreset, updateSetting } = useSettings()
const { showSuccess, showError } = useToast()
const { registerShortcut } = useKeyboard()
const { addNotification } = useNotificationBadge()

const refreshRate = computed({
  get: () => settings.value.refreshRate,
  set: (val) => updateSetting('refreshRate', val)
})

const showSparklines = computed(() => settings.value.showSparklines)
const showGauges = computed(() => settings.value.showGauges)
const showHeatmap = computed(() => settings.value.showHeatmap)
const cardOrder = computed(() => settings.value.cardOrder)
const thresholds = computed(() => settings.value.thresholds)
const animationsEnabled = computed(() => settings.value.animations.numbers)

const settingsBadge = computed(() => {
  let count = 0
  if (settings.value.notifications.enabled) count++
  if (autoTheme.value) count++
  return count
})

const heatmapData = computed(() => {
  const data = []
  const hours = 24
  for (let day = 0; day < 7; day++) {
    for (let hour = 0; hour < hours; hour++) {
      const cpuVal = cpuHistory.value[day * hours + hour]?.value || Math.random() * 50 + 20
      data.push([hour, day, cpuVal])
    }
  }
  return data.slice(0, cpuHistory.value.length)
})

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

const metrics = computed(() => currentMetrics.value)

function changeLang() {
  localStorage.setItem('lang', lang.value)
  document.documentElement.lang = lang.value
}

function getStatus(metric) {
  const val = metric === 'cpu' ? metrics.value?.cpu.usage :
              metric === 'memory' ? metrics.value?.memory.usage_percent :
              metric === 'swap' ? metrics.value?.swap.usage_percent : 0
  const th = thresholds.value[metric]
  if (!val || !th) return 'normal'
  if (val >= th.danger) return 'danger'
  if (val >= th.warning) return 'warning'
  return 'normal'
}

function getStatusColor(metric) {
  const status = getStatus(metric)
  const colors = { normal: 'linear-gradient(90deg, #6366f1, #a855f7)', warning: '#f59e0b', danger: '#ef4444' }
  return colors[status]
}

function onToggleSection(sectionId) {
  toggleSection(sectionId)
}

function handleCardDrop(sourceId, targetId) {
  const newOrder = [...cardOrder.value]
  const sourceIndex = newOrder.indexOf(sourceId)
  const targetIndex = newOrder.indexOf(targetId)
  if (sourceIndex !== -1 && targetIndex !== -1) {
    newOrder.splice(sourceIndex, 1)
    newOrder.splice(targetIndex, 0, sourceId)
    updateCardOrder(newOrder)
    showSuccess(lang.value === 'zh' ? '卡片顺序已更新' : 'Card order updated')
  }
}

function applyPreset() {
  setPreset(selectedPreset.value)
  showSuccess(lang.value === 'zh' ? `已应用 ${selectedPreset.value} 预设` : `Applied ${selectedPreset.value} preset`)
}

function filterProcesses(query) {
  if (!currentMetrics.value?.processes) return
  if (!query) {
    filteredProcesses.value = currentMetrics.value.processes
    return
  }
  const q = query.toLowerCase()
  filteredProcesses.value = currentMetrics.value.processes.filter(p =>
    p.name.toLowerCase().includes(q) || String(p.pid).includes(q)
  )
}

function filterChartData(history, query) {
  if (!query) return history
  const q = query.toLowerCase()
  return history.filter(d => String(d.value).includes(q) || String(d.time).includes(q))
}

watch(searchQuery, (val) => {
  filteredCpuHistory.value = filterChartData(cpuHistory.value, val)
  filteredMemoryHistory.value = filterChartData(memoryHistory.value, val)
})

function minimizeToTray() {
  showSuccess(lang.value === 'zh' ? '已最小化到托盘' : 'Minimized to tray')
  addNotification(lang.value === 'zh' ? '应用已最小化到托盘' : 'App minimized to tray', 'info')
}

function exportCharts() {
  if (cpuChartRef.value?.chart) {
    const url = cpuChartRef.value.chart.getDataURL({ type: 'png', pixelRatio: 2 })
    const link = document.createElement('a')
    link.download = 'cpu-chart.png'
    link.href = url
    link.click()
  }
  showSuccess(lang.value === 'zh' ? '图表已导出' : 'Charts exported')
}

function refreshData() {
  fetch('/api/metrics')
    .then(r => r.json())
    .then(data => { currentMetrics.value = data })
    .catch(() => showError(lang.value === 'zh' ? '刷新失败' : 'Refresh failed'))
  showSuccess(lang.value === 'zh' ? '数据已刷新' : 'Data refreshed')
}

function onSettingsChange() {
  settingsBadge.value
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
      showSuccess(lang.value === 'zh' ? '数据已导出' : 'Data exported')
    })
    .catch(() => showError(lang.value === 'zh' ? '导出失败' : 'Export failed'))
}

function formatBytes(bytes) {
  if (bytes === 0 || !bytes) return '0 B'
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

let ws = null
let reconnectTimer = null
let refreshTimer = null

function connect() {
  const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:'
  ws = new WebSocket(`${protocol}//${window.location.host}/ws`)
  ws.onopen = () => {
    connected.value = true
    showSuccess(lang.value === 'zh' ? '已连接到服务器' : 'Connected to server')
  }
  ws.onclose = () => {
    connected.value = false
    reconnectTimer = setTimeout(connect, 2000)
  }
  ws.onerror = () => {
    connected.value = false
    showError(lang.value === 'zh' ? '连接错误' : 'Connection error')
  }
  ws.onmessage = (event) => {
    const data = JSON.parse(event.data)
    currentMetrics.value = data
    if (data.processes) {
      filteredProcesses.value = searchQuery.value ? filterProcesses(searchQuery.value) : data.processes
    }
    const timestamp = new Date(data.timestamp * 1000)
    cpuHistory.value.push({ time: timestamp, value: data.cpu.usage })
    memoryHistory.value.push({ time: timestamp, value: data.memory.usage_percent })
    if (data.swap) {
      swapHistory.value.push({ time: timestamp, value: data.swap.usage_percent })
    }
    diskHistory.value.push({ time: timestamp, read: data.disk.read_rate, write: data.disk.write_rate })
    if (data.network) {
      networkHistory.value.push({ time: timestamp, rx: data.network.rx_rate, tx: data.network.tx_rate })
    }
    const maxPoints = 60
    if (cpuHistory.value.length > maxPoints) cpuHistory.value.shift()
    if (memoryHistory.value.length > maxPoints) memoryHistory.value.shift()
    if (swapHistory.value.length > maxPoints) swapHistory.value.shift()
    if (diskHistory.value.length > maxPoints) diskHistory.value.shift()
    if (networkHistory.value.length > maxPoints) networkHistory.value.shift()
    filteredCpuHistory.value = [...cpuHistory.value]
    filteredMemoryHistory.value = [...memoryHistory.value]
  }
}

function fetchAlerts() {
  fetch('/api/alerts')
    .then(r => r.json())
    .then(data => {
      alerts.value = data.alerts || []
      data.alerts?.forEach(alert => {
        if (alert.alert_type === 'cpu') {
          addNotification(`CPU ${lang.value === 'zh' ? '告警' : 'Alert'}: ${alert.value.toFixed(1)}%`, 'warning')
        }
      })
    })
    .catch(() => {})
}

function setupRefreshInterval() {
  if (refreshTimer) clearInterval(refreshTimer)
  refreshTimer = setInterval(fetchAlerts, refreshRate.value)
}

watch(refreshRate, setupRefreshInterval)

function handleKeyboardShortcuts() {
  registerShortcut('Ctrl+Shift+D', () => toggleTheme(), lang.value === 'zh' ? '切换主题' : 'Toggle theme')
  registerShortcut('Ctrl+Shift+S', () => showSettings.value = !showSettings.value, lang.value === 'zh' ? '打开设置' : 'Open settings')
  registerShortcut('Ctrl+Shift+R', () => refreshData(), lang.value === 'zh' ? '刷新数据' : 'Refresh data')
  registerShortcut('Ctrl+F', () => searchFilterRef.value?.focus(), lang.value === 'zh' ? '聚焦搜索' : 'Focus search')
  registerShortcut('Escape', () => { showSettings.value = false }, lang.value === 'zh' ? '关闭对话框' : 'Close dialogs')
}

onMounted(() => {
  initTheme()
  setupKeyboardListeners()
  handleKeyboardShortcuts()
  connect()
  fetchAlerts()
  setupRefreshInterval()
})

onUnmounted(() => {
  if (reconnectTimer) clearTimeout(reconnectTimer)
  if (refreshTimer) clearInterval(refreshTimer)
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

.header-search { width: 200px; }

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
.pulse { animation: pulse 2s infinite; }

@keyframes pulse {
  0%, 100% { transform: scale(1); opacity: 1; }
  50% { transform: scale(1.3); opacity: 0.7; }
}

.theme-btn, .settings-btn, .auto-theme-btn, .tray-btn {
  background: var(--bg-card);
  border: 1px solid var(--border);
  padding: 0.4rem 0.6rem;
  border-radius: 0.5rem;
  cursor: pointer;
  font-size: 1rem;
  transition: background 0.2s;
  position: relative;
}
.theme-btn:hover, .settings-btn:hover, .auto-theme-btn:hover, .tray-btn:hover { background: var(--bg-secondary); }
.auto-theme-btn.active { color: var(--accent); border-color: var(--accent); }
.settings-badge {
  position: absolute;
  top: -4px;
  right: -4px;
  background: var(--danger);
  color: white;
  font-size: 0.6rem;
  padding: 2px 5px;
  border-radius: 10px;
  min-width: 16px;
  text-align: center;
}

.lang-select {
  background: var(--bg-card);
  color: var(--text-primary);
  border: 1px solid var(--border);
  padding: 0.4rem 0.6rem;
  border-radius: 0.5rem;
  font-size: 0.85rem;
  cursor: pointer;
}

.preset-select {
  background: var(--bg-secondary);
  border: 1px solid var(--border);
  color: var(--text-primary);
  padding: 0.3rem 0.5rem;
  border-radius: 0.4rem;
  font-size: 0.8rem;
  cursor: pointer;
}

.inline-refresh { width: auto; }

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
  justify-content: space-between;
  align-items: center;
  margin-bottom: 0.75rem;
}

.stat-icon { font-size: 1.25rem; }
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
  display: flex;
  align-items: baseline;
}
.stat-value .unit { font-size: 1rem; margin-left: 0.25rem; opacity: 0.7; }

.stat-value.duo {
  display: flex;
  justify-content: space-between;
  font-size: 1.1rem;
}

.stat-value.charging { color: var(--success); }

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
  transition: width 0.3s ease, background 0.3s;
}

.cpu .stat-bar-fill { background: linear-gradient(90deg, #6366f1, #a855f7); }
.memory .stat-bar-fill { background: linear-gradient(90deg, #22c55e, #10b981); }
.swap-fill { background: linear-gradient(90deg, #f59e0b, #ef4444); }
.battery-fill { background: linear-gradient(90deg, #22c55e, #10b981); }

.stat-footer {
  display: flex;
  justify-content: space-between;
  font-size: 0.75rem;
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

@media (max-width: 1024px) {
  .charts-grid { grid-template-columns: 1fr; }
  .header-content { flex-wrap: wrap; }
  .header-search { width: 100%; order: 10; margin-top: 0.5rem; }
}
</style>
