<template>
  <div ref="gaugeRef" class="gauge-chart"></div>
</template>

<script setup>
import { ref, watch, onMounted, onUnmounted } from 'vue'
import * as echarts from 'echarts'

const props = defineProps({
  value: { type: Number, default: 0 },
  min: { type: Number, default: 0 },
  max: { type: Number, default: 100 },
  name: { type: String, default: '' },
  color: { type: String, default: '#6366f1' },
  thresholds: {
    type: Object,
    default: () => ({
      warning: 60,
      danger: 80
    })
  }
})

const gaugeRef = ref(null)
let chart = null

function getColor(value) {
  if (value >= props.thresholds.danger) return '#ef4444'
  if (value >= props.thresholds.warning) return '#f59e0b'
  return props.color
}

function initChart() {
  if (!gaugeRef.value) return
  
  chart = echarts.init(gaugeRef.value)
  updateChart()
}

function updateChart() {
  if (!chart) return
  
  const percentage = ((props.value - props.min) / (props.max - props.min)) * 100
  const color = getColor(props.value)
  
  chart.setOption({
    series: [{
      type: 'gauge',
      startAngle: 180,
      endAngle: 0,
      min: props.min,
      max: props.max,
      splitNumber: 5,
      axisLine: {
        lineStyle: {
          width: 12,
          color: [[1, 'var(--bg-secondary)']]
        }
      },
      pointer: {
        icon: 'path://M12.8,0.7l12,40.1H0.7L12.8,0.7z',
        length: '55%',
        width: 8,
        offsetCenter: [0, '-20%'],
        itemStyle: { color }
      },
      axisTick: { length: 6, lineStyle: { color: 'auto', width: 1 } },
      splitLine: { length: 10, lineStyle: { color: 'auto', width: 2 } },
      axisLabel: { show: false },
      title: {
        offsetCenter: [0, '40%'],
        fontSize: 12,
        color: 'var(--text-secondary)'
      },
      detail: {
        valueAnimation: true,
        fontSize: 24,
        fontWeight: 'bold',
        offsetCenter: [0, '10%'],
        formatter: '{value}%',
        color: 'var(--text-primary)'
      },
      data: [{ value: props.value, name: props.name }]
    }]
  })
}

watch(() => props.value, updateChart)

onMounted(() => {
  initChart()
  window.addEventListener('resize', () => chart?.resize())
})

onUnmounted(() => {
  if (chart) chart.dispose()
})
</script>

<style scoped>
.gauge-chart {
  width: 100%;
  height: 150px;
}
</style>
