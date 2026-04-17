<script setup lang="ts">
import type { DelayActionItem } from '../../../types';

const props = defineProps<{
  modelValue: DelayActionItem;
}>();

const emit = defineEmits<{
  'update:modelValue': [action: DelayActionItem];
}>();

function updateField<K extends keyof DelayActionItem>(field: K, value: DelayActionItem[K]) {
  emit('update:modelValue', { ...props.modelValue, [field]: value });
}
</script>

<template>
  <div class="delay-form">
    <div class="form-field">
      <label>延迟时间 (ms)</label>
      <input
        :value="modelValue.duration_ms"
        type="number"
        min="0"
        class="number-input"
        @input="updateField('duration_ms', Number(($event.target as HTMLInputElement).value))"
      >
    </div>
  </div>
</template>

<style scoped>
.delay-form {
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

.number-input {
  padding: 8px 12px;
  font-size: 13px;
  background: var(--color-background);
  color: var(--color-text);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  font-family: inherit;
}

.number-input:focus {
  outline: none;
  border-color: var(--color-primary);
}
</style>
