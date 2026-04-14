<script setup lang="ts">
import type { ActionItem } from '../types';
import { getActionTypeLabel, formatActionDetail, getTypeClass } from '../utils/actionHelpers';

defineProps<{
  actions: ActionItem[];
  selectedIndex: number | null;
}>();

const emit = defineEmits<{
  select: [index: number];
}>();
</script>

<template>
  <div class="action-list-readonly">
    <div class="list-header">
      <h4>动作列表</h4>
    </div>
    <div v-if="actions.length === 0" class="empty-state">
      暂无动作
    </div>
    <div v-else class="action-items">
      <div
        v-for="(item, idx) in actions"
        :key="idx"
        class="action-item"
        :class="{ selected: selectedIndex === idx }"
        @click="emit('select', idx)"
      >
        <div class="action-info">
          <div class="action-header">
            <span class="action-index">#{{ idx }}</span>
            <span class="type-badge" :class="getTypeClass(item)">
              {{ getActionTypeLabel(item) }}
            </span>
          </div>
          <div class="action-name" v-if="item.name">{{ item.name }}</div>
          <div class="action-detail">{{ formatActionDetail(item) }}</div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.action-list-readonly {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  background: var(--color-surface);
  border-left: 1px solid var(--color-border);
  border-right: 1px solid var(--color-border);
}

.list-header {
  padding: 12px 16px;
  border-bottom: 1px solid var(--color-border);
}

.list-header h4 {
  margin: 0;
  font-size: 13px;
  font-weight: 600;
  color: var(--color-text);
}

.action-list-readonly::-webkit-scrollbar {
  width: 6px;
}

.action-list-readonly::-webkit-scrollbar-track {
  background: var(--color-surface);
}

.action-list-readonly::-webkit-scrollbar-thumb {
  background: var(--color-border);
  border-radius: 3px;
}

.action-list-readonly::-webkit-scrollbar-thumb:hover {
  background: var(--color-text-muted);
}

.empty-state {
  text-align: center;
  padding: 24px 12px;
  color: var(--color-text-muted);
  font-size: 13px;
}

.action-items {
  flex: 1;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 4px;
  padding: 8px;
}

.action-item {
  display: flex;
  align-items: center;
  padding: 10px 12px;
  background: var(--color-surface-secondary);
  border-radius: var(--radius-sm);
  cursor: pointer;
  transition: background 0.15s;
  border: 1px solid transparent;
}

.action-item:hover {
  background: var(--color-surface-hover);
}

.action-item.selected {
  background: var(--color-primary-bg);
  border-color: var(--color-primary);
}

.action-info {
  flex: 1;
  min-width: 0;
}

.action-header {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 4px;
}

.action-index {
  font-family: monospace;
  font-weight: 600;
  font-size: 12px;
  color: var(--color-primary);
}

.type-badge {
  display: inline-block;
  padding: 2px 6px;
  font-size: 10px;
  font-weight: 600;
  border-radius: 3px;
  text-transform: uppercase;
}

.type-badge.key {
  background: #3b82f6;
  color: white;
}

.type-badge.key_sequence {
  background: #8b5cf6;
  color: white;
}

.type-badge.mouse_click {
  background: #f59e0b;
  color: white;
}

.type-badge.mouse_move {
  background: #10b981;
  color: white;
}

.type-badge.mouse_scroll {
  background: #06b6d4;
  color: white;
}

.type-badge.delay {
  background: #6b7280;
  color: white;
}

.type-badge.text {
  background: #ec4899;
  color: white;
}

.action-name {
  font-size: 13px;
  font-weight: 500;
  color: var(--color-text);
  margin-bottom: 2px;
}

.action-detail {
  font-size: 11px;
  color: var(--color-text-muted);
  font-family: monospace;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
</style>
