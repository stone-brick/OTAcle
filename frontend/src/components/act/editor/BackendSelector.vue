<script setup lang="ts">
import SegmentedControl from '@/components/ui/SegmentedControl.vue'

defineProps<{
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
</script>

<template>
  <SegmentedControl
    :model-value="modelValue"
    :options="options"
    :hide-default="hideDefault"
    @update:model-value="emit('update:modelValue', $event)"
  />
</template>
