<template>
  <div class="collapsible" :class="{ collapsed: isCollapsed }">
    <div class="collapsible-header" @click="toggle" :style="{ cursor: collapsible ? 'pointer' : 'default' }">
      <span class="collapse-icon" v-if="collapsible">{{ isCollapsed ? '▶' : '▼' }}</span>
      <slot name="header">
        <h3>{{ title }}</h3>
      </slot>
      <slot name="actions"></slot>
    </div>
    <Transition name="collapse">
      <div class="collapsible-content" v-show="!isCollapsedLocal">
        <slot></slot>
      </div>
    </Transition>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue'

const props = defineProps({
  title: String,
  collapsible: { type: Boolean, default: true },
  collapsed: { type: Boolean, default: false },
  sectionId: String
})

const emit = defineEmits(['toggle'])

const isCollapsedLocal = ref(props.collapsed)

const isCollapsed = computed(() => isCollapsedLocal.value)

function toggle() {
  if (!props.collapsible) return
  isCollapsedLocal.value = !isCollapsedLocal.value
  emit('toggle', isCollapsedLocal.value)
}

defineExpose({ toggle, isCollapsed: isCollapsedLocal })
</script>

<style scoped>
.collapsible {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: 1rem;
  overflow: hidden;
}

.collapsible-header {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 1rem 1.25rem;
  background: var(--bg-secondary);
  user-select: none;
}

.collapse-icon {
  font-size: 0.7rem;
  color: var(--text-secondary);
  transition: transform 0.2s;
}

.collapsed .collapse-icon {
  transform: rotate(-90deg);
}

.collapsible-header h3 {
  margin: 0;
  font-size: 1rem;
  color: var(--text-primary);
  flex: 1;
}

.collapsible-content {
  padding: 1.25rem;
}

.collapse-enter-active,
.collapse-leave-active {
  transition: all 0.3s ease;
  overflow: hidden;
}

.collapse-enter-from,
.collapse-leave-to {
  opacity: 0;
  max-height: 0;
  padding-top: 0;
  padding-bottom: 0;
}
</style>
