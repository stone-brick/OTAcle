<script setup lang="ts">
import { ref, watch, computed } from 'vue';
import type { ActionItem } from '../../types';
import {
  isKeyAction, isKeySequenceAction, isMouseClickAction,
  isMouseMoveAction, isMouseScrollAction, isDelayAction, isTextAction,
  getActionTypeLabel
} from '../../types/act/actionTypes';
import KeyActionForm from './forms/KeyActionForm.vue';
import KeySequenceForm from './forms/KeySequenceForm.vue';
import MouseClickForm from './forms/MouseClickForm.vue';
import MouseMoveForm from './forms/MouseMoveForm.vue';
import MouseScrollForm from './forms/MouseScrollForm.vue';
import DelayForm from './forms/DelayForm.vue';
import TextActionForm from './forms/TextActionForm.vue';

const props = defineProps<{
  action: ActionItem;
  actionIndex: number;
}>();

const emit = defineEmits<{
  update: [index: number, action: ActionItem];
}>();

const editedAction = ref<ActionItem>(props.action);
const hasChanges = ref(false);

watch(() => props.action, (newAction) => {
  editedAction.value = newAction;
  hasChanges.value = false;
}, { deep: true });

watch(editedAction, () => {
  hasChanges.value = true;
}, { deep: true });

function handleSave() {
  emit('update', props.actionIndex, editedAction.value);
  hasChanges.value = false;
}

function handleCancel() {
  editedAction.value = props.action;
  hasChanges.value = false;
}

function handleFormUpdate(action: ActionItem) {
  editedAction.value = action;
}

const actionTypeLabel = computed(() => getActionTypeLabel(props.action.type));

const formComponent = computed(() => {
  const action = props.action;
  if (isKeyAction(action)) return KeyActionForm;
  if (isKeySequenceAction(action)) return KeySequenceForm;
  if (isMouseClickAction(action)) return MouseClickForm;
  if (isMouseMoveAction(action)) return MouseMoveForm;
  if (isMouseScrollAction(action)) return MouseScrollForm;
  if (isDelayAction(action)) return DelayForm;
  if (isTextAction(action)) return TextActionForm;
  return null;
});
</script>

<template>
  <div class="action-form">
    <div class="form-header">
      <h4>编辑动作 #{{ actionIndex }}</h4>
      <span class="action-type-label">{{ actionTypeLabel }}</span>
    </div>

    <div class="form-field">
      <label>名称（可选）</label>
      <input
        v-model="editedAction.name"
        type="text"
        placeholder="输入动作名称..."
        class="text-input"
      >
    </div>

    <component
      :is="formComponent"
      v-if="formComponent"
      :model-value="(editedAction as any)"
      @update:model-value="handleFormUpdate"
    />

    <div class="form-actions">
      <button
        class="btn-secondary"
        :disabled="!hasChanges"
        @click="handleCancel"
      >
        取消
      </button>
      <button
        class="btn-primary"
        :disabled="!hasChanges"
        @click="handleSave"
      >
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

.text-input {
  padding: 8px 12px;
  font-size: 13px;
  background: var(--color-background);
  color: var(--color-text);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  font-family: inherit;
}

.text-input:focus {
  outline: none;
  border-color: var(--color-primary);
}

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
