<script setup lang="ts">
import type { TextActionItem } from '../../../types';
import BackendSelector from '../BackendSelector.vue';

const props = defineProps<{
  modelValue: TextActionItem;
}>();

const emit = defineEmits<{
  'update:modelValue': [action: TextActionItem];
}>();

function updateField<K extends keyof TextActionItem>(field: K, value: TextActionItem[K]) {
  emit('update:modelValue', { ...props.modelValue, [field]: value });
}
</script>

<template>
  <div class="text-action-form">
    <div class="form-field">
      <label>文本内容</label>
      <textarea
        :value="modelValue.content"
        @input="updateField('content', ($event.target as HTMLTextAreaElement).value)"
        placeholder="输入要发送的文本..."
        class="textarea-input"
        rows="4"
      ></textarea>
      <span class="field-hint">支持通过 variables 动态参数覆盖</span>
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
.text-action-form {
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

.textarea-input {
  padding: 8px 12px;
  font-size: 13px;
  background: var(--color-background);
  color: var(--color-text);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  font-family: inherit;
  resize: vertical;
  min-height: 80px;
}

.textarea-input:focus {
  outline: none;
  border-color: var(--color-primary);
}

.field-hint {
  font-size: 11px;
  color: var(--color-text-muted);
}
</style>
