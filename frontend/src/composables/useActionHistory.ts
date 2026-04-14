import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useLog } from './useLog';

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

// Undo - restore previous state (calls backend)
async function undo(): Promise<void> {
  const { addLog } = useLog();

  try {
    await invoke('undo_action');
    await refreshHistoryCount();
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
    await refreshHistoryCount();
    addLog(`重做 (${historyCount.value.redo} 步可用)`, 'info');
  } catch (e) {
    addLog(`重做失败: ${e}`, 'error');
  }
}

// Check if undo is available
function canUndo(): boolean {
  return historyCount.value.undo > 0;
}

// Check if redo is available
function canRedo(): boolean {
  return historyCount.value.redo > 0;
}

export function useActionHistory() {
  return {
    historyCount,
    refreshHistoryCount,
    undo,
    redo,
    canUndo,
    canRedo,
  };
}
