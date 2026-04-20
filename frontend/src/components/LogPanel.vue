<script setup lang="ts">
import { ref, computed } from 'vue';
import type { LogEntry, LogSource } from '../types';

const props = defineProps<{
  logs: LogEntry[];
}>();

const emit = defineEmits<{
  clear: [];
}>();

type FilterType = 'all' | 'info' | 'warn' | 'success' | 'error';
type SourceFilter = 'all' | LogSource;

const activeFilter = ref<FilterType>('all');
const activeSource = ref<SourceFilter>('all');

const filteredLogs = computed(() => {
  let result = props.logs;
  if (activeFilter.value !== 'all') {
    result = result.filter(log => log.type === activeFilter.value);
  }
  if (activeSource.value !== 'all') {
    result = result.filter(log => log.source === activeSource.value);
  }
  return result;
});

function setFilter(filter: FilterType) {
  activeFilter.value = filter;
}

function handleClear() {
  emit('clear');
}
</script>

<template>
  <div class="log-panel">
    <div class="log-header">
      <h3>操作日志</h3>
      <div class="filter-group">
        <button
          class="filter-btn"
          :class="{ active: activeFilter === 'all' }"
          @click="setFilter('all')"
        >
          全部
        </button>
        <button
          class="filter-btn info"
          :class="{ active: activeFilter === 'info' }"
          @click="setFilter('info')"
        >
          信息
        </button>
        <button
          class="filter-btn success"
          :class="{ active: activeFilter === 'success' }"
          @click="setFilter('success')"
        >
          成功
        </button>
        <button
          class="filter-btn warn"
          :class="{ active: activeFilter === 'warn' }"
          @click="setFilter('warn')"
        >
          警告
        </button>
        <button
          class="filter-btn error"
          :class="{ active: activeFilter === 'error' }"
          @click="setFilter('error')"
        >
          错误
        </button>
      </div>
      <div class="source-filter">
        <select v-model="activeSource" class="source-select">
          <option value="all">全部来源</option>
          <option value="system">系统</option>
          <option value="comm">通信</option>
          <option value="observe">观察</option>
          <option value="action">动作</option>
          <option value="window">窗口</option>
        </select>
      </div>
      <button
        class="clear-btn"
        @click="handleClear"
      >
        清空
      </button>
    </div>

    <div class="log-list">
      <div
        v-for="(log, index) in filteredLogs"
        :key="index"
        class="log-entry"
        :class="log.type"
      >
        <span class="log-time">[{{ log.time }}]</span>
        <span class="log-message">{{ log.message }}</span>
      </div>

      <div
        v-if="filteredLogs.length === 0"
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
  gap: 8px;
}

.log-header h3 {
  margin: 0;
  font-size: 13px;
  font-weight: 600;
  flex-shrink: 0;
}

.filter-group {
  display: flex;
  gap: 4px;
  flex: 1;
}

.filter-btn {
  padding: 2px 8px;
  font-size: 11px;
  background: transparent;
  color: var(--color-text-secondary);
  border: 1px solid var(--color-border);
  border-radius: 3px;
  cursor: pointer;
}

.filter-btn:hover {
  background: var(--color-hover);
}

.filter-btn.active {
  border-color: var(--color-border-active, var(--color-text-secondary));
  color: var(--color-text);
}

.filter-btn.success.active {
  color: var(--color-success);
  border-color: var(--color-success);
}

.filter-btn.warn.active {
  color: var(--color-warn);
  border-color: var(--color-warn);
}

.filter-btn.error.active {
  color: var(--color-error);
  border-color: var(--color-error);
}

.clear-btn {
  padding: 2px 10px;
  font-size: 11px;
  background: transparent;
  color: var(--color-text-secondary);
  border: 1px solid var(--color-border);
  border-radius: 3px;
  cursor: pointer;
  flex-shrink: 0;
}

.clear-btn:hover {
  background: var(--color-hover);
}

.source-filter {
  flex-shrink: 0;
}

.source-select {
  padding: 2px 8px;
  font-size: 11px;
  background: var(--color-surface);
  color: var(--color-text-secondary);
  border: 1px solid var(--color-border);
  border-radius: 3px;
  cursor: pointer;
}

.source-select:focus {
  outline: none;
  border-color: var(--color-primary);
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

.log-entry.warn .log-message {
  color: var(--color-warn);
}

.empty-state {
  text-align: center;
  padding: 16px;
  color: var(--color-text-muted);
  font-size: 12px;
}
</style>
