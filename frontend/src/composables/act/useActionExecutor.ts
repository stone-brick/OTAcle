import { invoke } from '@tauri-apps/api/core';
import type { InputBackend } from '../../types';
import { useLog } from '../useLog';

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
    addLog(`目标窗口已设置为: ${window ?? '(空)'}`, 'success');
  } catch (e) {
    addLog(`设置目标窗口失败: ${e}`, 'error');
  }
}

async function setExecutionBackend(backend: InputBackend): Promise<void> {
  const { addLog } = useLog();

  try {
    await invoke('set_execution_backend', { backend });
    addLog(`执行后端已设置为: ${backend}`, 'success');
  } catch (e) {
    addLog(`设置执行后端失败: ${e}`, 'error');
  }
}

export function useActionExecutor() {
  return {
    executeActionWithParams,
    setTargetWindow,
    setExecutionBackend,
  };
}
