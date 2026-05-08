<template>
  <div class="heatmap-container">
    <div class="heatmap-header">
      <h4>{{ title }}</h4>
      <div class="heatmap-legend">
        <span class="legend-low">{{ minLabel }}</span>
        <div class="legend-gradient"></div>
        <span class="legend-high">{{ maxLabel }}</span>
      </div>
    </div>
    <div ref="heatmapRef" class="heatmap"></div>
  </div>
</template>

<script setup>
import { ref, watch, onMounted, onUnmounted } from 'vue'
import * as echarts from 'echarts'

const props = defineProps({
  data: { type: Array, default: () => [] },
  title: { type: String, default: 'Heatmap' },
  xLabels: { type: Array, default: () => [] },
  yLabels: { type: Array, default: () => [] },
  min: { type: Number, default: 0 },
  max: { type: Number, default: 100 },
  minLabel: { type: String, default: 'Low' },
  maxLabel: { type: String, default: 'High' }
})

const heatmapRef = ref(null)
let chart = null

function initChart() {
  if (!heatmapRef.value) return
  
  chart = echarts.init(heatmapRef.value)
  updateChart()
}

function updateChart() {
  if (!chart) return
  
  chart.setOption({
    tooltip: {
      position: 'top',
      formatter: (params) => `${params.marker} ${params.data[2]}%`
    },
    grid: { left: '3%', right: '4%', bottom: '10%', top: '15%', containLabel: true },
    xAxis: {
      type: 'category',
      data: props.xLabels.length ? props.xLabels : Array.from({ length: 24 }, (_, i) => `${i}:00`),
      splitArea: { show: true },
      axisLabel: { fontSize: 10, color: 'var(--text-secondary)' }
    },
    yAxis: {
      type: 'category',
      data: props.yLabels.length ? props.yLabels : ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'],
      splitArea: { show: true },
      axisLabel: { fontSize: 10, color: 'var(--text-secondary)' }
    },
    visualMap: {
      min: props.min,
      max: props.max,
      calculable: true,
      orient: 'horizontal',
      left: 'center',
      bottom: '0%',
      inRange: {
        color: ['#22c55e', '#84cc16', '#f59e0b', '#ef4444']
      },
      textStyle: { color: 'var(--text-secondary)', fontSize: 10 }
    },
    series: [{
      type: 'heatmap',
      data: props.data,
      label: { show: false },
      emphasis: {
        itemStyle: { shadowBlur: 10, shadowColor: 'rgba(0, 0, 0, 0.5)' }
      }
    }]
  })
}

watch(() => props.data, updateChart, { deep: true })

onMounted(() => {
  initChart()
  window.addEventListener('resize', () => chart?.resize())
})

onUnmounted(() => {
  if (chart) chart.dispose()
})
</script>

<style scoped>
.heatmap-container {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: 1rem;
  padding: 1rem;
}

.heatmap-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
}

.heatmap-header h4 {
  margin: 0;
  font-size: 0.9rem;
  color: var(--text-primary);
}

.heatmap-legend {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-size: 0.7rem;
  color: var(--text-secondary);
}

.legend-gradient {
  width: 60px;
  height: 8px;
  border-radius: 4px;
  background: linear-gradient(90deg, #22c55e, #84cc16, #f59e0b, #ef4444);
}

.heatmap {
  width: 100%;
  height: 200px;
}
</style>
