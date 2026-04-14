import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { InputBackend } from '../types';
import { useLog } from './useLog';

const targetWindow = ref<string | null>(null);
const executionBackend = ref<InputBackend>('win32');

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
  }
}

export function useActionExecutor() {
  return {
    targetWindow,
    executionBackend,
    executeActionWithParams,
    setTargetWindow,
    setExecutionBackend,
  };
}
