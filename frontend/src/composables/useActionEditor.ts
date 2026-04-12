import { ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { Action, ActionItem, InputBackend } from '../types';
import { useLog } from './useLog';

const actions = ref<ActionItem[]>([]);
const defaultBackend = ref<InputBackend>('win32');
const configPath = ref<string>('');
const isDirty = ref(false);
const selectedIndex = ref<number | null>(null);
const isLoaded = ref(false);

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

    // Get default backend
    const backend = await invoke<InputBackend>('get_default_backend');
    defaultBackend.value = backend;

    isLoaded.value = true;
    isDirty.value = false;
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
    const index = await invoke<number>('create_action', {
      action,
      name: name || null,
    });

    // Refresh the action list
    const actionList = await invoke<ActionItem[]>('get_action_list');
    actions.value = actionList;
    isDirty.value = true;
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
    await invoke('update_action', {
      index,
      action,
      name: name || null,
    });

    // Refresh the action list
    const actionList = await invoke<ActionItem[]>('get_action_list');
    actions.value = actionList;
    isDirty.value = true;
    addLog(`已更新动作 #${index}`, 'success');
  } catch (e) {
    addLog(`更新动作失败: ${e}`, 'error');
    throw e;
  }
}

async function deleteAction(index: number): Promise<void> {
  const { addLog } = useLog();

  try {
    await invoke('delete_action', {
      index,
    });

    // Refresh the action list
    const actionList = await invoke<ActionItem[]>('get_action_list');
    actions.value = actionList;
    isDirty.value = true;

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

function setDefaultBackend(backend: InputBackend): void {
  defaultBackend.value = backend;
  isDirty.value = true;
}

function clearEditor(): void {
  actions.value = [];
  defaultBackend.value = 'win32';
  configPath.value = '';
  isDirty.value = false;
  selectedIndex.value = null;
  isLoaded.value = false;
}

// Helper to create default action based on type
function createDefaultAction(type: string): Action {
  switch (type) {
    case 'key':
      return { type: 'key', key: '' };
    case 'key_sequence':
      return { type: 'key_sequence', keys: [], default_interval_ms: 5 };
    case 'mouse_click':
      return { type: 'mouse_click', button: 'left', count: '1' };
    case 'mouse_move':
      return { type: 'mouse_move', x: '0', y: '0' };
    case 'mouse_scroll':
      return { type: 'mouse_scroll', direction: 'up', amount: 1 };
    case 'delay':
      return { type: 'delay', duration_ms: 100 };
    case 'text':
      return { type: 'text', content: '' };
    default:
      throw new Error(`Unknown action type: ${type}`);
  }
}

export function useActionEditor() {
  const selectedAction = computed(() => {
    if (selectedIndex.value === null) return null;
    return actions.value.find(a => a.index === selectedIndex.value) || null;
  });

  return {
    // State
    actions,
    defaultBackend,
    configPath,
    isDirty,
    selectedIndex,
    isLoaded,

    // Computed
    selectedAction,

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
  };
}
