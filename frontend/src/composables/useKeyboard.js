import { onMounted, onUnmounted } from 'vue'

const shortcuts = new Map()

function registerShortcut(key, callback, description = '') {
  const shortcutId = `${key}-${Date.now()}`
  shortcuts.set(shortcutId, { key, callback, description })
  return shortcutId
}

function unregisterShortcut(shortcutId) {
  shortcuts.delete(shortcutId)
}

function handleKeydown(event) {
  const key = [
    event.ctrlKey && 'Ctrl',
    event.shiftKey && 'Shift',
    event.altKey && 'Alt',
    event.metaKey && 'Meta',
    event.key.toLowerCase()
  ].filter(Boolean).join('+')

  for (const [, shortcut] of shortcuts) {
    if (shortcut.key.toLowerCase() === key.toLowerCase()) {
      event.preventDefault()
      event.stopPropagation()
      shortcut.callback(event)
      break
    }
  }
}

function setupKeyboardListeners() {
  window.addEventListener('keydown', handleKeydown)
}

function cleanupKeyboardListeners() {
  window.removeEventListener('keydown', handleKeydown)
  shortcuts.clear()
}

export function useKeyboard() {
  return {
    registerShortcut,
    unregisterShortcut,
    setupKeyboardListeners,
    cleanupKeyboardListeners
  }
}

export { setupKeyboardListeners, cleanupKeyboardListeners }
