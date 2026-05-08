<template>
  <span class="animated-number" :class="{ animating }">
    {{ displayValue }}
  </span>
</template>

<script setup>
import { ref, watch, computed } from 'vue'

const props = defineProps({
  value: { type: Number, default: 0 },
  decimals: { type: Number, default: 1 },
  duration: { type: Number, default: 500 },
  enabled: { type: Boolean, default: true }
})

const displayValue = ref('0')
const animating = ref(false)
let animationFrame = null

function easeOutQuart(t) {
  return 1 - Math.pow(1 - t, 4)
}

function animateTo(target) {
  if (!props.enabled) {
    displayValue.value = target.toFixed(props.decimals)
    return
  }
  
  const start = parseFloat(displayValue.value) || 0
  const diff = target - start
  const startTime = performance.now()
  
  animating.value = true
  
  function update(currentTime) {
    const elapsed = currentTime - startTime
    const progress = Math.min(elapsed / props.duration, 1)
    const eased = easeOutQuart(progress)
    
    const current = start + diff * eased
    displayValue.value = current.toFixed(props.decimals)
    
    if (progress < 1) {
      animationFrame = requestAnimationFrame(update)
    } else {
      animating.value = false
      displayValue.value = target.toFixed(props.decimals)
    }
  }
  
  if (animationFrame) {
    cancelAnimationFrame(animationFrame)
  }
  animationFrame = requestAnimationFrame(update)
}

watch(() => props.value, (newVal) => {
  animateTo(newVal)
}, { immediate: true })
</script>

<style scoped>
.animated-number {
  font-variant-numeric: tabular-nums;
  transition: color 0.3s ease;
}

.animating {
  color: var(--accent);
}
</style>
