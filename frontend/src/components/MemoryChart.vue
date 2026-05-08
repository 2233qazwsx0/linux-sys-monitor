<template>
  <div class="chart-container">
    <div class="chart-header">
      <h3>Memory Usage</h3>
      <button @click="$emit('fullscreen', data)" class="fullscreen-btn" :title="lang === 'zh' ? '全屏' : 'Fullscreen'">⛶</button>
    </div>
    <div ref="chartRef" class="chart"></div>
  </div>
</template>

<script setup>
import { ref, watch, onMounted, onUnmounted } from 'vue'

const props = defineProps({
  data: Array
})

defineEmits(['fullscreen'])

const lang = ref(localStorage.getItem('lang') || (navigator.language.toLowerCase().includes('zh') ? 'zh' : 'en'))
const chartRef = ref(null)
let chart = null

function initChart() {
  if (!chartRef.value) return
  chart = echarts.init(chartRef.value)

  chart.setOption({
    tooltip: { trigger: 'axis' },
    xAxis: { type: 'time', boundaryGap: false },
    yAxis: { type: 'value', min: 0, max: 100, axisLabel: { formatter: '{value}%' } },
    series: [{
      name: 'Memory',
      type: 'line',
      smooth: true,
      color: '#9b59b6',
      areaStyle: { color: 'rgba(155, 89, 182, 0.3)' },
      data: []
    }],
    animation: false
  })
}

function updateChart() {
  if (!chart) return
  chart.setOption({
    series: [{
      data: props.data.map(d => [d.time, d.value])
    }]
  })
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

.chart-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 15px;
}

h3 {
  margin: 0;
  color: #2c3e50;
}

.fullscreen-btn {
  background: none;
  border: none;
  font-size: 1.25rem;
  cursor: pointer;
  color: #666;
  padding: 0.25rem;
  border-radius: 0.25rem;
  transition: background 0.2s;
}

.fullscreen-btn:hover {
  background: rgba(0, 0, 0, 0.1);
}

.chart {
  height: 250px;
}
</style>
