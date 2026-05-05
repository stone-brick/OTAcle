<script setup lang="ts">
import { ref, watch, computed } from 'vue';
import type { ActionItem } from '../../../types';
import {
  isKeyAction, isKeySequenceAction, isMouseClickAction,
  isMouseMoveAction, isMouseScrollAction, isDelayAction, isTextAction
} from '../../../types/act/actionTypes';
import KeyActionForm from './forms/KeyActionForm.vue';
import KeySequenceForm from './forms/KeySequenceForm.vue';
import MouseClickForm from './forms/MouseClickForm.vue';
import MouseMoveForm from './forms/MouseMoveForm.vue';
import MouseScrollForm from './forms/MouseScrollForm.vue';
import DelayForm from './forms/DelayForm.vue';
import TextActionForm from './forms/TextActionForm.vue';
import CardBox from '@/components/ui/CardBox.vue';
import CardBoxComponentHeader from '@/components/ui/CardBoxComponentHeader.vue';
import CardBoxComponentBody from '@/components/ui/CardBoxComponentBody.vue';
import BaseButton from '@/components/ui/BaseButton.vue';
import FormControl from '@/components/ui/FormControl.vue';

const props = defineProps<{
  action: ActionItem;
  actionIndex: number;
}>();

const emit = defineEmits<{
  update: [index: number, action: ActionItem];
}>();

const editedAction = ref<ActionItem>(props.action);
const hasChanges = ref(false);

watch(() => props.action, (newAction) => {
  editedAction.value = newAction;
  hasChanges.value = false;
}, { deep: true });

watch(editedAction, () => {
  hasChanges.value = true;
}, { deep: true });

function handleSave() {
  emit('update', props.actionIndex, editedAction.value);
  hasChanges.value = false;
}

function handleCancel() {
  editedAction.value = props.action;
  hasChanges.value = false;
}

function handleFormUpdate(action: ActionItem) {
  editedAction.value = action;
}

const formComponent = computed(() => {
  const action = props.action;
  if (isKeyAction(action)) return KeyActionForm;
  if (isKeySequenceAction(action)) return KeySequenceForm;
  if (isMouseClickAction(action)) return MouseClickForm;
  if (isMouseMoveAction(action)) return MouseMoveForm;
  if (isMouseScrollAction(action)) return MouseScrollForm;
  if (isDelayAction(action)) return DelayForm;
  if (isTextAction(action)) return TextActionForm;
  return null;
});

function hasDuplicateVariables(vars: { param_name: string; field_name: string }[] | undefined): boolean {
  if (!vars || vars.length === 0) return false;
  const paramNames = vars.map(v => v.param_name).filter(Boolean);
  const fieldNames = vars.map(v => v.field_name).filter(Boolean);
  return new Set(paramNames).size !== paramNames.length ||
         new Set(fieldNames).size !== fieldNames.length;
}

const hasInvalidVariables = computed(() => {
  if (!editedAction.value) return false;
  const vars = (editedAction.value as any).variables;
  return hasDuplicateVariables(vars);
});
</script>

<template>
  <CardBox>
    <CardBoxComponentHeader
      :title="`编辑动作 #${actionIndex}`"
      :icon="undefined"
    />

    <CardBoxComponentBody class="flex flex-col gap-4">
      <div>
        <label class="block text-xs font-medium text-gray-500 dark:text-slate-400 mb-1.5">名称（可选）</label>
        <FormControl
          v-model="editedAction.name"
          type="text"
          placeholder="输入动作名称..."
        />
      </div>

      <component
        :is="formComponent"
        v-if="formComponent"
        :model-value="(editedAction as any)"
        @update:model-value="handleFormUpdate"
      />

      <div class="flex justify-end gap-2 pt-3 border-t border-gray-100 dark:border-slate-800">
        <BaseButton
          label="取消"
          color="whiteDark"
          :disabled="!hasChanges"
          @click="handleCancel"
        />
        <BaseButton
          label="确认"
          color="info"
          :disabled="!hasChanges || hasInvalidVariables"
          @click="handleSave"
        />
      </div>
    </CardBoxComponentBody>
  </CardBox>
</template>
