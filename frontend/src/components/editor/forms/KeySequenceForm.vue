<script setup lang="ts">
import type { KeySequenceActionItem, KeySequenceItem } from '../../../types';
import BackendSelector from '../BackendSelector.vue';

const props = defineProps<{
  modelValue: KeySequenceActionItem;
}>();

const emit = defineEmits<{
  'update:modelValue': [action: KeySequenceActionItem];
}>();

function updateField<K extends keyof KeySequenceActionItem>(field: K, value: KeySequenceActionItem[K]) {
  emit('update:modelValue', { ...props.modelValue, [field]: value });
}

function addKey() {
  const keys = [...props.modelValue.keys];
  keys.push({ key: '', hold_time_ms: 5 });
  updateField('keys', keys);
}

function removeKey(index: number) {
  const keys = [...props.modelValue.keys];
  keys.splice(index, 1);
  updateField('keys', keys);
}

function updateKey(index: number, field: keyof KeySequenceItem, value: string | number) {
  const keys = [...props.modelValue.keys];
  keys[index] = { ...keys[index], [field]: value };
  updateField('keys', keys);
}
</script>

<template>
  <div class="key-sequence-form">
    <div class="form-field">
      <label>默认间隔 (ms)</label>
      <input
        :value="modelValue.default_interval_ms"
        type="number"
        min="0"
        class="number-input"
        @input="updateField('default_interval_ms', Number(($event.target as HTMLInputElement).value))"
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
      <label>按键序列</label>
      <div class="key-sequence-list">
        <div
          v-for="(keyItem, index) in modelValue.keys"
          :key="index"
          class="key-sequence-item"
        >
          <input
            :value="keyItem.key"
            type="text"
            placeholder="按键"
            class="text-input key-input"
            @input="updateKey(index, 'key', ($event.target as HTMLInputElement).value)"
          >
          <input
            :value="keyItem.hold_time_ms"
            type="number"
            min="0"
            class="number-input hold-input"
            @input="updateKey(index, 'hold_time_ms', Number(($event.target as HTMLInputElement).value))"
          >
          <span class="unit-label">ms 按住</span>
          <input
            :value="keyItem.interval_ms"
            type="number"
            min="0"
            class="number-input interval-input"
            @input="updateKey(index, 'interval_ms', Number(($event.target as HTMLInputElement).value))"
          >
          <span class="unit-label">ms 间隔</span>
          <button
            class="remove-key-btn"
            @click="removeKey(index)"
          >
            ✕
          </button>
        </div>
        <button
          class="add-key-btn"
          @click="addKey"
        >
          + 添加按键
        </button>
      </div>
      <span class="field-hint">间隔留空则使用默认间隔</span>
    </div>
  </div>
</template>

<style scoped>
.key-sequence-form {
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

.interval-input {
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

.field-hint {
  font-size: 11px;
  color: var(--color-text-muted);
}
</style>
