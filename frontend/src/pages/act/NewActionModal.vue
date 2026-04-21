<script setup lang="ts">
import { ref } from 'vue';
import BaseButton from '@/components/ui/BaseButton.vue';
import FormControl from '@/components/ui/FormControl.vue';

defineProps<{
  show: boolean;
}>();

const emit = defineEmits<{
  confirm: [type: string, name: string | undefined];
  cancel: [];
}>();

const selectedType = ref<string | null>(null);
const newActionName = ref('');

const actionTypes = [
  { value: 'key', label: '按键' },
  { value: 'key_sequence', label: '序列' },
  { value: 'mouse_click', label: '点击' },
  { value: 'mouse_move', label: '移动' },
  { value: 'mouse_scroll', label: '滚动' },
  { value: 'delay', label: '延迟' },
  { value: 'text', label: '文本' },
];

function handleConfirm() {
  if (!selectedType.value) return;
  emit('confirm', selectedType.value, newActionName.value || undefined);
  resetState();
}

function handleCancel() {
  emit('cancel');
  resetState();
}

function resetState() {
  selectedType.value = null;
  newActionName.value = '';
}

function handleKeydown(e: KeyboardEvent) {
  if (e.key === 'Enter' && selectedType.value) {
    handleConfirm();
  }
}
</script>

<template>
  <Teleport to="body">
    <div
      v-if="show"
      class="fixed inset-0 bg-black/50 flex items-center justify-center z-[1000]"
      @click.self="handleCancel"
    >
      <div
        class="bg-white dark:bg-slate-900 rounded-lg p-6 max-w-lg w-[90%]"
        @click.stop
      >
        <h4 class="text-lg font-semibold mb-3">
          新建动作
        </h4>

        <div class="mb-5">
          <label class="block text-xs font-medium text-gray-500 dark:text-slate-400 mb-1.5">动作类型</label>
          <div class="grid grid-cols-4 gap-2">
            <button
              v-for="type in actionTypes"
              :key="type.value"
              type="button"
              class="py-3 px-2 text-sm rounded-lg border cursor-pointer transition-colors"
              :class="selectedType === type.value
                ? 'bg-blue-50 dark:bg-blue-900/30 border-blue-500 dark:border-blue-400 text-blue-600 dark:text-blue-400'
                : 'bg-gray-50 dark:bg-slate-800 border-gray-200 dark:border-slate-700 text-gray-700 dark:text-slate-300 hover:bg-gray-100 dark:hover:bg-slate-700'"
              @click="selectedType = type.value"
            >
              {{ type.label }}
            </button>
          </div>
        </div>

        <div class="mb-5">
          <label class="block text-xs font-medium text-gray-500 dark:text-slate-400 mb-1.5">名称（可选）</label>
          <FormControl
            v-model="newActionName"
            type="text"
            placeholder="输入动作名称..."
            @keydown="handleKeydown"
          />
        </div>

        <div class="flex justify-end gap-2">
          <BaseButton
            label="取消"
            color="whiteDark"
            @click="handleCancel"
          />
          <BaseButton
            label="创建"
            color="info"
            :disabled="!selectedType"
            @click="handleConfirm"
          />
        </div>
      </div>
    </div>
  </Teleport>
</template>
