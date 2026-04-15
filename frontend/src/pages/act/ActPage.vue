<script setup lang="ts">
import { ref } from 'vue';
import ActTabNav from '../../components/nav/ActTabNav.vue';
import ZmqMonitor from '../../components/ZmqMonitor.vue';
import ConfigPanel from './ConfigPanel.vue';
import WindowsTabPanel from './WindowsTabPanel.vue';
import { useWindows } from '../../composables/useWindows';
import { useActionEditor } from '../../composables/act/useActionEditor';
import { useActionHistory } from '../../composables/act/useActionHistory';
import { useActionExecutor } from '../../composables/act/useActionExecutor';
import type { InputBackend, SearchMode } from '../../types';

const activeTab = ref<'monitor' | 'config' | 'windows'>('monitor');

const {
  windows, selectedWindow, isLoading, refreshWindows, selectWindow,
  findWindowsByTitle, findWindowsByTitleContains,
  findWindowByClass, findWindowsByPid,
  findWindowsByExe, findWindowByHwnd, getWindowsInfoByHwnds
} = useWindows();

const actionEditor = useActionEditor();
const actionHistory = useActionHistory(() => actionEditor.refreshActionList());
const actionExecutor = useActionExecutor();

async function searchWindows(mode: SearchMode, value: string): Promise<number[]> {
  switch (mode) {
    case 'title':
      return findWindowsByTitle(value);
    case 'titleContains':
      return findWindowsByTitleContains(value);
    case 'class': {
      const hwnd = await findWindowByClass(value);
      return hwnd !== null ? [hwnd] : [];
    }
    case 'pid':
      return findWindowsByPid(parseInt(value));
    case 'exe':
      return findWindowsByExe(value);
    case 'hwnd': {
      const hwnd = await findWindowByHwnd(parseInt(value));
      return hwnd !== null ? [hwnd] : [];
    }
    default:
      return [];
  }
}

// Config actions
async function handleLoad(path: string) {
  await actionEditor.loadConfig(path);
}

async function handleSave() {
  await actionEditor.saveConfig();
}

async function handleSaveAs() {
  const path = prompt('输入新配置文件路径:', actionEditor.configPath.value || 'actions.json');
  if (path) {
    await actionEditor.saveConfig(path);
  }
}

function handleSelectAction(index: number | null) {
  actionEditor.selectAction(index);
}

async function handleUpdateAction(index: number, action: any) {
  await actionEditor.updateAction(index, action);
}

async function handleDeleteAction(index: number) {
  await actionEditor.deleteAction(index);
}

async function handleDiscardChanges() {
  await actionEditor.discardChanges();
}

async function handleUndo() {
  await actionHistory.undo();
}

async function handleRedo() {
  await actionHistory.redo();
}

function handleSetDefaultBackend(backend: InputBackend) {
  actionEditor.setDefaultBackend(backend);
}

function handleSetExecutionBackend(backend: InputBackend) {
  actionExecutor.setExecutionBackend(backend);
}

async function handleCreateAction(type: string, name: string | undefined) {
  const index = await actionEditor.createAction(type, name);
  actionEditor.selectAction(index);
}

// Window actions
async function handleSelectWindow(hwnd: number | null) {
  if (hwnd !== null) {
    await selectWindow(hwnd);
  }
  const windowSpec = hwnd !== null ? `id:${hwnd}` : null;
  await actionExecutor.setTargetWindow(windowSpec);
}

async function handleExecuteAction(payload: { actionId: number; params: Record<string, any> }) {
  await actionExecutor.executeActionWithParams(payload.actionId, payload.params);
}
</script>

<template>
  <div class="act-page">
    <ActTabNav v-model:activeTab="activeTab" />

    <div class="act-content">
      <ZmqMonitor v-if="activeTab === 'monitor'" />

      <ConfigPanel
        v-else-if="activeTab === 'config'"
        :actions="actionEditor.actions.value"
        :defaultBackend="actionEditor.defaultBackend.value"
        :configPath="actionEditor.configPath.value"
        :selectedIndex="actionEditor.selectedIndex.value"
        :isLoaded="actionEditor.isLoaded.value"
        :selectedAction="actionEditor.selectedAction.value"
        :hasChanges="actionEditor.hasChanges.value"
        :canUndo="actionHistory.canUndo.value"
        :canRedo="actionHistory.canRedo.value"
        @load="handleLoad"
        @save="handleSave"
        @saveAs="handleSaveAs"
        @selectAction="handleSelectAction"
        @updateAction="handleUpdateAction"
        @deleteAction="handleDeleteAction"
        @discardChanges="handleDiscardChanges"
        @undo="handleUndo"
        @redo="handleRedo"
        @setDefaultBackend="handleSetDefaultBackend"
        @setExecutionBackend="handleSetExecutionBackend"
        @createAction="handleCreateAction"
      />

      <WindowsTabPanel
        v-else-if="activeTab === 'windows'"
        :windows="windows"
        :selectedWindow="selectedWindow"
        :isLoading="isLoading"
        :actions="actionEditor.actions.value"
        :isLoaded="actionEditor.isLoaded.value"
        :searchWindows="searchWindows"
        :getWindowsInfoByHwnds="getWindowsInfoByHwnds"
        @selectWindow="handleSelectWindow"
        @refreshWindows="refreshWindows"
        @executeAction="handleExecuteAction"
      />
    </div>
  </div>
</template>

<style scoped>
.act-page {
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow: hidden;
}

.act-content {
  flex: 1;
  overflow: hidden;
}

.act-content > :deep(*) {
  height: 100%;
  overflow: auto;
}
</style>