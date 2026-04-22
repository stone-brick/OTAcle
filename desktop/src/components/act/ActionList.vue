<script setup lang="ts">
import type { ActionItem } from '../../types';
import { getActionTypeLabel, formatActionDetail } from '../../utils/actionHelpers';
import BaseButton from '@/components/ui/BaseButton.vue';
import CardBox from '@/components/ui/CardBox.vue';
import CardBoxComponentBody from '@/components/ui/CardBoxComponentBody.vue';
import { mdiClose } from '@mdi/js';

defineProps<{
  actions: ActionItem[];
  selectedIndex: number | null;
  showDelete?: boolean;
}>();

const emit = defineEmits<{
  select: [index: number];
  delete: [index: number];
}>();
</script>

<template>
  <CardBox class="flex flex-col flex-1 border-l border-gray-100 dark:border-slate-800">
    <div class="px-3 py-2 border-b border-gray-100 dark:border-slate-800">
      <h3 class="m-0 text-sm font-semibold text-gray-700 dark:text-slate-200">
        动作列表
      </h3>
    </div>

    <CardBoxComponentBody
      v-if="actions.length === 0"
      class="flex flex-col items-center justify-center flex-1"
    >
      <p class="text-gray-500 dark:text-slate-400 text-sm">
        <slot name="empty">
          暂无动作
        </slot>
      </p>
    </CardBoxComponentBody>

    <CardBoxComponentBody
      v-else
      class="flex-1 overflow-hidden"
    >
      <div class="flex-1 overflow-y-auto p-1.5 flex flex-col gap-1">
        <div
          v-for="(item, idx) in actions"
          :key="idx"
          class="group flex items-center justify-between p-3 rounded-lg cursor-pointer transition-colors border border-transparent"
          :class="selectedIndex === idx
            ? 'bg-blue-50 dark:bg-blue-900/30 border-blue-500 dark:border-blue-400'
            : 'bg-gray-50 dark:bg-slate-800/50 hover:bg-gray-100 dark:hover:bg-slate-800'"
          @click="emit('select', idx)"
        >
          <div class="flex-1 min-w-0">
            <div class="flex items-center gap-2 mb-1">
              <span class="font-mono font-semibold text-xs text-blue-600 dark:text-blue-400">#{{ idx }}</span>
              <span
                class="inline-flex items-center px-2 py-0.5 text-xs font-semibold rounded capitalize"
                :class="{
                  'bg-blue-500 text-white': item.type === 'key',
                  'bg-purple-500 text-white': item.type === 'key_sequence',
                  'bg-orange-500 text-white': item.type === 'mouse_click',
                  'bg-green-500 text-white': item.type === 'mouse_move',
                  'bg-red-500 text-white': item.type === 'mouse_scroll',
                  'bg-gray-500 text-white': item.type === 'delay',
                  'bg-indigo-500 text-white': item.type === 'text',
                }"
              >
                {{ getActionTypeLabel(item) }}
              </span>
            </div>
            <div
              v-if="item.name"
              class="text-sm font-medium text-gray-700 dark:text-slate-200 mb-0.5"
            >
              {{ item.name }}
            </div>
            <div class="text-xs text-gray-500 dark:text-slate-400 font-mono truncate">
              {{ formatActionDetail(item) }}
            </div>
          </div>

          <div
            v-if="showDelete"
            class="ml-2 opacity-0 group-hover:opacity-100 transition-opacity"
          >
            <BaseButton
              :icon="mdiClose"
              small
              transparent-bg
              @click.stop="emit('delete', idx)"
            />
          </div>
        </div>
      </div>
    </CardBoxComponentBody>
  </CardBox>
</template>
