import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { InputBackend, MouseButton } from '../types';
import { useLog } from './useLog';

const backend = ref<InputBackend>('win32');

async function sendKey(key: string, direction: string = 'click', window?: string): Promise<void> {
  const { addLog } = useLog();

  try {
    await invoke('send_key', {
      key,
      direction,
      window,
    });
    addLog(`发送按键: ${key} (${direction})`, 'success');
  } catch (e) {
    addLog(`发送按键失败: ${e}`, 'error');
    throw e;
  }
}

async function sendText(text: string, window?: string): Promise<void> {
  const { addLog } = useLog();

  try {
    await invoke('send_text', {
      text,
      window,
    });
    addLog(`发送文本: ${text.substring(0, 20)}${text.length > 20 ? '...' : ''}`, 'success');
  } catch (e) {
    addLog(`发送文本失败: ${e}`, 'error');
    throw e;
  }
}

async function sendMouseClick(
  x: number,
  y: number,
  button: MouseButton = 'left',
  hwnd?: number
): Promise<void> {
  const { addLog } = useLog();

  try {
    await invoke('send_mouse_click', {
      hwnd,
      x,
      y,
      button,
      backend: backend.value,
    });
    addLog(`鼠标点击: (${x}, ${y}) ${button}键`, 'success');
  } catch (e) {
    addLog(`鼠标点击失败: ${e}`, 'error');
    throw e;
  }
}

async function sendMouseMove(x: number, y: number, hwnd?: number): Promise<void> {
  const { addLog } = useLog();

  try {
    await invoke('send_mouse_move', {
      hwnd,
      x,
      y,
      backend: backend.value,
    });
    addLog(`鼠标移动: (${x}, ${y})`, 'success');
  } catch (e) {
    addLog(`鼠标移动失败: ${e}`, 'error');
    throw e;
  }
}

async function getMousePosition(): Promise<[number, number]> {
  const { addLog } = useLog();

  try {
    const result = await invoke<[number, number]>('get_mouse_position');
    addLog(`当前鼠标位置: (${result[0]}, ${result[1]})`, 'info');
    return result;
  } catch (e) {
    addLog(`获取鼠标位置失败: ${e}`, 'error');
    throw e;
  }
}

async function sendCombinationKey(keys: string[], window?: string): Promise<void> {
  const { addLog } = useLog();

  try {
    await invoke('send_combination_key', {
      keys,
      window,
      backend: backend.value,
    });
    addLog(`发送组合键: ${keys.join('+')}`, 'success');
  } catch (e) {
    addLog(`发送组合键失败: ${e}`, 'error');
    throw e;
  }
}

async function activateWindow(hwnd: number): Promise<void> {
  const { addLog } = useLog();

  try {
    await invoke('activate_window', { hwnd });
    addLog(`已激活窗口 HWND: ${hwnd}`, 'success');
  } catch (e) {
    addLog(`激活窗口失败: ${e}`, 'error');
    throw e;
  }
}

export function useInput() {
  return {
    backend,
    sendKey,
    sendText,
    sendMouseClick,
    sendMouseMove,
    getMousePosition,
    sendCombinationKey,
    activateWindow,
  };
}
