import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useLog } from '../useLog';

const isExecuting = ref(false);

export function useActionExecutor() {
  const { addLog } = useLog();

  async function executeActionWithParams(
    actionIdx: number,
    params: Record<string, number | string>
  ): Promise<void> {
    isExecuting.value = true;
    try {
      await invoke('act_execute_action_with_params', {
        actionIdx,
        params,
      });
      addLog(`执行动作 #${actionIdx} 成功`, 'success', 'action');
    } catch (e) {
      addLog(`执行动作 #${actionIdx} 失败: ${e}`, 'error', 'action');
      throw e;
    } finally {
      isExecuting.value = false;
    }
  }

  async function setTargetWindow(window: string | null): Promise<void> {
    try {
      await invoke('window_set_target', { window });
      addLog(`目标窗口已设置为: ${window ?? '(空)'}`, 'success', 'action');
    } catch (e) {
      addLog(`设置目标窗口失败: ${e}`, 'error', 'action');
    }
  }

  return {
    isExecuting,
    executeActionWithParams,
    setTargetWindow,
  };
}
