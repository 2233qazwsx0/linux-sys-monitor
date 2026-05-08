import { ref } from 'vue'

const toasts = ref([])
let toastId = 0

function addToast(message, type = 'info', duration = 3000) {
  const id = ++toastId
  toasts.value.push({
    id,
    message,
    type,
    timestamp: Date.now()
  })

  if (duration > 0) {
    setTimeout(() => {
      removeToast(id)
    }, duration)
  }

  return id
}

function removeToast(id) {
  const index = toasts.value.findIndex(t => t.id === id)
  if (index !== -1) {
    toasts.value.splice(index, 1)
  }
}

function clearAllToasts() {
  toasts.value = []
}

function showSuccess(message, duration) {
  return addToast(message, 'success', duration)
}

function showError(message, duration) {
  return addToast(message, 'error', duration)
}

function showWarning(message, duration) {
  return addToast(message, 'warning', duration)
}

function showInfo(message, duration) {
  return addToast(message, 'info', duration)
}

export function useToast() {
  return {
    toasts,
    addToast,
    removeToast,
    clearAllToasts,
    showSuccess,
    showError,
    showWarning,
    showInfo
  }
}
