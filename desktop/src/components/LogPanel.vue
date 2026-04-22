<script setup lang="ts">
import { ref, computed } from 'vue';
import type { LogEntry, LogSource } from '../types';
import BaseButton from '@/components/ui/BaseButton.vue';
import FormControl from '@/components/ui/FormControl.vue';
import { getStatusTextColor } from '@/utils/colors';

const props = defineProps<{
  logs: LogEntry[];
}>();

const emit = defineEmits<{
  clear: [];
}>();

type FilterType = 'all' | 'info' | 'warn' | 'success' | 'error';
type SourceFilter = 'all' | LogSource;

const activeFilter = ref<FilterType>('all');
const activeSource = ref<SourceFilter>('all');

const filteredLogs = computed(() => {
  let result = props.logs;
  if (activeFilter.value !== 'all') {
    result = result.filter(log => log.type === activeFilter.value);
  }
  if (activeSource.value !== 'all') {
    result = result.filter(log => log.source === activeSource.value);
  }
  return result;
});

function setFilter(filter: FilterType) {
  activeFilter.value = filter;
}

function handleClear() {
  emit('clear');
}
</script>

<template>
  <div class="h-36 flex flex-col bg-white dark:bg-slate-900/70 border-t border-gray-100 dark:border-slate-800">
    <div class="flex justify-between items-center px-4 py-2 border-b border-gray-100 dark:border-slate-800 gap-2">
      <h3 class="m-0 text-xs font-semibold text-gray-700 dark:text-slate-200 flex-shrink-0">
        操作日志
      </h3>
      <div class="flex gap-1 flex-1 justify-start">
        <BaseButton
          label="全部"
          color="whiteDark"
          :small="true"
          :active="activeFilter === 'all'"
          @click="setFilter('all')"
        />
        <BaseButton
          label="信息"
          color="whiteDark"
          :small="true"
          :active="activeFilter === 'info'"
          @click="setFilter('info')"
        />
        <BaseButton
          label="成功"
          color="whiteDark"
          :small="true"
          :active="activeFilter === 'success'"
          @click="setFilter('success')"
        />
        <BaseButton
          label="警告"
          color="whiteDark"
          :small="true"
          :active="activeFilter === 'warn'"
          @click="setFilter('warn')"
        />
        <BaseButton
          label="错误"
          color="whiteDark"
          :small="true"
          :active="activeFilter === 'error'"
          @click="setFilter('error')"
        />
      </div>
      <FormControl
        v-model="activeSource"
        class="!w-24 flex-shrink-0"
        :options="[
          { value: 'all', label: '全部来源' },
          { value: 'system', label: '系统' },
          { value: 'comm', label: '通信' },
          { value: 'observe', label: '观察' },
          { value: 'action', label: '动作' },
          { value: 'window', label: '窗口' },
        ]"
      />
      <BaseButton
        label="清空"
        color="whiteDark"
        small
        class="flex-shrink-0"
        @click="handleClear"
      />
    </div>

    <div class="flex-1 overflow-y-auto px-4 py-2 font-mono text-xs">
      <div
        v-for="(log, index) in filteredLogs"
        :key="index"
        class="flex gap-3 py-1 border-b border-gray-50 dark:border-slate-800/50 last:border-0"
      >
        <span class="text-gray-400 flex-shrink-0">{{ log.time }}</span>
        <span
          :class="['break-all', getStatusTextColor(log.type)]"
        >{{ log.message }}</span>
      </div>

      <div
        v-if="filteredLogs.length === 0"
        class="py-4 text-center text-gray-400"
      >
        暂无日志
      </div>
    </div>
  </div>
</template>
