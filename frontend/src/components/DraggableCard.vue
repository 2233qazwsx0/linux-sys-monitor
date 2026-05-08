<template>
  <div
    class="metric-card-wrapper"
    :class="{ dragging: isDragging }"
    draggable="true"
    @dragstart="onDragStart"
    @dragend="onDragEnd"
    @dragover.prevent
    @drop="onDrop"
  >
    <div class="drag-handle">⋮⋮</div>
    <slot></slot>
  </div>
</template>

<script setup>
import { ref } from 'vue'

const props = defineProps({
  id: { type: String, required: true }
})

const emit = defineEmits(['drag-start', 'drag-end', 'drop'])

const isDragging = ref(false)

function onDragStart(e) {
  isDragging.value = true
  e.dataTransfer.effectAllowed = 'move'
  e.dataTransfer.setData('text/plain', props.id)
  emit('drag-start', props.id)
}

function onDragEnd() {
  isDragging.value = false
  emit('drag-end')
}

function onDrop(e) {
  const sourceId = e.dataTransfer.getData('text/plain')
  emit('drop', sourceId, props.id)
}
</script>

<style scoped>
.metric-card-wrapper {
  position: relative;
  transition: opacity 0.3s, transform 0.3s;
}

.metric-card-wrapper.dragging {
  opacity: 0.5;
  transform: scale(1.02);
}

.drag-handle {
  position: absolute;
  top: 0.5rem;
  left: 0.5rem;
  cursor: grab;
  color: var(--text-secondary);
  font-size: 0.9rem;
  padding: 0.25rem;
  border-radius: 0.25rem;
  opacity: 0;
  transition: opacity 0.2s;
  z-index: 10;
  user-select: none;
}

.metric-card-wrapper:hover .drag-handle {
  opacity: 1;
}

.drag-handle:active {
  cursor: grabbing;
}
</style>
