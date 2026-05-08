<template>
  <div class="notification-badge" @click="showPanel = !showPanel">
    <span class="badge-icon">🔔</span>
    <span v-if="badgeCount > 0" class="badge-count">{{ badgeCount > 99 ? '99+' : badgeCount }}</span>
    
    <Transition name="panel">
      <div v-if="showPanel" class="notification-panel" @click.stop>
        <div class="panel-header">
          <h4>{{ lang === 'zh' ? '通知' : 'Notifications' }}</h4>
          <div class="panel-actions">
            <button @click="markAllAsRead" v-if="badgeCount > 0">
              {{ lang === 'zh' ? '全部已读' : 'Mark all read' }}
            </button>
            <button @click="clearAll" class="clear-btn">
              {{ lang === 'zh' ? '清空' : 'Clear' }}
            </button>
          </div>
        </div>
        <div class="panel-content">
          <div v-if="notifications.length === 0" class="empty-state">
            {{ lang === 'zh' ? '暂无通知' : 'No notifications' }}
          </div>
          <div
            v-for="notif in notifications"
            :key="notif.id"
            :class="['notification-item', { unread: !notif.read }]"
            @click="markAsRead(notif.id)"
          >
            <span class="notif-icon">{{ getIcon(notif.type) }}</span>
            <span class="notif-message">{{ notif.message }}</span>
          </div>
        </div>
      </div>
    </Transition>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useNotificationBadge } from '../composables/useNotificationBadge'

const { badgeCount, notifications, markAsRead, markAllAsRead, clearNotifications } = useNotificationBadge()

const showPanel = ref(false)
const lang = computed(() => localStorage.getItem('lang') || 'en')

function clearAll() {
  clearNotifications()
}

function getIcon(type) {
  const icons = { info: 'ℹ', success: '✓', warning: '⚠', error: '✕' }
  return icons[type] || icons.info
}

function handleClickOutside(e) {
  if (!e.target.closest('.notification-badge')) {
    showPanel.value = false
  }
}

onMounted(() => {
  document.addEventListener('click', handleClickOutside)
})

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside)
})
</script>

<style scoped>
.notification-badge {
  position: relative;
  cursor: pointer;
  padding: 0.4rem;
}

.badge-icon {
  font-size: 1.1rem;
}

.badge-count {
  position: absolute;
  top: -4px;
  right: -4px;
  min-width: 18px;
  height: 18px;
  background: var(--danger);
  color: white;
  border-radius: 9px;
  font-size: 0.65rem;
  font-weight: bold;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 0 4px;
}

.notification-panel {
  position: absolute;
  top: 100%;
  right: 0;
  margin-top: 0.5rem;
  width: 320px;
  max-height: 400px;
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: 0.75rem;
  overflow: hidden;
  z-index: 200;
  box-shadow: 0 10px 40px rgba(0, 0, 0, 0.4);
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.75rem 1rem;
  border-bottom: 1px solid var(--border);
}

.panel-header h4 {
  margin: 0;
  font-size: 0.9rem;
}

.panel-actions {
  display: flex;
  gap: 0.5rem;
}

.panel-actions button {
  background: none;
  border: none;
  color: var(--accent);
  font-size: 0.75rem;
  cursor: pointer;
}

.panel-actions .clear-btn {
  color: var(--danger);
}

.panel-content {
  max-height: 320px;
  overflow-y: auto;
}

.empty-state {
  padding: 2rem;
  text-align: center;
  color: var(--text-secondary);
  font-size: 0.85rem;
}

.notification-item {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.75rem 1rem;
  border-bottom: 1px solid var(--border);
  cursor: pointer;
  transition: background 0.2s;
}

.notification-item:hover {
  background: var(--bg-secondary);
}

.notification-item.unread {
  background: rgba(99, 102, 241, 0.05);
}

.notif-icon {
  font-size: 1rem;
}

.notif-message {
  flex: 1;
  font-size: 0.85rem;
  color: var(--text-primary);
}

.panel-enter-active,
.panel-leave-active {
  transition: all 0.2s ease;
}

.panel-enter-from,
.panel-leave-to {
  opacity: 0;
  transform: translateY(-10px);
}
</style>
