<script setup lang="ts">
import type { ActionItem } from '../../types';
import { getActionTypeLabel, formatActionDetail, getTypeClass } from '../../utils/actionHelpers';

const props = defineProps<{
  actions: ActionItem[];
  selectedIndex: number | null;
  changedIndices?: number[];
}>();

const emit = defineEmits<{
  select: [index: number];
  delete: [index: number];
  discard: [index: number];
}>();

function isChanged(index: number): boolean {
  return props.changedIndices?.includes(index) ?? false;
}
</script>

<template>
  <div class="action-list-editor">
    <div v-if="actions.length === 0" class="empty-state">
      <template v-if="changedIndices && changedIndices.length > 0">
        已筛选变更动作（无匹配项）
      </template>
      <template v-else>
        暂无动作，请点击上方「新建动作」创建
      </template>
    </div>
    <div v-else class="action-items">
      <div
        v-for="(item, idx) in actions"
        :key="idx"
        class="action-item"
        :class="{
          selected: selectedIndex === idx,
          changed: isChanged(idx)
        }"
        @click="emit('select', idx)"
      >
        <div class="action-info">
          <div class="action-header">
            <span class="action-index">#{{ idx }}</span>
            <span class="type-badge" :class="getTypeClass(item)">
              {{ getActionTypeLabel(item) }}
            </span>
            <span v-if="isChanged(idx)" class="changed-indicator" title="已修改">●</span>
          </div>
          <div class="action-name" v-if="item.name">{{ item.name }}</div>
          <div class="action-detail">{{ formatActionDetail(item) }}</div>
        </div>
        <div class="action-buttons">
          <button
            v-if="isChanged(idx)"
            class="discard-btn"
            @click.stop="emit('discard', idx)"
            title="撤销此动作的修改"
          >
            ↩
          </button>
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
.action-list-editor {
  flex: 1;
  overflow-y: auto;
  padding-bottom: 8px;
}

.action-list-editor::-webkit-scrollbar {
  width: 6px;
}

.action-list-editor::-webkit-scrollbar-track {
  background: var(--color-surface);
}

.action-list-editor::-webkit-scrollbar-thumb {
  background: var(--color-border);
  border-radius: 3px;
}

.action-list-editor::-webkit-scrollbar-thumb:hover {
  background: var(--color-text-muted);
}

.empty-state {
  text-align: center;
  padding: 24px 12px;
  color: var(--color-text-muted);
  font-size: 13px;
}

.action-items {
  display: flex;
  flex-direction: column;
  gap: 6px;
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

.action-item.changed {
  border-color: var(--color-warning);
  background: color-mix(in srgb, var(--color-warning-bg) 30%, var(--color-surface-secondary));
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

.changed-indicator {
  color: var(--color-warning);
  font-size: 8px;
  margin-left: 2px;
}

.action-buttons {
  display: flex;
  gap: 4px;
}

.discard-btn,
.delete-btn {
  padding: 4px 8px;
  font-size: 11px;
  background: transparent;
  color: var(--color-text-muted);
  border: none;
  border-radius: var(--radius-sm);
  cursor: pointer;
  opacity: 0;
  transition: opacity 0.15s, color 0.15s;
}

.action-item:hover .discard-btn,
.action-item:hover .delete-btn {
  opacity: 1;
}

.discard-btn:hover {
  color: var(--color-warning);
  background: var(--color-warning-bg);
}

.delete-btn:hover {
  color: var(--color-error);
  background: var(--color-error-bg);
}
</style>
