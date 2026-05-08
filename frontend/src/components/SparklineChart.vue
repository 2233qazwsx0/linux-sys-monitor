<template>
  <canvas ref="sparklineRef" :width="width" :height="height" class="sparkline"></canvas>
</template>

<script setup>
import { ref, watch, onMounted, onUnmounted } from 'vue'
import * as echarts from 'echarts'

const props = defineProps({
  data: { type: Array, default: () => [] },
  color: { type: String, default: '#6366f1' },
  width: { type: Number, default: 100 },
  height: { type: Number, default: 30 },
  showArea: { type: Boolean, default: true }
})

const sparklineRef = ref(null)
let chart = null

function initChart() {
  if (!sparklineRef.value) return
  
  chart = echarts.init(sparklineRef.value)
  updateChart()
}

function updateChart() {
  if (!chart) return
  
  const isDark = document.documentElement.getAttribute('data-theme') === 'dark'
  const textColor = isDark ? '#a1a1aa' : '#64748b'
  
  chart.setOption({
    grid: { left: 0, top: 0, right: 0, bottom: 0 },
    xAxis: { show: false, type: 'category', data: props.data.map((_, i) => i) },
    yAxis: { show: false, type: 'value', min: 0, max: 100 },
    series: [{
      type: 'line',
      data: props.data,
      smooth: true,
      symbol: 'none',
      lineStyle: { color: props.color, width: 2 },
      areaStyle: props.showArea ? {
        color: new echarts.graphic.LinearGradient(0, 0, 0, 1, [
          { offset: 0, color: props.color + '80' },
          { offset: 1, color: props.color + '10' }
        ])
      } : undefined
    }],
    animation: false
  })
}

watch(() => props.data, updateChart, { deep: true })
watch(() => props.color, updateChart)

onMounted(() => {
  initChart()
  window.addEventListener('themechange', updateChart)
})

onUnmounted(() => {
  if (chart) chart.dispose()
  window.removeEventListener('themechange', updateChart)
})
</script>

<style scoped>
.sparkline {
  display: block;
}
</style>
