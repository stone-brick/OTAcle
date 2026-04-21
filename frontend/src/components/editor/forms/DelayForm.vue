<script setup lang="ts">
import type { DelayActionItem } from '../../../types';
import FormControl from '@/components/ui/FormControl.vue';

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
  <div class="flex flex-col gap-4">
    <div class="flex flex-col gap-1.5">
      <label class="text-xs font-medium text-gray-500 dark:text-slate-400">延迟时间 (ms)</label>
      <FormControl
        :model-value="modelValue.duration_ms"
        type="number"
        @update:model-value="updateField('duration_ms', Number($event))"
      />
    </div>
  </div>
</template>
