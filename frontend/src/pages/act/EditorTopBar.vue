<script setup lang="ts">
import type { InputBackend } from '../../types';

defineProps<{
  defaultBackend: InputBackend;
  hasChanges: boolean;
  canUndo: boolean;
  canRedo: boolean;
}>();

const emit = defineEmits<{
  backendChange: [backend: InputBackend];
  openNewActionModal: [];
  discardAll: [];
  undo: [];
  redo: [];
}>();

function handleBackendChange(backend: InputBackend) {
  emit('backendChange', backend);
}
</script>

<template>
  <div class="top-bar">
    <div class="top-bar-left">
      <span class="top-bar-label">执行后台</span>
      <div class="backend-selector">
        <label class="radio-label" :class="{ active: defaultBackend === 'win32' }">
          <input
            type="radio"
            :checked="defaultBackend === 'win32'"
            @change="handleBackendChange('win32')"
          />
          <span>Win32</span>
        </label>
        <label class="radio-label" :class="{ active: defaultBackend === 'enigo' }">
          <input
            type="radio"
            :checked="defaultBackend === 'enigo'"
            @change="handleBackendChange('enigo')"
          />
          <span>Enigo</span>
        </label>
      </div>
    </div>

    <div class="top-bar-right">
      <button class="btn-primary" @click="emit('openNewActionModal')">
        + 新建动作
      </button>
      <button
        class="btn-secondary"
        :disabled="!hasChanges"
        @click="emit('discardAll')"
      >
        撤销全部
      </button>
      <button
        class="btn-secondary"
        :disabled="!canUndo"
        @click="emit('undo')"
      >
        撤销
      </button>
      <button
        class="btn-secondary"
        :disabled="!canRedo"
        @click="emit('redo')"
      >
        重做
      </button>
    </div>
  </div>
</template>

<style scoped>
.top-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
  padding: 12px 16px;
  background: var(--color-surface);
  border-radius: var(--radius-md);
  flex-wrap: wrap;
}

.top-bar-left {
  display: flex;
  align-items: center;
  gap: 12px;
}

.top-bar-right {
  display: flex;
  gap: 8px;
  align-items: center;
}

.top-bar-label {
  font-size: 13px;
  color: var(--color-text-secondary);
  font-weight: 500;
}

.backend-selector {
  display: flex;
  gap: 8px;
}

.radio-label {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px;
  background: var(--color-surface-secondary);
  border-radius: var(--radius-sm);
  cursor: pointer;
  font-size: 13px;
}

.radio-label.active {
  background: var(--color-primary-bg);
  border: 1px solid var(--color-primary);
}

.radio-label input[type="radio"] {
  accent-color: var(--color-primary);
}

.btn-primary {
  padding: 8px 16px;
  font-size: 13px;
  font-weight: 500;
  background: var(--color-primary);
  color: white;
  border: none;
  border-radius: var(--radius-sm);
  cursor: pointer;
}

.btn-primary:hover {
  background: var(--color-primary-hover);
}

.btn-secondary {
  padding: 8px 16px;
  font-size: 13px;
  font-weight: 500;
  background: var(--color-surface-secondary);
  color: var(--color-text);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  cursor: pointer;
}

.btn-secondary:hover:not(:disabled) {
  background: var(--color-surface-hover);
}

.btn-secondary:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}
</style>
