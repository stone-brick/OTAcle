//! 全局快捷键管理
//!
//! 使用 tauri-plugin-global-shortcut 实现一键启动/停止所有连接

import { ref } from 'vue'
import { register, unregister } from '@tauri-apps/plugin-global-shortcut'
import { invoke } from '@tauri-apps/api/core'
import { useLog } from './useLog'

const isShortcutRegistered = ref(false)

export function useGlobalShortcut() {
  const { addLog } = useLog()

  const SHORTCUT = 'CommandOrControl+Shift+S'

  async function toggleAll() {
    try {
      const [actRunning, thinkRunning, observeRunning] = await invoke<[boolean, boolean, boolean]>('comm_get_toggle_status')

      if (actRunning || thinkRunning || observeRunning) {
        // 正在运行，停止
        await invoke('comm_stop_pull')
        await invoke('think_stop')
        await invoke('observe_stop')
        addLog('快捷键：已停止所有连接', 'info', 'comm')
      } else {
        // 未运行，全部启动
        await invoke('comm_start_pull')
        await invoke('think_start')

        // 检查是否可以启动 Observe
        const [shouldObserve, window, configJson] = await invoke<[boolean, string, string | null]>('comm_check_observe_start')
        if (shouldObserve && window) {
          const config = configJson ? JSON.parse(configJson) : { capture: { frame_rate: 3, target_width: 640, target_height: 480 }, crop_regions: [] }
          await invoke('observe_start', { window, config })
        }

        addLog('快捷键：已启动所有连接', 'info', 'comm')
      }
    } catch (e) {
      addLog(`快捷键切换失败: ${e}`, 'error', 'comm')
    }
  }

  async function registerShortcut() {
    try {
      await register(SHORTCUT, (event) => {
        if (event.state === 'Pressed') {
          toggleAll()
        }
      })
      isShortcutRegistered.value = true
      addLog(`快捷键已注册: ${SHORTCUT}`, 'info', 'comm')
    } catch (e) {
      addLog(`注册快捷键失败: ${e}`, 'error', 'comm')
    }
  }

  async function unregisterShortcut() {
    try {
      await unregister(SHORTCUT)
      isShortcutRegistered.value = false
      addLog('快捷键已取消注册', 'info', 'comm')
    } catch (e) {
      addLog(`取消注册快捷键失败: ${e}`, 'error', 'comm')
    }
  }

  return {
    SHORTCUT,
    isShortcutRegistered,
    registerShortcut,
    unregisterShortcut,
    toggleAll,
  }
}