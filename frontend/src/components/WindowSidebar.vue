<script setup lang="ts">
import { ref, computed } from 'vue';
import type { WindowInfo } from '../types';

const props = defineProps<{
  windows: WindowInfo[];
  selectedHwnd: number | null;
  isLoading: boolean;
}>();

const emit = defineEmits<{
  select: [hwnd: number];
  refresh: [];
  search: [mode: SearchMode, value: string];
}>();

type SearchMode = 'title' | 'titleContains' | 'class' | 'pid' | 'exe' | 'hwnd';

const searchMode = ref<SearchMode>('title');
const searchValue = ref('');

const filteredWindows = computed(() => {
  if (!searchValue.value.trim()) {
    return props.windows;
  }
  const query = searchValue.value.toLowerCase();
  return props.windows.filter(w =>
    w.title.toLowerCase().includes(query) ||
    w.processName.toLowerCase().includes(query) ||
    w.className.toLowerCase().includes(query)
  );
});

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
  <div class="window-sidebar">
    <div class="sidebar-header">
      <h3>窗口列表</h3>
      <button class="refresh-btn" @click="handleRefresh" :disabled="isLoading">
        {{ isLoading ? '加载中...' : '刷新' }}
      </button>
    </div>

    <!-- Search Mode Selector -->
    <div class="search-section">
      <div class="search-mode-select">
        <select v-model="searchMode" class="mode-select">
          <option value="title">标题(前缀)</option>
          <option value="titleContains">标题(包含)</option>
          <option value="class">类名</option>
          <option value="pid">进程ID</option>
          <option value="exe">进程名</option>
          <option value="hwnd">HWND</option>
        </select>
      </div>
      <div class="search-input-row">
        <input
          v-model="searchValue"
          :type="searchMode === 'pid' || searchMode === 'hwnd' ? 'number' : 'text'"
          :placeholder="searchMode === 'title' ? '窗口标题...' :
                        searchMode === 'titleContains' ? '窗口标题(包含)...' :
                        searchMode === 'class' ? '窗口类名...' :
                        searchMode === 'pid' ? '进程ID...' :
                        searchMode === 'exe' ? '进程名(如 notepad)...' :
                        'HWND值...'"
          class="search-input"
          @keyup.enter="handleSearch"
        />
        <button class="search-btn" @click="handleSearch">查找</button>
      </div>
    </div>

    <!-- Window List -->
    <div class="window-list">
      <div
        v-for="win in filteredWindows"
        :key="win.hwnd"
        class="window-item"
        :class="{ selected: win.hwnd === selectedHwnd }"
        @click="handleSelect(win.hwnd)"
      >
        <div class="window-title" :title="win.title">
          {{ truncateTitle(win.title) }}
        </div>
        <div class="window-meta">
          <span class="process-name">{{ win.processName }}</span>
          <span class="visibility" :class="{ visible: win.isVisible }">
            {{ win.isVisible ? '可见' : '隐藏' }}
          </span>
        </div>
        <div class="window-hwnd">HWND: 0x{{ win.hwnd.toString(16) }}</div>
      </div>

      <div v-if="filteredWindows.length === 0" class="empty-state">
        <template v-if="searchValue">未找到匹配的窗口</template>
        <template v-else>暂无窗口</template>
      </div>
    </div>
  </div>
</template>

<style scoped>
.window-sidebar {
  width: 280px;
  height: 100%;
  background: var(--color-surface);
  border-right: 1px solid var(--color-border);
  display: flex;
  flex-direction: column;
}

.sidebar-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  border-bottom: 1px solid var(--color-border);
}

.sidebar-header h3 {
  margin: 0;
  font-size: 14px;
  font-weight: 600;
}

.refresh-btn {
  padding: 4px 12px;
  font-size: 12px;
  background: var(--color-primary);
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.refresh-btn:hover:not(:disabled) {
  background: var(--color-primary-hover);
}

.refresh-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.search-section {
  padding: 8px 12px;
  border-bottom: 1px solid var(--color-border);
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.search-mode-select {
  display: flex;
}

.mode-select {
  width: 100%;
  padding: 6px 10px;
  font-size: 12px;
  background: var(--color-background);
  color: var(--color-text);
  border: 1px solid var(--color-border);
  border-radius: 4px;
}

.mode-select:focus {
  outline: none;
  border-color: var(--color-primary);
}

.search-input-row {
  display: flex;
  gap: 4px;
}

.search-input {
  flex: 1;
  padding: 6px 10px;
  font-size: 12px;
  background: var(--color-background);
  color: var(--color-text);
  border: 1px solid var(--color-border);
  border-radius: 4px;
}

.search-input:focus {
  outline: none;
  border-color: var(--color-primary);
}

.search-btn {
  padding: 6px 12px;
  font-size: 12px;
  background: var(--color-surface-secondary);
  color: var(--color-text);
  border: 1px solid var(--color-border);
  border-radius: 4px;
  cursor: pointer;
}

.search-btn:hover {
  background: var(--color-hover);
}

.window-list {
  flex: 1;
  overflow-y: auto;
  padding: 8px;
}

.window-list::-webkit-scrollbar {
  width: 6px;
}

.window-list::-webkit-scrollbar-track {
  background: var(--color-surface);
}

.window-list::-webkit-scrollbar-thumb {
  background: var(--color-border);
  border-radius: 3px;
}

.window-list::-webkit-scrollbar-thumb:hover {
  background: var(--color-text-muted);
}

.window-item {
  padding: 10px 12px;
  margin-bottom: 4px;
  border-radius: 6px;
  cursor: pointer;
  transition: background 0.15s;
}

.window-item:hover {
  background: var(--color-hover);
}

.window-item.selected {
  background: var(--color-primary-bg);
  border: 1px solid var(--color-primary);
}

.window-title {
  font-size: 13px;
  font-weight: 500;
  color: var(--color-text);
  margin-bottom: 4px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.window-meta {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 11px;
  color: var(--color-text-secondary);
  margin-bottom: 2px;
}

.process-name {
  background: var(--color-surface-secondary);
  padding: 2px 6px;
  border-radius: 3px;
}

.visibility {
  color: var(--color-text-muted);
}

.visibility.visible {
  color: var(--color-success);
}

.window-hwnd {
  font-size: 10px;
  color: var(--color-text-muted);
  font-family: monospace;
}

.empty-state {
  text-align: center;
  padding: 24px;
  color: var(--color-text-secondary);
  font-size: 13px;
}
</style>
