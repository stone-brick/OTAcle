<script setup lang="ts">
import type { LogEntry } from '../types';

defineProps<{
  logs: LogEntry[];
}>();

const emit = defineEmits<{
  clear: [];
}>();

function handleClear() {
  emit('clear');
}
</script>

<template>
  <div class="log-panel">
    <div class="log-header">
      <h3>操作日志</h3>
      <button
        class="clear-btn"
        @click="handleClear"
      >
        清空
      </button>
    </div>

    <div class="log-list">
      <div
        v-for="(log, index) in logs"
        :key="index"
        class="log-entry"
        :class="log.type"
      >
        <span class="log-time">[{{ log.time }}]</span>
        <span class="log-message">{{ log.message }}</span>
      </div>

      <div
        v-if="logs.length === 0"
        class="empty-state"
      >
        暂无日志
      </div>
    </div>
  </div>
</template>

<style scoped>
.log-panel {
  height: 150px;
  display: flex;
  flex-direction: column;
  background: var(--color-surface);
  border-top: 1px solid var(--color-border);
}

.log-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 16px;
  border-bottom: 1px solid var(--color-border);
}

.log-header h3 {
  margin: 0;
  font-size: 13px;
  font-weight: 600;
}

.clear-btn {
  padding: 2px 10px;
  font-size: 11px;
  background: transparent;
  color: var(--color-text-secondary);
  border: 1px solid var(--color-border);
  border-radius: 3px;
  cursor: pointer;
}

.clear-btn:hover {
  background: var(--color-hover);
}

.log-list {
  flex: 1;
  overflow-y: auto;
  padding: 8px 16px;
  font-family: 'Consolas', 'Monaco', monospace;
  font-size: 12px;
}

.log-list::-webkit-scrollbar {
  width: var(--scrollbar-width);
}

.log-list::-webkit-scrollbar-track {
  background: var(--color-surface);
}

.log-list::-webkit-scrollbar-thumb {
  background: var(--color-border);
  border-radius: var(--scrollbar-radius);
}

.log-list::-webkit-scrollbar-thumb:hover {
  background: var(--color-text-muted);
}

.log-entry {
  display: flex;
  gap: 8px;
  padding: 4px 0;
  border-bottom: 1px solid var(--color-border-light);
}

.log-entry:last-child {
  border-bottom: none;
}

.log-time {
  color: var(--color-text-muted);
  flex-shrink: 0;
}

.log-message {
  color: var(--color-text);
}

.log-entry.success .log-message {
  color: var(--color-success);
}

.log-entry.error .log-message {
  color: var(--color-error);
}

.log-entry.info .log-message {
  color: var(--color-text-secondary);
}

.empty-state {
  text-align: center;
  padding: 16px;
  color: var(--color-text-muted);
  font-size: 12px;
}
</style>
