<template>
  <div class="process-list">
    <h3>📋 Top Processes</h3>
    <table>
      <thead>
        <tr>
          <th>PID</th>
          <th>Name</th>
          <th>CPU %</th>
          <th>Memory %</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="proc in processes.slice(0, 10)" :key="proc.pid">
          <td>{{ proc.pid }}</td>
          <td class="name">{{ proc.name }}</td>
          <td>{{ proc.cpu.toFixed(1) }}%</td>
          <td>{{ proc.memory.toFixed(1) }}%</td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script setup>
defineProps({
  processes: Array
})
</script>

<style scoped>
.process-list {
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

table {
  width: 100%;
  border-collapse: collapse;
}

th, td {
  text-align: left;
  padding: 0.75rem 0.5rem;
  border-bottom: 1px solid var(--border);
}

th {
  font-size: 0.75rem;
  font-weight: 600;
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

td {
  font-size: 0.85rem;
  color: var(--text-primary);
  font-variant-numeric: tabular-nums;
}

td.name {
  max-width: 200px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

tr:last-child td {
  border-bottom: none;
}

tr:hover td {
  background: rgba(99, 102, 241, 0.1);
}
</style>
