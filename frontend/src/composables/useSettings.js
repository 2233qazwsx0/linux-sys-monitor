import { ref, watch } from 'vue'

const SETTINGS_KEY = 'system_monitor_settings'

const defaultSettings = {
  refreshRate: 1000,
  cpuThreshold: 80,
  memoryThreshold: 85,
  diskThreshold: 90,
  networkThreshold: 80,
  dashboardPreset: 'default',
  cardOrder: ['cpu', 'memory', 'swap', 'disk', 'network', 'battery'],
  thresholds: {
    cpu: { warning: 60, danger: 80 },
    memory: { warning: 70, danger: 85 },
    disk: { warning: 70, danger: 90 },
    network: { warning: 70, danger: 90 }
  },
  chartZoom: true,
  showSparklines: true,
  showGauges: false,
  showHeatmap: false,
  animations: {
    numbers: true,
    transitions: true
  },
  notifications: {
    enabled: true,
    sound: false,
    desktop: false
  },
  collapsedSections: []
}

const settings = ref(loadSettings())

function loadSettings() {
  try {
    const saved = localStorage.getItem(SETTINGS_KEY)
    if (saved) {
      return { ...defaultSettings, ...JSON.parse(saved) }
    }
  } catch (e) {
    console.warn('Failed to load settings:', e)
  }
  return { ...defaultSettings }
}

function saveSettings() {
  try {
    localStorage.setItem(SETTINGS_KEY, JSON.stringify(settings.value))
  } catch (e) {
    console.warn('Failed to save settings:', e)
  }
}

watch(settings, saveSettings, { deep: true })

function updateSetting(key, value) {
  settings.value[key] = value
}

function resetSettings() {
  settings.value = { ...defaultSettings }
}

function toggleSection(section) {
  const index = settings.value.collapsedSections.indexOf(section)
  if (index === -1) {
    settings.value.collapsedSections.push(section)
  } else {
    settings.value.collapsedSections.splice(index, 1)
  }
}

function isCollapsed(section) {
  return settings.value.collapsedSections.includes(section)
}

function updateCardOrder(newOrder) {
  settings.value.cardOrder = newOrder
}

function setPreset(presetName) {
  settings.value.dashboardPreset = presetName
  const presets = {
    default: { cardOrder: ['cpu', 'memory', 'swap', 'disk', 'network', 'battery'], showSparklines: true, showGauges: false },
    minimal: { cardOrder: ['cpu', 'memory'], showSparklines: false, showGauges: false },
    detailed: { cardOrder: ['cpu', 'memory', 'swap', 'disk', 'network', 'battery'], showSparklines: true, showGauges: true },
    performance: { cardOrder: ['cpu', 'network', 'disk', 'memory'], showSparklines: true, showGauges: false }
  }
  if (presets[presetName]) {
    Object.assign(settings.value, presets[presetName])
  }
}

export function useSettings() {
  return {
    settings,
    updateSetting,
    resetSettings,
    toggleSection,
    isCollapsed,
    updateCardOrder,
    setPreset,
    defaultSettings
  }
}
