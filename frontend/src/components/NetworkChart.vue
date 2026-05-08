<template>
  <div class="network-security">
    <div class="chart-container">
      <div class="chart-header">
        <h3>🌐 Network Traffic</h3>
        <button @click="$emit('fullscreen', data)" class="fullscreen-btn" :title="lang === 'zh' ? '全屏' : 'Fullscreen'">⛶</button>
      </div>
      <div ref="chartRef" class="chart"></div>
    </div>

    <div class="security-grid">
      <div class="security-card">
        <h4>📡 Listening Ports</h4>
        <div class="port-list" v-if="securityInfo?.listening_ports?.length">
          <div v-for="(port, idx) in securityInfo.listening_ports.slice(0, 8)" :key="idx" class="port-item">
            <span class="port-protocol">{{ port.protocol }}</span>
            <span class="port-number">{{ port.local_port }}</span>
            <span class="port-program">{{ port.program || 'unknown' }}</span>
          </div>
        </div>
        <div v-else class="empty-state">No listening ports</div>
      </div>

      <div class="security-card">
        <h4>🔌 Interface Details</h4>
        <div class="interface-list" v-if="securityInfo?.interfaces?.length">
          <div v-for="iface in securityInfo.interfaces.slice(0, 4)" :key="iface.name" class="interface-item" :class="{ primary: iface.name === securityInfo.primary_interface }">
            <div class="interface-header">
              <span class="interface-name">{{ iface.name }}</span>
              <span v-if="iface.name === securityInfo.primary_interface" class="primary-badge">Primary</span>
            </div>
            <div class="interface-stats">
              <span>IP: {{ iface.ip_address }}</span>
              <span>MAC: {{ iface.mac_address }}</span>
            </div>
            <div class="interface-traffic">
              <span class="rx">↓ {{ formatBytes(iface.rx_bytes) }}</span>
              <span class="tx">↑ {{ formatBytes(iface.tx_bytes) }}</span>
            </div>
          </div>
        </div>
        <div v-else class="empty-state">No interfaces found</div>
      </div>

      <div class="security-card">
        <h4>📊 Bandwidth Quota</h4>
        <div class="quota-info">
          <div class="quota-item">
            <span class="quota-label">Session Download</span>
            <span class="quota-value">{{ formatBytes(securityInfo?.bandwidth_quota?.total_rx - securityInfo?.bandwidth_quota?.session_start_rx || 0) }}</span>
          </div>
          <div class="quota-item">
            <span class="quota-label">Session Upload</span>
            <span class="quota-value">{{ formatBytes(securityInfo?.bandwidth_quota?.total_tx - securityInfo?.bandwidth_quota?.session_start_tx || 0) }}</span>
          </div>
          <div class="quota-item">
            <span class="quota-label">Total Received</span>
            <span class="quota-value">{{ formatBytes(securityInfo?.bandwidth_quota?.total_rx || 0) }}</span>
          </div>
          <div class="quota-item">
            <span class="quota-label">Total Transmitted</span>
            <span class="quota-value">{{ formatBytes(securityInfo?.bandwidth_quota?.total_tx || 0) }}</span>
          </div>
        </div>
      </div>

      <div class="security-card">
        <h4>🔐 SSH/SFTP Sessions</h4>
        <div class="ssh-list" v-if="securityInfo?.ssh_sessions?.length">
          <div v-for="session in securityInfo.ssh_sessions" :key="session.pid" class="ssh-item">
            <span class="ssh-type">{{ session.session_type }}</span>
            <span class="ssh-user">{{ session.user }}</span>
            <span class="ssh-pid">PID: {{ session.pid }}</span>
          </div>
        </div>
        <div v-else class="empty-state">No active SSH sessions</div>
      </div>

      <div class="security-card">
        <h4>🌍 DNS Resolvers</h4>
        <div class="dns-list" v-if="securityInfo?.dns_servers?.nameservers?.length">
          <div v-for="ns in securityInfo.dns_servers.nameservers" :key="ns" class="dns-item">
            {{ ns }}
          </div>
        </div>
        <div v-else class="empty-state">No DNS servers configured</div>
      </div>

      <div class="security-card">
        <h4>🚪 Gateway</h4>
        <div class="gateway-info">
          <div class="gateway-label">Default Gateway</div>
          <div class="gateway-value">{{ securityInfo?.gateway?.default_gateway || 'N/A' }}</div>
        </div>
      </div>

      <div class="security-card">
        <h4>🔒 SSL Certificates</h4>
        <div class="cert-list" v-if="securityInfo?.ssl_certificates?.length">
          <div v-for="cert in securityInfo.ssl_certificates" :key="cert.path" class="cert-item" :class="{ warning: cert.days_until_expiry < 30, expired: !cert.is_valid }">
            <div class="cert-subject">{{ cert.subject || cert.path }}</div>
            <div class="cert-expiry">
              <span>{{ cert.days_until_expiry }} days</span>
              <span v-if="!cert.is_valid" class="badge expired">Expired</span>
              <span v-else-if="cert.days_until_expiry < 30" class="badge warning">Expiring</span>
            </div>
          </div>
        </div>
        <div v-else class="empty-state">No certificates found</div>
      </div>

      <div class="security-card">
        <h4>📈 Connection States</h4>
        <div class="state-list" v-if="securityInfo?.connection_states?.length">
          <div v-for="state in securityInfo.connection_states" :key="state.state" class="state-item">
            <span class="state-name" :class="getStateClass(state.state)">{{ state.state }}</span>
            <span class="state-count">{{ state.count }}</span>
          </div>
        </div>
        <div v-else class="empty-state">No connection data</div>
      </div>

      <div class="security-card wide">
        <h4>⚠️ Packet Loss / Reliability</h4>
        <div class="packet-loss-list" v-if="securityInfo?.packet_loss?.length">
          <div v-for="pkt in securityInfo.packet_loss" :key="pkt.interface" class="packet-loss-item">
            <div class="packet-header">
              <span class="packet-iface">{{ pkt.interface }}</span>
              <span class="packet-rate" :class="{ warning: pkt.drop_rate > 0.1, danger: pkt.drop_rate > 1 }">
                {{ pkt.drop_rate.toFixed(3) }}% drop
              </span>
            </div>
            <div class="packet-stats">
              <div class="stat"><span class="label">RX Errors:</span> {{ pkt.rx_errors }}</div>
              <div class="stat"><span class="label">TX Errors:</span> {{ pkt.tx_errors }}</div>
              <div class="stat"><span class="label">RX Dropped:</span> {{ pkt.rx_dropped }}</div>
              <div class="stat"><span class="label">TX Dropped:</span> {{ pkt.tx_dropped }}</div>
              <div class="stat"><span class="label">Total Packets:</span> {{ formatNumber(pkt.total_packets) }}</div>
            </div>
          </div>
        </div>
        <div v-else class="empty-state">No packet loss data</div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, watch, onMounted, onUnmounted } from 'vue'

