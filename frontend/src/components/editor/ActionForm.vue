<script setup lang="ts">
import { ref, watch, computed } from 'vue';
import type { ActionItem, Action, KeyAction, KeySequenceAction, MouseClickAction, MouseMoveAction, MouseScrollAction, DelayAction, TextAction, Variable } from '../../types';

const props = defineProps<{
  action: ActionItem;
}>();

const emit = defineEmits<{
  update: [index: number, action: Action, name: string | null];
}>();

// Helper to construct Action from flattened ActionItem
function getActionFromItem(item: ActionItem): Action {
  const { index, name, type, ...actionData } = item;
  return { type, ...actionData } as Action;
}

// Local editing state
const editedAction = ref<Action>(getActionFromItem(props.action));
const editedName = ref<string | null>(props.action.name);
const hasChanges = ref(false);

// Watch for prop changes
watch(() => props.action, (newAction) => {
  editedAction.value = getActionFromItem(newAction);
  editedName.value = newAction.name;
  hasChanges.value = false;
}, { deep: true });

// Track changes
watch([editedAction, editedName], () => {
  hasChanges.value = true;
}, { deep: true });

function handleSave() {
  emit('update', props.action.index, editedAction.value, editedName.value);
  hasChanges.value = false;
}

function handleCancel() {
  editedAction.value = getActionFromItem(props.action);
  editedName.value = props.action.name;
  hasChanges.value = false;
}

// Key sequence management
function addKeyToSequence() {
  const seq = editedAction.value as KeySequenceAction;
  seq.keys.push({ key: '', hold_time_ms: 5 });
}

function removeKeyFromSequence(index: number) {
  const seq = editedAction.value as KeySequenceAction;
  seq.keys.splice(index, 1);
}

// Variables management
function getVariables(): Variable[] {
  if (editedAction.value.type === 'mouse_move') {
    const action = editedAction.value as MouseMoveAction;
    return action.variables || [];
  } else if (editedAction.value.type === 'mouse_click') {
    const action = editedAction.value as MouseClickAction;
    return action.variables || [];
  }
  return [];
}

function setVariables(vars: Variable[]) {
  if (editedAction.value.type === 'mouse_move') {
    (editedAction.value as MouseMoveAction).variables = vars.length > 0 ? vars : undefined;
  } else if (editedAction.value.type === 'mouse_click') {
    (editedAction.value as MouseClickAction).variables = vars.length > 0 ? vars : undefined;
  }
}

function addVariable() {
  const vars = getVariables();
  vars.push({ param_name: '', field_name: '' });
  setVariables(vars);
}

function removeVariable(index: number) {
  const vars = getVariables();
  vars.splice(index, 1);
  setVariables(vars);
}

function updateVariableField(index: number, field: 'param_name' | 'field_name', value: string) {
  const vars = getVariables();
  vars[index][field] = value;
  setVariables(vars);
}

// Computed action type label
const actionTypeLabel = computed(() => {
  const labels: Record<string, string> = {
    key: '按键',
    key_sequence: '按键序列',
    mouse_click: '鼠标点击',
    mouse_move: '鼠标移动',
    mouse_scroll: '鼠标滚动',
    delay: '延迟',
    text: '文本输入',
  };
  return labels[editedAction.value.type] || editedAction.value.type;
});

// Available fields for variables
const availableFields = computed(() => {
  if (editedAction.value.type === 'mouse_move') {
    return ['x', 'y'];
  } else if (editedAction.value.type === 'mouse_click') {
    return ['count'];
  }
  return [];
});

// Whether action supports variables
const supportsVariables = computed(() => {
  return ['mouse_move', 'mouse_click'].includes(editedAction.value.type);
});
</script>

