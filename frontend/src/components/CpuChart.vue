<template>
  <div class="chart-container">
    <h3>CPU Usage</h3>
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
    tooltip: { trigger: 'axis' },
    xAxis: { type: 'time', boundaryGap: false },
    yAxis: { type: 'value', min: 0, max: 100, axisLabel: { formatter: '{value}%' } },
    series: [{
      name: 'CPU',
      type: 'line',
      smooth: true,
      areaStyle: { opacity: 0.3 },
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

h3 {
  margin: 0 0 15px 0;
  color: #2c3e50;
}

.chart {
  height: 250px;
}
</style>