const props = defineProps({
  data: Array,
  securityInfo: Object
})

defineEmits(['fullscreen'])

const lang = ref(localStorage.getItem('lang') || (navigator.language.toLowerCase().includes('zh') ? 'zh' : 'en'))
const chartRef = ref(null)
let chart = null

function initChart() {
  if (!chartRef.value) return
  chart = echarts.init(chartRef.value)
  
  chart.setOption({
    backgroundColor: 'transparent',
    tooltip: {
      trigger: 'axis',
      backgroundColor: 'rgba(22, 33, 62, 0.95)',
      borderColor: '#27272a',
      textStyle: { color: '#e4e4e7' },
      formatter: (params) => {
        let result = params[0].axisValue + '<br/>'
        params.forEach(p => {
          result += `<span style="display:inline-block;margin-right:4px;border-radius:10px;width:9px;height:9px;background-color:${p.color};"></span>${p.seriesName}: ${formatBytes(p.value)}/s<br/>`
        })
        return result
      }
    },
    legend: {
      data: ['Download', 'Upload'],
      textStyle: { color: '#a1a1aa' },
      top: 0
    },
    grid: { left: 60, right: 20, top: 40, bottom: 30 },
    xAxis: {
      type: 'time',
      boundaryGap: false,
      axisLine: { lineStyle: { color: '#27272a' } },
      axisLabel: { color: '#a1a1aa', fontSize: 11 }
    },
    yAxis: {
      type: 'value',
      axisLine: { lineStyle: { color: '#27272a' } },
      axisLabel: { color: '#a1a1aa', fontSize: 11, formatter: (v) => formatBytes(v) },
      splitLine: { lineStyle: { color: '#27272a', type: 'dashed' } }
    },
    series: [
      { name: 'Download', type: 'line', smooth: true, color: '#22c55e', data: [], lineStyle: { width: 2 }, areaStyle: { color: 'rgba(34, 197, 94, 0.1)' } },
      { name: 'Upload', type: 'line', smooth: true, color: '#f59e0b', data: [], lineStyle: { width: 2 }, areaStyle: { color: 'rgba(245, 158, 11, 0.1)' } }
    ],
    animation: true,
    animationDuration: 300
  })
}

