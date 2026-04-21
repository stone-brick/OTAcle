<script setup lang="ts">
import type { MouseMoveActionItem } from '../../../types';
import BackendSelector from '../BackendSelector.vue';
import VariableEditor from '../VariableEditor.vue';
import FormControl from '@/components/ui/FormControl.vue';

const props = defineProps<{
  modelValue: MouseMoveActionItem;
}>();

const emit = defineEmits<{
  'update:modelValue': [action: MouseMoveActionItem];
}>();

function updateField<K extends keyof MouseMoveActionItem>(field: K, value: MouseMoveActionItem[K]) {
  emit('update:modelValue', { ...props.modelValue, [field]: value });
}

const availableFields = ['x', 'y'];
</script>

<template>
  <div class="flex flex-col gap-4">
    <div class="flex flex-col gap-1.5">
      <label class="text-xs font-medium text-gray-500 dark:text-slate-400">X 坐标</label>
      <FormControl
        :model-value="modelValue.x"
        type="number"
        @update:model-value="updateField('x', Number($event))"
      />
    </div>
    <div class="flex flex-col gap-1.5">
      <label class="text-xs font-medium text-gray-500 dark:text-slate-400">Y 坐标</label>
      <FormControl
        :model-value="modelValue.y"
        type="number"
        @update:model-value="updateField('y', Number($event))"
      />
    </div>
    <div class="flex flex-col gap-1.5">
      <label class="text-xs font-medium text-gray-500 dark:text-slate-400">移动时长 (ms，可选)</label>
      <FormControl
        :model-value="modelValue.duration_ms"
        type="number"
        @update:model-value="updateField('duration_ms', Number($event))"
      />
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
