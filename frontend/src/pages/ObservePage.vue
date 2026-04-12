<script setup lang="ts">
import { useWindows } from '../composables/useWindows'
import ObservePanel from '../components/ObservePanel.vue'

const { selectedWindow } = useWindows()
</script>

<template>
  <div class="observe-page">
    <main class="observe-content">
      <ObservePanel :window="selectedWindow" />
    </main>
  </div>
</template>

<style scoped>
.observe-page {
  display: flex;
  height: 100%;
  overflow: hidden;
}

.observe-content {
  flex: 1;
  overflow: auto;
  background: var(--color-background);
}

.observe-content::-webkit-scrollbar {
  width: 8px;
}

.observe-content::-webkit-scrollbar-track {
  background: var(--color-background);
}

.observe-content::-webkit-scrollbar-thumb {
  background: var(--color-border);
  border-radius: 4px;
}

.observe-content::-webkit-scrollbar-thumb:hover {
  background: var(--color-text-muted);
}
</style>
