<script setup lang="ts">
import type { ActionItem, Action } from '../../types';

defineProps<{
  actions: ActionItem[];
  selectedIndex: number | null;
}>();

const emit = defineEmits<{
  select: [index: number];
  delete: [index: number];
}>();

function getActionTypeLabel(action: Action): string {
  switch (action.type) {
    case 'key':
      return '按键';
    case 'key_sequence':
      return '序列';
    case 'mouse_click':
      return '点击';
    case 'mouse_move':
      return '移动';
    case 'mouse_scroll':
      return '滚动';
    case 'delay':
      return '延迟';
    case 'text':
      return '文本';
    default:
      return '未知';
  }
}

function formatActionDetail(action: Action): string {
  switch (action.type) {
    case 'key':
      return action.key || '(未设置)';
    case 'key_sequence':
      return `${action.keys.length} 个按键`;
    case 'mouse_click':
      return `${action.button}键 x${action.count}`;
    case 'mouse_move':
      return `(${action.x}, ${action.y})`;
    case 'mouse_scroll':
      return `${action.direction} ${action.amount}`;
    case 'delay':
      return `${action.duration_ms}ms`;
    case 'text':
      const content = action.content.length > 15 ? action.content.substring(0, 15) + '...' : action.content;
      return `"${content}"`;
    default:
      return '';
  }
}

function getTypeClass(action: Action): string {
  return action.type;
}
</script>

<template>
  <div class="action-list-editor">
    <div v-if="actions.length === 0" class="empty-state">
      暂无动作，请点击上方「新建动作」创建
    </div>
    <div v-else class="action-items">
      <div
        v-for="item in actions"
        :key="item.index"
        class="action-item"
        :class="{ selected: selectedIndex === item.index }"
        @click="emit('select', item.index)"
      >
        <div class="action-info">
          <div class="action-header">
            <span class="action-index">#{{ item.index }}</span>
            <span class="type-badge" :class="getTypeClass(item.data)">
              {{ getActionTypeLabel(item.data) }}
            </span>
          </div>
          <div class="action-name" v-if="item.name">{{ item.name }}</div>
          <div class="action-detail">{{ formatActionDetail(item.data) }}</div>
        </div>
        <button
          class="delete-btn"
          @click.stop="emit('delete', item.index)"
          title="删除动作"
        >
          ✕
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.action-list-editor {
  flex: 1;
  overflow-y: auto;
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

.action-item:hover .delete-btn {
  opacity: 1;
}

.delete-btn:hover {
  color: var(--color-error);
  background: var(--color-error-bg);
}
</style>
