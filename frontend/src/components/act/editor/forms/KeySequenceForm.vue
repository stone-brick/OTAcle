<script setup lang="ts">
import type { KeySequenceActionItem, KeySequenceItem } from '../../../../types';
import BackendSelector from '../BackendSelector.vue';
import BaseButton from '@/components/ui/BaseButton.vue';
import FormControl from '@/components/ui/FormControl.vue';
import { mdiClose } from '@mdi/js';

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
  <div class="flex flex-col gap-4">
    <div class="flex flex-col gap-1.5">
      <label class="text-xs font-medium text-gray-500 dark:text-slate-400">默认间隔 (ms)</label>
      <FormControl
        :model-value="modelValue.default_interval_ms"
        type="number"
        @update:model-value="updateField('default_interval_ms', Number($event))"
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
      <label class="text-xs font-medium text-gray-500 dark:text-slate-400">按键序列</label>
      <div class="flex flex-col gap-2">
        <div
          v-for="(keyItem, index) in modelValue.keys"
          :key="index"
          class="flex items-center gap-2"
        >
          <FormControl
            :model-value="keyItem.key"
            type="text"
            placeholder="按键"
            @update:model-value="updateKey(index, 'key', $event)"
          />
          <FormControl
            :model-value="keyItem.hold_time_ms"
            type="number"
            @update:model-value="updateKey(index, 'hold_time_ms', Number($event))"
          />
          <span class="text-xs text-gray-500 w-12">ms 按住</span>
          <FormControl
            :model-value="keyItem.interval_ms"
            type="number"
            @update:model-value="updateKey(index, 'interval_ms', Number($event))"
          />
          <span class="text-xs text-gray-500 w-12">ms 间隔</span>
          <BaseButton
            :icon="mdiClose"
            color="whiteDark"
            small
            @click="removeKey(index)"
          />
        </div>
        <button
          type="button"
          class="py-2 text-sm text-blue-600 dark:text-blue-400 border border-dashed border-blue-300 dark:border-blue-700 rounded-lg hover:bg-blue-50 dark:hover:bg-blue-900/20 cursor-pointer"
          @click="addKey"
        >
          + 添加按键
        </button>
      </div>
      <span class="text-xs text-gray-500">间隔留空则使用默认间隔</span>
    </div>
  </div>
</template>
