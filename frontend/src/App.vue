<script setup lang="ts">
import { ref, computed } from 'vue';
import type { AppStatus, MouseButton, InputBackend } from './types';
import { useWindows } from './composables/useWindows';
import { useInput } from './composables/useInput';
import { useActionConfig } from './composables/useActionConfig';
import { useLog } from './composables/useLog';

import StatusBar from './components/StatusBar.vue';
import WindowSidebar from './components/WindowSidebar.vue';
import KeypadGrid from './components/KeypadGrid.vue';
import MouseControl from './components/MouseControl.vue';
import ActionConfig from './components/ActionConfig.vue';
import LogPanel from './components/LogPanel.vue';
import ObservePanel from './components/ObservePanel.vue';

// Composables
const {
  windows, selectedWindow, isLoading,
  refreshWindows, selectWindow,
  findFirstWindowByTitle, findFirstWindowByTitleContains,
  findWindowByClass, findWindowsByPid, findWindowsByExe, findWindowByHwnd
} = useWindows();
const { backend, sendKey, sendText, sendMouseClick, sendMouseMove, getMousePosition, sendCombinationKey } = useInput();
const { config, configPath, currentBackend, loadConfig, executeAction, executeActionRange, setBackend } = useActionConfig();
const { logs, addLog, clearLogs } = useLog();

// UI State
const status = ref<AppStatus>('ready');
const activeTab = ref<'action' | 'config' | 'observe'>('action');

// Computed
const selectedHwnd = computed(() => selectedWindow.value?.hwnd ?? null);

// Handlers
async function handleKeyPress(key: string) {
  status.value = 'sending';
  try {
    await sendKey(key, 'click', selectedWindow.value?.title ?? undefined);
  } catch {
    status.value = 'error';
    return;
  }
  status.value = 'ready';
}

async function handleComboPress(keys: string[]) {
  status.value = 'sending';
  try {
    await sendCombinationKey(keys, selectedWindow.value?.title ?? undefined);
  } catch {
    status.value = 'error';
    return;
  }
  status.value = 'ready';
}

async function handleTextSubmit(text: string) {
  if (!text.trim()) return;
  status.value = 'sending';
  try {
    await sendText(text, selectedWindow.value?.title ?? undefined);
  } catch {
    status.value = 'error';
    return;
  }
  status.value = 'ready';
}

async function handleMouseClick(x: number, y: number, button: MouseButton) {
  status.value = 'sending';
  try {
    await sendMouseClick(x, y, button, selectedHwnd.value ?? undefined);
  } catch {
    status.value = 'error';
    return;
  }
  status.value = 'ready';
}

async function handleMouseMove(x: number, y: number) {
  status.value = 'sending';
  try {
    await sendMouseMove(x, y, selectedHwnd.value ?? undefined);
  } catch {
    status.value = 'error';
    return;
  }
  status.value = 'ready';
}

async function handleGetMousePosition() {
  try {
    const [x, y] = await getMousePosition();
    // Update mouse control component's coordinates via emit
    mouseX.value = x;
    mouseY.value = y;
  } catch {
    // Error already logged
  }
}

// Mouse control local state
const mouseX = ref(0);
const mouseY = ref(0);

// Tab content components expose these
const textInput = ref('');

async function handleTextSend() {
  if (!textInput.value.trim()) return;
  await handleTextSubmit(textInput.value);
  textInput.value = '';
}

async function handleActionConfigLoad(path: string) {
  status.value = 'sending';
  try {
    await loadConfig(path);
  } catch {
    status.value = 'error';
    return;
  }
  status.value = 'ready';
}

async function handleActionExecute(id: number) {
  status.value = 'sending';
  try {
    await executeAction(id, selectedWindow.value?.title ?? undefined);
  } catch {
    status.value = 'error';
    return;
  }
  status.value = 'ready';
}

async function handleActionRange(start: number, end: number, interval: number) {
  status.value = 'sending';
  try {
    await executeActionRange(start, end, interval, selectedWindow.value?.title ?? undefined);
  } catch {
    status.value = 'error';
    return;
  }
  status.value = 'ready';
}

function handleBackendChange(backend: InputBackend) {
  setBackend(backend);
}

function handleClearLogs() {
  clearLogs();
}

function handleWindowSelect(hwnd: number) {
  selectWindow(hwnd);
}

function handleWindowRefresh() {
  refreshWindows();
}

async function handleWindowSearch(mode: string, value: string) {
  status.value = 'sending';
  let hwnd: number | null = null;

  try {
    switch (mode) {
      case 'title':
        hwnd = await findFirstWindowByTitle(value);
        break;
      case 'titleContains':
        hwnd = await findFirstWindowByTitleContains(value);
        break;
      case 'class':
        hwnd = await findWindowByClass(value);
        break;
      case 'pid':
        const results = await findWindowsByPid(Number(value));
        if (results.length > 0) {
          hwnd = results[0];
        }
        break;
      case 'exe':
        const exeResults = await findWindowsByExe(value);
        if (exeResults.length > 0) {
          hwnd = exeResults[0];
        }
        break;
      case 'hwnd':
        hwnd = await findWindowByHwnd(Number(value));
        break;
    }

    if (hwnd !== null) {
      await selectWindow(hwnd);
    } else {
      addLog(`未找到窗口`, 'info');
    }
  } catch {
    status.value = 'error';
    return;
  }
  status.value = 'ready';
}

