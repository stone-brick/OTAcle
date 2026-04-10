<script setup lang="ts">
import { ref } from 'vue';
import type { ActionConfig, InputBackend } from '../types';
import ActionList from './ActionList.vue';

defineProps<{
  config: ActionConfig | null;
  configPath: string;
  currentBackend: InputBackend;
  selectedHwnd: number | null;
}>();

const emit = defineEmits<{
  load: [path: string];
  execute: [id: number];
  executeRange: [start: number, end: number, interval: number];
  setBackend: [backend: InputBackend];
}>();

const pathInput = ref('');
const startId = ref(0);
const endId = ref(0);
const intervalMs = ref(100);

function handleLoad() {
  if (pathInput.value.trim()) {
    emit('load', pathInput.value.trim());
  }
}

function handleExecute(id: number) {
  emit('execute', id);
}

function handleExecuteRange() {
  emit('executeRange', startId.value, endId.value, intervalMs.value);
}

function handleBackendChange(backend: InputBackend) {
  emit('setBackend', backend);
}
</script>

<template>
  <div class="action-config">
    <!-- Config Path Input -->
    <div class="config-section">
      <h4>配置文件</h4>
      <div class="path-input-row">
        <input
          v-model="pathInput"
          type="text"
          class="path-input"
          placeholder="输入配置文件路径..."
        />
        <button class="load-btn" @click="handleLoad">加载</button>
      </div>
      <div v-if="configPath" class="current-config">
        当前配置: {{ configPath }}
      </div>
    </div>

    <!-- Backend Selector -->
    <div class="config-section">
      <h4>输入后端</h4>
      <div class="backend-selector">
        <label class="radio-label">
          <input
            type="radio"
            :checked="currentBackend === 'win32'"
            @change="handleBackendChange('win32')"
          />
          <span class="backend-info">
            <strong>Win32</strong>
            <small>直接发送到目标窗口（推荐）</small>
          </span>
        </label>
        <label class="radio-label">
          <input
            type="radio"
            :checked="currentBackend === 'enigo'"
            @change="handleBackendChange('enigo')"
          />
          <span class="backend-info">
            <strong>Enigo</strong>
            <small>全局输入，需要激活窗口</small>
          </span>
        </label>
      </div>
    </div>

    <!-- Action List -->
    <div class="config-section" v-if="config">
      <h4>动作列表</h4>
      <ActionList
        :actions="config"
        :selectedHwnd="selectedHwnd"
        @execute="handleExecute"
      />
    </div>

    <!-- Batch Execution -->
    <div class="config-section" v-if="config">
      <h4>批量执行</h4>
      <div class="batch-controls">
        <div class="range-inputs">
          <div class="input-group">
            <label>从 ID</label>
            <input type="number" v-model.number="startId" min="0" class="id-input" />
          </div>
          <div class="input-group">
            <label>到 ID</label>
            <input type="number" v-model.number="endId" min="0" class="id-input" />
          </div>
          <div class="input-group">
            <label>间隔(ms)</label>
            <input type="number" v-model.number="intervalMs" min="0" class="id-input" />
          </div>
        </div>
        <button class="batch-btn" @click="handleExecuteRange" :disabled="!selectedHwnd">
          连续执行
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.action-config {
  display: flex;
  flex-direction: column;
  gap: 16px;
  padding: 16px;
}

.config-section {
  padding: 12px;
  background: var(--color-surface);
  border-radius: var(--radius-md);
}

.config-section h4 {
  margin: 0 0 10px 0;
  font-size: 12px;
  font-weight: 600;
  color: var(--color-text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.path-input-row {
  display: flex;
  gap: 8px;
}

.path-input {
  flex: 1;
  padding: 8px 12px;
  font-size: 13px;
  background: var(--color-background);
  color: var(--color-text);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
}

.path-input:focus {
  outline: none;
  border-color: var(--color-primary);
}

.load-btn {
  padding: 8px 20px;
  font-size: 13px;
  font-weight: 500;
  background: var(--color-primary);
  color: white;
  border: none;
  border-radius: var(--radius-sm);
  cursor: pointer;
}

.load-btn:hover {
  background: var(--color-primary-hover);
}

.current-config {
  margin-top: 8px;
  font-size: 11px;
  color: var(--color-text-muted);
  font-family: monospace;
}

.backend-selector {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.radio-label {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 8px 12px;
  background: var(--color-surface-secondary);
  border-radius: var(--radius-sm);
  cursor: pointer;
}

.radio-label:has(input:checked) {
  background: var(--color-primary-bg);
  border: 1px solid var(--color-primary);
}

.radio-label input[type="radio"] {
  accent-color: var(--color-primary);
}

.backend-info {
  display: flex;
  flex-direction: column;
}

.backend-info strong {
  font-size: 13px;
  color: var(--color-text);
}

.backend-info small {
  font-size: 11px;
  color: var(--color-text-muted);
}

.batch-controls {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.range-inputs {
  display: flex;
  gap: 12px;
}

.input-group {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.input-group label {
  font-size: 11px;
  color: var(--color-text-secondary);
}

.id-input {
  width: 70px;
  padding: 6px 8px;
  font-size: 13px;
  background: var(--color-background);
  color: var(--color-text);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
}

.id-input:focus {
  outline: none;
  border-color: var(--color-primary);
}

.batch-btn {
  padding: 10px;
  font-size: 13px;
  font-weight: 500;
  background: var(--color-warning);
  color: white;
  border: none;
  border-radius: var(--radius-sm);
  cursor: pointer;
}

.batch-btn:hover:not(:disabled) {
  opacity: 0.9;
}

.batch-btn:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}
</style>
