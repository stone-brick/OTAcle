import { ref, computed } from 'vue';
import type { ActionItem, InputBackend } from '../../types';
import { useLog } from '../useLog';
import { useComm } from '../useComm';
import { useActionHistory } from './useActionHistory';
import { useProjectEvents, type ProjectEvent } from '../useProjectEvents';
import { useProject } from '../useProject';

const actions = ref<ActionItem[]>([]);
const baselineActions = ref<ActionItem[]>([]);  // 基准线快照
const baselineDefaultBackend = ref<InputBackend>('win32');
const defaultBackend = ref<InputBackend>('win32');
const configPath = ref<string>('');
const selectedIndex = ref<number | null>(null);
const isLoaded = ref(false);

// 组合 useActionHistory
const { refreshHistoryCount } = useActionHistory();

// 项目事件监听器清理函数
let unsubscribeProject: (() => void) | null = null;

// 从后端刷新动作列表（导出供 undo/redo 使用）
async function refreshActionList(): Promise<void> {
  const comm = useComm();
  const actionList = await comm.actGetList();
  const backend = await comm.actGetDefaultBackend();
  actions.value = actionList;
  defaultBackend.value = backend;
}

async function loadConfig(path: string): Promise<void> {
  const { addLog } = useLog();
  const comm = useComm();

  try {
    await comm.actLoadConfig(path, defaultBackend.value);
    configPath.value = path;

    // 获取带名称的动作列表
    const actionList = await comm.actGetList();
    actions.value = actionList;
    baselineActions.value = JSON.parse(JSON.stringify(actionList));

    // 获取默认后端
    const backend = await comm.actGetDefaultBackend();
    defaultBackend.value = backend;
    baselineDefaultBackend.value = backend;

    isLoaded.value = true;

    // 加载时清除后端历史
    await comm.actClearHistory();
    await refreshHistoryCount();

    addLog(`Act 配置已加载: ${path}`, 'success', 'action');
  } catch {
    // 配置文件不存在时创建默认配置
    actions.value = [];
    baselineActions.value = [];
    defaultBackend.value = 'win32';
    baselineDefaultBackend.value = 'win32';
    isLoaded.value = true;
    configPath.value = path;

    // 保存默认配置
    await comm.actSaveConfig(path, defaultBackend.value, []);
    addLog(`已创建默认动作配置文件: ${path}`, 'info', 'action');
  }
}

async function saveConfig(path?: string): Promise<void> {
  const { addLog } = useLog();
  const comm = useComm();
  const savePath = path || configPath.value;

  if (!savePath) {
    throw new Error('No config path specified');
  }

  try {
    await comm.actSaveConfig(savePath, defaultBackend.value, actions.value);
    configPath.value = savePath;

    // 保存成功 - 更新原始快照
    // 注意：这里不清除历史 - 保存不应破坏撤销/重做能力
    baselineActions.value = JSON.parse(JSON.stringify(actions.value));
    baselineDefaultBackend.value = defaultBackend.value;
    // 历史不会被清除 - 用户保存后仍可以撤销/重做

    addLog(`已保存配置文件: ${savePath}`, 'success', 'action');
  } catch (e) {
    addLog(`保存配置文件失败: ${e}`, 'error', 'action');
    throw e;
  }
}

async function createAction(type: string, name?: string): Promise<number> {
  const { addLog } = useLog();
  const comm = useComm();

  // 根据类型创建默认动作
  const action = createDefaultAction(type);

  try {
    // 调用后端创建动作（会保存到历史）
    const index = await comm.actCreate(action, name || null);

    // Refresh state from backend
    await refreshActionList();
    await refreshHistoryCount();

    addLog(`已创建动作 #${index}: ${name || type}`, 'success', 'action');
    return index;
  } catch (e) {
    addLog(`创建动作失败: ${e}`, 'error', 'action');
    throw e;
  }
}

async function updateAction(index: number, action: ActionItem, name?: string): Promise<void> {
  const { addLog } = useLog();
  const comm = useComm();

  try {
    // 调用后端更新动作（会保存到历史）
    await comm.actUpdate(index, action, name || null);

    // 从后端刷新状态
    await refreshActionList();
    await refreshHistoryCount();

    addLog(`已更新动作 #${index}`, 'success', 'action');
  } catch (e) {
    addLog(`更新动作失败: ${e}`, 'error', 'action');
    throw e;
  }
}

