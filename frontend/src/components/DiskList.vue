<template>
  <div class="disk-list" v-if="disks && disks.length > 0">
    <h3>💿 {{ lang === 'zh' ? '磁盘空间' : 'Disk Space' }}</h3>
    <div class="disk-grid">
      <div class="disk-item" v-for="disk in disks" :key="disk.mount_point">
        <div class="disk-header">
          <span class="disk-name">{{ disk.name || disk.mount_point }}</span>
          <span class="disk-mount">{{ disk.mount_point }}</span>
        </div>
        <div class="disk-bar">
          <div class="disk-bar-fill" :style="{ width: disk.usage_percent + '%' }"></div>
        </div>
        <div class="disk-info">
          <span>{{ formatBytes(disk.used) }} / {{ formatBytes(disk.total) }}</span>
          <span>{{ disk.usage_percent.toFixed(1) }}%</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'

const lang = ref(window.i18n?.isZh ? 'zh' : 'en')

defineProps({
  disks: Array
})

function formatBytes(bytes) {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i]
}
</script>

<style scoped>
.disk-list {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: 1rem;
  padding: 1.25rem;
  margin-bottom: 1.5rem;
}

h3 {
  margin: 0 0 1rem 0;
  font-size: 1rem;
  color: var(--text-primary);
}

.disk-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 1rem;
}

.disk-item {
  background: var(--bg-secondary);
  border-radius: 0.75rem;
  padding: 1rem;
}

.disk-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 0.75rem;
}

.disk-name {
  font-weight: 600;
  font-size: 0.9rem;
}

.disk-mount {
  font-size: 0.75rem;
  color: var(--text-secondary);
}

.disk-bar {
  height: 8px;
  background: var(--bg-card);
  border-radius: 4px;
  overflow: hidden;
  margin-bottom: 0.5rem;
}

.disk-bar-fill {
  height: 100%;
  background: linear-gradient(90deg, #6366f1, #a855f7);
  border-radius: 4px;
  transition: width 0.3s ease;
}

.disk-info {
  display: flex;
  justify-content: space-between;
  font-size: 0.75rem;
  color: var(--text-secondary);
}
</style>
