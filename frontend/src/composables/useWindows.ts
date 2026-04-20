import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { WindowInfo } from '../types';
import { useLog } from './useLog';

const windows = ref<WindowInfo[]>([]);
const selectedWindow = ref<WindowInfo | null>(null);
const isLoading = ref(false);

// ============================================================================
// 窗口列表操作
// ============================================================================

async function refreshWindows(): Promise<void> {
  isLoading.value = true;
  const { addLog } = useLog();

  try {
    const result = await invoke<WindowInfo[]>('window_list');
    windows.value = result;
    addLog(`窗口列表已刷新，共 ${result.length} 个窗口`, 'success');
  } catch (e) {
    addLog(`刷新窗口列表失败: ${e}`, 'error');
  } finally {
    isLoading.value = false;
  }
}

async function selectWindow(hwnd: number): Promise<void> {
  const { addLog } = useLog();

  try {
    const info = await invoke<WindowInfo>('window_get_info', { hwnd });
    selectedWindow.value = info;
    addLog(`已选择窗口: ${info.title}`, 'info');
  } catch (e) {
    addLog(`选择窗口失败: ${e}`, 'error');
  }
}

// ============================================================================
// 专用窗口搜索函数
// ============================================================================

/**
 * 按窗口标题查找所有窗口（前缀匹配）
 */
async function findWindowsByTitle(title: string): Promise<number[]> {
  const { addLog } = useLog();
  try {
    const results = await invoke<number[]>('window_find_by_title', { title });
    addLog(`按标题查找 "${title}"：找到 ${results.length} 个窗口`, 'info');
    return results;
  } catch (e) {
    addLog(`按标题查找失败: ${e}`, 'error');
    throw e;
  }
}

/**
 * 按窗口标题查找所有窗口（包含匹配）
 */
async function findWindowsByTitleContains(title: string): Promise<number[]> {
  const { addLog } = useLog();
  try {
    const results = await invoke<number[]>('window_find_one_contains', { title });
    addLog(`按标题包含查找 "${title}"：找到 ${results.length} 个窗口`, 'info');
    return results;
  } catch (e) {
    addLog(`按标题包含查找失败: ${e}`, 'error');
    throw e;
  }
}

/**
 * 按精确类名查找窗口
 */
async function findWindowByClass(className: string): Promise<number | null> {
  const { addLog } = useLog();
  try {
    const result = await invoke<number | null>('window_find_by_class', { className });
    if (result !== null) {
      addLog(`按类名查找 "${className}"：找到 HWND ${result}`, 'info');
    } else {
      addLog(`按类名查找 "${className}"：未找到`, 'info');
    }
    return result;
  } catch (e) {
    addLog(`按类名查找失败: ${e}`, 'error');
    throw e;
  }
}

/**
 * 按进程 ID 查找所有窗口
 */
async function findWindowsByPid(pid: number): Promise<number[]> {
  const { addLog } = useLog();
  try {
    const results = await invoke<number[]>('window_find_by_pid', { pid });
    addLog(`按进程ID查找 ${pid}：找到 ${results.length} 个窗口`, 'info');
    return results;
  } catch (e) {
    addLog(`按进程ID查找失败: ${e}`, 'error');
    throw e;
  }
}

/**
 * 按可执行文件名查找所有窗口
 */
async function findWindowsByExe(processName: string): Promise<number[]> {
  const { addLog } = useLog();
  try {
    const results = await invoke<number[]>('window_find_by_exe', { processName });
    addLog(`按进程名查找 "${processName}"：找到 ${results.length} 个窗口`, 'info');
    return results;
  } catch (e) {
    addLog(`按进程名查找失败: ${e}`, 'error');
    throw e;
  }
}

/**
 * 按精确 HWND 查找窗口
 */
async function findWindowByHwnd(hwnd: number): Promise<number | null> {
  const { addLog } = useLog();
  try {
    const result = await invoke<number | null>('window_find_by_hwnd', { hwnd });
    if (result !== null) {
      addLog(`按HWND查找 ${hwnd}：找到窗口`, 'info');
    } else {
      addLog(`按HWND查找 ${hwnd}：未找到`, 'info');
    }
    return result;
  } catch (e) {
    addLog(`按HWND查找失败: ${e}`, 'error');
    throw e;
  }
}

/**
 * 获取多个 HWND 的 WindowInfo
 */
async function getWindowsInfoByHwnds(hwnds: number[]): Promise<WindowInfo[]> {
  const results = await Promise.all(
    hwnds.map(hwnd => invoke<WindowInfo | null>('window_get_info', { hwnd }))
  );
  return results.filter((info): info is WindowInfo => info !== null);
}

export function useWindows() {
  return {
    windows,
    selectedWindow,
    isLoading,
    refreshWindows,
    selectWindow,
    // 专用搜索函数
    findWindowsByTitle,
    findWindowsByTitleContains,
    findWindowByClass,
    findWindowsByPid,
    findWindowsByExe,
    findWindowByHwnd,
    getWindowsInfoByHwnds,
  };
}