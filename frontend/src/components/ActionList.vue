<script setup lang="ts">
import type { Action } from '../types';

defineProps<{
  actions: Record<string, Action>;
  selectedHwnd: number | null;
}>();

const emit = defineEmits<{
  execute: [id: number];
}>();

function formatAction(_id: string, action: Action): string {
  switch (action.type) {
    case 'key':
      return `按键: ${action.key}`;
    case 'key_sequence':
      return `序列: ${action.keys.join(' → ')}`;
    case 'mouse_click':
      return `点击: ${action.button}键 x${action.count}`;
    case 'mouse_move':
      return `移动: (${action.x}, ${action.y})`;
    case 'text':
      const content = action.content.length > 15 ? action.content.substring(0, 15) + '...' : action.content;
      return `文本: "${content}"`;
    default:
      return '未知动作';
  }
}

function handleExecute(id: number) {
  emit('execute', id);
}
</script>

<template>
  <div class="action-list">
    <table class="action-table">
      <thead>
        <tr>
          <th>ID</th>
          <th>类型</th>
          <th>详情</th>
          <th>操作</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="(action, id) in actions" :key="id">
          <td class="action-id">{{ id }}</td>
          <td>
            <span class="type-badge" :class="action.type">
              {{ action.type }}
            </span>
          </td>
          <td class="action-detail">{{ formatAction(id, action) }}</td>
          <td>
            <button
              class="execute-btn"
              @click="handleExecute(Number(id))"
              :disabled="!selectedHwnd"
            >
              执行
            </button>
          </td>
        </tr>
      </tbody>
    </table>

    <div v-if="Object.keys(actions).length === 0" class="empty-state">
      暂无动作配置
    </div>
  </div>
</template>

<style scoped>
.action-list {
  padding: 16px;
}

.action-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 13px;
}

.action-table th {
  text-align: left;
  padding: 10px 12px;
  font-weight: 600;
  color: var(--color-text-secondary);
  border-bottom: 2px solid var(--color-border);
  font-size: 11px;
  text-transform: uppercase;
}

.action-table td {
  padding: 10px 12px;
  border-bottom: 1px solid var(--color-border-light);
  color: var(--color-text);
}

.action-table tr:hover {
  background: var(--color-hover);
}

.action-id {
  font-family: monospace;
  font-weight: 600;
  color: var(--color-primary);
}

.type-badge {
  display: inline-block;
  padding: 2px 8px;
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

.type-badge.text {
  background: #ec4899;
  color: white;
}

.action-detail {
  color: var(--color-text-secondary);
  font-family: monospace;
}

.execute-btn {
  padding: 4px 14px;
  font-size: 12px;
  font-weight: 500;
  background: var(--color-success);
  color: white;
  border: none;
  border-radius: var(--radius-sm);
  cursor: pointer;
}

.execute-btn:hover:not(:disabled) {
  opacity: 0.9;
}

.execute-btn:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

.empty-state {
  text-align: center;
  padding: 24px;
  color: var(--color-text-muted);
}
</style>
