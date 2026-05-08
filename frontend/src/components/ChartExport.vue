<template>
  <div class="chart-export">
    <button @click="showMenu = !showMenu" class="export-btn" :title="title">
      📷 {{ title }}
    </button>
    <div v-if="showMenu" class="export-menu">
      <button @click="exportAs('png')">PNG</button>
      <button @click="exportAs('jpeg')">JPEG</button>
      <button @click="exportAs('svg')" v-if="supportsSvg">SVG</button>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue'

const props = defineProps({
  chartInstance: Object,
  title: { type: String, default: 'Export' },
  filename: { type: String, default: 'chart' },
  supportsSvg: { type: Boolean, default: false }
})

const emit = defineEmits(['exported'])

const showMenu = ref(false)

function exportAs(format) {
  if (!props.chartInstance) return
  
  const url = props.chartInstance.getDataURL({
    type: format,
    pixelRatio: 2,
    backgroundColor: document.documentElement.getAttribute('data-theme') === 'dark' ? '#0f0f1a' : '#ffffff'
  })
  
  const link = document.createElement('a')
  link.download = `${props.filename}.${format}`
  link.href = url
  link.click()
  
  showMenu.value = false
  emit('exported', format)
}

function handleClickOutside(e) {
  if (!e.target.closest('.chart-export')) {
    showMenu.value = false
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
.chart-export {
  position: relative;
}

.export-btn {
  background: var(--bg-card);
  border: 1px solid var(--border);
  color: var(--text-primary);
  padding: 0.5rem 0.75rem;
  border-radius: 0.5rem;
  cursor: pointer;
  font-size: 0.85rem;
  transition: all 0.2s;
}

.export-btn:hover {
  border-color: var(--accent);
}

.export-menu {
  position: absolute;
  top: 100%;
  right: 0;
  margin-top: 0.5rem;
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: 0.5rem;
  overflow: hidden;
  z-index: 100;
  box-shadow: 0 10px 40px rgba(0, 0, 0, 0.3);
}

.export-menu button {
  display: block;
  width: 100%;
  padding: 0.6rem 1.5rem;
  background: none;
  border: none;
  color: var(--text-primary);
  cursor: pointer;
  text-align: left;
  font-size: 0.85rem;
  transition: background 0.2s;
}

.export-menu button:hover {
  background: var(--bg-secondary);
}
</style>
