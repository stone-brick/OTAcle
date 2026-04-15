import { ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { ActionItem, InputBackend } from '../types';
import { useLog } from './useLog';
import { useActionHistory } from './useActionHistory';

const actions = ref<ActionItem[]>([]);
const originalActions = ref<ActionItem[]>([]);  // 原始数据快照
const originalDefaultBackend = ref<InputBackend>('win32');
const defaultBackend = ref<InputBackend>('win32');
const configPath = ref<string>('');
const selectedIndex = ref<number | null>(null);
const isLoaded = ref(false);

// 组合 useActionHistory
const { refreshHistoryCount } = useActionHistory();

// Refresh action list from backend (exported for use by undo/redo)
async function refreshActionList(): Promise<void> {
  const actionList = await invoke<ActionItem[]>('get_action_list');
  const backend = await invoke<InputBackend>('get_default_backend');
  actions.value = actionList;
  defaultBackend.value = backend;
}

async function loadConfig(path: string): Promise<void> {
  const { addLog } = useLog();

  try {
    await invoke('load_action_config', {
      path,
      backend: defaultBackend.value,
    });
    configPath.value = path;

    // Fetch the action list with names
    const actionList = await invoke<ActionItem[]>('get_action_list');
    actions.value = actionList;
    originalActions.value = JSON.parse(JSON.stringify(actionList));

    // Get default backend
    const backend = await invoke<InputBackend>('get_default_backend');
    defaultBackend.value = backend;
    originalDefaultBackend.value = backend;

    isLoaded.value = true;

    // Clear backend history on load
    await invoke('clear_action_history');
    await refreshHistoryCount();

    addLog(`已加载配置文件: ${path}`, 'success');
  } catch (e) {
    addLog(`加载配置文件失败: ${e}`, 'error');
    throw e;
  }
}

async function saveConfig(path?: string): Promise<void> {
  const { addLog } = useLog();
  const savePath = path || configPath.value;

  if (!savePath) {
    throw new Error('No config path specified');
  }

  try {
    await invoke('save_action_config', {
      path: savePath,
      defaultBackend: defaultBackend.value,
      actions: actions.value,
    });
    configPath.value = savePath;

    // Save successful - update original snapshot
    // Note: do NOT clear history here - saving should not destroy undo/redo capability
    originalActions.value = JSON.parse(JSON.stringify(actions.value));
    originalDefaultBackend.value = defaultBackend.value;
    // History is NOT cleared - user can still undo/redo after save

    addLog(`已保存配置文件: ${savePath}`, 'success');
  } catch (e) {
    addLog(`保存配置文件失败: ${e}`, 'error');
    throw e;
  }
}

async function createAction(type: string, name?: string): Promise<number> {
  const { addLog } = useLog();

  // Create a default action based on type
  const action = createDefaultAction(type);

  try {
    // Call backend to create action (which saves to history)
    const index = await invoke<number>('create_action', {
      action,
      name: name || null,
    });

    // Refresh state from backend
    await refreshActionList();
    await refreshHistoryCount();

    addLog(`已创建动作 #${index}: ${name || type}`, 'success');
    return index;
  } catch (e) {
    addLog(`创建动作失败: ${e}`, 'error');
    throw e;
  }
}

async function updateAction(index: number, action: ActionItem, name?: string): Promise<void> {
  const { addLog } = useLog();

  try {
    // Call backend to update action (which saves to history)
    await invoke('update_action', {
      index,
      action,
      name: name || null,
    });

    // Refresh state from backend
    await refreshActionList();
    await refreshHistoryCount();

    addLog(`已更新动作 #${index}`, 'success');
  } catch (e) {
    addLog(`更新动作失败: ${e}`, 'error');
    throw e;
  }
}

async function deleteAction(index: number): Promise<void> {
  const { addLog } = useLog();

  try {
    // Call backend to delete action (which saves to history)
    await invoke('delete_action', { index });

    // Refresh state from backend (indices are rebuilt by backend)
    await refreshActionList();
    await refreshHistoryCount();

    // Adjust selectedIndex if needed (may now be out of bounds)
    if (selectedIndex.value !== null && selectedIndex.value >= actions.value.length) {
      selectedIndex.value = actions.value.length > 0 ? actions.value.length - 1 : null;
    }

    addLog(`已删除动作`, 'success');
  } catch (e) {
    addLog(`删除动作失败: ${e}`, 'error');
    throw e;
  }
}

async function getNextIndex(): Promise<number> {
  return await invoke<number>('get_next_action_index');
}

function selectAction(index: number | null): void {
  selectedIndex.value = index;
}

async function setDefaultBackend(backend: InputBackend): Promise<void> {
  const { addLog } = useLog();

  try {
    // Call backend to set default backend (which saves to history)
    await invoke('set_default_backend', { backend });
    defaultBackend.value = backend;
    await refreshHistoryCount();
  } catch (e) {
    addLog(`设置默认后端失败: ${e}`, 'error');
  }
}

function clearEditor(): void {
  actions.value = [];
  originalActions.value = [];
  originalDefaultBackend.value = 'win32';
  defaultBackend.value = 'win32';
  configPath.value = '';
  selectedIndex.value = null;
  isLoaded.value = false;
}

// Check if a specific action has changed from original (by array position)
function hasActionChanged(index: number): boolean {
  const original = originalActions.value[index];
  const current = actions.value[index];
  if (!original && !current) return false;
  if (!original || !current) return true;
  return JSON.stringify(original) !== JSON.stringify(current);
}

// Get list of changed action indices (array positions)
function getChangedIndices(): number[] {
  return actions.value
    .map((_, idx) => idx)
    .filter(idx => hasActionChanged(idx));
}

// Sync originalActions snapshot to current state (call after undo/redo)
function syncOriginalActions(): void {
  originalActions.value = JSON.parse(JSON.stringify(actions.value));
  originalDefaultBackend.value = defaultBackend.value;
}

// Discard all changes - restore to original state (calls backend for undo/redo support)
async function discardChanges(): Promise<void> {
  await invoke('discard_changes');
  await refreshActionList();
  await refreshHistoryCount();
  syncOriginalActions();
}

// Discard changes for a specific action (by array position) - calls backend for undo/redo support
async function discardAction(index: number): Promise<void> {
  await invoke('discard_action', { index });
  await refreshActionList();
  await refreshHistoryCount();
  syncOriginalActions();
}

// Helper to create default action based on type
function createDefaultAction(type: string): ActionItem {
  switch (type) {
    case 'key':
      return { type: 'key', key: 'a' };
    case 'key_sequence':
      return { type: 'key_sequence', keys: [{ key: 'a', hold_time_ms: 5 }], default_interval_ms: 5 };
    case 'mouse_click':
      return { type: 'mouse_click', button: 'left', count: 1 };
    case 'mouse_move':
      return { type: 'mouse_move', x: 0, y: 0 };
    case 'mouse_scroll':
      return { type: 'mouse_scroll', direction: 'up', amount: 1 };
    case 'delay':
      return { type: 'delay', duration_ms: 100 };
    case 'text':
      return { type: 'text', content: 'text' };
    default:
      throw new Error(`Unknown action type: ${type}`);
  }
}

export function useActionEditor() {
  const selectedAction = computed(() => {
    if (selectedIndex.value === null) return null;
    return actions.value[selectedIndex.value] || null;
  });

  const hasChanges = computed(() => {
    const backendChanged = defaultBackend.value !== originalDefaultBackend.value;
    // Check length changes (deletions)
    if (actions.value.length !== originalActions.value.length) return true;
    // Check each action for modifications
    const actionsChanged = actions.value.some((_, idx) => hasActionChanged(idx));
    return backendChanged || actionsChanged;
  });

  return {
    // State
    actions,
    originalActions,
    defaultBackend,
    configPath,
    selectedIndex,
    isLoaded,

    // Computed
    selectedAction,
    hasChanges,

    // Actions
    loadConfig,
    saveConfig,
    createAction,
    updateAction,
    deleteAction,
    getNextIndex,
    selectAction,
    setDefaultBackend,
    clearEditor,
    refreshActionList,

    // Change tracking
    hasActionChanged,
    getChangedIndices,
    discardChanges,
    discardAction,
    syncOriginalActions,
  };
}
