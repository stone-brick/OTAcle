<script setup lang="ts">
import { ref, watch } from 'vue';

const props = defineProps<{
  configPath: string;
  isDirty: boolean;
  isLoaded: boolean;
}>();

const emit = defineEmits<{
  load: [path: string];
  save: [];
  saveAs: [];
}>();

const pathInput = ref('');
const isPathFocused = ref(false);

// Sync with prop
watch(() => props.configPath, (newPath) => {
  if (!isPathFocused.value) {
    pathInput.value = newPath;
  }
}, { immediate: true });

function handleFocus() {
  isPathFocused.value = true;
}

function handleBlur(e: FocusEvent) {
  const target = e.relatedTarget as HTMLElement | null;
  if (!target || !target.closest('.btn-primary')) {
    pathInput.value = props.configPath;
  }
  isPathFocused.value = false;
}

function handleLoad() {
  if (pathInput.value.trim()) {
    emit('load', pathInput.value.trim());
  }
}

function handleKeydown(e: KeyboardEvent) {
  if (e.key === 'Enter') {
    handleLoad();
  }
}
</script>

<template>
  <div class="editor-header">
    <div class="header-left">
      <h3>编辑动作配置</h3>
    </div>
    <div class="header-actions">
      <div class="header-actions-left">
        <input
          v-model="pathInput"
          type="text"
          class="path-input"
          :class="{ 'path-input--faded': !isPathFocused && configPath }"
          placeholder="输入配置文件路径..."
          @focus="handleFocus"
          @blur="handleBlur"
          @keydown="handleKeydown"
        >
        <span
          v-if="isDirty"
          class="dirty-indicator"
        >● 已修改</span>
      </div>
      <div class="header-actions-right">
        <button
          class="btn-primary"
          @click="handleLoad"
        >
          加载
        </button>
        <button
          class="btn-secondary"
          :disabled="!isLoaded || !isDirty"
          @click="emit('save')"
        >
          保存
        </button>
        <button
          class="btn-secondary"
          :disabled="!isLoaded"
          @click="emit('saveAs')"
        >
          另存为
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.editor-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  background: var(--color-surface);
  border-radius: var(--radius-md);
}

.header-left {
  display: flex;
  align-items: center;
  gap: 12px;
}

.header-left h3 {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
}

.dirty-indicator {
  color: var(--color-warning);
  font-size: 12px;
}

.header-actions {
  display: flex;
  align-items: center;
  justify-content: space-between;
  flex: 1;
  gap: 8px;
  margin-left: 24px;
}

.header-actions-left {
  flex: 1;
  display: flex;
  align-items: center;
  gap: 12px;
}

.header-actions-right {
  display: flex;
  gap: 8px;
}

.path-input {
  width: 420px;
  flex-shrink: 0;
  padding: 8px 12px;
  font-size: 13px;
  background: var(--color-background);
  color: var(--color-text);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  font-family: monospace;
  box-sizing: border-box;
}

.path-input:focus {
  outline: none;
  border-color: var(--color-primary);
}

.path-input--faded {
  color: var(--color-text-muted);
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
