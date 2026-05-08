<template>
  <div class="settings-overlay" @click.self="$emit('close')">
    <div class="settings-panel">
      <div class="settings-header">
        <h2>{{ lang === 'zh' ? '设置' : 'Settings' }}</h2>
        <button @click="$emit('close')">✕</button>
      </div>

      <div class="settings-tabs">
        <button
          v-for="tab in tabs"
          :key="tab.id"
          :class="{ active: activeTab === tab.id }"
          @click="activeTab = tab.id"
        >
          {{ tab.icon }} {{ tab.label }}
        </button>
      </div>

      <div class="settings-content">
        <div v-if="activeTab === 'general'" class="tab-content">
          <div class="setting-group">
            <h3>{{ lang === 'zh' ? '显示选项' : 'Display Options' }}</h3>
            <label class="toggle-label">
              <span>{{ lang === 'zh' ? '显示趋势线' : 'Show Sparklines' }}</span>
              <input type="checkbox" v-model="localSettings.showSparklines" @change="saveSettings" />
            </label>
            <label class="toggle-label">
              <span>{{ lang === 'zh' ? '显示仪表盘' : 'Show Gauges' }}</span>
              <input type="checkbox" v-model="localSettings.showGauges" @change="saveSettings" />
            </label>
            <label class="toggle-label">
              <span>{{ lang === 'zh' ? '显示热力图' : 'Show Heatmap' }}</span>
              <input type="checkbox" v-model="localSettings.showHeatmap" @change="saveSettings" />
            </label>
            <label class="toggle-label">
              <span>{{ lang === 'zh' ? '数字动画' : 'Number Animations' }}</span>
              <input type="checkbox" v-model="localSettings.animations.numbers" @change="saveSettings" />
            </label>
          </div>

          <div class="setting-group">
            <h3>{{ lang === 'zh' ? '仪表板预设' : 'Dashboard Presets' }}</h3>
            <PresetSelector v-model="localSettings.dashboardPreset" @update:modelValue="applyPreset" />
          </div>
        </div>

        <div v-if="activeTab === 'thresholds'" class="tab-content">
          <div class="setting-group">
            <h3>{{ lang === 'zh' ? '阈值颜色设置' : 'Threshold Color Settings' }}</h3>
            <ThresholdColors v-model="localSettings.thresholds" @update:modelValue="saveSettings" />
          </div>

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
        </div>

        <div v-if="activeTab === 'notifications'" class="tab-content">
          <div class="setting-group">
            <h3>{{ lang === 'zh' ? '通知设置' : 'Notification Settings' }}</h3>
            <label class="toggle-label">
              <span>{{ lang === 'zh' ? '启用通知' : 'Enable Notifications' }}</span>
              <input type="checkbox" v-model="localSettings.notifications.enabled" @change="saveSettings" />
            </label>
            <label class="toggle-label">
              <span>{{ lang === 'zh' ? '声音提醒' : 'Sound Alerts' }}</span>
              <input type="checkbox" v-model="localSettings.notifications.sound" @change="saveSettings" />
            </label>
            <label class="toggle-label">
              <span>{{ lang === 'zh' ? '桌面通知' : 'Desktop Notifications' }}</span>
              <input type="checkbox" v-model="localSettings.notifications.desktop" @change="saveSettings" />
            </label>
          </div>

          <div class="setting-group">
            <h3>{{ lang === 'zh' ? '键盘快捷键' : 'Keyboard Shortcuts' }}</h3>
            <KeyboardShortcuts />
          </div>
        </div>

        <div v-if="activeTab === 'advanced'" class="tab-content">
          <div class="setting-group">
            <h3>{{ lang === 'zh' ? '刷新率' : 'Refresh Rate' }}</h3>
            <RefreshRateSelector v-model="localSettings.refreshRate" @update:modelValue="saveSettings" />
          </div>

          <div class="setting-group">
            <h3>{{ lang === 'zh' ? '数据设置' : 'Data Settings' }}</h3>
            <label>
              <span>{{ lang === 'zh' ? '历史数据点数' : 'History Data Points' }}</span>
              <input type="number" v-model.number="localSettings.maxDataPoints" min="10" max="200" @change="saveSettings" />
            </label>
          </div>

          <div class="setting-group">
            <h3>{{ lang === 'zh' ? '重置设置' : 'Reset Settings' }}</h3>
            <button @click="resetSettings" class="reset-btn">
              {{ lang === 'zh' ? '恢复默认设置' : 'Reset to Defaults' }}
            </button>
          </div>
        </div>

        <div class="setting-actions">
          <button @click="saveAllConfig" class="save-btn">
            {{ lang === 'zh' ? '保存所有设置' : 'Save All Settings' }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, watch } from 'vue'
import PresetSelector from './PresetSelector.vue'
import ThresholdColors from './ThresholdColors.vue'
import KeyboardShortcuts from './KeyboardShortcuts.vue'
import RefreshRateSelector from './RefreshRateSelector.vue'

const emit = defineEmits(['close', 'settings-change'])

const lang = ref(localStorage.getItem('lang') || 'en')
const activeTab = ref('general')

const tabs = computed(() => [
  { id: 'general', icon: '⚙️', label: lang.value === 'zh' ? '常规' : 'General' },
  { id: 'thresholds', icon: '🎨', label: lang.value === 'zh' ? '阈值' : 'Thresholds' },
  { id: 'notifications', icon: '🔔', label: lang.value === 'zh' ? '通知' : 'Notifications' },
  { id: 'advanced', icon: '🔧', label: lang.value === 'zh' ? '高级' : 'Advanced' }
])

