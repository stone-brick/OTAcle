<script setup lang="ts">
import { ref } from 'vue';
import type { WindowInfo, SearchMode } from '../types';
import BaseButton from '@/components/ui/BaseButton.vue';
import FormControl from '@/components/ui/FormControl.vue';
import CardBox from '@/components/ui/CardBox.vue';
import CardBoxComponentBody from '@/components/ui/CardBoxComponentBody.vue';

defineProps<{
  windows: WindowInfo[];
  selectedHwnd: number | null;
  isLoading: boolean;
}>();

const emit = defineEmits<{
  select: [hwnd: number];
  refresh: [];
  search: [mode: SearchMode, value: string];
}>();

const searchMode = ref<SearchMode>('title');
const searchValue = ref('');

function handleSelect(hwnd: number) {
  emit('select', hwnd);
}

function handleRefresh() {
  emit('refresh');
}

function handleSearch() {
  if (!searchValue.value.trim()) return;
  emit('search', searchMode.value, searchValue.value.trim());
}

function truncateTitle(title: string, maxLen: number = 30): string {
  if (title.length <= maxLen) return title;
  return title.substring(0, maxLen) + '...';
}
</script>

<template>
  <CardBox class="flex flex-col flex-1 border-l border-gray-100 dark:border-slate-800">
    <!-- Header -->
    <div class="flex justify-between items-center px-3 py-2 border-b border-gray-100 dark:border-slate-800">
      <h3 class="m-0 text-sm font-semibold text-gray-700 dark:text-slate-200">
        窗口列表
      </h3>
      <BaseButton
        :label="isLoading ? '加载中...' : '刷新'"
        color="info"
        :disabled="isLoading"
        small
        @click="handleRefresh"
      />
    </div>

    <!-- Search Section -->
    <div class="p-2 border-b border-gray-100 dark:border-slate-800 flex flex-col gap-1.5">
      <FormControl
        v-model="searchMode"
        :options="[
          { value: 'title', label: '标题(前缀)' },
          { value: 'title_contains', label: '标题(包含)' },
          { value: 'class', label: '类名' },
          { value: 'pid', label: '进程ID' },
          { value: 'exe', label: '进程名' },
          { value: 'hwnd', label: 'HWND' },
        ]"
      />
      <div class="flex gap-1">
        <FormControl
          v-model="searchValue"
          :type="searchMode === 'pid' || searchMode === 'hwnd' ? 'number' : 'text'"
          :placeholder="searchMode === 'title' ? '窗口标题...' :
            searchMode === 'title_contains' ? '窗口标题(包含)...' :
            searchMode === 'class' ? '窗口类名...' :
            searchMode === 'pid' ? '进程ID...' :
            searchMode === 'exe' ? '进程名(如 notepad)...' :
            'HWND值...'"
          class="flex-1"
          @keyup.enter="handleSearch"
        />
        <BaseButton
          label="查找"
          color="whiteDark"
          small
          @click="handleSearch"
        />
      </div>
    </div>

    <!-- Window List -->
    <CardBoxComponentBody
      v-if="windows.length === 0"
      class="flex flex-col items-center justify-center flex-1"
    >
      <p class="text-gray-500 dark:text-slate-400 text-sm">
        <template v-if="searchValue">
          未找到匹配的窗口
        </template>
        <template v-else>
          暂无窗口
        </template>
      </p>
    </CardBoxComponentBody>

    <CardBoxComponentBody
      v-else
      class="flex-1 overflow-hidden"
    >
      <div class="flex-1 overflow-y-auto p-1.5 flex flex-col gap-1">
        <div
          v-for="win in windows"
          :key="win.hwnd"
          class="group p-2.5 rounded-lg cursor-pointer transition-colors border border-transparent"
          :class="win.hwnd === selectedHwnd
            ? 'bg-blue-50 dark:bg-blue-900/30 border-blue-500 dark:border-blue-400'
            : 'bg-gray-50 dark:bg-slate-800/50 hover:bg-gray-100 dark:hover:bg-slate-800'"
          @click="handleSelect(win.hwnd)"
        >
          <div class="flex items-center gap-2 mb-1">
            <span class="font-mono text-xs text-blue-600 dark:text-blue-400">0x{{ win.hwnd.toString(16) }}</span>
            <span
              class="inline-flex items-center px-1.5 py-0.5 text-[10px] font-semibold rounded"
              :class="win.is_visible
                ? 'bg-emerald-100 text-emerald-700 dark:bg-emerald-900/30 dark:text-emerald-400'
                : 'bg-gray-100 text-gray-500 dark:bg-slate-700 dark:text-slate-400'"
            >
              {{ win.is_visible ? '可见' : '隐藏' }}
            </span>
          </div>
          <div
            class="text-sm font-medium text-gray-700 dark:text-slate-200 mb-0.5 truncate"
            :title="win.title"
          >
            {{ truncateTitle(win.title) }}
          </div>
          <div class="text-xs text-gray-500 dark:text-slate-400">
            {{ win.process_name }}
          </div>
        </div>
      </div>
    </CardBoxComponentBody>
  </CardBox>
</template>
