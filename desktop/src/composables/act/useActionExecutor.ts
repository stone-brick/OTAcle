import { ref } from 'vue';
import { useLog } from '../useLog';
import { useComm } from '../useComm';

const isExecuting = ref(false);

export function useActionExecutor() {
  const { addLog } = useLog();
  const comm = useComm();

  async function executeActionWithParams(
    actionIdx: number,
    params: Record<string, number | string>
  ): Promise<void> {
    isExecuting.value = true;
    try {
      await comm.actExecuteActionWithParams(actionIdx, params);
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
      await comm.windowSetTarget(window);
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
