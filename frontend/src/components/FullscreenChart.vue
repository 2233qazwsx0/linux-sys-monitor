<template>
  <Teleport to="body">
    <div class="fullscreen-overlay" v-if="show" @click.self="closeFullscreen">
      <div class="fullscreen-container">
        <div class="fullscreen-header">
          <h3>{{ title }}</h3>
          <button @click="closeFullscreen">✕</button>
        </div>
        <div ref="chartRef" class="fullscreen-chart"></div>
      </div>
    </div>
  </Teleport>
</template>

<script setup>
import { ref, watch, nextTick } from 'vue'

const props = defineProps({
  show: Boolean,
  title: String,
  data: Array,
  type: { type: String, default: 'line' }
})

const emit = defineEmits(['close'])

const chartRef = ref(null)
let chart = null

function closeFullscreen() {
  emit('close')
  if (chart) {
    chart.dispose()
    chart = null
  }
}

function initChart() {
  if (!chartRef.value) return
  
  chart = echarts.init(chartRef.value)
  
  const option = {
    backgroundColor: 'transparent',
    tooltip: { trigger: 'axis' },
    grid: { left: '10%', right: '5%', top: '15%', bottom: '10%' },
    xAxis: { type: 'time', boundaryGap: false },
    yAxis: { type: 'value', axisLabel: { formatter: '{value}%' } },
    series: [{
      name: props.title,
      type: props.type,
      smooth: true,
      areaStyle: { opacity: 0.3 },
      data: []
    }],
    animation: true
  }
  
  if (props.type === 'line') {
    option.yAxis.min = 0
    option.yAxis.max = 100
  }
  
  chart.setOption(option)
}

function updateChart() {
  if (!chart) return
  
  let seriesData = []
  if (Array.isArray(props.data) && props.data.length > 0) {
    if (props.data[0].value !== undefined) {
      seriesData = props.data.map(d => [d.time, d.value])
    } else if (props.data[0].rx !== undefined) {
      seriesData = props.data.map(d => [d.time, d.rx, d.tx])
      chart.setOption({
        series: [
          { name: 'RX', data: props.data.map(d => [d.time, d.rx]) },
          { name: 'TX', data: props.data.map(d => [d.time, d.tx]) }
        ]
      })
      return
    }
  }
  
  chart.setOption({ series: [{ data: seriesData }] })
}

watch(() => props.show, async (val) => {
  if (val) {
    await nextTick()
    initChart()
    updateChart()
    if (chart) {
      chart.resize()
    }
  }
})

watch(() => props.data, updateChart, { deep: true })
</script>

<style scoped>
.fullscreen-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.9);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 500;
}

.fullscreen-container {
  width: 95vw;
  height: 90vh;
  background: var(--bg-card);
  border-radius: 1rem;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.fullscreen-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem 1.5rem;
  border-bottom: 1px solid var(--border);
}

.fullscreen-header h3 { margin: 0; font-size: 1.25rem; }
.fullscreen-header button {
  background: none;
  border: none;
  font-size: 1.5rem;
  cursor: pointer;
  color: var(--text-secondary);
}

.fullscreen-chart {
  flex: 1;
  min-height: 0;
}
</style>
