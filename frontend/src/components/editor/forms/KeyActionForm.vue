<script setup lang="ts">
import type { KeyActionItem } from '../../../types';
import BackendSelector from '../BackendSelector.vue';
import FormControl from '@/components/ui/FormControl.vue';

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
  <div class="flex flex-col gap-4">
    <div class="flex flex-col gap-1.5">
      <label class="text-xs font-medium text-gray-500 dark:text-slate-400">按键</label>
      <FormControl
        :model-value="modelValue.key"
        type="text"
        placeholder="例如: space, ctrl, a"
        @update:model-value="updateField('key', $event)"
      />
    </div>
    <div class="flex flex-col gap-1.5">
      <label class="text-xs font-medium text-gray-500 dark:text-slate-400">按住时间 (ms)</label>
      <FormControl
        :model-value="modelValue.hold_time_ms"
        type="number"
        @update:model-value="updateField('hold_time_ms', Number($event))"
      />
    </div>
    <div class="flex flex-col gap-1.5">
      <label class="text-xs font-medium text-gray-500 dark:text-slate-400">输入后端</label>
      <BackendSelector
        :model-value="modelValue.backend ?? 'default'"
        @update:model-value="updateField('backend', $event === 'default' ? null : $event)"
      />
    </div>
  </div>
</template>
