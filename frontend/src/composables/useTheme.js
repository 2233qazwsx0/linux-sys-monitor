import { ref, watch, computed } from 'vue'

const theme = ref(localStorage.getItem('theme') || 'dark')
const autoTheme = ref(localStorage.getItem('autoTheme') === 'true')
const isInitialized = ref(false)

function detectSystemTheme() {
  if (!autoTheme.value) return theme.value
  return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light'
}

function applyTheme(newTheme) {
  document.documentElement.setAttribute('data-theme', newTheme)
}

function toggleTheme() {
  if (autoTheme.value) {
    autoTheme.value = false
    localStorage.setItem('autoTheme', 'false')
  }
  theme.value = theme.value === 'dark' ? 'light' : 'dark'
  localStorage.setItem('theme', theme.value)
  applyTheme(theme.value)
}

function setAutoTheme(enabled) {
  autoTheme.value = enabled
  localStorage.setItem('autoTheme', String(enabled))
  if (enabled) {
    theme.value = detectSystemTheme()
    applyTheme(theme.value)
  }
}

function initTheme() {
  if (isInitialized.value) return
  isInitialized.value = true

  if (autoTheme.value) {
    theme.value = detectSystemTheme()
  }
  applyTheme(theme.value)

  window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
    if (autoTheme.value) {
      theme.value = e.matches ? 'dark' : 'light'
      applyTheme(theme.value)
    }
  })
}

export function useTheme() {
  return {
    theme: computed(() => autoTheme.value ? detectSystemTheme() : theme.value),
    rawTheme: theme,
    autoTheme,
    toggleTheme,
    setAutoTheme,
    initTheme
  }
}
