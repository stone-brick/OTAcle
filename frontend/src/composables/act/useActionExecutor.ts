import { invoke } from '@tauri-apps/api/core';
import { useLog } from '../useLog';

async function executeActionWithParams(
  actionIdx: number,
  params: Record<string, any>
): Promise<void> {
  const { addLog } = useLog();

  try {
    await invoke('act_execute_action_with_params', {
      actionIdx,
      params,
    });
    addLog(`执行动作 #${actionIdx} 成功`, 'success');
  } catch (e) {
    addLog(`执行动作 #${actionIdx} 失败: ${e}`, 'error');
    throw e;
  }
}

async function setTargetWindow(window: string | null): Promise<void> {
  const { addLog } = useLog();

  try {
    await invoke('window_set_target', { window });
    addLog(`目标窗口已设置为: ${window ?? '(空)'}`, 'success');
  } catch (e) {
    addLog(`设置目标窗口失败: ${e}`, 'error');
  }
}

export function useActionExecutor() {
  return {
    executeActionWithParams,
    setTargetWindow,
  };
}
