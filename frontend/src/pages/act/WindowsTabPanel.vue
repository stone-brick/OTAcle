<script setup lang="ts">
import { ref, computed } from 'vue';
import type { ActionItem, WindowInfo, SearchMode } from '../../types';
import WindowSidebar from '../../components/WindowSidebar.vue';
import ActionList from '../../components/ActionList.vue';
import ActionTestPanel from '../../components/ActionTestPanel.vue';

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
  executeAction: [payload: { actionId: number; params: Record<string, any> }];
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

function handleExecute(payload: { actionId: number; params: Record<string, any> }) {
  emit('executeAction', payload);
}
</script>

<template>
  <div class="windows-tab-panel">
    <WindowSidebar
      :windows="displayWindows"
      :selectedHwnd="selectedWindow?.hwnd ?? null"
      :isLoading="isLoading"
      @select="handleSelectWindow"
      @refresh="emit('refreshWindows'); searchResults = null"
      @search="handleSearch"
    />
    <ActionList
      :actions="actions"
      :selectedIndex="selectedTestActionIndex"
      @select="handleSelectTestAction"
    />
    <ActionTestPanel
      :selectedWindow="selectedWindow"
      :actions="actions"
      :isLoaded="isLoaded"
      :selectedActionIndex="selectedTestActionIndex"
      @execute="handleExecute"
    />
  </div>
</template>

<style scoped>
.windows-tab-panel {
  display: flex;
  flex-direction: row;
  height: 100%;
  overflow: hidden;
}

.windows-tab-panel > :first-child {
  flex: 0 0 35%;
  width: 35%;
}

.windows-tab-panel > :nth-child(2) {
  flex: 0 0 25%;
  width: 25%;
  min-width: 0;
}

.windows-tab-panel > :last-child {
  flex: 0 0 40%;
  width: 40%;
}
</style>