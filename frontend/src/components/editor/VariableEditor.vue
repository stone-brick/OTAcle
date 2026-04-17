<script setup lang="ts">
import type { Variable } from '../../types';

const props = defineProps<{
  modelValue: Variable[];
  availableFields: string[];
}>();

const emit = defineEmits<{
  'update:modelValue': [vars: Variable[]];
}>();

function addVariable() {
  emit('update:modelValue', [...props.modelValue, { param_name: '', field_name: '' }]);
}

function removeVariable(index: number) {
  const vars = [...props.modelValue];
  vars.splice(index, 1);
  emit('update:modelValue', vars);
}

function updateField(index: number, field: 'param_name' | 'field_name', value: string) {
  const vars = [...props.modelValue];
  vars[index][field] = value;
  emit('update:modelValue', vars);
}
</script>

<template>
  <div class="variable-editor">
    <div class="variables-list">
      <div
        v-for="(variable, index) in modelValue"
        :key="index"
        class="variable-item"
      >
        <select
          :value="variable.field_name"
          class="select-input"
          @change="updateField(index, 'field_name', ($event.target as HTMLSelectElement).value)"
        >
          <option value="">
            选择字段...
          </option>
          <option
            v-for="field in availableFields"
            :key="field"
            :value="field"
          >
            {{ field }}
          </option>
        </select>
        <span class="arrow">→</span>
        <input
          :value="variable.param_name"
          type="text"
          placeholder="参数名"
          class="text-input param-input"
          @input="updateField(index, 'param_name', ($event.target as HTMLInputElement).value)"
        >
        <button
          class="remove-var-btn"
          @click="removeVariable(index)"
        >
          ✕
        </button>
      </div>
      <button
        class="add-var-btn"
        @click="addVariable"
      >
        + 添加参数
      </button>
    </div>
    <span class="field-hint">定义可动态覆盖的字段和参数名称，运行时通过 ZMQ params 传值</span>
  </div>
</template>

<style scoped>
.variable-editor {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

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

.field-hint {
  font-size: 11px;
  color: var(--color-text-muted);
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

.select-input {
  padding: 8px 12px;
  font-size: 13px;
  background: var(--color-background);
  color: var(--color-text);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  font-family: inherit;
}

.select-input:focus {
  outline: none;
  border-color: var(--color-primary);
}
</style>
