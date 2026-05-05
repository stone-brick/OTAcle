<script setup lang="ts">
import type { MouseScrollActionItem, ScrollDirection } from '../../../../types';
import BackendSelector from '../BackendSelector.vue';
import VariableEditor from '../VariableEditor.vue';
import FormControl from '@/components/ui/FormControl.vue';

const props = defineProps<{
  modelValue: MouseScrollActionItem;
}>();

const emit = defineEmits<{
  'update:modelValue': [action: MouseScrollActionItem];
}>();

function updateField<K extends keyof MouseScrollActionItem>(field: K, value: MouseScrollActionItem[K]) {
  emit('update:modelValue', { ...props.modelValue, [field]: value });
}

const availableFields = ['amount'];
</script>

<template>
  <div class="flex flex-col gap-4">
    <div class="flex flex-col gap-1.5">
      <label class="text-xs font-medium text-gray-500 dark:text-slate-400">滚动方向</label>
      <FormControl
        :model-value="modelValue.direction"
        :options="[
          { value: 'up', label: '向上' },
          { value: 'down', label: '向下' },
          { value: 'left', label: '向左' },
          { value: 'right', label: '向右' },
        ]"
        @update:model-value="updateField('direction', $event as ScrollDirection)"
      />
    </div>
    <div class="flex flex-col gap-1.5">
      <label class="text-xs font-medium text-gray-500 dark:text-slate-400">滚动量</label>
      <FormControl
        :model-value="modelValue.amount"
        type="number"
        @update:model-value="updateField('amount', Number($event))"
      />
      <span class="text-xs text-gray-500 dark:text-slate-400">Windows 默认滚轮 delta 为 120</span>
    </div>
    <div class="flex flex-col gap-1.5">
      <label class="text-xs font-medium text-gray-500 dark:text-slate-400">输入后端</label>
      <BackendSelector
        :model-value="modelValue.backend ?? 'default'"
        @update:model-value="updateField('backend', $event === 'default' ? null : $event)"
      />
    </div>
    <div class="flex flex-col gap-1.5">
      <label class="text-xs font-medium text-gray-500 dark:text-slate-400">动态参数 (variables)</label>
      <VariableEditor
        :model-value="modelValue.variables || []"
        :available-fields="availableFields"
        @update:model-value="updateField('variables', $event.length > 0 ? $event : undefined)"
      />
    </div>
  </div>
</template>