<template>
  <div class="action-form">
    <div class="form-header">
      <h4>编辑动作 #{{ action.index }}</h4>
      <span class="action-type-label">{{ actionTypeLabel }}</span>
    </div>

    <!-- Name Field -->
    <div class="form-field">
      <label>名称（可选）</label>
      <input
        v-model="editedName"
        type="text"
        placeholder="输入动作名称..."
        class="text-input"
      />
    </div>

    <!-- Key Action Form -->
    <template v-if="editedAction.type === 'key'">
      <div class="form-field">
        <label>按键</label>
        <input
          v-model="(editedAction as KeyAction).key"
          type="text"
          placeholder="例如: space, ctrl, a"
          class="text-input"
        />
      </div>
      <div class="form-field">
        <label>按住时间 (ms)</label>
        <input
          v-model.number="(editedAction as KeyAction).hold_time_ms"
          type="number"
          min="0"
          class="number-input"
        />
      </div>
    </template>

    <!-- Key Sequence Action Form -->
    <template v-if="editedAction.type === 'key_sequence'">
      <div class="form-field">
        <label>默认间隔 (ms)</label>
        <input
          v-model.number="(editedAction as KeySequenceAction).default_interval_ms"
          type="number"
          min="0"
          class="number-input"
        />
      </div>
      <div class="form-field">
        <label>按键序列</label>
        <div class="key-sequence-list">
          <div
            v-for="(keyItem, index) in (editedAction as KeySequenceAction).keys"
            :key="index"
            class="key-sequence-item"
          >
            <input
              v-model="keyItem.key"
              type="text"
              placeholder="按键"
              class="text-input key-input"
            />
            <input
              v-model.number="keyItem.hold_time_ms"
              type="number"
              min="0"
              class="number-input hold-input"
            />
            <span class="unit-label">ms</span>
            <button class="remove-key-btn" @click="removeKeyFromSequence(index)">✕</button>
          </div>
          <button class="add-key-btn" @click="addKeyToSequence">+ 添加按键</button>
        </div>
      </div>
    </template>

    <!-- Mouse Click Action Form -->
    <template v-if="editedAction.type === 'mouse_click'">
      <div class="form-field">
        <label>鼠标按钮</label>
        <select v-model="(editedAction as MouseClickAction).button" class="select-input">
          <option value="left">左键</option>
          <option value="right">右键</option>
          <option value="middle">中键</option>
        </select>
      </div>
      <div class="form-field">
        <label>点击次数</label>
        <input
          v-model.number="(editedAction as MouseClickAction).count"
          type="number"
          min="1"
          class="number-input"
        />
      </div>
      <div class="form-field">
        <label>间隔 (ms)</label>
        <input
          v-model.number="(editedAction as MouseClickAction).interval_ms"
          type="number"
          min="0"
          class="number-input"
        />
      </div>
    </template>

    <!-- Mouse Move Action Form -->
    <template v-if="editedAction.type === 'mouse_move'">
      <div class="form-field">
        <label>X 坐标</label>
        <input
          v-model.number="(editedAction as MouseMoveAction).x"
          type="number"
          class="number-input"
        />
      </div>
      <div class="form-field">
        <label>Y 坐标</label>
        <input
          v-model.number="(editedAction as MouseMoveAction).y"
          type="number"
          class="number-input"
        />
      </div>
      <div class="form-field">
        <label>移动时长 (ms，可选)</label>
        <input
          v-model.number="(editedAction as MouseMoveAction).duration_ms"
          type="number"
          min="0"
          class="number-input"
        />
      </div>
    </template>

    <!-- Variables Section (for actions that support it) -->
    <template v-if="supportsVariables">
      <div class="form-field">
        <label>动态参数 (variables)</label>
        <div class="variables-list">
          <div
            v-for="(variable, index) in getVariables()"
            :key="index"
            class="variable-item"
          >
            <select
              :value="variable.field_name"
              @change="updateVariableField(index, 'field_name', ($event.target as HTMLSelectElement).value)"
              class="select-input"
            >
              <option value="">选择字段...</option>
              <option v-for="field in availableFields" :key="field" :value="field">
                {{ field }}
              </option>
            </select>
            <span class="arrow">→</span>
            <input
              :value="variable.param_name"
              @input="updateVariableField(index, 'param_name', ($event.target as HTMLInputElement).value)"
              type="text"
              placeholder="参数名"
              class="text-input param-input"
            />
            <button class="remove-var-btn" @click="removeVariable(index)">✕</button>
          </div>
          <button class="add-var-btn" @click="addVariable">+ 添加参数</button>
        </div>
        <span class="field-hint">定义可动态覆盖的字段和参数名称，运行时通过 ZMQ params 传值</span>
      </div>
    </template>

    <!-- Mouse Scroll Action Form -->
    <template v-if="editedAction.type === 'mouse_scroll'">
      <div class="form-field">
        <label>滚动方向</label>
        <select v-model="(editedAction as MouseScrollAction).direction" class="select-input">
          <option value="up">向上</option>
          <option value="down">向下</option>
          <option value="left">向左</option>
          <option value="right">向右</option>
        </select>
      </div>
      <div class="form-field">
        <label>滚动量</label>
        <input
          v-model.number="(editedAction as MouseScrollAction).amount"
          type="number"
          min="1"
          class="number-input"
        />
        <span class="field-hint">Windows 默认滚轮 delta 为 120</span>
      </div>
    </template>

    <!-- Delay Action Form -->
    <template v-if="editedAction.type === 'delay'">
      <div class="form-field">
        <label>延迟时间 (ms)</label>
        <input
          v-model.number="(editedAction as DelayAction).duration_ms"
          type="number"
          min="0"
          class="number-input"
        />
      </div>
    </template>

    <!-- Text Action Form -->
    <template v-if="editedAction.type === 'text'">
      <div class="form-field">
        <label>文本内容</label>
        <textarea
          v-model="(editedAction as TextAction).content"
          placeholder="输入要发送的文本..."
          class="textarea-input"
          rows="4"
        ></textarea>
        <span class="field-hint">支持通过 variables 动态参数覆盖</span>
      </div>
    </template>

    <!-- Actions -->
    <div class="form-actions">
      <button class="btn-secondary" @click="handleCancel" :disabled="!hasChanges">
        取消
      </button>
      <button class="btn-primary" @click="handleSave" :disabled="!hasChanges">
        确认
      </button>
    </div>
  </div>