async function deleteAction(index: number): Promise<void> {
  const { addLog } = useLog();
  const comm = useComm();

  try {
    // 调用后端删除动作（会保存到历史）
    await comm.actDelete(index);

    // 从后端刷新状态（索引由后端重建）
    await refreshActionList();
    await refreshHistoryCount();

    // 必要时调整 selectedIndex（可能已超出范围）
    if (selectedIndex.value !== null && selectedIndex.value >= actions.value.length) {
      selectedIndex.value = actions.value.length > 0 ? actions.value.length - 1 : null;
    }

    addLog(`已删除动作`, 'success', 'action');
  } catch (e) {
    addLog(`删除动作失败: ${e}`, 'error', 'action');
    throw e;
  }
}

async function getNextIndex(): Promise<number> {
  const comm = useComm();
  return await comm.actGetNextIndex();
}

function selectAction(index: number | null): void {
  selectedIndex.value = index;
}

async function setDefaultBackend(backend: InputBackend): Promise<void> {
  const { addLog } = useLog();
  const comm = useComm();

  try {
    // 调用后端设置默认后端（会保存到历史）
    await comm.actSetDefaultBackend(backend);
    defaultBackend.value = backend;
    await refreshHistoryCount();
  } catch (e) {
    addLog(`设置默认后端失败: ${e}`, 'error', 'action');
  }
}

function clearEditor(): void {
  actions.value = [];
  baselineActions.value = [];
  baselineDefaultBackend.value = 'win32';
  defaultBackend.value = 'win32';
  configPath.value = '';
  selectedIndex.value = null;
  isLoaded.value = false;
}

// 检查特定动作是否已从基准线更改（按数组位置）
function hasActionChanged(index: number): boolean {
  const baseline = baselineActions.value[index];
  const current = actions.value[index];
  if (!baseline && !current) return false;
  if (!baseline || !current) return true;
  return JSON.stringify(baseline) !== JSON.stringify(current);
}

// 获取已更改动作的索引列表（数组位置）
function getChangedIndices(): number[] {
  return actions.value
    .map((_, idx) => idx)
    .filter(idx => hasActionChanged(idx));
}

// 将 baselineActions 快照同步到当前状态（undo/redo 后调用）
function syncBaselineActions(): void {
  baselineActions.value = JSON.parse(JSON.stringify(actions.value));
  baselineDefaultBackend.value = defaultBackend.value;
}

// 丢弃所有更改 - 恢复到原始状态（调用后端以支持撤销/重做）
async function discardChanges(): Promise<void> {
  const { addLog } = useLog();
  const comm = useComm();
  await comm.actDiscardAll();
  await refreshActionList();
  await refreshHistoryCount();
  syncBaselineActions();
  addLog('已丢弃所有未保存的更改', 'info', 'action')
}

// 初始化项目事件监听（项目打开时自动加载配置）
function initProjectEventListener(): void {
  if (unsubscribeProject) {
    unsubscribeProject();
  }

  const { onProjectEvent } = useProjectEvents();
  const { getProjectActionsConfigPath } = useProject();

  unsubscribeProject = onProjectEvent(async (event: ProjectEvent) => {
    if (event.type === 'opened' && event.project) {
      const configPath = await getProjectActionsConfigPath();
      if (configPath) {
        try {
          await loadConfig(configPath);
        } catch {
          // 静默忽略（loadConfig 内部已处理）
        }
      }
    } else if (event.type === 'closed') {
      clearEditor();
    }
  });
}

// 根据类型创建默认动作的帮助函数
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
    const backendChanged = defaultBackend.value !== baselineDefaultBackend.value;
    // Check length changes (deletions)
    if (actions.value.length !== baselineActions.value.length) return true;
    // Check each action for modifications
    const actionsChanged = actions.value.some((_, idx) => hasActionChanged(idx));
    return backendChanged || actionsChanged;
  });

  return {
    // State
    actions,
    baselineActions,
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
    initProjectEventListener,

    // Change tracking
    hasActionChanged,
    getChangedIndices,
    discardChanges,
    syncBaselineActions,
  };
}
