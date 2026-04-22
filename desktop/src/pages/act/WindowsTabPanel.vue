<script setup lang="ts">
import { ref, computed } from 'vue';
import type { ActionItem, WindowInfo, SearchMode } from '../../types';
import WindowList from '../../components/act/WindowList.vue';
import ActionList from '../../components/act/ActionList.vue';
import ActionTestPanel from '../../components/act/ActionTestPanel.vue';

const props = defineProps<{
  windows: WindowInfo[];
  selectedWindow: WindowInfo | null;
  isLoading: boolean;
  actions: ActionItem[];
  isLoaded: boolean;
  searchWindows: (mode: SearchMode, value: string) => Promise<number[]>;
  getWindowsInfoByHwnds: (hwnds: number[]) => Promise<WindowInfo[]>;
}>();

const emit = defineEmits<{
  selectWindow: [hwnd: number | null];
  refreshWindows: [];
  executeAction: [payload: { actionIdx: number; params: Record<string, number | string> }];
}>();

const selectedTestActionIndex = ref<number | null>(null);
const searchResults = ref<WindowInfo[] | null>(null);

const displayWindows = computed(() => searchResults.value ?? props.windows);

async function handleSearch(mode: SearchMode, value: string) {
  if (!value.trim()) {
    searchResults.value = null;
    return;
  }
  const hwnds = await props.searchWindows(mode, value);
  if (hwnds.length === 0) {
    searchResults.value = [];
    return;
  }
  const windowInfos = await props.getWindowsInfoByHwnds(hwnds);
  searchResults.value = windowInfos;
}

function handleSelectWindow(hwnd: number | null) {
  emit('selectWindow', hwnd);
}

function handleSelectTestAction(index: number) {
  selectedTestActionIndex.value = index;
}

function handleExecute(payload: { actionIdx: number; params: Record<string, number | string> }) {
  emit('executeAction', payload);
}
</script>

<template>
  <div class="flex flex-row p-3 gap-3 h-full box-border">
    <div class="w-[30%] flex-shrink-0">
      <WindowList
        :windows="displayWindows"
        :selected-hwnd="selectedWindow?.hwnd ?? null"
        :is-loading="isLoading"
        class="h-full rounded-xl overflow-hidden"
        @select="handleSelectWindow"
        @refresh="emit('refreshWindows'); searchResults = null"
        @search="handleSearch"
      />
    </div>
    <div class="w-[30%] flex-shrink-0">
      <ActionList
        :actions="actions"
        :selected-index="selectedTestActionIndex"
        class="h-full rounded-xl overflow-hidden"
        @select="handleSelectTestAction"
      />
    </div>
    <div class="flex-1 flex-shrink-0">
      <ActionTestPanel
        :selected-window="selectedWindow"
        :actions="actions"
        :is-loaded="isLoaded"
        :selected-action-index="selectedTestActionIndex"
        class="h-full rounded-xl overflow-hidden"
        @execute="handleExecute"
      />
    </div>
  </div>
</template>