function updateChart() {
  if (!chart) return
  chart.setOption({
    series: [
      { data: props.data.map(d => [d.time, d.rx]) },
      { data: props.data.map(d => [d.time, d.tx]) }
    ]
  })
}

function formatBytes(bytes) {
  if (bytes === 0 || bytes === undefined || bytes === null) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i]
}

function formatNumber(num) {
  if (num === undefined || num === null) return '0'
  return num.toLocaleString()
}

function getStateClass(state) {
  const classes = {
    'ESTABLISHED': 'established',
    'LISTEN': 'listen',
    'TIME_WAIT': 'time-wait',
    'CLOSE_WAIT': 'close-wait',
    'SYN_SENT': 'syn-sent',
    'FIN_WAIT1': 'fin-wait',
    'FIN_WAIT2': 'fin-wait',
  }
  return classes[state] || ''
}

watch(() => props.data?.length, updateChart)
onMounted(initChart)
onUnmounted(() => chart?.dispose())
</script>

<style scoped>
.network-security {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.chart-container {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: 1rem;
  padding: 1.25rem;
}

.chart-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
}

h3 {
  margin: 0 0 1rem 0;
  font-size: 1rem;
  color: var(--text-primary);
}

.chart {
  height: 280px;
}

.security-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 1rem;
}

.security-card {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: 1rem;
  padding: 1rem;
}

.security-card.wide {
  grid-column: span 2;
}

.security-card h4 {
  margin: 0 0 0.75rem 0;
  font-size: 0.9rem;
  color: var(--text-primary);
  border-bottom: 1px solid var(--border);
  padding-bottom: 0.5rem;
}

.port-list, .interface-list, .ssh-list, .dns-list, .cert-list, .state-list, .packet-loss-list {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  max-height: 200px;
  overflow-y: auto;
}

.port-item {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.35rem 0.5rem;
  background: var(--bg-secondary);
  border-radius: 0.35rem;
  font-size: 0.8rem;
}

.port-protocol {
  color: var(--accent);
  font-weight: 600;
}

.port-number {
  color: var(--text-primary);
  font-weight: 600;
}

.port-program {
  color: var(--text-secondary);
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.interface-item {
  padding: 0.5rem;
  background: var(--bg-secondary);
  border-radius: 0.5rem;
  font-size: 0.8rem;
}

.interface-item.primary {
  border: 1px solid var(--accent);
}

.interface-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 0.25rem;
}

.interface-name {
  font-weight: 600;
  color: var(--text-primary);
}

.primary-badge {
  background: var(--accent);
  color: white;
  padding: 0.1rem 0.4rem;
  border-radius: 0.25rem;
  font-size: 0.65rem;
}

.interface-stats {
  display: flex;
  flex-direction: column;
  color: var(--text-secondary);
  font-size: 0.75rem;
  margin-bottom: 0.25rem;
}

.interface-traffic {
  display: flex;
  gap: 1rem;
  font-size: 0.75rem;
}

