<script setup lang="ts">
import { computed } from 'vue';

const props = defineProps<{
  modelValue: 'default' | 'win32' | 'enigo';
  hideDefault?: boolean;
}>();

const emit = defineEmits<{
  'update:modelValue': [value: 'default' | 'win32' | 'enigo'];
}>();

const options = [
  { value: 'default', label: '默认' },
  { value: 'win32', label: 'Win32' },
  { value: 'enigo', label: 'Enigo' },
] as const;

const visibleOptions = computed(() =>
  props.hideDefault
    ? options.filter(o => o.value !== 'default')
    : options
);
</script>

<template>
  <div class="backend-selector">
    <label
      v-for="opt in visibleOptions"
      :key="opt.value"
      class="radio-label"
      :class="{ active: modelValue === opt.value }"
    >
      <input
        type="radio"
        :checked="modelValue === opt.value"
        @change="emit('update:modelValue', opt.value)"
      />
      <span>{{ opt.label }}</span>
    </label>
  </div>
</template>

<style scoped>
.backend-selector {
  display: flex;
  gap: 8px;
}

.radio-label {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px;
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  cursor: pointer;
  font-size: 13px;
  transition: all var(--transition-duration);
}

.radio-label input {
  display: none;
}

.radio-label:hover {
  border-color: var(--color-primary);
}

.radio-label.active {
  border-color: var(--color-primary);
  background: var(--color-primary-bg);
  color: var(--color-primary);
}
</style>
