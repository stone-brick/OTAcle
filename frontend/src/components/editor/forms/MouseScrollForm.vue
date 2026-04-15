<script setup lang="ts">
import type { MouseScrollActionItem } from '../../../types';
import type { ScrollDirection } from '../../../types';
import BackendSelector from '../BackendSelector.vue';

const props = defineProps<{
  modelValue: MouseScrollActionItem;
}>();

const emit = defineEmits<{
  'update:modelValue': [action: MouseScrollActionItem];
}>();

function updateField<K extends keyof MouseScrollActionItem>(field: K, value: MouseScrollActionItem[K]) {
  emit('update:modelValue', { ...props.modelValue, [field]: value });
}
</script>

<template>
  <div class="mouse-scroll-form">
    <div class="form-field">
      <label>滚动方向</label>
      <select
        :value="modelValue.direction"
        @change="updateField('direction', ($event.target as HTMLSelectElement).value as ScrollDirection)"
        class="select-input"
      >
        <option value="up">向上</option>
        <option value="down">向下</option>
        <option value="left">向左</option>
        <option value="right">向右</option>
      </select>
    </div>
    <div class="form-field">
      <label>滚动量</label>
      <input
        :value="modelValue.amount"
        @input="updateField('amount', Number(($event.target as HTMLInputElement).value))"
        type="number"
        min="1"
        class="number-input"
      />
      <span class="field-hint">Windows 默认滚轮 delta 为 120</span>
    </div>
    <div class="form-field">
      <label>输入后端</label>
      <BackendSelector
        :modelValue="modelValue.backend ?? 'default'"
        @update:modelValue="updateField('backend', $event === 'default' ? null : $event)"
      />
    </div>
  </div>
</template>

<style scoped>
.mouse-scroll-form {
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

.field-hint {
  font-size: 11px;
  color: var(--color-text-muted);
}
</style>
