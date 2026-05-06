<script setup lang="ts">
import { computed } from 'vue';
import type { Variable } from '../../../types';
import { mdiClose } from '@mdi/js';
import BaseButton from '@/components/ui/BaseButton.vue';
import FormControl from '@/components/ui/FormControl.vue';

const props = defineProps<{
  modelValue: Variable[];
  availableFields: string[];
}>();

const emit = defineEmits<{
  'update:modelValue': [vars: Variable[]];
}>();

function hasDuplicateParamName(items: Variable[]): boolean {
  const names = items.map(v => v.param_name).filter(Boolean);
  return new Set(names).size !== names.length;
}

function hasDuplicateFieldName(items: Variable[]): boolean {
  const names = items.map(v => v.field_name).filter(Boolean);
  return new Set(names).size !== names.length;
}

const duplicateParamWarning = computed(() =>
  hasDuplicateParamName(props.modelValue) ? '参数名不能重复' : ''
);

const duplicateFieldWarning = computed(() =>
  hasDuplicateFieldName(props.modelValue) ? '字段名不能重复' : ''
);

const canSave = computed(() =>
  !hasDuplicateParamName(props.modelValue) && !hasDuplicateFieldName(props.modelValue)
);

function addVariable() {
  emit('update:modelValue', [...props.modelValue, { param_name: '', field_name: '' }]);
}

function removeVariable(index: number) {
  const vars = [...props.modelValue];
  vars.splice(index, 1);
  emit('update:modelValue', vars);
}

function updateField(index: number, field: 'param_name' | 'field_name', value: string) {
  const vars = [...props.modelValue];
  vars[index][field] = value;
  emit('update:modelValue', vars);
}

const fieldOptions = () =>
  props.availableFields.map(f => ({ value: f, label: f }));

defineExpose({ canSave });
</script>

<template>
  <div class="flex flex-col gap-2">
    <div class="flex flex-col gap-2">
      <div
        v-for="(variable, index) in modelValue"
        :key="index"
        class="flex items-center gap-2"
      >
        <FormControl
          :model-value="variable.field_name"
          class="!w-24"
          :options="[{ value: '', label: '选择字段...' }, ...fieldOptions()]"
          @update:model-value="updateField(index, 'field_name', $event)"
        />
        <span class="text-gray-500 dark:text-slate-400 text-sm">→</span>
        <FormControl
          :model-value="variable.param_name"
          type="text"
          placeholder="参数名"
          class="flex-1"
          @update:model-value="updateField(index, 'param_name', $event)"
        />
        <BaseButton
          :icon="mdiClose"
          color="whiteDark"
          small
          transparent-bg
          @click="removeVariable(index)"
        />
      </div>
      <BaseButton
        label="+ 添加参数"
        color="info"
        outline
        @click="addVariable"
      />
    </div>
    <div v-if="duplicateParamWarning || duplicateFieldWarning" class="text-xs text-red-500 dark:text-red-400">
      <span v-if="duplicateParamWarning">{{ duplicateParamWarning }}</span>
      <span v-if="duplicateParamWarning && duplicateFieldWarning">；</span>
      <span v-if="duplicateFieldWarning">{{ duplicateFieldWarning }}</span>
    </div>
    <span class="text-xs text-gray-500 dark:text-slate-400">定义可动态覆盖的字段和参数名称，运行时通过通信参数传值</span>
  </div>
</template>
