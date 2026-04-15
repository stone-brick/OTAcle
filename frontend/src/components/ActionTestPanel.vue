<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import type { ActionItem, WindowInfo, Variable } from '../types';

const props = defineProps<{
  selectedWindow: WindowInfo | null;
  actions: ActionItem[];
  isLoaded: boolean;
  selectedActionIndex: number | null;
}>();

const emit = defineEmits<{
  execute: [payload: { actionId: number; params: Record<string, any> }];
}>();

// Runtime params - key is param_name, value is the override value
const runtimeParams = ref<Record<string, string>>({});

const selectedAction = computed(() => {
  if (props.selectedActionIndex === null) return null;
  return props.actions[props.selectedActionIndex] || null;
});

// Extract variables from the selected action
const variables = computed((): Variable[] => {
  if (!selectedAction.value) return [];

  const action = selectedAction.value;
  switch (action.type) {
    case 'mouse_move':
    case 'mouse_click':
      return action.variables || [];
    default:
      return [];
  }
});

// Get the action's current value for a given field_name
function getFieldValue(fieldName: string): any {
  if (!selectedAction.value) return undefined;

  const action = selectedAction.value;
  switch (action.type) {
    case 'mouse_move':
      if (fieldName === 'x') return action.x;
      if (fieldName === 'y') return action.y;
      break;
    case 'mouse_click':
      if (fieldName === 'count') return action.count;
      break;
  }
  return undefined;
}

// Get field type for input rendering
function getFieldType(fieldName: string): 'number' | 'text' {
  const action = selectedAction.value;
  if (!action) return 'text';

  switch (action.type) {
    case 'mouse_move':
      if (fieldName === 'x' || fieldName === 'y') return 'number';
      break;
    case 'mouse_click':
      if (fieldName === 'count') return 'number';
      break;
  }
  return 'text';
}

// When action changes, reset runtime params
watch(selectedAction, () => {
  runtimeParams.value = {};
  // Initialize params with empty values
  for (const v of variables.value) {
    runtimeParams.value[v.param_name] = '';
  }
});

function buildParams(): Record<string, any> {
  const params: Record<string, any> = {};

  // Only include params that have non-empty values
  for (const [key, value] of Object.entries(runtimeParams.value)) {
    if (value !== '' && value !== undefined && value !== null) {
      // Convert to appropriate type based on field
      const fieldType = getFieldType(
        variables.value.find(v => v.param_name === key)?.field_name || ''
      );
      if (fieldType === 'number') {
        params[key] = Number(value);
      } else {
        params[key] = value;
      }
    }
  }

  return params;
}

function handleExecute() {
  if (props.selectedActionIndex === null) {
    return;
  }

  if (variables.value.length > 0) {
    const hasFilledParam = Object.values(runtimeParams.value).some(v => v !== '');
    if (!hasFilledParam) {
      return;
    }
  }

  const params = buildParams();
  emit('execute', {
    actionId: props.selectedActionIndex,
    params,
  });
}

function formatFieldName(fieldName: string): string {
  const labels: Record<string, string> = {
    x: 'X 坐标',
    y: 'Y 坐标',
    count: '点击次数',
  };
  return labels[fieldName] || fieldName;
}
</script>

<template>
  <div class="action-test-panel">
    <!-- Header -->
    <div class="panel-header">
      <h3>动作测试</h3>
    </div>

    <!-- No config loaded state -->
    <div v-if="!isLoaded" class="empty-state">
      <p>请先在「配置」标签页加载动作配置文件</p>
    </div>

    <template v-else>
      <!-- Target Window Info -->
      <div class="section">
        <div class="section-title">目标窗口</div>
        <div v-if="selectedWindow" class="window-info">
          <div class="window-title">{{ selectedWindow.title }}</div>
          <div class="window-meta">
            <span class="hwnd">HWND: 0x{{ selectedWindow.hwnd.toString(16) }}</span>
            <span class="process">{{ selectedWindow.processName }}</span>
          </div>
        </div>
        <div v-else class="no-window">
          未选择窗口（将使用前台窗口）
        </div>
      </div>

      <!-- Selected Action Info -->
      <div v-if="selectedAction" class="section">
        <div class="section-title">当前动作</div>
        <div class="selected-action-info">
          <span class="action-badge" :class="selectedAction.type">
            {{ selectedAction.type }}
          </span>
          <span class="action-name">{{ selectedAction.name || '(无名称)' }}</span>
        </div>
      </div>

      <!-- Variable Params Form -->
      <div v-else class="section">
        <div class="no-selection">
          请在中间列表选择一个动作
        </div>
      </div>

      <div v-if="selectedAction && variables.length > 0" class="section">
        <div class="section-title">可调参数</div>
        <div class="param-hint">
          以下参数可在运行时动态调整
        </div>
        <div class="param-form">
          <div
            v-for="variable in variables"
            :key="variable.param_name"
            class="param-row"
          >
            <label class="param-label">
              {{ variable.param_name }}
              <span class="field-name">({{ formatFieldName(variable.field_name) }})</span>
            </label>
            <input
              v-model="runtimeParams[variable.param_name]"
              :type="getFieldType(variable.field_name)"
              class="param-input"
              :placeholder="`默认值: ${getFieldValue(variable.field_name)}`"
            />
          </div>
        </div>
      </div>

      <!-- No variables hint -->
      <div v-else-if="selectedAction && variables.length === 0" class="section">
        <div class="no-variables">
          此动作未配置可调参数
        </div>
      </div>

      <!-- Execute Button -->
      <div class="section">
        <button
          class="execute-btn"
          :disabled="selectedActionIndex === null"
          @click="handleExecute"
        >
          执行动作
        </button>
      </div>
    </template>
  </div>
