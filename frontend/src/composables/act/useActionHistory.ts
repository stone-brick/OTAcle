import { ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useLog } from '../useLog';

const historyCount = ref({ undo: 0, redo: 0 });

// 从后端刷新历史计数
async function refreshHistoryCount(): Promise<void> {
  try {
    const [undo, redo] = await invoke<[number, number]>('act_get_history_status');
    historyCount.value = { undo, redo };
  } catch {
    historyCount.value = { undo: 0, redo: 0 };
  }
}

export function useActionHistory(onMutated?: () => Promise<void>) {
  // 撤销 - 恢复之前的状态（调用后端）
  async function undo(): Promise<void> {
    const { addLog } = useLog();

    try {
      await invoke('act_undo');
      await refreshHistoryCount();
      addLog(`撤销 (${historyCount.value.undo} 步可用)`, 'info');
      await onMutated?.();
    } catch (e) {
      addLog(`撤销失败: ${e}`, 'error');
    }
  }

  // 重做 - 恢复下一个状态（调用后端）
  async function redo(): Promise<void> {
    const { addLog } = useLog();

    try {
      await invoke('act_redo');
      await refreshHistoryCount();
      addLog(`重做 (${historyCount.value.redo} 步可用)`, 'info');
      await onMutated?.();
    } catch (e) {
      addLog(`重做失败: ${e}`, 'error');
    }
  }

  // 检查撤销是否可用
  const canUndo = computed(() => historyCount.value.undo > 0);

  // 检查重做是否可用
  const canRedo = computed(() => historyCount.value.redo > 0);

  return {
    historyCount,
    refreshHistoryCount,
    undo,
    redo,
    canUndo,
    canRedo,
  };
}