const defaultSettings = {
  showSparklines: true,
  showGauges: false,
  showHeatmap: false,
  dashboardPreset: 'default',
  maxDataPoints: 60,
  refreshRate: 1000,
  animations: { numbers: true },
  notifications: { enabled: true, sound: false, desktop: false },
  thresholds: {
    cpu: { warning: 60, danger: 80 },
    memory: { warning: 70, danger: 85 },
    disk: { warning: 70, danger: 90 },
    network: { warning: 70, danger: 90 }
  }
}

const localSettings = ref(loadSettings())
const config = ref({ cpu_threshold: 80, memory_threshold: 85 })

function loadSettings() {
  try {
    const saved = localStorage.getItem('system_monitor_settings')
    if (saved) {
      return { ...defaultSettings, ...JSON.parse(saved) }
    }
  } catch (e) {}
  return { ...defaultSettings }
}

function saveSettings() {
  localStorage.setItem('system_monitor_settings', JSON.stringify(localSettings.value))
  emit('settings-change', localSettings.value)
}

function applyPreset(preset) {
  localSettings.value.dashboardPreset = preset
  const presets = {
    default: { cardOrder: ['cpu', 'memory', 'swap', 'disk', 'network', 'battery'], showSparklines: true, showGauges: false },
    minimal: { cardOrder: ['cpu', 'memory'], showSparklines: false, showGauges: false },
    detailed: { cardOrder: ['cpu', 'memory', 'swap', 'disk', 'network', 'battery'], showSparklines: true, showGauges: true },
    performance: { cardOrder: ['cpu', 'network', 'disk', 'memory'], showSparklines: true, showGauges: false }
  }
  if (presets[preset]) {
    Object.assign(localSettings.value, presets[preset])
  }
  saveSettings()
}

function resetSettings() {
  localSettings.value = { ...defaultSettings }
  saveSettings()
}

function saveAllConfig() {
  fetch('/api/alerts/config', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(config.value)
  })
  .then(() => {
    saveSettings()
    alert(lang.value === 'zh' ? '所有设置已保存!' : 'All settings saved!')
  })
  .catch(() => alert(lang.value === 'zh' ? '保存失败!' : 'Save failed!'))
}

onMounted(() => {
  fetch('/api/alerts')
    .then(r => r.json())
    .then(data => {
      if (data.config) {
        config.value = { ...data.config }
      }
    })
    .catch(() => {})
})
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
  width: 550px;
  max-width: 90vw;
  max-height: 85vh;
  overflow-y: auto;
}

.settings-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem 1.5rem;
  border-bottom: 1px solid var(--border);
  position: sticky;
  top: 0;
  background: var(--bg-card);
  z-index: 10;
}

.settings-header h2 { margin: 0; font-size: 1.25rem; }
.settings-header button { background: none; border: none; font-size: 1.25rem; cursor: pointer; color: var(--text-secondary); }

.settings-tabs {
  display: flex;
  gap: 0.25rem;
  padding: 0.75rem 1rem;
  border-bottom: 1px solid var(--border);
  overflow-x: auto;
}

.settings-tabs button {
  flex: 1;
  min-width: fit-content;
  padding: 0.5rem 0.75rem;
  background: none;
  border: none;
  color: var(--text-secondary);
  cursor: pointer;
  border-radius: 0.5rem;
  font-size: 0.8rem;
  transition: all 0.2s;
  white-space: nowrap;
}

.settings-tabs button:hover {
  background: var(--bg-secondary);
  color: var(--text-primary);
}

.settings-tabs button.active {
  background: var(--accent);
  color: white;
}

.settings-content { padding: 1.5rem; }

.tab-content { animation: fadeIn 0.2s; }

@keyframes fadeIn {
  from { opacity: 0; transform: translateY(10px); }
  to { opacity: 1; transform: translateY(0); }
}

.setting-group {
  margin-bottom: 1.5rem;
}

.setting-group h3 {
  margin: 0 0 1rem 0;
  font-size: 0.95rem;
  color: var(--text-primary);
  padding-bottom: 0.5rem;
  border-bottom: 1px solid var(--border);
}

.setting-group label {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 0.75rem;
  font-size: 0.85rem;
}

.setting-group input[type="number"] {
  background: var(--bg-secondary);
  border: 1px solid var(--border);
  color: var(--text-primary);
  padding: 0.5rem 0.75rem;
  border-radius: 0.5rem;
  width: 80px;
  text-align: center;
}

.toggle-label {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.5rem 0;
}

.toggle-label input[type="checkbox"] {
  width: 40px;
  height: 20px;
  appearance: none;
  background: var(--bg-secondary);
  border-radius: 10px;
  position: relative;
  cursor: pointer;
  transition: background 0.2s;
}

.toggle-label input[type="checkbox"]::after {
  content: '';
  position: absolute;
  top: 2px;
  left: 2px;
  width: 16px;
  height: 16px;
  background: white;
  border-radius: 50%;
  transition: transform 0.2s;
}

.toggle-label input[type="checkbox"]:checked {
  background: var(--accent);
}

.toggle-label input[type="checkbox"]:checked::after {
  transform: translateX(20px);
}

.reset-btn {
  width: 100%;
  background: rgba(239, 68, 68, 0.1);
  border: 1px solid var(--danger);
  color: var(--danger);
  padding: 0.75rem;
  border-radius: 0.5rem;
  cursor: pointer;
  font-weight: 600;
  transition: all 0.2s;
}

.reset-btn:hover {
  background: var(--danger);
  color: white;
}

.setting-actions {
  margin-top: 1.5rem;
  padding-top: 1rem;
  border-top: 1px solid var(--border);
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
  transition: opacity 0.2s;
}

.save-btn:hover { opacity: 0.9; }
</style>