</template>

<style scoped>
.action-test-panel {
  flex: 1;
  display: flex;
  flex-direction: column;
  background: var(--color-surface);
  border-left: 1px solid var(--color-border);
  overflow-y: auto;
}

.panel-header {
  padding: 12px 16px;
  border-bottom: 1px solid var(--color-border);
}

.panel-header h3 {
  margin: 0;
  font-size: 14px;
  font-weight: 600;
}

.section {
  padding: 12px 16px;
  border-bottom: 1px solid var(--color-border);
}

.section-title {
  font-size: 12px;
  font-weight: 500;
  color: var(--color-text-secondary);
  margin-bottom: 8px;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  flex: 1;
  color: var(--color-text-muted);
  font-size: 13px;
  padding: 24px;
}

.window-info {
  background: var(--color-surface-secondary);
  border-radius: var(--radius-sm);
  padding: 10px 12px;
}

.window-title {
  font-size: 13px;
  font-weight: 500;
  color: var(--color-text);
  margin-bottom: 4px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.window-meta {
  display: flex;
  gap: 12px;
  font-size: 11px;
  color: var(--color-text-secondary);
}

.hwnd {
  font-family: monospace;
}

.no-window {
  color: var(--color-text-muted);
  font-size: 13px;
  font-style: italic;
}

.selected-action-info {
  display: flex;
  align-items: center;
  gap: 10px;
}

.action-badge {
  padding: 4px 10px;
  border-radius: 4px;
  font-size: 12px;
  font-weight: 600;
  text-transform: uppercase;
  background: var(--color-primary-bg);
  color: var(--color-primary);
}

.action-badge.key { background: #3b82f6; color: white; }
.action-badge.key_sequence { background: #8b5cf6; color: white; }
.action-badge.mouse_click { background: #f59e0b; color: white; }
.action-badge.mouse_move { background: #10b981; color: white; }
.action-badge.mouse_scroll { background: #06b6d4; color: white; }
.action-badge.delay { background: #6b7280; color: white; }
.action-badge.text { background: #ec4899; color: white; }

.action-name {
  font-size: 14px;
  font-weight: 500;
  color: var(--color-text);
}

.no-selection {
  color: var(--color-text-muted);
  font-size: 13px;
  font-style: italic;
}

.param-hint {
  font-size: 12px;
  color: var(--color-text-muted);
  margin-bottom: 12px;
}

.param-form {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.param-row {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.param-label {
  font-size: 12px;
  color: var(--color-text);
  font-weight: 500;
}

.param-label .field-name {
  color: var(--color-text-muted);
  font-weight: normal;
}

.param-input {
  padding: 8px 10px;
  font-size: 13px;
  background: var(--color-background);
  color: var(--color-text);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  width: 100%;
  box-sizing: border-box;
}

.param-input:focus {
  outline: none;
  border-color: var(--color-primary);
}

.param-input::placeholder {
  color: var(--color-text-muted);
}

.no-variables {
  color: var(--color-text-muted);
  font-size: 13px;
  font-style: italic;
}

.execute-btn {
  width: 100%;
  padding: 10px 16px;
  font-size: 14px;
  font-weight: 500;
  background: var(--color-primary);
  color: white;
  border: none;
  border-radius: var(--radius-sm);
  cursor: pointer;
  transition: background var(--transition-duration);
}

.execute-btn:hover:not(:disabled) {
  background: var(--color-primary-hover);
}

.execute-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
</style>
