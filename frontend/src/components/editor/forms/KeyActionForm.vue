<script setup lang="ts">
import type { KeyActionItem } from '../../../types';
import BackendSelector from '../BackendSelector.vue';

const props = defineProps<{
  modelValue: KeyActionItem;
}>();

const emit = defineEmits<{
  'update:modelValue': [action: KeyActionItem];
}>();

function updateField<K extends keyof KeyActionItem>(field: K, value: KeyActionItem[K]) {
  emit('update:modelValue', { ...props.modelValue, [field]: value });
}
</script>

<template>
  <div class="key-action-form">
    <div class="form-field">
      <label>按键</label>
      <input
        :value="modelValue.key"
        @input="updateField('key', ($event.target as HTMLInputElement).value)"
        type="text"
        placeholder="例如: space, ctrl, a"
        class="text-input"
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
  </div>
</template>

<style scoped>
.key-action-form {
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
.number-input {
  padding: 8px 12px;
  font-size: 13px;
  background: var(--color-background);
  color: var(--color-text);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  font-family: inherit;
}

.text-input:focus,
.number-input:focus {
  outline: none;
  border-color: var(--color-primary);
}
</style>
