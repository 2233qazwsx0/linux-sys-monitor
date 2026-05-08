import { ref, watch } from 'vue'

const badgeCount = ref(0)
const notifications = ref([])
let notifId = 0

function addNotification(message, type = 'info') {
  const id = ++notifId
  notifications.value.push({
    id,
    message,
    type,
    read: false,
    timestamp: Date.now()
  })
  badgeCount.value = notifications.value.filter(n => !n.read).length
  return id
}

function markAsRead(id) {
  const notif = notifications.value.find(n => n.id === id)
  if (notif) {
    notif.read = true
    badgeCount.value = notifications.value.filter(n => !n.read).length
  }
}

function markAllAsRead() {
  notifications.value.forEach(n => n.read = true)
  badgeCount.value = 0
}

function clearNotifications() {
  notifications.value = []
  badgeCount.value = 0
}

function removeNotification(id) {
  const index = notifications.value.findIndex(n => n.id === id)
  if (index !== -1) {
    notifications.value.splice(index, 1)
    badgeCount.value = notifications.value.filter(n => !n.read).length
  }
}

export function useNotificationBadge() {
  return {
    badgeCount,
    notifications,
    addNotification,
    markAsRead,
    markAllAsRead,
    clearNotifications,
    removeNotification
  }
}