.interface-traffic .rx {
  color: var(--success);
}

.interface-traffic .tx {
  color: var(--warning);
}

.quota-info {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.quota-item {
  display: flex;
  justify-content: space-between;
  padding: 0.35rem 0.5rem;
  background: var(--bg-secondary);
  border-radius: 0.35rem;
  font-size: 0.8rem;
}

.quota-label {
  color: var(--text-secondary);
}

.quota-value {
  color: var(--text-primary);
  font-weight: 600;
}

.ssh-item, .dns-item {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.35rem 0.5rem;
  background: var(--bg-secondary);
  border-radius: 0.35rem;
  font-size: 0.8rem;
}

.ssh-type {
  background: var(--accent);
  color: white;
  padding: 0.1rem 0.4rem;
  border-radius: 0.25rem;
  font-size: 0.65rem;
  font-weight: 600;
}

.ssh-user, .dns-item {
  color: var(--text-primary);
}

.ssh-pid {
  color: var(--text-secondary);
  font-size: 0.75rem;
}

.gateway-info {
  padding: 0.75rem;
  background: var(--bg-secondary);
  border-radius: 0.5rem;
}

.gateway-label {
  color: var(--text-secondary);
  font-size: 0.75rem;
  margin-bottom: 0.25rem;
}

.gateway-value {
  color: var(--text-primary);
  font-weight: 600;
  font-size: 1rem;
}

.cert-item {
  padding: 0.5rem;
  background: var(--bg-secondary);
  border-radius: 0.5rem;
  font-size: 0.8rem;
}

.cert-item.warning {
  border-left: 3px solid var(--warning);
}

.cert-item.expired {
  border-left: 3px solid var(--danger);
}

.cert-subject {
  color: var(--text-primary);
  margin-bottom: 0.25rem;
  font-size: 0.75rem;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.cert-expiry {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  color: var(--text-secondary);
  font-size: 0.75rem;
}

.badge {
  padding: 0.1rem 0.4rem;
  border-radius: 0.25rem;
  font-size: 0.65rem;
  font-weight: 600;
}

.badge.expired {
  background: var(--danger);
  color: white;
}

.badge.warning {
  background: var(--warning);
  color: black;
}

.state-item {
  display: flex;
  justify-content: space-between;
  padding: 0.35rem 0.5rem;
  background: var(--bg-secondary);
  border-radius: 0.35rem;
  font-size: 0.8rem;
}

.state-name {
  font-weight: 500;
}

.state-name.established {
  color: var(--success);
}

.state-name.listen {
  color: var(--accent);
}

.state-name.time-wait {
  color: var(--warning);
}

.state-name.close-wait {
  color: var(--danger);
}

.state-name.syn-sent {
  color: #8b5cf6;
}

.state-name.fin-wait {
  color: #ec4899;
}

.state-count {
  color: var(--text-secondary);
}

.packet-loss-item {
  padding: 0.5rem;
  background: var(--bg-secondary);
  border-radius: 0.5rem;
}

.packet-header {
  display: flex;
  justify-content: space-between;
  margin-bottom: 0.5rem;
}

.packet-iface {
  font-weight: 600;
  color: var(--text-primary);
}

.packet-rate {
  color: var(--success);
  font-size: 0.85rem;
}

.packet-rate.warning {
  color: var(--warning);
}

.packet-rate.danger {
  color: var(--danger);
}

.packet-stats {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
}

.stat {
  font-size: 0.75rem;
  color: var(--text-secondary);
}

.stat .label {
  color: var(--text-primary);
}

.empty-state {
  color: var(--text-secondary);
  font-size: 0.85rem;
  text-align: center;
  padding: 1rem;
}

@media (max-width: 1200px) {
  .security-grid {
    grid-template-columns: repeat(2, 1fr);
  }
  .security-card.wide {
    grid-column: span 2;
  }
}

@media (max-width: 768px) {
  .security-grid {
    grid-template-columns: 1fr;
  }
  .security-card.wide {
    grid-column: span 1;
  }
}
</style>