// Initialize
refreshWindows();
</script>

<template>
  <div class="app-container">
    <!-- Status Bar -->
    <StatusBar :status="status">
      <span class="backend-badge" :class="backend">
        {{ backend === 'win32' ? 'Win32' : 'Enigo' }}
      </span>
      <span v-if="selectedWindow" class="selected-window">
        {{ selectedWindow.title.substring(0, 25) }}{{ selectedWindow.title.length > 25 ? '...' : '' }}
      </span>
    </StatusBar>

    <!-- Main Layout -->
    <div class="main-layout">
      <!-- Window Sidebar -->
      <WindowSidebar
        :windows="windows"
        :selectedHwnd="selectedHwnd"
        :isLoading="isLoading"
        @select="handleWindowSelect"
        @refresh="handleWindowRefresh"
        @search="handleWindowSearch"
      />

      <!-- Content Area -->
      <div class="content-area">
        <!-- Tab Bar -->
        <div class="tab-bar">
          <button
            class="tab-btn"
            :class="{ active: activeTab === 'action' }"
            @click="activeTab = 'action'"
          >
            动作面板
          </button>
          <button
            class="tab-btn"
            :class="{ active: activeTab === 'config' }"
            @click="activeTab = 'config'"
          >
            动作配置
          </button>
          <button
            class="tab-btn"
            :class="{ active: activeTab === 'observe' }"
            @click="activeTab = 'observe'"
          >
            观察区
          </button>
        </div>

        <!-- Tab Content -->
        <div class="tab-content">
          <!-- Action Panel Tab -->
          <div v-if="activeTab === 'action'" class="action-panel">
            <!-- Left Column: Keypad and Text -->
            <div class="panel-column">
              <KeypadGrid
                @keyPress="handleKeyPress"
                @comboPress="handleComboPress"
              />

              <div class="text-input-section">
                <h4>文本输入</h4>
                <div class="text-input-row">
                  <input
                    v-model="textInput"
                    type="text"
                    placeholder="输入文本..."
                    class="text-input"
                    @keyup.enter="handleTextSend"
                  />
                  <button class="send-btn" @click="handleTextSend">发送</button>
                </div>
              </div>
            </div>

            <!-- Right Column: Mouse Control -->
            <div class="panel-column">
              <MouseControl
                :selectedHwnd="selectedHwnd"
                @click="handleMouseClick"
                @move="handleMouseMove"
                @getPosition="handleGetMousePosition"
              />
            </div>
          </div>

          <!-- Config Tab -->
          <ActionConfig
            v-if="activeTab === 'config'"
            :config="config"
            :configPath="configPath"
            :currentBackend="currentBackend"
            :selectedHwnd="selectedHwnd"
            @load="handleActionConfigLoad"
            @execute="handleActionExecute"
            @executeRange="handleActionRange"
            @setBackend="handleBackendChange"
          />

          <!-- Observe Tab -->
          <ObservePanel v-if="activeTab === 'observe'" />
        </div>
      </div>
    </div>

    <!-- Log Panel -->
    <LogPanel :logs="logs" @clear="handleClearLogs" />
  </div>
</template>

<style scoped>
.app-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background: var(--color-background);
}

.main-layout {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.content-area {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.tab-bar {
  display: flex;
  gap: 4px;
  padding: 8px 16px;
  background: var(--color-surface);
  border-bottom: 1px solid var(--color-border);
}

.tab-btn {
  padding: 8px 20px;
  font-size: 13px;
  font-weight: 500;
  background: transparent;
  color: var(--color-text-secondary);
  border: 1px solid transparent;
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: all 0.15s;
}

.tab-btn:hover {
  background: var(--color-hover);
  color: var(--color-text);
}

.tab-btn.active {
  background: var(--color-primary-bg);
  color: var(--color-primary);
  border-color: var(--color-primary);
}

.tab-content {
  flex: 1;
  overflow: hidden;
}

.action-panel {
  display: flex;
  height: 100%;
  overflow: auto;
}

.panel-column {
  flex: 1;
  overflow: auto;
}

.panel-column:first-child {
  border-right: 1px solid var(--color-border);
}

.text-input-section {
  padding: 16px;
  border-top: 1px solid var(--color-border);
}

.text-input-section h4 {
  margin: 0 0 10px 0;
  font-size: 12px;
  font-weight: 600;
  color: var(--color-text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.text-input-row {
  display: flex;
  gap: 8px;
}

.text-input {
  flex: 1;
  padding: 10px 14px;
  font-size: 14px;
  background: var(--color-background);
  color: var(--color-text);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
}

.text-input:focus {
  outline: none;
  border-color: var(--color-primary);
}

.send-btn {
  padding: 10px 24px;
  font-size: 14px;
  font-weight: 500;
  background: var(--color-primary);
  color: white;
  border: none;
  border-radius: var(--radius-md);
  cursor: pointer;
}

.send-btn:hover {
  background: var(--color-primary-hover);
}

.backend-badge {
  padding: 2px 8px;
  font-size: 10px;
  font-weight: 600;
  border-radius: 3px;
  text-transform: uppercase;
}

.backend-badge.win32 {
  background: #3b82f6;
  color: white;
}

.backend-badge.enigo {
  background: #8b5cf6;
  color: white;
}

.selected-window {
  font-size: 12px;
  color: var(--color-text-muted);
  max-width: 200px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
</style>
