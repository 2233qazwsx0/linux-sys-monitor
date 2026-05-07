<template>
  <div class="container">
    <header>
      <h1>🐧 Linux System Monitor</h1>
      <div class="status">
        <span :class="['indicator', connected ? 'connected' : 'disconnected']"></span>
        {{ connected ? 'Connected' : 'Disconnected' }}
      </div>
    </header>

    <main>
      <SystemOverview :metrics="currentMetrics" />

      <div class="charts-grid">
        <CpuChart :data="cpuHistory" />
        <MemoryChart :data="memoryHistory" />
        <DiskChart :data="diskHistory" />
      </div>
    </main>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue'
import CpuChart from './components/CpuChart.vue'
import MemoryChart from './components/MemoryChart.vue'
import DiskChart from './components/DiskChart.vue'
import SystemOverview from './components/SystemOverview.vue'

const connected = ref(false)
const currentMetrics = ref(null)
const cpuHistory = ref([])
const memoryHistory = ref([])
const diskHistory = ref([])

let ws = null

function connect() {
  const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:'
  ws = new WebSocket(`${protocol}//${window.location.host}/ws`)

  ws.onopen = () => {
    connected.value = true
  }

  ws.onclose = () => {
    connected.value = false
    setTimeout(connect, 2000)
  }

  ws.onmessage = (event) => {
    const data = JSON.parse(event.data)
    currentMetrics.value = data

    const timestamp = new Date(data.timestamp * 1000)

    cpuHistory.value.push({ time: timestamp, value: data.cpu.usage })
    memoryHistory.value.push({ time: timestamp, value: data.memory.usage_percent })
    diskHistory.value.push({
      time: timestamp,
      read: data.disk.read_rate,
      write: data.disk.write_rate
    })

    const maxPoints = 60
    if (cpuHistory.value.length > maxPoints) cpuHistory.value.shift()
    if (memoryHistory.value.length > maxPoints) memoryHistory.value.shift()
    if (diskHistory.value.length > maxPoints) diskHistory.value.shift()
  }
}

onMounted(() => {
  connect()
})

onUnmounted(() => {
  if (ws) ws.close()
})
</script>

<style scoped>
.container {
  max-width: 1400px;
  margin: 0 auto;
  padding: 20px;
}

header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 30px;
}

h1 {
  margin: 0;
  color: #2c3e50;
}

.status {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 14px;
}

.indicator {
  width: 10px;
  height: 10px;
  border-radius: 50%;
}

.connected {
  background: #2ecc71;
  box-shadow: 0 0 10px #2ecc71;
}

.disconnected {
  background: #e74c3c;
}

.charts-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(400px, 1fr));
  gap: 20px;
}
</style>
