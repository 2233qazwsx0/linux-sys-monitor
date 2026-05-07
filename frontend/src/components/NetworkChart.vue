<template>
  <div class="chart-container">
    <h3>🌐 Network Traffic</h3>
    <div ref="chartRef" class="chart"></div>
  </div>
</template>

<script setup>
import { ref, watch, onMounted, onUnmounted } from 'vue'

const props = defineProps({
  data: Array
})

const chartRef = ref(null)
let chart = null

function initChart() {
  if (!chartRef.value) return
  chart = echarts.init(chartRef.value)
  
  chart.setOption({
    backgroundColor: 'transparent',
    tooltip: {
      trigger: 'axis',
      backgroundColor: 'rgba(22, 33, 62, 0.95)',
      borderColor: '#27272a',
      textStyle: { color: '#e4e4e7' },
      formatter: (params) => {
        let result = params[0].axisValue + '<br/>'
        params.forEach(p => {
          result += `<span style="display:inline-block;margin-right:4px;border-radius:10px;width:9px;height:9px;background-color:${p.color};"></span>${p.seriesName}: ${formatBytes(p.value)}/s<br/>`
        })
        return result
      }
    },
    legend: {
      data: ['Download', 'Upload'],
      textStyle: { color: '#a1a1aa' },
      top: 0
    },
    grid: { left: 60, right: 20, top: 40, bottom: 30 },
    xAxis: {
      type: 'time',
      boundaryGap: false,
      axisLine: { lineStyle: { color: '#27272a' } },
      axisLabel: { color: '#a1a1aa', fontSize: 11 }
    },
    yAxis: {
      type: 'value',
      axisLine: { lineStyle: { color: '#27272a' } },
      axisLabel: { color: '#a1a1aa', fontSize: 11, formatter: (v) => formatBytes(v) },
      splitLine: { lineStyle: { color: '#27272a', type: 'dashed' } }
    },
    series: [
      { name: 'Download', type: 'line', smooth: true, color: '#22c55e', data: [], lineStyle: { width: 2 }, areaStyle: { color: 'rgba(34, 197, 94, 0.1)' } },
      { name: 'Upload', type: 'line', smooth: true, color: '#f59e0b', data: [], lineStyle: { width: 2 }, areaStyle: { color: 'rgba(245, 158, 11, 0.1)' } }
    ],
    animation: true,
    animationDuration: 300
  })
}

function updateChart() {
  if (!chart) return
  chart.setOption({
    series: [
      { data: props.data.map(d => [d.time, d.rx]) },
      { data: props.data.map(d => [d.time, d.tx]) }
    ]
  })
}

function formatBytes(bytes) {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i]
}

watch(() => props.data?.length, updateChart)
onMounted(initChart)
onUnmounted(() => chart?.dispose())
</script>

<style scoped>
.chart-container {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: 1rem;
  padding: 1.25rem;
}

h3 {
  margin: 0 0 1rem 0;
  font-size: 1rem;
  color: var(--text-primary);
}

.chart {
  height: 280px;
}
</style>
