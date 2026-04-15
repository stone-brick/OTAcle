<script setup lang="ts">
import type { ActionItem } from '../types';
import { getActionTypeLabel, formatActionDetail, getTypeClass } from '../utils/actionHelpers';

defineProps<{
  actions: ActionItem[];
  selectedIndex: number | null;
  showDelete?: boolean;
}>();

const emit = defineEmits<{
  select: [index: number];
  delete: [index: number];
}>();
</script>

<template>
  <div class="action-list">
    <div class="list-header">
      <h4>动作列表</h4>
    </div>
    <div v-if="actions.length === 0" class="empty-state">
      <slot name="empty">暂无动作</slot>
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
        <div v-if="showDelete" class="action-buttons">
          <button
            class="delete-btn"
            @click.stop="emit('delete', idx)"
            title="删除动作"
          >
            ✕
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.action-list {
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

.action-list::-webkit-scrollbar {
  width: var(--scrollbar-width);
}

.action-list::-webkit-scrollbar-track {
  background: var(--color-surface);
}

.action-list::-webkit-scrollbar-thumb {
  background: var(--color-border);
  border-radius: var(--scrollbar-radius);
}

.action-list::-webkit-scrollbar-thumb:hover {
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
  justify-content: space-between;
  padding: 10px 12px;
  background: var(--color-surface-secondary);
  border-radius: var(--radius-sm);
  cursor: pointer;
  transition: background var(--transition-duration);
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
  background: var(--color-action-key);
  color: white;
}

.type-badge.key_sequence {
  background: var(--color-action-key_sequence);
  color: white;
}

.type-badge.mouse_click {
  background: var(--color-action-mouse_click);
  color: white;
}

.type-badge.mouse_move {
  background: var(--color-action-mouse_move);
  color: white;
}

.type-badge.mouse_scroll {
  background: var(--color-action-mouse_scroll);
  color: white;
}

.type-badge.delay {
  background: var(--color-action-delay);
  color: white;
}

.type-badge.text {
  background: var(--color-action-text);
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

.action-buttons {
  display: flex;
  gap: 4px;
}

.delete-btn {
  padding: 4px 8px;
  font-size: 11px;
  background: transparent;
  color: var(--color-text-muted);
  border: none;
  border-radius: var(--radius-sm);
  cursor: pointer;
  opacity: 0;
  transition: opacity var(--transition-duration), color var(--transition-duration);
}

.action-item:hover .delete-btn {
  opacity: 1;
}

.delete-btn:hover {
  color: var(--color-error);
  background: var(--color-error-bg);
}
</style>
