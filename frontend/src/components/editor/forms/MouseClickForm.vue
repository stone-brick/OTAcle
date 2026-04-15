<script setup lang="ts">
import type { MouseClickActionItem } from '../../../types';
import BackendSelector from '../BackendSelector.vue';
import VariableEditor from '../VariableEditor.vue';

const props = defineProps<{
  modelValue: MouseClickActionItem;
}>();

const emit = defineEmits<{
  'update:modelValue': [action: MouseClickActionItem];
}>();

function updateField<K extends keyof MouseClickActionItem>(field: K, value: MouseClickActionItem[K]) {
  emit('update:modelValue', { ...props.modelValue, [field]: value });
}

const availableFields = ['count'];
</script>

<template>
  <div class="mouse-click-form">
    <div class="form-field">
      <label>鼠标按钮</label>
      <select
        :value="modelValue.button"
        @change="updateField('button', ($event.target as HTMLSelectElement).value as 'left' | 'right' | 'middle')"
        class="select-input"
      >
        <option value="left">左键</option>
        <option value="right">右键</option>
        <option value="middle">中键</option>
      </select>
    </div>
    <div class="form-field">
      <label>点击次数</label>
      <input
        :value="modelValue.count"
        @input="updateField('count', Number(($event.target as HTMLInputElement).value))"
        type="number"
        min="1"
        class="number-input"
      />
    </div>
    <div class="form-field">
      <label>间隔 (ms，可选)</label>
      <input
        :value="modelValue.interval_ms"
        @input="updateField('interval_ms', Number(($event.target as HTMLInputElement).value))"
        type="number"
        min="0"
        class="number-input"
      />
    </div>
    <div class="form-field">
      <label>按住时间 (ms)</label>
      <input
        :value="modelValue.hold_time_ms"
        @input="updateField('hold_time_ms', Number(($event.target as HTMLInputElement).value))"
        type="number"
        min="0"
        class="number-input"
      />
    </div>
    <div class="form-field">
      <label>输入后端</label>
      <BackendSelector
        :modelValue="modelValue.backend ?? 'default'"
        @update:modelValue="updateField('backend', $event === 'default' ? null : $event)"
      />
    </div>
    <div class="form-field">
      <label>动态参数 (variables)</label>
      <VariableEditor
        :modelValue="modelValue.variables || []"
        :availableFields="availableFields"
        @update:modelValue="updateField('variables', $event.length > 0 ? $event : undefined)"
      />
    </div>
  </div>
</template>

<style scoped>
.mouse-click-form {
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
