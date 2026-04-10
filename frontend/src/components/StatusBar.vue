<script setup lang="ts">
import type { AppStatus } from '../types';

defineProps<{
  status: AppStatus;
}>();
</script>

<template>
  <div class="status-bar">
    <div class="status-indicator">
      <span class="status-dot" :class="status"></span>
      <span class="status-text">
        {{ status === 'ready' ? '就绪' : status === 'sending' ? '发送中...' : '错误' }}
      </span>
    </div>
    <div class="status-info">
      <slot></slot>
    </div>
  </div>
</template>

<style scoped>
.status-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 16px;
  background: var(--color-surface);
  border-bottom: 1px solid var(--color-border);
}

.status-indicator {
  display: flex;
  align-items: center;
  gap: 8px;
}

.status-dot {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  background: var(--color-success);
}

.status-dot.ready {
  background: var(--color-success);
}

.status-dot.sending {
  background: var(--color-warning);
  animation: pulse 1s infinite;
}

.status-dot.error {
  background: var(--color-error);
}

.status-text {
  font-size: 13px;
  color: var(--color-text-secondary);
}

.status-info {
  display: flex;
  align-items: center;
  gap: 16px;
  font-size: 12px;
  color: var(--color-text-secondary);
}

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.5; }
}
</style>
