<script setup lang="ts">
import { ref } from 'vue';
import type { MouseButton } from '../types';

defineProps<{
  selectedHwnd: number | null;
}>();

const emit = defineEmits<{
  click: [x: number, y: number, button: MouseButton];
  move: [x: number, y: number];
  getPosition: [];
}>();

const x = ref(0);
const y = ref(0);
const selectedButton = ref<MouseButton>('left');

function handleClick() {
  emit('click', x.value, y.value, selectedButton.value);
}

function handleDoubleClick() {
  emit('click', x.value, y.value, selectedButton.value);
  setTimeout(() => {
    emit('click', x.value, y.value, selectedButton.value);
  }, 100);
}

function handleMove() {
  emit('move', x.value, y.value);
}

function handleGetPosition() {
  emit('getPosition');
}
</script>

<template>
  <div class="mouse-control">
    <h4>鼠标操作</h4>

    <div class="mouse-inputs">
      <div class="input-group">
        <label>X</label>
        <input
          v-model.number="x"
          type="number"
          class="coord-input"
        >
      </div>
      <div class="input-group">
        <label>Y</label>
        <input
          v-model.number="y"
          type="number"
          class="coord-input"
        >
      </div>
      <button
        class="get-pos-btn"
        @click="handleGetPosition"
      >
        获取位置
      </button>
    </div>

    <div class="button-select">
      <label class="radio-label">
        <input
          v-model="selectedButton"
          type="radio"
          value="left"
        >
        <span>左键</span>
      </label>
      <label class="radio-label">
        <input
          v-model="selectedButton"
          type="radio"
          value="right"
        >
        <span>右键</span>
      </label>
      <label class="radio-label">
        <input
          v-model="selectedButton"
          type="radio"
          value="middle"
        >
        <span>中键</span>
      </label>
    </div>

    <div class="mouse-actions">
      <button
        class="action-btn"
        :disabled="!selectedHwnd"
        @click="handleClick"
      >
        点击
      </button>
      <button
        class="action-btn"
        :disabled="!selectedHwnd"
        @click="handleDoubleClick"
      >
        双击
      </button>
      <button
        class="action-btn"
        :disabled="!selectedHwnd"
        @click="handleMove"
      >
        移动
      </button>
    </div>

    <div
      v-if="!selectedHwnd"
      class="hint"
    >
      请先在侧边栏选择一个窗口
    </div>
  </div>
</template>

<style scoped>
.mouse-control {
  padding: 16px;
  border-top: 1px solid var(--color-border);
}

.mouse-control h4 {
  margin: 0 0 12px 0;
  font-size: 12px;
  font-weight: 600;
  color: var(--color-text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.mouse-inputs {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 12px;
}

.input-group {
  display: flex;
  align-items: center;
  gap: 6px;
}

.input-group label {
  font-size: 12px;
  color: var(--color-text-secondary);
}

.coord-input {
  width: 80px;
  padding: 6px 10px;
  font-size: 13px;
  background: var(--color-background);
  color: var(--color-text);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
}

.coord-input:focus {
  outline: none;
  border-color: var(--color-primary);
}

.get-pos-btn {
  padding: 6px 12px;
  font-size: 12px;
  background: var(--color-surface-secondary);
  color: var(--color-text);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  cursor: pointer;
}

.get-pos-btn:hover {
  background: var(--color-hover);
}

.button-select {
  display: flex;
  gap: 16px;
  margin-bottom: 12px;
}

.radio-label {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 13px;
  color: var(--color-text);
  cursor: pointer;
}

.radio-label input[type="radio"] {
  accent-color: var(--color-primary);
}

.mouse-actions {
  display: flex;
  gap: 8px;
}

.action-btn {
  flex: 1;
  padding: 10px 16px;
  font-size: 13px;
  font-weight: 500;
  background: var(--color-primary);
  color: white;
  border: none;
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: background var(--transition-duration);
}

.action-btn:hover:not(:disabled) {
  background: var(--color-primary-hover);
}

.action-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.hint {
  margin-top: 8px;
  font-size: 11px;
  color: var(--color-text-muted);
  text-align: center;
}
</style>
