import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { ActionConfig, InputBackend } from '../types';
import { useLog } from './useLog';

const config = ref<ActionConfig | null>(null);
const configPath = ref<string>('');
const currentBackend = ref<InputBackend>('win32');
const isLoaded = ref(false);

async function loadConfig(path: string): Promise<void> {
  const { addLog } = useLog();

  try {
    await invoke('load_action_config', {
      path,
      backend: currentBackend.value,
    });
    configPath.value = path;
    // Fetch the config to display the action list
    const result = await invoke<ActionConfig>('get_config');
    config.value = result;
    isLoaded.value = true;
    addLog(`已加载配置文件: ${path}`, 'success');
  } catch (e) {
    addLog(`加载配置文件失败: ${e}`, 'error');
    throw e;
  }
}

async function executeAction(actionId: number, window?: string): Promise<void> {
  const { addLog } = useLog();

  try {
    await invoke('execute_action', {
      actionId,
      window,
      backend: currentBackend.value,
    });
    addLog(`执行动作 #${actionId} 成功`, 'success');
  } catch (e) {
    addLog(`执行动作 #${actionId} 失败: ${e}`, 'error');
    throw e;
  }
}

async function executeActionRange(
  startId: number,
  endId: number,
  intervalMs: number = 100,
  window?: string
): Promise<void> {
  const { addLog } = useLog();

  for (let id = startId; id <= endId; id++) {
    try {
      await executeAction(id, window);
      if (id < endId && intervalMs > 0) {
        await new Promise(resolve => setTimeout(resolve, intervalMs));
      }
    } catch (e) {
      addLog(`执行动作 #${id} 失败: ${e}`, 'error');
      break;
    }
  }
}

function setBackend(newBackend: InputBackend): void {
  currentBackend.value = newBackend;
}

function clearConfig(): void {
  config.value = null;
  configPath.value = '';
  isLoaded.value = false;
}

export function useActionConfig() {
  return {
    config,
    configPath,
    currentBackend,
    isLoaded,
    loadConfig,
    executeAction,
    executeActionRange,
    setBackend,
    clearConfig,
  };
}
