<template>
  <div class="chart-container">
    <h3>Disk I/O</h3>
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
    tooltip: { trigger: 'axis', formatter: (params) => {
      const time = params[0].axisValue
      let result = time + '<br/>'
      params.forEach(p => {
        result += p.marker + p.seriesName + ': ' + formatBytes(p.value) + '/s<br/>'
      })
      return result
    }},
    legend: { data: ['Read', 'Write'] },
    xAxis: { type: 'time', boundaryGap: false },
    yAxis: { type: 'value', axisLabel: { formatter: (v) => formatBytes(v) } },
    series: [
      { name: 'Read', type: 'line', smooth: true, color: '#2ecc71', data: [] },
      { name: 'Write', type: 'line', smooth: true, color: '#e74c3c', data: [] }
    ],
    animation: false
  })
}

function updateChart() {
  if (!chart) return
  chart.setOption({
    series: [
      { data: props.data.map(d => [d.time, d.read]) },
      { data: props.data.map(d => [d.time, d.write]) }
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

watch(() => props.data.length, updateChart)
onMounted(initChart)
onUnmounted(() => chart?.dispose())
</script>

<style scoped>
.chart-container {
  background: white;
  border-radius: 12px;
  padding: 20px;
  box-shadow: 0 2px 10px rgba(0,0,0,0.1);
}

h3 {
  margin: 0 0 15px 0;
  color: #2c3e50;
}

.chart {
  height: 250px;
}
</style>
