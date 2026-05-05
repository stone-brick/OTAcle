<script setup lang="ts">
import type { MouseClickActionItem } from '../../../../types';
import BackendSelector from '../BackendSelector.vue';
import VariableEditor from '../VariableEditor.vue';
import FormControl from '@/components/ui/FormControl.vue';

const props = defineProps<{
  modelValue: MouseClickActionItem;
}>();

const emit = defineEmits<{
  'update:modelValue': [action: MouseClickActionItem];
}>();

function updateField<K extends keyof MouseClickActionItem>(field: K, value: MouseClickActionItem[K]) {
  emit('update:modelValue', { ...props.modelValue, [field]: value });
}

const availableFields = ['count', 'hold_time_ms'];
</script>

<template>
  <div class="flex flex-col gap-4">
    <div class="flex flex-col gap-1.5">
      <label class="text-xs font-medium text-gray-500 dark:text-slate-400">鼠标按钮</label>
      <FormControl
        :model-value="modelValue.button"
        :options="[
          { value: 'left', label: '左键' },
          { value: 'right', label: '右键' },
          { value: 'middle', label: '中键' },
        ]"
        @update:model-value="updateField('button', $event as 'left' | 'right' | 'middle')"
      />
    </div>
    <div class="flex flex-col gap-1.5">
      <label class="text-xs font-medium text-gray-500 dark:text-slate-400">点击次数</label>
      <FormControl
        :model-value="modelValue.count"
        type="number"
        @update:model-value="updateField('count', Number($event))"
      />
    </div>
    <div class="flex flex-col gap-1.5">
      <label class="text-xs font-medium text-gray-500 dark:text-slate-400">间隔 (ms，可选)</label>
      <FormControl
        :model-value="modelValue.interval_ms"
        type="number"
        @update:model-value="updateField('interval_ms', Number($event))"
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
