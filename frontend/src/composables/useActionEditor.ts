import { ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { Action, ActionItem, InputBackend } from '../types';
import { useLog } from './useLog';

const actions = ref<ActionItem[]>([]);
const originalActions = ref<ActionItem[]>([]);  // 原始数据快照
const originalDefaultBackend = ref<InputBackend>('win32');
const defaultBackend = ref<InputBackend>('win32');
const configPath = ref<string>('');
const isDirty = ref(false);
const selectedIndex = ref<number | null>(null);
const isLoaded = ref(false);
const targetWindow = ref<string | null>(null);
const executionBackend = ref<InputBackend>('win32');

// History count from backend
const historyCount = ref({ undo: 0, redo: 0 });

// Refresh history count from backend
async function refreshHistoryCount(): Promise<void> {
  try {
    const [undo, redo] = await invoke<[number, number]>('get_history_status');
    historyCount.value = { undo, redo };
  } catch {
    historyCount.value = { undo: 0, redo: 0 };
  }
}

// Refresh action list from backend
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
    isDirty.value = false;

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
    isDirty.value = false;

    // Save successful - update original snapshot and clear backend history
    originalActions.value = JSON.parse(JSON.stringify(actions.value));
    originalDefaultBackend.value = defaultBackend.value;
    await invoke('clear_action_history');
    await refreshHistoryCount();

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
    updateDirtyState();

    addLog(`已创建动作 #${index}: ${name || type}`, 'success');
    return index;
  } catch (e) {
    addLog(`创建动作失败: ${e}`, 'error');
    throw e;
  }
}

async function updateAction(index: number, action: Action, name?: string): Promise<void> {
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
    updateDirtyState();

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

    // Refresh state from backend
    await refreshActionList();
    await refreshHistoryCount();
    updateDirtyState();

    if (selectedIndex.value === index) {
      selectedIndex.value = null;
    }

    addLog(`已删除动作 #${index}`, 'success');
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
    updateDirtyState();
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
  isDirty.value = false;
  selectedIndex.value = null;
  isLoaded.value = false;
  targetWindow.value = null;
  executionBackend.value = 'win32';
}

// Check if a specific action has changed from original
function hasActionChanged(index: number): boolean {
  const original = originalActions.value.find(a => a.index === index);
  const current = actions.value.find(a => a.index === index);
  if (!original && !current) return false;
  if (!original || !current) return true;
  return JSON.stringify(original) !== JSON.stringify(current);
}

// Get list of changed action indices
function getChangedIndices(): number[] {
  return actions.value.filter(a => hasActionChanged(a.index)).map(a => a.index);
}

// Discard all changes and restore original state
function discardChanges(): void {
  actions.value = JSON.parse(JSON.stringify(originalActions.value));
  defaultBackend.value = originalDefaultBackend.value;
  isDirty.value = false;
}

// Discard changes for a specific action
function discardAction(index: number): void {
  const original = originalActions.value.find(a => a.index === index);

  // If action doesn't exist in original, nothing to discard
  if (!original) return;

  const idx = actions.value.findIndex(a => a.index === index);

  if (idx === -1) {
    // Action was deleted - restore it
    actions.value.push(JSON.parse(JSON.stringify(original)));
  } else {
    // Action was modified - restore original
    actions.value[idx] = JSON.parse(JSON.stringify(original));
  }

  updateDirtyState();
}

// Update isDirty based on actual changes
function updateDirtyState(): void {
  const backendChanged = defaultBackend.value !== originalDefaultBackend.value;
  const actionsChanged = actions.value.some(a => hasActionChanged(a.index));
  isDirty.value = backendChanged || actionsChanged;
}

async function executeActionWithParams(
  actionId: number,
  params: Record<string, any>
): Promise<void> {
  const { addLog } = useLog();

  try {
    await invoke('execute_action_with_params', {
      actionId,
      params,
    });
    addLog(`执行动作 #${actionId} 成功`, 'success');
  } catch (e) {
    addLog(`执行动作 #${actionId} 失败: ${e}`, 'error');
    throw e;
  }
}

async function setTargetWindow(window: string | null): Promise<void> {
  const { addLog } = useLog();

  try {
    await invoke('set_target_window', { window });
    // Fetch the actual stored HWND from backend
    const hwnd = await invoke<number | null>('get_target_window');
    targetWindow.value = hwnd !== null ? `id:${hwnd}` : null;
    addLog(`目标窗口已设置为: ${hwnd !== null ? `HWND ${hwnd}` : '(空)'}`, 'success');
  } catch (e) {
    addLog(`设置目标窗口失败: ${e}`, 'error');
    throw e;
  }
}

async function setExecutionBackend(backend: InputBackend): Promise<void> {
  const { addLog } = useLog();

  try {
    await invoke('set_execution_backend', { backend });
    executionBackend.value = backend;
    addLog(`执行后端已设置为: ${backend}`, 'success');
  } catch (e) {
    addLog(`设置执行后端失败: ${e}`, 'error');
    throw e;
  }
}

// Undo - restore previous state (calls backend)
async function undo(): Promise<void> {
  const { addLog } = useLog();

  try {
    await invoke('undo_action');
    await refreshActionList();
    await refreshHistoryCount();
    updateDirtyState();
    addLog(`撤销 (${historyCount.value.undo} 步可用)`, 'info');
  } catch (e) {
    addLog(`撤销失败: ${e}`, 'error');
  }
}

// Redo - restore next state (calls backend)
async function redo(): Promise<void> {
  const { addLog } = useLog();

  try {
    await invoke('redo_action');
    await refreshActionList();
    await refreshHistoryCount();
    updateDirtyState();
    addLog(`重做 (${historyCount.value.redo} 步可用)`, 'info');
  } catch (e) {
    addLog(`重做失败: ${e}`, 'error');
  }
}

// Check if undo is available (from backend)
function canUndoState(): boolean {
  return historyCount.value.undo > 0;
}

// Check if redo is available (from backend)
function canRedoState(): boolean {
  return historyCount.value.redo > 0;
}

// Helper to create default action based on type
function createDefaultAction(type: string): Action {
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
    return actions.value.find(a => a.index === selectedIndex.value) || null;
  });

  const hasChanges = computed(() => {
    return actions.value.some(a => hasActionChanged(a.index));
  });

  return {
    // State
    actions,
    originalActions,
    defaultBackend,
    configPath,
    isDirty,
    selectedIndex,
    isLoaded,
    targetWindow,
    executionBackend,

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
    executeActionWithParams,
    setTargetWindow,
    setExecutionBackend,

    // Change tracking
    hasActionChanged,
    getChangedIndices,
    discardChanges,
    discardAction,

    // Undo/Redo
    undo,
    redo,
    canUndo: canUndoState,
    canRedo: canRedoState,
  };
}
