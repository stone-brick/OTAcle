<script setup lang="ts">
import type { MouseMoveActionItem } from '../../../types';
import BackendSelector from '../BackendSelector.vue';
import VariableEditor from '../VariableEditor.vue';

const props = defineProps<{
  modelValue: MouseMoveActionItem;
}>();

const emit = defineEmits<{
  'update:modelValue': [action: MouseMoveActionItem];
}>();

function updateField<K extends keyof MouseMoveActionItem>(field: K, value: MouseMoveActionItem[K]) {
  emit('update:modelValue', { ...props.modelValue, [field]: value });
}

const availableFields = ['x', 'y'];
</script>

<template>
  <div class="mouse-move-form">
    <div class="form-field">
      <label>X 坐标</label>
      <input
        :value="modelValue.x"
        type="number"
        class="number-input"
        @input="updateField('x', Number(($event.target as HTMLInputElement).value))"
      >
    </div>
    <div class="form-field">
      <label>Y 坐标</label>
      <input
        :value="modelValue.y"
        type="number"
        class="number-input"
        @input="updateField('y', Number(($event.target as HTMLInputElement).value))"
      >
    </div>
    <div class="form-field">
      <label>移动时长 (ms，可选)</label>
      <input
        :value="modelValue.duration_ms"
        type="number"
        min="0"
        class="number-input"
        @input="updateField('duration_ms', Number(($event.target as HTMLInputElement).value))"
      >
    </div>
    <div class="form-field">
      <label>输入后端</label>
      <BackendSelector
        :model-value="modelValue.backend ?? 'default'"
        @update:model-value="updateField('backend', $event === 'default' ? null : $event)"
      />
    </div>
    <div class="form-field">
      <label>动态参数 (variables)</label>
      <VariableEditor
        :model-value="modelValue.variables || []"
        :available-fields="availableFields"
        @update:model-value="updateField('variables', $event.length > 0 ? $event : undefined)"
      />
    </div>
  </div>
</template>

<style scoped>
.mouse-move-form {
  display: flex;
  flex-direction: column;
  gap: 16px;
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
.select-input {
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
.select-input:focus {
  outline: none;
  border-color: var(--color-primary);
}
</style>
