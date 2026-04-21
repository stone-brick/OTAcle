<script setup lang="ts">
import type { TextActionItem } from '../../../types';
import BackendSelector from '../BackendSelector.vue';
import FormControl from '@/components/ui/FormControl.vue';

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
  <div class="flex flex-col gap-4">
    <div class="flex flex-col gap-1.5">
      <label class="text-xs font-medium text-gray-500 dark:text-slate-400">文本内容</label>
      <FormControl
        :model-value="modelValue.content"
        type="textarea"
        placeholder="输入要发送的文本..."
        @update:model-value="updateField('content', $event)"
      />
      <span class="text-xs text-gray-500 dark:text-slate-400">支持通过 variables 动态参数覆盖</span>
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