</template>

<style scoped>
.action-form {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.form-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding-bottom: 12px;
  border-bottom: 1px solid var(--color-border);
}

.form-header h4 {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
}

.action-type-label {
  padding: 4px 12px;
  background: var(--color-primary-bg);
  color: var(--color-primary);
  border-radius: var(--radius-sm);
  font-size: 12px;
  font-weight: 500;
}

.form-field {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.form-field label {
  font-size: 12px;
  font-weight: 500;
  color: var(--color-text-secondary);
}

.text-input,
.number-input,
.select-input,
.textarea-input {
  padding: 8px 12px;
  font-size: 13px;
  background: var(--color-background);
  color: var(--color-text);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  font-family: inherit;
}

.text-input:focus,
.number-input:focus,
.select-input:focus,
.textarea-input:focus {
  outline: none;
  border-color: var(--color-primary);
}

.textarea-input {
  resize: vertical;
  min-height: 80px;
}

.field-hint {
  font-size: 11px;
  color: var(--color-text-muted);
}

/* Key Sequence */
.key-sequence-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.key-sequence-item {
  display: flex;
  align-items: center;
  gap: 8px;
}

.key-input {
  flex: 1;
}

.hold-input {
  width: 70px;
}

.unit-label {
  font-size: 11px;
  color: var(--color-text-muted);
}

.remove-key-btn {
  padding: 4px 8px;
  font-size: 11px;
  background: transparent;
  color: var(--color-text-muted);
  border: none;
  cursor: pointer;
}

.remove-key-btn:hover {
  color: var(--color-error);
}

.add-key-btn {
  padding: 8px;
  font-size: 12px;
  background: var(--color-surface-secondary);
  color: var(--color-primary);
  border: 1px dashed var(--color-border);
  border-radius: var(--radius-sm);
  cursor: pointer;
}

.add-key-btn:hover {
  background: var(--color-surface-hover);
}

/* Variables */
.variables-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.variable-item {
  display: flex;
  align-items: center;
  gap: 8px;
}

.variable-item .select-input {
  width: 100px;
}

.arrow {
  color: var(--color-text-muted);
  font-size: 12px;
}

.param-input {
  flex: 1;
}

.remove-var-btn {
  padding: 4px 8px;
  font-size: 11px;
  background: transparent;
  color: var(--color-text-muted);
  border: none;
  cursor: pointer;
}

.remove-var-btn:hover {
  color: var(--color-error);
}

.add-var-btn {
  padding: 8px;
  font-size: 12px;
  background: var(--color-surface-secondary);
  color: var(--color-primary);
  border: 1px dashed var(--color-border);
  border-radius: var(--radius-sm);
  cursor: pointer;
}

.add-var-btn:hover {
  background: var(--color-surface-hover);
}

/* Actions */
.form-actions {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
  padding-top: 12px;
  border-top: 1px solid var(--color-border);
}

.btn-primary {
  padding: 8px 20px;
  font-size: 13px;
  font-weight: 500;
  background: var(--color-primary);
  color: white;
  border: none;
  border-radius: var(--radius-sm);
  cursor: pointer;
}

.btn-primary:hover:not(:disabled) {
  background: var(--color-primary-hover);
}

.btn-primary:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

.btn-secondary {
  padding: 8px 20px;
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
