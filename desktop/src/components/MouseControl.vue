<script setup lang="ts">
import { ref } from 'vue';
import type { MouseButton } from '../types';
import BaseButton from '@/components/ui/BaseButton.vue';
import FormControl from '@/components/ui/FormControl.vue';

defineProps<{
  selectedHwnd: number | null;
}>();

const emit = defineEmits<{
  click: [x: number, y: number, button: MouseButton];
  move: [x: number, y: number];
  getPosition: [];
}>();

const x = ref(0);
const y = ref(0);
const selectedButton = ref<MouseButton>('left');

function handleClick() {
  emit('click', x.value, y.value, selectedButton.value);
}

function handleDoubleClick() {
  emit('click', x.value, y.value, selectedButton.value);
  setTimeout(() => {
    emit('click', x.value, y.value, selectedButton.value);
  }, 100);
}

function handleMove() {
  emit('move', x.value, y.value);
}

function handleGetPosition() {
  emit('getPosition');
}
</script>

<template>
  <div class="p-4 border-t border-gray-100 dark:border-slate-800">
    <h4 class="m-0 mb-3 text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wide">
      鼠标操作
    </h4>

    <div class="flex items-center gap-3 mb-3">
      <div class="flex items-center gap-1.5">
        <label class="text-xs text-gray-500 dark:text-gray-400">X</label>
        <FormControl
          v-model.number="x"
          type="number"
          class="!w-20"
        />
      </div>
      <div class="flex items-center gap-1.5">
        <label class="text-xs text-gray-500 dark:text-gray-400">Y</label>
        <FormControl
          v-model.number="y"
          type="number"
          class="!w-20"
        />
      </div>
      <BaseButton
        label="获取位置"
        color="whiteDark"
        @click="handleGetPosition"
      />
    </div>

    <div class="flex gap-4 mb-3">
      <label class="flex items-center gap-1.5 text-sm text-gray-700 dark:text-gray-200 cursor-pointer">
        <input
          v-model="selectedButton"
          type="radio"
          value="left"
          class="accent-blue-500"
        >
        <span>左键</span>
      </label>
      <label class="flex items-center gap-1.5 text-sm text-gray-700 dark:text-gray-200 cursor-pointer">
        <input
          v-model="selectedButton"
          type="radio"
          value="right"
          class="accent-blue-500"
        >
        <span>右键</span>
      </label>
      <label class="flex items-center gap-1.5 text-sm text-gray-700 dark:text-gray-200 cursor-pointer">
        <input
          v-model="selectedButton"
          type="radio"
          value="middle"
          class="accent-blue-500"
        >
        <span>中键</span>
      </label>
    </div>

    <div class="flex gap-2">
      <BaseButton
        class="flex-1"
        label="点击"
        color="info"
        :disabled="!selectedHwnd"
        @click="handleClick"
      />
      <BaseButton
        class="flex-1"
        label="双击"
        color="info"
        :disabled="!selectedHwnd"
        @click="handleDoubleClick"
      />
      <BaseButton
        class="flex-1"
        label="移动"
        color="info"
        :disabled="!selectedHwnd"
        @click="handleMove"
      />
    </div>

    <p
      v-if="!selectedHwnd"
      class="mt-2 text-xs text-gray-400 dark:text-gray-500 text-center"
    >
      请先在侧边栏选择一个窗口
    </p>
  </div>
</template>
