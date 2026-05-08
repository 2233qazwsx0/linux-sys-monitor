<template>
  <div class="settings-overlay" @click.self="$emit('close')">
    <div class="settings-panel">
      <div class="settings-header">
        <h2>{{ lang === 'zh' ? '设置' : 'Settings' }}</h2>
        <button @click="$emit('close')">✕</button>
      </div>
      
      <div class="settings-content">
        <div class="setting-group">
          <h3>{{ lang === 'zh' ? '告警阈值' : 'Alert Thresholds' }}</h3>
          <label>
            <span>CPU {{ lang === 'zh' ? '告警' : 'Alert' }} (%)</span>
            <input type="number" v-model.number="config.cpu_threshold" min="0" max="100" />
          </label>
          <label>
            <span>Memory {{ lang === 'zh' ? '告警' : 'Alert' }} (%)</span>
            <input type="number" v-model.number="config.memory_threshold" min="0" max="100" />
          </label>
        </div>

        <div class="setting-group">
          <h3>{{ lang === 'zh' ? '显示设置' : 'Display Settings' }}</h3>
          <label>
            <span>{{ lang === 'zh' ? '刷新间隔' : 'Refresh Interval' }} (ms)</span>
            <select v-model.number="refreshInterval" @change="updateRefreshRate">
              <option :value="500">500ms</option>
              <option :value="1000">1s</option>
              <option :value="2000">2s</option>
              <option :value="5000">5s</option>
              <option :value="10000">10s</option>
            </select>
          </label>
          <label class="checkbox-label">
            <input type="checkbox" v-model="showQuickStats" @change="updateShowQuickStats" />
            <span>{{ lang === 'zh' ? '显示快速统计栏' : 'Show Quick Stats Bar' }}</span>
          </label>
        </div>

        <button @click="saveConfig" class="save-btn">
          {{ lang === 'zh' ? '保存设置' : 'Save Settings' }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'

defineEmits(['close'])

const lang = ref(localStorage.getItem('lang') || 'en')
const config = ref({ cpu_threshold: 80, memory_threshold: 85 })
const refreshInterval = ref(parseInt(localStorage.getItem('refreshInterval') || '1000'))
const showQuickStats = ref(localStorage.getItem('showQuickStats') !== 'false')

onMounted(() => {
  fetch('/api/alerts')
    .then(r => r.json())
    .then(data => { config.value = { ...data.config } })
    .catch(() => {})
})

function updateRefreshRate() {
  localStorage.setItem('refreshInterval', refreshInterval.value.toString())
  window.dispatchEvent(new CustomEvent('refresh-rate-change', { detail: refreshInterval.value }))
}

function updateShowQuickStats() {
  localStorage.setItem('showQuickStats', showQuickStats.value.toString())
  window.dispatchEvent(new CustomEvent('quick-stats-toggle', { detail: showQuickStats.value }))
}

function saveConfig() {
  fetch('/api/alerts/config', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(config.value)
  })
  .then(() => alert(lang.value === 'zh' ? '设置已保存!' : 'Settings saved!'))
  .catch(() => alert(lang.value === 'zh' ? '保存失败!' : 'Save failed!'))
}
</script>

<style scoped>
.settings-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 200;
}

.settings-panel {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: 1rem;
  width: 400px;
  max-width: 90vw;
  max-height: 80vh;
  overflow-y: auto;
}

.settings-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem 1.5rem;
  border-bottom: 1px solid var(--border);
}

.settings-header h2 { margin: 0; font-size: 1.25rem; }
.settings-header button { background: none; border: none; font-size: 1.25rem; cursor: pointer; color: var(--text-secondary); }

.settings-content { padding: 1.5rem; }

.setting-group h3 { margin: 0 0 1rem 0; font-size: 1rem; color: var(--text-secondary); }

.setting-group label {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
}

.setting-group input {
  background: var(--bg-secondary);
  border: 1px solid var(--border);
  color: var(--text-primary);
  padding: 0.5rem 0.75rem;
  border-radius: 0.5rem;
  width: 100px;
  text-align: center;
}

.setting-group select {
  background: var(--bg-secondary);
  border: 1px solid var(--border);
  color: var(--text-primary);
  padding: 0.5rem 0.75rem;
  border-radius: 0.5rem;
  width: 100px;
  cursor: pointer;
}

.checkbox-label {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
  cursor: pointer;
}

.checkbox-label input[type="checkbox"] {
  width: 18px;
  height: 18px;
  cursor: pointer;
}

.save-btn {
  width: 100%;
  background: var(--accent);
  border: none;
  color: white;
  padding: 0.75rem;
  border-radius: 0.5rem;
  cursor: pointer;
  font-weight: 600;
  margin-top: 1rem;
}
.save-btn:hover { opacity: 0.9; }
</style>
